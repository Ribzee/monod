use std::{collections::HashMap, fs, path::Path, thread::sleep, time::Duration};

use machine_info::Machine;
use monod::metrics::gpu;
use sysinfo::{Components, System};

fn main() {
    let result = fs::read_to_string("/proc/cpuinfo").unwrap_or("model name : unkown".to_string());

    let cpu = result
        .lines()
        .filter(|line| line.starts_with("model name"))
        .next()
        .unwrap_or("model name : unknown")
        .split(':')
        .nth(1)
        .unwrap_or("unknown")
        .trim()
        .to_string();

    println!("{}", cpu);

    println!(
        "{}",
        gpu::get_gpu_name().split(&['[', ']']).nth(1).unwrap_or("g")
    );

    let test = Machine::new()
        .system_info()
        .graphics
        .first()
        .unwrap()
        .memory;

    println!("{}", test);

    let net_usage = HashMap::from([("a", (5, 5)), ("d", (2, 2))]);

    let mut net_total = HashMap::from([("a", (3, 4))]);

    // To whoever will take the time to read this next piece of code: Sorry.  But for short it adds
    // net_usage to net_total
    for key in net_usage.keys() {
        net_total
            .entry(key)
            .and_modify(|e| {
                let (down, up) = net_usage.get(key).unwrap();
                *e = (e.0 + down, e.1 + up)
            })
            .or_insert(net_usage.get(key).unwrap().clone());
    }

    println!("{:?}", net_total);
}
