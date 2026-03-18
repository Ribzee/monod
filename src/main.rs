use std::{fs, path::PathBuf};

use amdgpu_sysfs::gpu_handle::GpuHandle;
use monod::{
    metrics::{collector::SystemMonitor, gpu},
    tui::app::app,
};
use nvml_wrapper::{Nvml, enum_wrappers::device::TemperatureSensor};
use sysinfo::{Components, System};

fn main() -> Result<(), anyhow::Error> {
    //let mut system = SystemMonitor::new();
    //let rate = 0.5f32;

    //loop {
    //    system.update_state(&rate);
    //    println!("{:#?}", system);
    //    sleep(Duration::from_secs_f32(rate));
    //}
    //

    Ok(())
}
