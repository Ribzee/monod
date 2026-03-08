use std::collections::HashMap;

use sysinfo::System;

#[derive(Debug, Clone)]
pub struct SystemState {
    pub cpu_usage: f32,
    pub cpu_temp: u32,
    pub cpu_watt: f32,
    pub cpu_clock: f32,
    pub core_usage: HashMap<usize, f32>,
    pub core_temp: HashMap<usize, u32>,
    pub gpu_usage: f32,
    pub gpu_mem: u64,
    pub gpu_temp: u32,
    pub gpu_watt: f32,
    pub mem_usage: u64,
    pub mem_percentage: f32,
    pub swap_usage: u64,
    pub swap_percentage: f32,
    pub disk_usage: HashMap<String, (u64, u64)>, // disk usage (used, available)
    pub disk_io: HashMap<String, (u64, u64)>,    //disk usage per disk (read, write) in percentage
    pub net_usage: HashMap<String, (u64, u64)>,  // download, upload per interface
    pub net_total: HashMap<String, (u64, u64)>,
}
pub struct SystemMonitor {
    pub system: System,
}

impl SystemMonitor {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        Self { system }
    }
}
