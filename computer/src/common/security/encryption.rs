// Data encryption utilities

// Data protection

#[derive(Debug, Clone, PartialEq)]
pub enum EncryptionAlgorithm {
    AES256,
    ChaCha20,
    RSA,
    ECC,
    Other(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum HashAlgorithm {
    SHA256,
    SHA512,
    Blake2b,
    Argon2id,
    Other(String),
}

// Simulated encryption key
#[derive(Debug, Clone)]
pub struct EncryptionKey {
    algorithm: EncryptionAlgorithm,
    key_id: String,
    key_data: Vec<u8>,
    created_at: u64, // Unix timestamp
    expires_at: Option<u64>,
}

impl EncryptionKey {
    pub fn new(algorithm: EncryptionAlgorithm, key_id: &str, key_data: Vec<u8>, expires_at: Option<u64>) -> Self {
        Self {
            algorithm,
            key_id: key_id.to_string(),
            key_data,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            expires_at,
        }
    }
    
    pub fn algorithm(&self) -> &EncryptionAlgorithm {
        &self.algorithm
    }
    
    pub fn key_id(&self) -> &str {
        &self.key_id
    }
    
    pub fn created_at(&self) -> u64 {
        self.created_at
    }
    
    pub fn expires_at(&self) -> Option<u64> {
        self.expires_at
    }
    
    pub fn is_expired(&self) -> bool {
        if let Some(expires) = self.expires_at {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            expires < now
        } else {
            false
        }
    }
}

// Basic encryption manager
#[derive(Debug, Default)]
pub struct EncryptionManager {
    keys: Vec<EncryptionKey>,
    default_algorithm: EncryptionAlgorithm,
}

impl EncryptionManager {
    pub fn new(default_algorithm: EncryptionAlgorithm) -> Self {
        Self {
            keys: Vec::new(),
            default_algorithm,
        }
    }
    
    pub fn add_key(&mut self, key: EncryptionKey) {
        self.keys.push(key);
    }
    
    pub fn get_key(&self, key_id: &str) -> Option<&EncryptionKey> {
        self.keys.iter().find(|k| k.key_id() == key_id && !k.is_expired())
    }
    
    pub fn default_algorithm(&self) -> &EncryptionAlgorithm {
        &self.default_algorithm
    }
    
    pub fn set_default_algorithm(&mut self, algorithm: EncryptionAlgorithm) {
        self.default_algorithm = algorithm;
    }
    
    pub fn cleanup_expired_keys(&mut self) {
        self.keys.retain(|key| !key.is_expired());
    }
    
    // Simulated encryption function
    pub fn encrypt(&self, data: &[u8], key_id: Option<&str>) -> Result<Vec<u8>, &'static str> {
        let key = if let Some(id) = key_id {
            self.get_key(id).ok_or("Key not found or expired")?
        } else {
            // Use the most recently added key for the default algorithm
            self.keys.iter()
                .filter(|k| k.algorithm() == &self.default_algorithm && !k.is_expired())
                .last()
                .ok_or("No suitable encryption key found")?
        };
        
        // This is just a simulation - in a real implementation, this would use a proper encryption library
        let mut result = Vec::with_capacity(data.len() + 16);
        
        // Add a marker that indicates the encryption algorithm and key ID
        result.extend_from_slice(format!("{}:{}", key.algorithm().as_str(), key.key_id()).as_bytes());
        result.push(0); // null terminator for our header
        
        // XOR each byte with the corresponding byte from the key (simple substitution for simulation)
        for (i, &byte) in data.iter().enumerate() {
            let key_byte = key.key_data[i % key.key_data.len()];
            result.push(byte ^ key_byte);
        }
        
        Ok(result)
    }
    
    // Simulated decryption function
    pub fn decrypt(&self, encrypted_data: &[u8]) -> Result<Vec<u8>, &'static str> {
        // Find the null terminator that separates our header from the data
        let header_end = encrypted_data.iter()
            .position(|&b| b == 0)
            .ok_or("Invalid encrypted data format")?;
        
        // Parse the header to get the algorithm and key ID
        let header = std::str::from_utf8(&encrypted_data[..header_end])
            .map_err(|_| "Invalid header encoding")?;
        
        let parts: Vec<&str> = header.split(':').collect();
        if parts.len() != 2 {
            return Err("Invalid header format");
        }
        
        let key_id = parts[1];
        let key = self.get_key(key_id).ok_or("Decryption key not found or expired")?;
        
        // Decrypt the data (XOR with key bytes)
        let encrypted_content = &encrypted_data[header_end + 1..];
        let mut result = Vec::with_capacity(encrypted_content.len());
        
        for (i, &byte) in encrypted_content.iter().enumerate() {
            let key_byte = key.key_data[i % key.key_data.len()];
            result.push(byte ^ key_byte);
        }
        
        Ok(result)
    }
}

impl EncryptionAlgorithm {
    pub fn as_str(&self) -> &str {
        match self {
            EncryptionAlgorithm::AES256 => "AES256",
            EncryptionAlgorithm::ChaCha20 => "ChaCha20",
            EncryptionAlgorithm::RSA => "RSA",
            EncryptionAlgorithm::ECC => "ECC",
            EncryptionAlgorithm::Other(s) => s,
        }
    }
}

impl HashAlgorithm {
    pub fn as_str(&self) -> &str {
        match self {
            HashAlgorithm::SHA256 => "SHA256",
            HashAlgorithm::SHA512 => "SHA512",
            HashAlgorithm::Blake2b => "Blake2b",
            HashAlgorithm::Argon2id => "Argon2id",
            HashAlgorithm::Other(s) => s,
        }
    }
    
    // Simulated hash function
    pub fn hash(&self, data: &[u8]) -> Vec<u8> {
        // This is just a simulation - in a real implementation, this would use a proper crypto library
        let mut result = Vec::with_capacity(32); // 256 bits
        
        // Simple hash algorithm (not secure, just for simulation)
        let mut value: u64 = 0x12345678;
        for &byte in data {
            value = value.wrapping_mul(31).wrapping_add(byte as u64);
        }
        
        // Convert to bytes
        for i in 0..8 {
            result.push(((value >> (i * 8)) & 0xFF) as u8);
        }
        
        // Pad to 32 bytes for consistent output size
        while result.len() < 32 {
            result.push(0);
        }
        
        result
    }
}