//implement wifi communication

pub struct Wifi {
    pub wifi_connected: bool,
    pub wifi_device: String,
}

impl Wifi {
    pub fn new() -> Self {
        Self { wifi_connected: false, wifi_device: String::new() }
    }

    pub fn wifi_connected(&self) -> bool {
        self.wifi_connected
    }

    pub fn wifi_device(&self) -> &String {
        &self.wifi_device
    }

    pub fn wifi_connect(&mut self, device: &String) {
        self.wifi_connected = true;
        self.wifi_device = device.clone();
        println!("Wifi connected to {}", device);
    }

    pub fn wifi_disconnect(&mut self) {
        self.wifi_connected = false;
        self.wifi_device = String::new();
        println!("Wifi disconnected");
    }

    pub fn wifi_send_data(&mut self, data: &String) {
        println!("Wifi sending data: {}", data);
    }

    pub fn wifi_receive_data(&mut self, data: &String) {
        println!("Wifi receiving data: {}", data);
    }

    pub fn wifi_discover_devices(&mut self) {
        println!("Wifi discovering devices");
    }

    pub fn wifi_pair(&mut self, device: &String) {
        println!("Wifi pairing with {}", device);
    }

    pub fn wifi_scan(&mut self) {
        println!("Wifi scanning for devices");
    }

    pub fn wifi_scan_results(&mut self) {
        println!("Wifi scan results");
    }

    pub fn wifi_scan_results_clear(&mut self) {
        println!("Wifi scan results cleared");
    }

    pub fn wifi_scan_results_save(&mut self, filename: &String) {
        println!("Wifi scan results saved to {}", filename);
    }

    pub fn wifi_scan_results_load(&mut self, filename: &String) {
        println!("Wifi scan results loaded from {}", filename);
    }
    
}