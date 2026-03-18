use amdgpu_sysfs::gpu_handle::GpuHandle;
use nvml_wrapper::{Nvml, enum_wrappers::device::TemperatureSensor};

pub fn get_gpu_info(nvml: Option<&Nvml>, gpu_handle: Option<&GpuHandle>) -> (u32, u64, u32, f32) {
    get_nvidia_info(nvml).unwrap_or(get_amd_info(gpu_handle).unwrap_or((0, 0, 0, 0.0)))
}

fn get_nvidia_info(nvml: Option<&Nvml>) -> Option<(u32, u64, u32, f32)> {
    let nvml = nvml?;
    let device = nvml.device_by_index(0).ok()?;

    let usage = device.utilization_rates().ok()?.gpu;
    let mem = device.memory_info().ok()?.used;
    let temp = device.temperature(TemperatureSensor::Gpu).ok()?;
    let watt = device.power_usage().unwrap() as f32 / 1000.0;

    Some((usage, mem, temp, watt))
}

fn get_amd_info(gpu_handle: Option<&GpuHandle>) -> Option<(u32, u64, u32, f32)> {
    let gpu_handle = gpu_handle?;

    let usage = gpu_handle.get_busy_percent().ok()? as u32;
    let mem = gpu_handle.get_used_vram().ok()?;
    let hwmon = gpu_handle.hw_monitors.first()?;
    let watt = hwmon.get_power_average().ok()? as f32;
    let temp = hwmon.get_temps().get("edge")?.current? as u32;

    Some((usage, mem, temp, watt))
}

pub fn get_gpu_name() -> String {
    getgpuname::get_gpu_name()
        .unwrap_or("unknown".to_string())
        .split(&['[', ']'])
        .nth(1)
        .unwrap_or("unknown")
        .to_string()
}

pub fn get_gpu_mem(nvml: Option<&Nvml>, gpu_handle: Option<&GpuHandle>) -> u64 {
    get_nvidia_mem(nvml).unwrap_or(get_amd_mem(gpu_handle).unwrap_or(0))
}

fn get_nvidia_mem(nvml: Option<&Nvml>) -> Option<u64> {
    let nvml = nvml?;
    let device = nvml.device_by_index(0).ok()?;

    Some(device.memory_info().ok()?.total)
}

fn get_amd_mem(gpu_handle: Option<&GpuHandle>) -> Option<u64> {
    let gpu_handle = gpu_handle?;

    gpu_handle.get_total_vram().ok()
}
