use std::{fs, path::Path, thread::sleep, time::Duration};

use sysinfo::{Components, System};

fn main() {
    let cpu_watt = fs::read_to_string("/sys/class/powercap/intel-rapl:0/energy_uj")
        .unwrap_or_else(|err| err.to_string());

    println!("{}", cpu_watt);

    let test = machine_info::Machine::new();

    let gpu = test.graphics_status();

    let usage = gpu.first().unwrap();

    println!("{}", usage.gpu);

    let result = fs::read_to_string("/proc/loadavg").unwrap_or("0.0 0.0 0.0".to_string());

    let avg = result
        .split_whitespace()
        .map(|str| str.parse::<f32>().unwrap_or(0.0))
        .collect::<Vec<f32>>();

    println!("{}; {:?}", result, avg)
}
