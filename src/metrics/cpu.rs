use std::{
    fs,
    time::{Duration, Instant},
};

use sysinfo::{Components, System};

pub fn get_cpu_info(system: &mut System) -> (f32, f32, f32, u64) {
    system.refresh_cpu_all();

    let usage = system.global_cpu_usage();

    let clock = system
        .cpus()
        .iter()
        .map(|cpu| cpu.frequency())
        .collect::<Vec<u64>>()[0];

    let components = Components::new_with_refreshed_list();
    let mut temp = 0.0;

    for component in &components {
        if component.label() == "coretemp Package id 0" {
            temp = component.temperature().unwrap_or(0.0);
            break;
        }
    }

    (usage, temp, 0.0, clock)
}

pub fn get_cores_info(system: &mut System) -> Vec<f32> {
    system
        .cpus()
        .iter()
        .map(|cpu| cpu.cpu_usage())
        .collect::<Vec<f32>>()
}

pub fn get_load_avg() -> Vec<f32> {
    let result = fs::read_to_string("/proc/loadavg").unwrap_or("0.0 0.0 0.0".to_string());

    let avg = result
        .split_whitespace()
        .map(|str| str.parse::<f32>().unwrap_or(0.0))
        .collect::<Vec<f32>>();

    avg.first_chunk::<3>()
        .unwrap_or(&[0.0f32, 0.0f32, 0.0f32])
        .to_vec()
}

pub fn get_sys_uptime() -> Instant {
    let result = fs::read_to_string("/proc/uptime").unwrap_or("0.0 0.0".to_string());

    let uptime = result
        .split_whitespace()
        .map(|str| str.parse::<u64>().unwrap_or(0))
        .collect::<Vec<u64>>()
        .first()
        .unwrap_or(&0)
        .to_owned();

    Instant::now()
        .checked_sub(Duration::from_secs(uptime))
        .unwrap_or(Instant::now())
}
