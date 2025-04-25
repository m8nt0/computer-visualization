//implement bluetooth communication

pub struct Bluetooth {
    pub bluetooth_connected: bool,
    pub bluetooth_device: String,
}

impl Bluetooth {
    pub fn new() -> Self {
        Self { bluetooth_connected: false, bluetooth_device: String::new() }
    }

    pub fn bluetooth_connected(&self) -> bool {
        self.bluetooth_connected
    }

    pub fn bluetooth_device(&self) -> &String {
        &self.bluetooth_device
    }

    pub fn bluetooth_connect(&mut self, device: &String) {
        self.bluetooth_connected = true;
        self.bluetooth_device = device.clone();
        println!("Bluetooth connected to {}", device);
    }

    pub fn bluetooth_disconnect(&mut self) {
        self.bluetooth_connected = false;
        self.bluetooth_device = String::new();
        println!("Bluetooth disconnected");
    }

    pub fn bluetooth_send_data(&mut self, data: &String) {
        println!("Bluetooth sending data: {}", data);
    }

    pub fn bluetooth_receive_data(&mut self, data: &String) {
        println!("Bluetooth receiving data: {}", data);
    }

    pub fn bluetooth_discover_devices(&mut self) {
        println!("Bluetooth discovering devices");
    }

    pub fn bluetooth_pair(&mut self, device: &String) {
        println!("Bluetooth pairing with {}", device);
    }
    
}
