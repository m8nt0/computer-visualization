// Data serialization utilities

/// Encode a byte slice to a hexadecimal string
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut hex = String::with_capacity(bytes.len() * 2);
    for &byte in bytes {
        hex.push_str(&format!("{:02x}", byte));
    }
    hex
}

/// Decode a hexadecimal string to a byte vector
pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, &'static str> {
    if hex.len() % 2 != 0 {
        return Err("Invalid hex string length");
    }
    
    let mut bytes = Vec::with_capacity(hex.len() / 2);
    
    for i in (0..hex.len()).step_by(2) {
        let byte_str = &hex[i..i + 2];
        let byte = u8::from_str_radix(byte_str, 16)
            .map_err(|_| "Invalid hex character")?;
        bytes.push(byte);
    }
    
    Ok(bytes)
}

/// Encode a byte slice to a base64 string (simplified implementation)
pub fn bytes_to_base64(bytes: &[u8]) -> String {
    // This is a simplified implementation for demonstration purposes only
    // In a real system, you'd use a proper base64 encoding library
    
    const BASE64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    
    for chunk in bytes.chunks(3) {
        let b = match chunk.len() {
            3 => [chunk[0], chunk[1], chunk[2], 0],
            2 => [chunk[0], chunk[1], 0, 0],
            1 => [chunk[0], 0, 0, 0],
            _ => unreachable!(),
        };
        
        let n = u32::from_be_bytes(b);
        
        let c1 = BASE64_CHARS[((n >> 18) & 0x3F) as usize];
        let c2 = BASE64_CHARS[((n >> 12) & 0x3F) as usize];
        let c3 = match chunk.len() {
            1 => b'=',
            _ => BASE64_CHARS[((n >> 6) & 0x3F) as usize],
        };
        let c4 = match chunk.len() {
            1 | 2 => b'=',
            _ => BASE64_CHARS[(n & 0x3F) as usize],
        };
        
        result.push(c1 as char);
        result.push(c2 as char);
        result.push(c3 as char);
        result.push(c4 as char);
    }
    
    result
}

/// Serialize a simple key-value structure to JSON (simplified implementation)
pub fn to_json(map: &std::collections::HashMap<String, String>) -> String {
    // This is a simplified implementation for demonstration purposes only
    // In a real system, you'd use a proper JSON serialization library
    
    let mut result = String::from("{");
    let mut first = true;
    
    for (key, value) in map {
        if !first {
            result.push(',');
        }
        first = false;
        
        result.push_str(&format!("\"{}\":\"{}\"", 
            key.replace('\\', "\\\\").replace('"', "\\\""),
            value.replace('\\', "\\\\").replace('"', "\\\"")
        ));
    }
    
    result.push('}');
    result
}

/// Serialize a simple key-value structure to XML (simplified implementation)
pub fn to_xml(map: &std::collections::HashMap<String, String>, root_name: &str) -> String {
    // This is a simplified implementation for demonstration purposes only
    // In a real system, you'd use a proper XML serialization library
    
    let mut result = format!("<{}>\n", root_name);
    
    for (key, value) in map {
        result.push_str(&format!("  <{}>{}</{}>\n", 
            key,
            value.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;"),
            key
        ));
    }
    
    result.push_str(&format!("</{}>", root_name));
    result
} 


// ===================================================================


// use serde::{Deserialize, Serialize};
// use std::io::{Read, Write};

// pub enum SerializationFormat {
//     Json,
//     Bincode,
//     MessagePack,
//     Cbor,
// }

// pub struct SerializationError {
//     pub message: String,
// }

