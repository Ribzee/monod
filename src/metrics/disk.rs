use std::collections::HashMap;

use sysinfo::Disks;

pub fn get_disk_usage() -> HashMap<String, (u64, u64)> {
    let disks = Disks::new_with_refreshed_list();

    let mut disk_usage = HashMap::new();

    for disk in &disks {
        disk_usage.insert(
            disk.name().to_string_lossy().to_string(),
            (
                disk.total_space() - disk.available_space(),
                disk.total_space(),
            ),
        );
    }

    disk_usage
}

pub fn get_disk_io() -> HashMap<String, (u64, u64)> {
    let disks = Disks::new_with_refreshed_list();

    let mut disk_io = HashMap::new();

    for disk in &disks {
        disk_io.insert(
            disk.name().to_string_lossy().to_string(),
            (disk.usage().read_bytes, disk.usage().written_bytes),
        );
    }

    disk_io
}
