use super::error::{ServiceError, ServiceResult};
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub struct NetworkManager {
    interfaces: HashMap<InterfaceId, NetworkInterface>,
    connections: HashMap<ConnectionId, Connection>,
    routes: RoutingTable,
    dns: DnsResolver,
    stats: NetworkStats,
}

struct NetworkInterface {
    id: InterfaceId,
    name: String,
    mac_addr: MacAddress,
    ipv4_config: Option<Ipv4Config>,
    ipv6_config: Option<Ipv6Config>,
    state: InterfaceState,
    driver: Box<dyn NetworkDriver>,
}

struct Connection {
    id: ConnectionId,
    interface: InterfaceId,
    config: ConnectionConfig,
    state: ConnectionState,
}

struct RoutingTable {
    ipv4_routes: Vec<Route<Ipv4Addr>>,
    ipv6_routes: Vec<Route<Ipv6Addr>>,
    default_gateway: Option<IpAddr>,
}

impl NetworkManager {
    pub fn new() -> Self {
        Self {
            interfaces: HashMap::new(),
            connections: HashMap::new(),
            routes: RoutingTable::new(),
            dns: DnsResolver::new(),
            stats: NetworkStats::default(),
        }
    }

    pub fn add_interface(&mut self, driver: Box<dyn NetworkDriver>) -> ServiceResult<InterfaceId> {
        let id = self.generate_interface_id();
        let info = driver.get_interface_info()?;
        
        let interface = NetworkInterface {
            id,
            name: info.name,
            mac_addr: info.mac_addr,
            ipv4_config: None,
            ipv6_config: None,
            state: InterfaceState::Down,
            driver,
        };
        
        self.interfaces.insert(id, interface);
        Ok(id)
    }

    pub fn configure_interface(&mut self, id: InterfaceId, config: InterfaceConfig) -> ServiceResult<()> {
        let interface = self.interfaces.get_mut(&id)
            .ok_or(ServiceError::InterfaceNotFound)?;
            
        match config {
            InterfaceConfig::Ipv4(cfg) => {
                interface.ipv4_config = Some(cfg);
                self.apply_ipv4_config(interface)?;
            }
            InterfaceConfig::Ipv6(cfg) => {
                interface.ipv6_config = Some(cfg);
                self.apply_ipv6_config(interface)?;
            }
        }
        
        Ok(())
    }

    pub fn create_connection(&mut self, config: ConnectionConfig) -> ServiceResult<ConnectionId> {
        let id = self.generate_connection_id();
        
        // Find suitable interface
        let interface_id = self.find_interface_for_connection(&config)?;
        
        let connection = Connection {
            id,
            interface: interface_id,
            config,
            state: ConnectionState::Disconnected,
        };
        
        self.connections.insert(id, connection);
        self.activate_connection(id)?;
        
        Ok(id)
    }
} 