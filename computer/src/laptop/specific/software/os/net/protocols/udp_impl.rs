use super::error::{NetError, NetResult};
use std::collections::HashMap;

pub struct UdpProtocolImpl {
    sockets: HashMap<SocketId, UdpSocket>,
    buffers: HashMap<SocketId, RingBuffer<u8>>,
    config: UdpConfig,
}

struct UdpSocket {
    id: SocketId,
    local_addr: SocketAddr,
    multicast_groups: Vec<IpAddr>,
    flags: SocketFlags,
}

impl UdpProtocolImpl {
    pub fn handle_packet(&mut self, packet: &UdpPacket) -> NetResult<()> {
        let socket_id = self.find_socket(packet.destination_port())?;
        
        let socket = self.sockets.get_mut(&socket_id)
            .ok_or(NetError::SocketNotFound)?;

        // Check if socket accepts packets from this source
        if !socket.can_receive_from(&packet.source()) {
            return Ok(());
        }

        // Store packet data in receive buffer
        let buffer = self.buffers.get_mut(&socket_id)
            .ok_or(NetError::BufferNotFound)?;

        buffer.write(&packet.payload())?;

        Ok(())
    }

    pub fn send(&mut self, socket_id: SocketId, dest: SocketAddr, data: &[u8]) -> NetResult<()> {
        let socket = self.sockets.get(&socket_id)
            .ok_or(NetError::SocketNotFound)?;

        let packet = UdpPacket::new(
            socket.local_addr,
            dest,
            data,
        )?;

        self.transmit_packet(packet)
    }
} 