// pub fn serialize<T: Serialize>(value: &T, format: SerializationFormat) -> Result<Vec<u8>, SerializationError> {
//     match format {
//         SerializationFormat::Json => {
//             serde_json::to_vec(value)
//                 .map_err(|e| SerializationError { message: format!("JSON serialization error: {}", e) })
//         },
//         SerializationFormat::Bincode => {
//             bincode::serialize(value)
//                 .map_err(|e| SerializationError { message: format!("Bincode serialization error: {}", e) })
//         },
//         SerializationFormat::MessagePack => {
//             rmp_serde::to_vec(value)
//                 .map_err(|e| SerializationError { message: format!("MessagePack serialization error: {}", e) })
//         },
//         SerializationFormat::Cbor => {
//             let mut buf = Vec::new();
//             let mut serializer = serde_cbor::Serializer::new(&mut buf);
//             value.serialize(&mut serializer)
//                 .map_err(|e| SerializationError { message: format!("CBOR serialization error: {}", e) })?;
//             Ok(buf)
//         },
//     }
// }

// pub fn deserialize<T: for<'de> Deserialize<'de>>(data: &[u8], format: SerializationFormat) -> Result<T, SerializationError> {
//     match format {
//         SerializationFormat::Json => {
//             serde_json::from_slice(data)
//                 .map_err(|e| SerializationError { message: format!("JSON deserialization error: {}", e) })
//         },
//         SerializationFormat::Bincode => {
//             bincode::deserialize(data)
//                 .map_err(|e| SerializationError { message: format!("Bincode deserialization error: {}", e) })
//         },
//         SerializationFormat::MessagePack => {
//             rmp_serde::from_slice(data)
//                 .map_err(|e| SerializationError { message: format!("MessagePack deserialization error: {}", e) })
//         },
//         SerializationFormat::Cbor => {
//             serde_cbor::from_slice(data)
//                 .map_err(|e| SerializationError { message: format!("CBOR deserialization error: {}", e) })
//         },
//     }
// }

// pub fn serialize_to_writer<T: Serialize, W: Write>(value: &T, writer: &mut W, format: SerializationFormat) -> Result<(), SerializationError> {
//     match format {
//         SerializationFormat::Json => {
//             serde_json::to_writer(writer, value)
//                 .map_err(|e| SerializationError { message: format!("JSON serialization error: {}", e) })
//         },
//         SerializationFormat::Bincode => {
//             bincode::serialize_into(writer, value)
//                 .map_err(|e| SerializationError { message: format!("Bincode serialization error: {}", e) })
//         },
//         SerializationFormat::MessagePack => {
//             rmp_serde::encode::write(writer, value)
//                 .map_err(|e| SerializationError { message: format!("MessagePack serialization error: {}", e) })
//         },
//         SerializationFormat::Cbor => {
//             let mut serializer = serde_cbor::Serializer::new(writer);
//             value.serialize(&mut serializer)
//                 .map_err(|e| SerializationError { message: format!("CBOR serialization error: {}", e) })
//         },
//     }
// }

// pub fn deserialize_from_reader<T: for<'de> Deserialize<'de>, R: Read>(reader: &mut R, format: SerializationFormat) -> Result<T, SerializationError> {
//     match format {
//         SerializationFormat::Json => {
//             serde_json::from_reader(reader)
//                 .map_err(|e| SerializationError { message: format!("JSON deserialization error: {}", e) })
//         },
//         SerializationFormat::Bincode => {
//             bincode::deserialize_from(reader)
//                 .map_err(|e| SerializationError { message: format!("Bincode deserialization error: {}", e) })
//         },
//         SerializationFormat::MessagePack => {
//             // This operation requires collecting the reader data first
//             let mut buf = Vec::new();
//             reader.read_to_end(&mut buf)
//                 .map_err(|e| SerializationError { message: format!("Read error: {}", e) })?;
//             rmp_serde::from_slice(&buf)
//                 .map_err(|e| SerializationError { message: format!("MessagePack deserialization error: {}", e) })
//         },
//         SerializationFormat::Cbor => {
//             serde_cbor::from_reader(reader)
//                 .map_err(|e| SerializationError { message: format!("CBOR deserialization error: {}", e) })
//         },
//     }
// }