use super::error::{DriverError, DriverResult};
use crate::hardware::network::{NetworkDevice, NetworkInterface};
use std::collections::VecDeque;

pub struct NetworkDriver {
    device: NetworkDevice,
    rx_queue: VecDeque<Packet>,
    tx_queue: VecDeque<Packet>,
    config: NetworkConfig,
    stats: NetworkStats,
}

struct Packet {
    data: Vec<u8>,
    protocol: Protocol,
    priority: u8,
    timestamp: u64,
}

enum Protocol {
    IPv4,
    IPv6,
    ARP,
    Other(u16),
}

impl NetworkDriver {
    pub fn new(device: NetworkDevice, config: NetworkConfig) -> Self {
        Self {
            device,
            rx_queue: VecDeque::new(),
            tx_queue: VecDeque::new(),
            config,
            stats: NetworkStats::default(),
        }
    }

    pub fn send_packet(&mut self, data: &[u8], protocol: Protocol) -> DriverResult<()> {
        let packet = Packet {
            data: data.to_vec(),
            protocol,
            priority: 0,
            timestamp: self.get_current_time(),
        };

        self.tx_queue.push_back(packet);
        self.process_tx_queue()?;
        Ok(())
    }

    pub fn receive_packet(&mut self) -> DriverResult<Option<Vec<u8>>> {
        self.poll_device()?;
        
        if let Some(packet) = self.rx_queue.pop_front() {
            self.stats.rx_packets += 1;
            self.stats.rx_bytes += packet.data.len();
            Ok(Some(packet.data))
        } else {
            Ok(None)
        }
    }

    fn process_tx_queue(&mut self) -> DriverResult<()> {
        while let Some(packet) = self.tx_queue.pop_front() {
            match self.device.transmit(&packet.data) {
                Ok(_) => {
                    self.stats.tx_packets += 1;
                    self.stats.tx_bytes += packet.data.len();
                }
                Err(e) => {
                    self.tx_queue.push_front(packet);
                    return Err(e.into());
                }
            }
        }
        Ok(())
    }

    fn poll_device(&mut self) -> DriverResult<()> {
        while let Some(data) = self.device.receive()? {
            let packet = Packet {
                data,
                protocol: self.detect_protocol(&data),
                priority: 0,
                timestamp: self.get_current_time(),
            };
            self.rx_queue.push_back(packet);
        }
        Ok(())
    }
} 