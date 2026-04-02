use std::ops::Add;

use sysinfo::System;
use tauri::Emitter;

#[derive(Clone, serde::Serialize, Debug)]
pub struct Payload {
    pub ram_capacity: u64,
    pub ram_used: u64,
    pub cpu_usage: f32,
}

pub fn listen_to_ram(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let mut system = System::new_all();
    // Clone the AppHandle so we can move it into the background thread
    let handle = app.handle().clone();
    // System::

    let mut counter = Payload {
        ram_capacity: system.total_memory(),
        ram_used: system.used_memory(),
        cpu_usage: system.cpus()[0].cpu_usage(),
    };

    // Background thread that fires every second
    std::thread::spawn(move || loop {
        system.refresh_memory();
        system.refresh_cpu_usage();

        counter.ram_used = system.used_memory();
        counter.cpu_usage = system.cpus()[0].cpu_usage();

        let cpu_duration = sysinfo::MINIMUM_CPU_UPDATE_INTERVAL;
        let sleep_duration = std::time::Duration::from_millis(0);

        std::thread::sleep(cpu_duration.add(sleep_duration));

        match handle.emit("system-info-update", &counter) {
            Ok(_) => {}
            Err(_) => {}
        }
    });

    Ok(())
}
