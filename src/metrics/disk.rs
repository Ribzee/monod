use std::{collections::HashMap, os::unix::ffi::OsStrExt};

use sysinfo::{Disk, Disks};

pub fn get_disk_usage() -> HashMap<String, (u64, u64)> {
    let disks = Disks::new_with_refreshed_list();

    let mut disk_usage = HashMap::new();

    for disk in &disks {
        if !is_valid_disk(disk) {
            continue;
        }
        disk_usage.insert(
            disk.mount_point().to_string_lossy().to_string(),
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
        if !is_valid_disk(disk) {
            continue;
        }

        disk_io.insert(
            disk.mount_point().to_string_lossy().to_string(),
            (disk.usage().read_bytes, disk.usage().written_bytes),
        );
    }

    disk_io
}

fn is_valid_disk(disk: &Disk) -> bool {
    matches!(
        disk.file_system().as_bytes(),
        b"ext4" | b"ext3" | b"ext2" | b"xfs" | b"btrfs" | b"vfat" | b"ntfs"
    )
}
