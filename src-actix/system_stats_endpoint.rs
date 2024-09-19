use actix_web::{get, HttpResponse, Responder};
use serde_json::json;
use std::env::current_exe;
use std::sync::Mutex;
use sysinfo::{Disks, System};
#[get("/")]
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

	//	let mut sys = System::new_all();
	sys.refresh_all(); // Refresh all system info
	let mut per_core_cpu_usage: Vec<f32> = vec![];

	for cpu in sys.cpus() {
		per_core_cpu_usage.push(cpu.cpu_usage());
	}

	println!("CPU Usage: {}%", sys.global_cpu_usage());
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

#[get("/storage")]
pub async fn get_storage_info() -> impl Responder {
	let disks_list = Disks::new_with_refreshed_list();
	let mut disks = vec![];
	let mut current_drive: String = "".to_string();
	for disk in disks_list.iter() {
		let is_current_drive = current_exe()
			.unwrap()
			.parent()
			.unwrap()
			.starts_with(disk.mount_point());
		if is_current_drive {
			current_drive = disk.name().to_str().unwrap().to_string();
		}
		disks.push(json!({
			"name": disk.name().to_str().unwrap(),
			"current_drive": is_current_drive,
			"file_system": disk.file_system().to_str().unwrap(),
			"mount_point": disk.mount_point().to_str().unwrap(),
			"total_space": disk.total_space(),
			"available_space": disk.available_space(),
			"used_space": disk.total_space() - disk.available_space(),
			"percentage_used": ((disk.total_space() as f64) - (disk.available_space() as f64)) / (disk.total_space() as f64) * 100.0
		}));
	}

	HttpResponse::Ok().json(json!({
        "current_drive": current_drive,
        "disks": disks
    }))
}
