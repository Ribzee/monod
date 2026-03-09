use machine_info::Machine;

pub fn get_gpu_info() -> (u32, u64, u32, f32) {
    let machine = Machine::new().graphics_status();
    let gpu = machine.first().unwrap();

    let usage = gpu.gpu;
    let mem = gpu.memory_used;
    let temp = gpu.temperature;

    (usage, mem, temp, 0.0)
}
