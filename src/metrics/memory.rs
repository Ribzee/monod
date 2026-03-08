use sysinfo::System;

pub fn get_memory_usage(system: &mut System) -> (u64, f32) {
    system.refresh_memory();
    (
        system.used_memory(),
        system.used_memory() as f32 / system.total_memory() as f32 * 100.0,
    )
}

pub fn get_swap_usage(system: &mut System) -> (u64, f32) {
    (
        system.used_swap(),
        system.used_swap() as f32 / system.total_swap() as f32 * 100.0,
    )
}
