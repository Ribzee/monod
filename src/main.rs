use std::{fs, path::Path, thread::sleep, time::Duration};

use sensors::Sensors;
use sysinfo::{Components, System};

fn main() {
    let cpu_watt = fs::read_to_string("/sys/class/powercap/intel-rapl:0/energy_uj")
        .unwrap_or_else(|err| err.to_string());

    println!("{}", cpu_watt);

    let mut sys = System::new_all();

    //  loop {
    //     sys.refresh_cpu_all();
    //     let total: Vec<u64> = sys.cpus().iter().map(|cpu| cpu.frequency()).collect();
    //
    //        let clock = total.clone().iter().sum::<u64>() as f32 / total.len() as f32;

    //    let clock = total[0];
    //    println!("{}", clock);
    //
    //     sleep(Duration::from_millis(500));
    //  }
}
