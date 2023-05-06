use std::fmt;
use bevy::prelude::Resource;
use sysinfo::{System, SystemExt, NetworkExt, ProcessExt};
use sysinfo::{Cpu, CpuExt};
use nvml_wrapper::{Nvml, Device};
use strum_macros::EnumIter;

#[derive(Resource)]
pub struct HWSystem {
    //TODO un-pub sysinfo and nvml
    pub sysinfo: System,
    pub nvml: Option<Nvml>,
}

impl HWSystem {
    pub fn new() -> Self {
        Self {
            sysinfo: System::new_all(),
            nvml: match Nvml::init() {
                Ok(nvml) => Some(nvml),
                Err(e) => {
                    eprintln!("Failed to initialize NVML: {}", e);
                    None
                },
            }
        }
    }

    pub fn refresh_all(&mut self) {
        self.sysinfo.refresh_all();
    }

    pub fn cpus(&self) -> Vec<&Cpu> {
        self.sysinfo.cpus()
    }

    pub fn cpu_strings(&self) -> Vec<String> {
        let mut data = Vec::<String>::new();
        for cpu in self.sysinfo.cpus() {
            data.push(format!("Brand: {} Name: {} Usage: {:.2}%", cpu.brand(), cpu.name(), cpu.cpu_usage()));
        }
        data
    }

    pub fn gpu_strings(&self) -> Vec<String> {
        let mut data = Vec::<String>::new();
        if let Some(nvml) = &self.nvml {
            let device_count = nvml.device_count().unwrap();
            for n in 0..device_count {
                let device = nvml.device_by_index(n).unwrap();
                data.push(format!("Brand: {:?} Name: {}", device.brand().unwrap(), device.name().unwrap()));
            }
        }
        data
    }

    pub fn ram_and_swap_strings(&self) -> Vec<String> {
        let mut data = Vec::<String>::new();
        data.push(format!("total memory: {} bytes", self.sysinfo.total_memory()));
        data.push(format!("used memory : {} bytes", self.sysinfo.used_memory()));
        data.push(format!("total swap  : {} bytes", self.sysinfo.total_swap()));
        data.push(format!("used swap   : {} bytes", self.sysinfo.used_swap()));
        data
    }

    pub fn disk_strings(&self) -> Vec<String> {
        let mut data = Vec::<String>::new();
        for disk in self.sysinfo.disks() {
            data.push(format!("{:?}", disk));
        }
        data
    }

    pub fn network_strings(&self) -> Vec<String> {
        let mut data = Vec::<String>::new();
        for (name, network) in self.sysinfo.networks() {
            data.push(format!("{}: {}/{} B", name, network.received(), network.transmitted()));
        }
        data
    }

    pub fn system_strings(&self) -> Vec<String> {
        let mut data = Vec::<String>::new();
        data.push(format!("System name: {}", self.sysinfo.name().unwrap()));
        data.push(format!("System kernel version: {}", self.sysinfo.kernel_version().unwrap()));
        data.push(format!("System OS version: {}", self.sysinfo.os_version().unwrap()));
        data.push(format!("System host name: {}", self.sysinfo.host_name().unwrap()));
        data
    }

    pub fn components_strings(&self) -> Vec<String> {
        let mut data = Vec::<String>::new();
        for component in self.sysinfo.components() {
            data.push(format!("{:?}", component));
        }
        data
    }

    pub fn processes_strings(&self) -> Vec<String> {
        let mut data = Vec::<String>::new();
        for (pid, process) in self.sysinfo.processes() {
            data.push(format!("[{}] {} {:?}", pid, process.name(), process.disk_usage()));
        }
        data
    }

}

#[derive(Debug, EnumIter)]
pub enum HWKind {
    CPU,
    GPU,
    RAM,
    DISK,
    NETWORK,
    SYSTEM,
    COMPONENTS,
    PROCESSES,
    MISC,
}

impl fmt::Display for HWKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HWKind::CPU => write!(f, "CPU"),
            HWKind::GPU => write!(f, "GPU"),
            HWKind::RAM => write!(f, "RAM"),
            HWKind::DISK => write!(f, "DISK"),
            HWKind::NETWORK => write!(f, "NETWORK"),
            HWKind::SYSTEM => write!(f, "SYSTEM"),
            HWKind::COMPONENTS => write!(f, "COMPONENTS"),
            HWKind::PROCESSES => write!(f, "PROCESSES"),
            HWKind::MISC => write!(f, "MISC"),
        }
    }
}