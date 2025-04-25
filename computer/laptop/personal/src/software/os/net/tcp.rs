pub struct TcpProtocol {
    connections: HashMap<ConnectionId, TcpConnection>,
    listeners: HashMap<u16, TcpListener>,
    config: TcpConfig,
}

impl TcpProtocol {
    pub fn handle_packet(&mut self, packet: &TcpPacket) -> NetResult<()> {
        match packet.flags() {
            TcpFlags::SYN => self.handle_syn(packet),
            TcpFlags::ACK => self.handle_ack(packet),
            TcpFlags::FIN => self.handle_fin(packet),
            _ => self.handle_data(packet),
        }
    }
} 