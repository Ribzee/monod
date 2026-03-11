use machine_info::Machine;

pub fn get_gpu_info() -> (u32, u64, u32, f32) {
    let machine = Machine::new().graphics_status();
    let gpu = machine.first().unwrap();

    let usage = gpu.gpu;
    let mem = gpu.memory_used;
    let temp = gpu.temperature;

    (usage, mem, temp, 0.0)
}

pub fn get_gpu_name() -> String {
    getgpuname::get_gpu_name()
        .unwrap_or("unknown".to_string())
        .split(&['[', ']'])
        .nth(1)
        .unwrap_or("unknown")
        .to_string()
}

pub fn get_gpu_mem() -> u64 {
    Machine::new()
        .system_info()
        .graphics
        .first()
        .unwrap()
        .memory
}
