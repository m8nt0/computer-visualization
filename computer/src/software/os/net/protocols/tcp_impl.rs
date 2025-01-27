use super::error::{NetError, NetResult};
use std::collections::HashMap;

pub struct TcpProtocolImpl {
    connections: HashMap<ConnectionId, TcpConnection>,
    pending_connections: HashMap<ConnectionId, TcpPendingConnection>,
    timers: TimerManager,
    config: TcpConfig,
}

struct TcpConnection {
    id: ConnectionId,
    state: TcpState,
    send_buffer: RingBuffer<u8>,
    receive_buffer: RingBuffer<u8>,
    send_window: Window,
    receive_window: Window,
    sequence_numbers: SequenceNumbers,
    timers: ConnectionTimers,
}

impl TcpProtocolImpl {
    pub fn handle_packet(&mut self, packet: &TcpPacket) -> NetResult<()> {
        let conn_id = self.get_connection_id(packet);

        match self.connections.get_mut(&conn_id) {
            Some(conn) => self.handle_established(conn, packet),
            None => {
                if packet.flags().contains(TcpFlags::SYN) {
                    self.handle_new_connection(packet)
                } else {
                    Err(NetError::ConnectionNotFound)
                }
            }
        }
    }

    fn handle_established(&mut self, conn: &mut TcpConnection, packet: &TcpPacket) -> NetResult<()> {
        // Validate sequence numbers
        if !conn.sequence_numbers.is_valid(packet.sequence_number()) {
            return Ok(()); // Silently drop out-of-sequence packets
        }

        // Process flags
        if packet.flags().contains(TcpFlags::RST) {
            return self.handle_reset(conn);
        }

        // Handle data
        if packet.has_data() {
            self.process_data(conn, packet)?;
        }

        // Update windows
        self.update_windows(conn, packet);

        // Handle acknowledgments
        if packet.flags().contains(TcpFlags::ACK) {
            self.handle_ack(conn, packet)?;
        }

        Ok(())
    }
} 