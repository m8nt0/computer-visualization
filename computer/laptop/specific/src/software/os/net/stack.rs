use super::error::{NetError, NetResult};
use std::collections::{HashMap, VecDeque};

pub struct NetworkStack {
    interfaces: HashMap<InterfaceId, NetworkInterface>,
    protocols: ProtocolManager,
    routing: RoutingTable,
    sockets: SocketManager,
    stats: NetworkStats,
}

struct NetworkInterface {
    id: InterfaceId,
    driver: Box<dyn NetworkDriver>,
    ip_config: IpConfig,
    rx_queue: VecDeque<Packet>,
    tx_queue: VecDeque<Packet>,
    state: InterfaceState,
}

struct ProtocolManager {
    ipv4: Ipv4Protocol,
    ipv6: Ipv6Protocol,
    tcp: TcpProtocol,
    udp: UdpProtocol,
    icmp: IcmpProtocol,
}

struct SocketManager {
    sockets: HashMap<SocketId, Socket>,
    port_allocator: PortAllocator,
    backlog: HashMap<SocketId, VecDeque<Packet>>,
}

impl NetworkStack {
    pub fn new(config: NetworkConfig) -> Self {
        Self {
            interfaces: HashMap::new(),
            protocols: ProtocolManager::new(),
            routing: RoutingTable::new(),
            sockets: SocketManager::new(),
            stats: NetworkStats::default(),
        }
    }

    pub fn create_socket(&mut self, domain: SocketDomain, type_: SocketType) -> NetResult<SocketId> {
        let socket = Socket::new(domain, type_);
        let id = self.sockets.allocate_id();
        
        self.sockets.insert(id, socket);
        Ok(id)
    }

    pub fn bind(&mut self, socket_id: SocketId, addr: SocketAddr) -> NetResult<()> {
        let socket = self.sockets.get_mut(&socket_id)
            .ok_or(NetError::InvalidSocket)?;
            
        // Check if address is available
        if self.sockets.is_addr_in_use(&addr) {
            return Err(NetError::AddressInUse);
        }
        
        socket.bind(addr)?;
        self.sockets.register_binding(socket_id, addr);
        
        Ok(())
    }

    pub fn process_packet(&mut self, interface_id: InterfaceId, packet: Packet) -> NetResult<()> {
        // Validate packet
        self.protocols.validate_packet(&packet)?;
        
        // Route packet to appropriate protocol handler
        match packet.protocol() {
            Protocol::IPv4 => self.protocols.ipv4.handle_packet(packet)?,
            Protocol::IPv6 => self.protocols.ipv6.handle_packet(packet)?,
            Protocol::TCP => self.protocols.tcp.handle_packet(packet)?,
            Protocol::UDP => self.protocols.udp.handle_packet(packet)?,
            Protocol::ICMP => self.protocols.icmp.handle_packet(packet)?,
        }
        
        self.stats.packets_processed += 1;
        Ok(())
    }
} 