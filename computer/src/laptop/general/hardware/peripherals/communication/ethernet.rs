//implement ethernet communication

pub struct Ethernet {
    pub ethernet_connected: bool,
    pub ethernet_device: String,
}

impl Ethernet {
    pub fn new() -> Self {
        Self { ethernet_connected: false, ethernet_device: String::new() }
    }

    pub fn ethernet_connected(&self) -> bool {
        self.ethernet_connected
    }

    pub fn ethernet_device(&self) -> &String {
        &self.ethernet_device
    }

    pub fn ethernet_connect(&mut self, device: &String) {
        self.ethernet_connected = true;
        self.ethernet_device = device.clone();
        println!("Ethernet connected to {}", device);
    }
    
    
}
