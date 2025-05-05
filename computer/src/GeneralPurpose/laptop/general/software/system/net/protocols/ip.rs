use super::error::{NetError, NetResult};
use std::collections::HashMap;

pub struct IpProtocol {
    routing_table: RoutingTable,
    interfaces: HashMap<InterfaceId, IpInterface>,
    fragment_cache: FragmentCache,
    config: IpConfig,
}

struct IpInterface {
    id: InterfaceId,
    addresses: Vec<IpAddr>,
    mtu: u16,
    metrics: InterfaceMetrics,
}

struct RoutingTable {
    routes: Vec<Route>,
    default_gateway: Option<IpAddr>,
    cache: RouteCache,
}

impl IpProtocol {
    pub fn handle_packet(&mut self, packet: &[u8], interface: InterfaceId) -> NetResult<()> {
        let header = IpHeader::parse(packet)?;
        
        // Validate header
        if !header.is_valid() {
            return Err(NetError::InvalidHeader);
        }

        // Check if packet is for us
        if !self.is_for_us(&header.destination) {
            return self.forward_packet(packet, &header);
        }

        // Handle fragments
        if header.is_fragment() {
            return self.handle_fragment(packet, &header);
        }

        // Pass to upper layer protocol
        match header.protocol {
            Protocol::TCP => self.handle_tcp(packet, &header),
            Protocol::UDP => self.handle_udp(packet, &header),
            Protocol::ICMP => self.handle_icmp(packet, &header),
            _ => Err(NetError::UnsupportedProtocol),
        }
    }

    fn forward_packet(&mut self, packet: &[u8], header: &IpHeader) -> NetResult<()> {
        // Check TTL
        if header.ttl <= 1 {
            return self.send_time_exceeded(header);
        }

        // Find route
        let route = self.routing_table.find_route(&header.destination)?;
        
        // Forward packet
        let interface = self.interfaces.get_mut(&route.interface)
            .ok_or(NetError::InvalidInterface)?;

        interface.transmit(packet)
    }
} 