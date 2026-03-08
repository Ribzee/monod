use std::collections::HashMap;

use sysinfo::Networks;

pub fn get_network_stats(prev_received: u64, prev_transmitted: u64) -> HashMap<String, (u64, u64)> {
    let networks = Networks::new_with_refreshed_list();

    let mut stats = HashMap::new();

    for (interface, data) in &networks {
        let down = data.total_received() - prev_received;
        let up = data.total_transmitted() - prev_transmitted;
        stats.insert(interface.to_string(), (down, up));
    }

    stats
}
