use std::collections::HashMap;

use sysinfo::Networks;

pub fn get_network_usage(
    total: &HashMap<String, (u64, u64)>,
    rate: &f32,
) -> HashMap<String, (u64, u64)> {
    let networks = Networks::new_with_refreshed_list();

    let mut stats = HashMap::new();

    for (interface, data) in &networks {
        let (total_down, total_up) = total.get(interface).unwrap_or(&(0, 0));
        let down = data.total_received() - total_down;
        let up = data.total_transmitted() - total_up;
        stats.insert(
            interface.to_string(),
            ((down as f32 / rate) as u64, (up as f32 / rate) as u64),
        );
    }

    stats
}

pub fn get_network_total() -> HashMap<String, (u64, u64)> {
    let networks = Networks::new_with_refreshed_list();

    let mut stats = HashMap::new();

    for (interface, data) in &networks {
        let down = data.total_received();
        let up = data.total_transmitted();
        stats.insert(interface.to_string(), (down, up));
    }

    stats
}
