use sysinfo::{System, Users};

#[derive(Debug, Clone)]
pub struct Process {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_usage: u64,
    pub memory_percentage: f32,
    pub user: String,
    pub parent_id: Option<u32>,
    pub command: String,
}

pub fn get_processes(system: &mut System) -> Result<Vec<Process>, anyhow::Error> {
    let users = Users::new_with_refreshed_list();
    let total_memory = system.total_memory();

    let cores = system.cpus().len() as f32;

    let processes = system
        .processes()
        .iter()
        .map(|(pid, proc)| {
            let user = proc
                .user_id()
                .and_then(|id| users.get_user_by_id(id))
                .map(|u| u.name().to_string())
                .unwrap_or_else(|| "unknown".to_string());

            Process {
                pid: pid.as_u32(),
                name: proc.name().to_string_lossy().to_string(),
                cpu_usage: proc.cpu_usage() / cores,
                memory_usage: proc.memory(),
                memory_percentage: (proc.memory() as f32 / total_memory as f32) * 100.0,
                user,
                parent_id: proc.parent().map(|id| id.as_u32()),
                command: proc
                    .cmd()
                    .iter()
                    .map(|arg| arg.to_string_lossy().to_string())
                    .collect::<Vec<String>>()
                    .join(" "),
            }
        })
        .collect::<Vec<Process>>();

    Ok(processes)
}
