use once_cell::sync::OnceCell;
use salvo::oapi::ToSchema;
use serde::Serialize;
use std::sync::Mutex;
use sysinfo::{ComponentExt, ProcessExt, System, SystemExt};

#[derive(Debug, Default, Serialize, ToSchema)]
pub struct SystemInfo {
    system_name: Option<String>,
    kernel_version: Option<String>,
    os_version: Option<String>,
    host_name: Option<String>,
    cpu_count: usize,
    components: Vec<ComponentInfo>,
    memory_info: MemoryInfo,
    percent_memory_used: f64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ComponentInfo {
    name: String,
    temperature: f32,
}

#[derive(Debug, Default, Serialize, ToSchema)]
pub struct MemoryInfo {
    total_memory: u64,
    used_memory: u64,
    total_swap: u64,
    used_swap: u64,
}

impl SystemInfo {
    pub fn gather() -> Self {
        let sys: &Mutex<System> = SYS.get_or_init(|| {
            let mut s = System::new_all();
            s.refresh_all();
            Mutex::new(s)
        });

        let mut sys = sys.lock().unwrap();
        sys.refresh_all();
        let components = sys
            .components()
            .iter()
            .map(|component| ComponentInfo {
                name: component.label().to_string(),
                temperature: component.temperature(),
            })
            .collect();

        let memory_info = MemoryInfo {
            total_memory: sys.total_memory(),
            used_memory: sys.used_memory(),
            total_swap: sys.total_swap(),
            used_swap: sys.used_swap(),
        };

        let total_memory = sys.total_memory();

        let total_process_memory: u64 = sys.processes().values().map(|p| p.memory()).sum();

        let percent_memory_used =
            ((total_process_memory as f64 / total_memory as f64) * 10000.0).round() / 100.0;
        Self {
            system_name: sys.name(),
            kernel_version: sys.kernel_version(),
            os_version: sys.os_version(),
            host_name: sys.host_name(),
            cpu_count: sys.cpus().len(),
            components,
            memory_info,
            percent_memory_used,
        }
    }
}

static SYS: OnceCell<Mutex<System>> = OnceCell::new();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sysinfo() {
        let _sys = SystemInfo::gather();
    }
}
