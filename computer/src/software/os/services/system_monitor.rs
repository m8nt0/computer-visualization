use super::error::{ServiceError, ServiceResult};
use std::collections::HashMap;
use std::time::Duration;

pub struct SystemMonitor {
    process_monitor: ProcessMonitor,
    resource_monitor: ResourceMonitor,
    network_monitor: NetworkMonitor,
    alerts: AlertManager,
    config: MonitorConfig,
}

struct ProcessMonitor {
    processes: HashMap<Pid, ProcessStats>,
    history: RingBuffer<ProcessSnapshot>,
    top_processes: Vec<(Pid, ResourceUsage)>,
}

struct ResourceMonitor {
    cpu_stats: CpuStats,
    memory_stats: MemoryStats,
    disk_stats: DiskStats,
    power_stats: PowerStats,
}

impl SystemMonitor {
    pub fn update(&mut self) -> ServiceResult<()> {
        // Update process statistics
        self.process_monitor.update()?;
        
        // Update system resource usage
        self.resource_monitor.update()?;
        
        // Update network statistics
        self.network_monitor.update()?;
        
        // Check resource thresholds
        self.check_alerts()?;
        
        Ok(())
    }

    pub fn get_system_load(&self) -> SystemLoad {
        SystemLoad {
            cpu_usage: self.resource_monitor.cpu_stats.usage(),
            memory_used: self.resource_monitor.memory_stats.used(),
            disk_io: self.resource_monitor.disk_stats.io_rate(),
            network_io: self.network_monitor.get_throughput(),
        }
    }

    pub fn get_process_info(&self, pid: Pid) -> ServiceResult<ProcessInfo> {
        let stats = self.process_monitor.processes.get(&pid)
            .ok_or(ServiceError::ProcessNotFound)?;
            
        Ok(ProcessInfo {
            pid,
            name: stats.name.clone(),
            state: stats.state,
            cpu_usage: stats.cpu_usage,
            memory_usage: stats.memory_usage,
            io_stats: stats.io_stats.clone(),
        })
    }

    fn check_alerts(&mut self) -> ServiceResult<()> {
        // Check CPU usage
        if self.resource_monitor.cpu_stats.usage() > self.config.cpu_threshold {
            self.alerts.trigger(AlertType::HighCpuUsage)?;
        }

        // Check memory usage
        if self.resource_monitor.memory_stats.used_percent() > self.config.memory_threshold {
            self.alerts.trigger(AlertType::HighMemoryUsage)?;
        }

        // Check disk usage
        if self.resource_monitor.disk_stats.used_percent() > self.config.disk_threshold {
            self.alerts.trigger(AlertType::LowDiskSpace)?;
        }

        Ok(())
    }
} 