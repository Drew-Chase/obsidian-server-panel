use std::env::current_dir;
use actix_web::{get, web, HttpResponse, Responder};
use futures_util::Stream;
use serde_json::json;
use std::pin::Pin;
use std::sync::Mutex;
use std::time::Duration;
use actix_web_lab::sse;
use sysinfo::{Disks, System};
use tokio::time::interval;

#[get("")]
pub async fn get_system_info() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "name": System::name(),
        "kernel_version": System::kernel_version(),
        "os_version": System::os_version(),
        "host_name": System::host_name(),
    }))
}

#[get("/usage")]
pub async fn get_system_usage(sys: actix_web::web::Data<Mutex<System>>) -> impl Responder {
    let mut sys = match sys.lock() {
        Ok(sys) => sys,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    sys.refresh_all(); // Refresh all system info
    let mut per_core_cpu_usage: Vec<f32> = vec![];

    for cpu in sys.cpus() {
        per_core_cpu_usage.push(cpu.cpu_usage());
    }

    HttpResponse::Ok().json(json!({
        "cpu_usage": sys.global_cpu_usage(),
        "cores": per_core_cpu_usage,
        "memory": {
            "total": sys.total_memory(),
            "used": sys.used_memory(),
            "free": sys.free_memory(),
            "swap_total": sys.total_swap(),
            "swap_used": sys.used_swap(),
            "swap_free": sys.free_swap()
        }
    }))
}



#[get("/usage/sse")]
pub async fn get_system_usage_sse() -> impl Responder {
    let (sender, receiver) = tokio::sync::mpsc::channel(2);

    actix_web::rt::spawn(async move {
        let mut sys = System::new_all();
        let mut ticker = interval(Duration::from_secs(1));

        loop {
            // Refresh all system info
            sys.refresh_all();

            // Collect per-core CPU usage
            let mut per_core_cpu_usage: Vec<f32> = vec![];
            for cpu in sys.cpus() {
                per_core_cpu_usage.push(cpu.cpu_usage());
            }

            // Create JSON payload with system usage data
            let json = json!({
                "cpu_usage": sys.global_cpu_usage(),
                "cores": per_core_cpu_usage,
                "memory": {
                    "total": sys.total_memory(),
                    "used": sys.used_memory(),
                    "free": sys.free_memory(),
                    "swap_total": sys.total_swap(),
                    "swap_used": sys.used_swap(),
                    "swap_free": sys.free_swap()
                }
            });

            // Send system usage data
            let msg = sse::Data::new(serde_json::to_string(&json).unwrap()).event("system_usage");

            if sender.send(msg.into()).await.is_err() {
                break;
            }

            // Wait for the next tick
            ticker.tick().await;
        }
    });

    sse::Sse::from_infallible_receiver(receiver).with_keep_alive(Duration::from_secs(3))
}

#[get("/storage")]
pub async fn get_storage_info() -> impl Responder {
    let disks_list = Disks::new_with_refreshed_list();
    let mut disks = vec![];
    let mut current_drive: String = "".to_string();
    let mut current_drive_mount_point: String = "".to_string();
    let mut contiguous_characters = 0;
    for disk in disks_list.iter() {
        let match_chars = common_prefix_length(
            current_dir().unwrap().to_str().unwrap(),
            disk.mount_point().to_str().unwrap(),
        );
        contiguous_characters = if match_chars > contiguous_characters {
            current_drive = disk.name().to_str().unwrap().to_string();
            current_drive_mount_point = disk.mount_point().to_str().unwrap().to_string();
            match_chars
        } else {
            contiguous_characters
        };
        disks.push(json!({
			"name": disk.name().to_str().unwrap(),
			"file_system": disk.file_system().to_str().unwrap(),
			"mount_point": disk.mount_point().to_str().unwrap(),
			"total_space": disk.total_space(),
			"available_space": disk.available_space(),
			"used_space": disk.total_space() - disk.available_space(),
			"percentage_used": ((disk.total_space() as f64) - (disk.available_space() as f64)) / (disk.total_space() as f64) * 100.0
		}));
    }

    HttpResponse::Ok().json(json!({
        "current_drive": {
            "name": current_drive,
            "mount_point": current_drive_mount_point
        },
        "disks": disks
    }))
}

fn common_prefix_length(s1: &str, s2: &str) -> usize {
    s1.chars()
        .zip(s2.chars())
        .take_while(|(c1, c2)| c1 == c2)
        .count()
}
