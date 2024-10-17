use actix_web::{get, rt, web, HttpResponse, Responder};
use actix_ws::AggregatedMessage;
use futures_util::StreamExt;
use serde_json::json;
use std::env::current_dir;
use std::sync::Mutex;
use sysinfo::{Disks, System};
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

#[get("/usage/ws")]
pub async fn get_system_usage_websocket(
    req: actix_web::HttpRequest,
    stream: web::Payload,
) -> Result<impl Responder, actix_web::Error> {
    let (res, mut session, stream) = actix_ws::handle(&req, stream)?;

    let mut sys = System::new_all();
    let mut stream = stream
        .aggregate_continuations()
        .max_continuation_size(2_usize.pow(20)); // 1MB

    rt::spawn(async move {
        sys.refresh_all(); // Refresh all system info
        let mut per_core_cpu_usage: Vec<f32> = vec![];

        for cpu in sys.cpus() {
            per_core_cpu_usage.push(cpu.cpu_usage());
        }

        //        let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));
        while let Some(msg) = stream.next().await {
            sys.refresh_all(); // Refresh all system info
            let mut per_core_cpu_usage: Vec<f32> = vec![];

            for cpu in sys.cpus() {
                per_core_cpu_usage.push(cpu.cpu_usage());
            }

            match msg {
                Ok(AggregatedMessage::Text(text)) => {
                    session.text(text).await.unwrap();
                }
                Ok(AggregatedMessage::Binary(_)) => {
                    session.close(None).await.unwrap();
                    break;
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    break;
                }

                _ => {}
            }
        }
    });

    Ok(res)
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
