use super::error::{ServiceError, ServiceResult};
use std::collections::HashMap;

pub struct InitSystem {
    services: HashMap<ServiceId, Service>,
    dependencies: HashMap<ServiceId, Vec<ServiceId>>,
    state: SystemState,
    config: InitConfig,
}

struct Service {
    id: ServiceId,
    name: String,
    state: ServiceState,
    process: Option<ProcessId>,
    restart_policy: RestartPolicy,
}

enum ServiceState {
    Stopped,
    Starting,
    Running,
    Stopping,
    Failed,
}

enum RestartPolicy {
    Never,
    OnFailure,
    Always,
}

impl InitSystem {
    pub fn new(config: InitConfig) -> Self {
        Self {
            services: HashMap::new(),
            dependencies: HashMap::new(),
            state: SystemState::Booting,
            config,
        }
    }

    pub fn boot(&mut self) -> ServiceResult<()> {
        // Start essential services in correct order
        let boot_order = self.calculate_boot_order()?;
        
        for service_id in boot_order {
            self.start_service(service_id)?;
        }

        self.state = SystemState::Running;
        Ok(())
    }

    pub fn start_service(&mut self, id: ServiceId) -> ServiceResult<()> {
        let service = self.services.get_mut(&id)
            .ok_or(ServiceError::NotFound)?;

        // Check dependencies
        if let Some(deps) = self.dependencies.get(&id) {
            for dep_id in deps {
                if !self.is_service_running(*dep_id) {
                    return Err(ServiceError::DependencyNotMet);
                }
            }
        }

        // Start service process
        let process_id = self.spawn_service_process(service)?;
        service.process = Some(process_id);
        service.state = ServiceState::Running;

        Ok(())
    }

    pub fn stop_service(&mut self, id: ServiceId) -> ServiceResult<()> {
        let service = self.services.get_mut(&id)
            .ok_or(ServiceError::NotFound)?;

        if let Some(process_id) = service.process {
            self.terminate_process(process_id)?;
            service.process = None;
            service.state = ServiceState::Stopped;
        }

        Ok(())
    }
} 