use std::{collections::HashMap, time::Instant};

use sysinfo::System;

use crate::metrics::{cpu, disk, gpu, memory, network};

#[derive(Debug, Clone)]
pub struct SystemState {
    pub cpu_usage: f32,
    pub cpu_temp: f32,
    pub cpu_watt: f32,
    pub cpu_clock: u64,
    pub core_usage: Vec<f32>,
    pub load_avg: (f32, f32, f32),
    pub gpu_usage: u32,
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
    pub net_top: HashMap<String, (u64, u64)>, // top speed per sec
}

impl SystemState {
    pub fn new() -> Self {
        Self {
            cpu_usage: 0.0,
            cpu_temp: 0.0,
            cpu_watt: 0.0,
            cpu_clock: 0,
            core_usage: Vec::new(),
            load_avg: (0.0, 0.0, 0.0),
            gpu_usage: 0,
            gpu_mem: 0,
            gpu_temp: 0,
            gpu_watt: 0.0,
            mem_usage: 0,
            mem_percentage: 0.0,
            swap_usage: 0,
            swap_percentage: 0.0,
            disk_usage: HashMap::new(),
            disk_io: HashMap::new(),
            net_usage: HashMap::new(),
            net_total: HashMap::new(),
            net_top: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub uptime: Instant,
    pub cpu_model: String,
    pub gpu_model: String,
    pub gpu_total_mem: u64,
    pub total_mem: u64,
    pub total_swap: u64,
}

#[derive(Debug)]
pub struct SystemMonitor {
    pub system: System,
    pub system_state: SystemState,
    pub system_info: SystemInfo,
}

impl SystemMonitor {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();

        let system_info = SystemMonitor::update_system_info(&system);

        Self {
            system,
            system_state: SystemState::new(),
            system_info,
        }
    }

    pub fn update_system_info(system: &System) -> SystemInfo {
        SystemInfo {
            uptime: cpu::get_sys_uptime(),
            cpu_model: cpu::get_cpu_name(),
            gpu_model: gpu::get_gpu_name(),
            gpu_total_mem: gpu::get_gpu_mem(),
            total_mem: system.total_memory(),
            total_swap: system.total_swap(),
        }
    }

    pub fn update_state(&mut self, rate: &f32) {
        let (cpu_usage, cpu_temp, cpu_watt, cpu_clock) = cpu::get_cpu_info(&mut self.system);
        let (gpu_usage, gpu_mem, gpu_temp, gpu_watt) = gpu::get_gpu_info();

        let net_usage = network::get_network_usage(&self.system_state.net_total, rate);
        let net_total = network::get_network_total();

        // Take highest between old top usage and current for each interface
        let net_top = net_usage
            .clone()
            .iter()
            .map(|(k, v)| {
                let old_value = self
                    .system_state
                    .net_top
                    .get(k)
                    .unwrap_or(&(0, 0))
                    .to_owned();

                (k.clone(), (v.0.max(old_value.0), v.1.max(old_value.1)))
            })
            .collect::<HashMap<String, (u64, u64)>>();

        let (mem_usage, mem_percentage) = memory::get_memory_usage(&mut self.system);
        let (swap_usage, swap_percentage) = memory::get_swap_usage(&self.system);

        self.system_state = SystemState {
            cpu_usage,
            cpu_temp,
            cpu_watt,
            cpu_clock,
            core_usage: cpu::get_cores_info(&mut self.system),
            load_avg: cpu::get_load_avg(),
            gpu_usage,
            gpu_mem,
            gpu_temp,
            gpu_watt,
            mem_usage,
            mem_percentage,
            swap_usage,
            swap_percentage,
            disk_usage: disk::get_disk_usage(),
            disk_io: disk::get_disk_io(),
            net_usage,
            net_total,
            net_top,
        }
    }
}
