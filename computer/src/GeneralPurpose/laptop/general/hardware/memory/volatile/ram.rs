pub struct RAM {
    memory_controller: MemoryController,
    memory_modules: Vec<MemoryModule>,
    memory_slots: Vec<MemorySlot>,
    memory_type: MemoryType,
    memory_speed: MemorySpeed,
}

pub struct MemoryController {
    memory_type: MemoryType,
    memory_speed: MemorySpeed,
}

pub struct MemoryModule {
    memory_type: MemoryType,
    memory_speed: MemorySpeed,
}

pub struct MemorySlot {
    memory_type: MemoryType,
    memory_speed: MemorySpeed,
}

pub struct MemoryType {
    memory_type: MemoryType,
}   

pub struct MemorySpeed {
    memory_speed: MemorySpeed,
}

pub struct MemoryError {
    memory_error: MemoryError,
}

pub struct MemoryResult<T> {
    memory_result: MemoryResult<T>,
}   

impl MemoryController {
    pub fn new() -> Self {
        Self { memory_type: MemoryType::new(), memory_speed: MemorySpeed::new() }
    }
}


impl MemoryModule {
    pub fn new() -> Self {
        Self { memory_type: MemoryType::new(), memory_speed: MemorySpeed::new() }
    }
}

impl MemorySlot {
    pub fn new() -> Self {
        Self { memory_type: MemoryType::new(), memory_speed: MemorySpeed::new() }
    }
}

impl MemoryType {
    pub fn new() -> Self {
        Self { memory_type: MemoryType::new() }
    }
}

impl MemorySpeed {
    pub fn new() -> Self {
        Self { memory_speed: MemorySpeed::new() }
    }
}

impl MemoryError {
    pub fn new() -> Self {
        Self { memory_error: MemoryError::new() }
    }
}

impl MemoryResult<T> {
    pub fn new() -> Self {
        Self { memory_result: MemoryResult::new() }
    }
}