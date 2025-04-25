use super::error::{IpcError, IpcResult};
use std::collections::{HashMap, VecDeque};

pub struct IpcManager {
    channels: HashMap<ChannelId, IpcChannel>,
    message_queues: HashMap<QueueId, MessageQueue>,
    shared_memory: SharedMemoryManager,
    semaphores: SemaphoreManager,
}

struct IpcChannel {
    id: ChannelId,
    sender: ProcessId,
    receiver: ProcessId,
    buffer: RingBuffer<Message>,
    flags: ChannelFlags,
}

struct MessageQueue {
    id: QueueId,
    messages: VecDeque<Message>,
    max_size: usize,
    readers: Vec<ProcessId>,
    writers: Vec<ProcessId>,
}

struct SharedMemoryManager {
    regions: HashMap<RegionId, SharedMemoryRegion>,
    page_allocator: PageAllocator,
}

impl IpcManager {
    pub fn new(config: IpcConfig) -> Self {
        Self {
            channels: HashMap::new(),
            message_queues: HashMap::new(),
            shared_memory: SharedMemoryManager::new(),
            semaphores: SemaphoreManager::new(),
        }
    }

    pub fn create_channel(&mut self, sender: ProcessId, receiver: ProcessId) -> IpcResult<ChannelId> {
        let id = self.generate_channel_id();
        
        let channel = IpcChannel {
            id,
            sender,
            receiver,
            buffer: RingBuffer::new(DEFAULT_CHANNEL_SIZE),
            flags: ChannelFlags::default(),
        };
        
        self.channels.insert(id, channel);
        Ok(id)
    }

    pub fn send_message(&mut self, channel_id: ChannelId, message: Message) -> IpcResult<()> {
        let channel = self.channels.get_mut(&channel_id)
            .ok_or(IpcError::InvalidChannel)?;
            
        // Check permissions
        if !channel.can_send(message.sender) {
            return Err(IpcError::PermissionDenied);
        }
        
        // Write message to channel buffer
        channel.buffer.write(&message)?;
        
        // Notify receiver
        self.notify_receiver(channel.receiver, channel_id)?;
        
        Ok(())
    }

    pub fn receive_message(&mut self, channel_id: ChannelId) -> IpcResult<Message> {
        let channel = self.channels.get_mut(&channel_id)
            .ok_or(IpcError::InvalidChannel)?;
            
        // Check permissions
        if !channel.can_receive(channel.receiver) {
            return Err(IpcError::PermissionDenied);
        }
        
        // Read message from channel buffer
        let message = channel.buffer.read()?;
        
        Ok(message)
    }

    pub fn create_shared_memory(&mut self, size: usize) -> IpcResult<RegionId> {
        let id = self.generate_region_id();
        
        // Allocate physical pages
        let pages = self.shared_memory.allocate_pages(size)?;
        
        let region = SharedMemoryRegion {
            id,
            size,
            pages,
            mappings: HashMap::new(),
        };
        
        self.shared_memory.regions.insert(id, region);
        Ok(id)
    }
} 