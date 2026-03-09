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
