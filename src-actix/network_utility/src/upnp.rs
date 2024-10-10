use easy_upnp::PortMappingProtocol::TCP;
use easy_upnp::UpnpConfig;
use lazy_static::lazy_static;
use log::{debug, error, trace};
use scheduler::schedule::{EqualsId, Schedule};
use scheduler::{add_schedule, duration, remove_schedule};
use std::sync::Mutex;

#[derive(Clone, Debug)]
pub struct OpenPort {
    id: u64,
    pub port: u16,
    pub description: String,
}
pub struct OpenPorts {
    pub ports: Vec<OpenPort>,
    is_renewal_thread_running: bool,
}

impl Default for OpenPorts {
    fn default() -> Self {
        Self::new()
    }
}

lazy_static! {
    pub static ref UPNP_SINGLETON: Mutex<OpenPorts> = Mutex::new(OpenPorts::new());
}

const REFRESH_DURATION: u64 = 60 * 5; // 5 minutes

impl OpenPorts {
    pub fn new() -> OpenPorts {
        OpenPorts {
            ports: Vec::new(),
            is_renewal_thread_running: false,
        }
    }
    pub fn add_port(&mut self, port: u16, description: String) {
        let id = add_schedule!(
            duration::Duration::from_minutes(5),
            true,
            true,
            |schedule| {
                debug!("Refreshing UPNP port {}", port.clone());

                for item in easy_upnp::add_ports([UpnpConfig {
                    address: None,
                    port: port.clone(),
                    comment: description,
                    protocol: TCP,
                    duration: REFRESH_DURATION as u32,
                }]) {
                    match item {
                        Ok(_) => trace!("port {} opened!", port),
                        Err(e) => error!("Failed to forward port: {}", e),
                    }
                }
            }
        );

        self.ports.push(OpenPort {
            id,
            port,
            description: description.clone(),
        });
        if !self.is_renewal_thread_running {
            self.open_renewal_thread();
        }
    }

    pub fn clear_ports(&mut self) {
        let ports = self.ports.clone();
        for port in ports.iter() {
            self.remove_port(port.port);
        }
    }

    pub fn remove_port(&mut self, port: u16) {
        let port_id = self.ports.iter().find(|x| x.port == port).unwrap().id;
        remove_schedule!(|s: &Schedule| s.equals_id(port_id));

        self.ports.retain(|x| x.port != port);
        for item in easy_upnp::delete_ports([UpnpConfig {
            address: None,
            port,
            comment: "".to_string(),
            protocol: TCP,
            duration: 0,
        }]) {
            match item {
                Ok(_) => trace!("port {} removed!", port),
                Err(e) => error!("Failed to remove port: {}", e),
            }
        }
    }

    pub fn get_ports(&self) -> Vec<OpenPort> {
        self.ports.clone()
    }

    fn create_upnp_config(&self) -> Vec<UpnpConfig> {
        self.ports
            .iter()
            .map(|x| UpnpConfig {
                address: None,
                port: x.port,
                comment: x.description.clone(),
                protocol: TCP,
                duration: REFRESH_DURATION as u32,
            })
            .collect()
    }

    fn open_renewal_thread(&mut self) {
        self.is_renewal_thread_running = true;
        std::thread::spawn(|| loop {
            let singleton = UPNP_SINGLETON.lock().unwrap();
            let configs = singleton.create_upnp_config();
            debug!(
                "Refreshing {} ports: {:?}",
                configs.len(),
                singleton.get_ports()
            );
            for item in easy_upnp::add_ports(configs) {
                match item {
                    Ok(_) => trace!("port refreshed!"),
                    Err(e) => error!("Failed to forward port: {}", e),
                }
            }
            std::thread::sleep(std::time::Duration::from_secs(REFRESH_DURATION));
        });
    }
}

#[macro_export]
macro_rules! open_port {
    ($port:expr, $description:expr) => {{
        let mut singleton = $crate::upnp::UPNP_SINGLETON.lock().unwrap();
        singleton.add_port($port, $description.to_string());
    }};
}

#[macro_export]
macro_rules! close_all_ports {
    () => {{
        let mut singleton = $crate::upnp::UPNP_SINGLETON.lock().unwrap();
        singleton.clear_ports();
    }};
}

#[macro_export]
macro_rules! close_port {
    ($port:expr) => {{
        let mut singleton = $crate::upnp::UPNP_SINGLETON.lock().unwrap();
        singleton.remove_port($port);
    }};
}
