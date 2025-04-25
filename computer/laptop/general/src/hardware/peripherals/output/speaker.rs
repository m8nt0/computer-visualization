pub struct Speaker {
    volume: u8,
    is_muted: bool,
    model: String,
    max_volume: u8,
}

impl Speaker {
    pub fn new(model: String, max_volume: u8) -> Self {
        Speaker {
            volume: 50,
            is_muted: false,
            model,
            max_volume,
        }
    }

    pub fn set_volume(&mut self, volume: u8) {
        self.volume = volume.min(self.max_volume);
    }

    pub fn get_volume(&self) -> u8 {
        self.volume
    }

    pub fn mute(&mut self) {
        self.is_muted = true;
    }

    pub fn unmute(&mut self) {
        self.is_muted = false;
    }

    pub fn is_muted(&self) -> bool {
        self.is_muted
    }

    pub fn play_sound(&self, frequency: u32, duration_ms: u32) {
        if !self.is_muted {
            println!("Playing sound at {}Hz for {}ms at volume {}", frequency, duration_ms, self.volume);
        }
    }
}