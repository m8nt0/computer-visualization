use super::error::{DMAError, DMAResult};
use std::collections::VecDeque;

pub struct DMAController {
    channels: Vec<DMAChannel>,
    transfer_queue: VecDeque<DMATransfer>,
    config: DMAConfig,
    stats: DMAStats,
}

struct DMAChannel {
    id: u8,
    state: ChannelState,
    current_transfer: Option<DMATransfer>,
}

struct DMATransfer {
    source: u64,
    destination: u64,
    size: usize,
    priority: u8,
    flags: TransferFlags,
}

impl DMAController {
    pub fn new(config: DMAConfig) -> Self {
        let channels = (0..config.num_channels)
            .map(|id| DMAChannel::new(id))
            .collect();

        Self {
            channels,
            transfer_queue: VecDeque::new(),
            config,
            stats: DMAStats::default(),
        }
    }

    pub fn request_transfer(&mut self, transfer: DMATransfer) -> DMAResult<()> {
        self.transfer_queue.push_back(transfer);
        Ok(())
    }

    pub fn tick(&mut self) {
        // Process pending transfers
        while let Some(transfer) = self.transfer_queue.pop_front() {
            if let Some(channel) = self.find_available_channel() {
                channel.start_transfer(transfer);
            } else {
                self.transfer_queue.push_front(transfer);
                break;
            }
        }

        // Update active channels
        for channel in &mut self.channels {
            channel.tick();
        }
    }
} 