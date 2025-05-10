// Type conversion utilities

/// Convert bytes to a human-readable size string
pub fn bytes_to_human_readable(bytes: u64) -> String {
    const UNITS: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
    
    if bytes == 0 {
        return "0 B".to_string();
    }
    
    let base = 1024.0;
    let exponent = (bytes as f64).log(base).floor() as usize;
    let value = bytes as f64 / base.powi(exponent as i32);
    
    if exponent >= UNITS.len() {
        format!("{:.2} {}", value * base.powi((exponent - UNITS.len() + 1) as i32), UNITS[UNITS.len() - 1])
    } else {
        format!("{:.2} {}", value, UNITS[exponent])
    }
}

/// Convert milliseconds to a duration string (e.g., "2d 5h 30m 15s")
pub fn ms_to_duration_string(ms: u64) -> String {
    const SECOND: u64 = 1000;
    const MINUTE: u64 = 60 * SECOND;
    const HOUR: u64 = 60 * MINUTE;
    const DAY: u64 = 24 * HOUR;
    
    let days = ms / DAY;
    let hours = (ms % DAY) / HOUR;
    let minutes = (ms % HOUR) / MINUTE;
    let seconds = (ms % MINUTE) / SECOND;
    let remaining_ms = ms % SECOND;
    
    let mut parts = Vec::new();
    
    if days > 0 {
        parts.push(format!("{}d", days));
    }
    
    if hours > 0 || !parts.is_empty() {
        parts.push(format!("{}h", hours));
    }
    
    if minutes > 0 || !parts.is_empty() {
        parts.push(format!("{}m", minutes));
    }
    
    if seconds > 0 || remaining_ms > 0 || parts.is_empty() {
        if remaining_ms > 0 {
            parts.push(format!("{}.{}s", seconds, remaining_ms));
        } else {
            parts.push(format!("{}s", seconds));
        }
    }
    
    parts.join(" ")
}

/// Convert a temperature from Celsius to Fahrenheit
pub fn celsius_to_fahrenheit(celsius: f32) -> f32 {
    celsius * 9.0 / 5.0 + 32.0
}

/// Convert a temperature from Fahrenheit to Celsius
pub fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

/// Convert an angle from degrees to radians
pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}

/// Convert an angle from radians to degrees
pub fn radians_to_degrees(radians: f32) -> f32 {
    radians * 180.0 / std::f32::consts::PI
}

/// Convert a string to a boolean value
pub fn string_to_bool(s: &str) -> Option<bool> {
    match s.to_lowercase().as_str() {
        "true" | "yes" | "y" | "1" | "on" => Some(true),
        "false" | "no" | "n" | "0" | "off" => Some(false),
        _ => None,
    }
}

/// Convert a hex color string (e.g., "#FF5500") to RGB components
pub fn hex_color_to_rgb(hex: &str) -> Result<(u8, u8, u8), &'static str> {
    let hex = hex.trim_start_matches('#');
    
    if hex.len() != 6 {
        return Err("Invalid hex color format (should be 6 characters)");
    }
    
    let r = u8::from_str_radix(&hex[0..2], 16)
        .map_err(|_| "Invalid hex character in red component")?;
    let g = u8::from_str_radix(&hex[2..4], 16)
        .map_err(|_| "Invalid hex character in green component")?;
    let b = u8::from_str_radix(&hex[4..6], 16)
        .map_err(|_| "Invalid hex character in blue component")?;
    
    Ok((r, g, b))
}

/// Convert RGB components to a hex color string (e.g., "#FF5500")
pub fn rgb_to_hex_color(r: u8, g: u8, b: u8) -> String {
    format!("#{:02X}{:02X}{:02X}", r, g, b)
} 



// ===============================================================


// use std::str::FromStr;

// pub fn str_to_u32(s: &str) -> Result<u32, ConversionError> {
//     u32::from_str(s).map_err(|_| ConversionError::new("Failed to convert string to u32"))
// }

// pub fn str_to_i32(s: &str) -> Result<i32, ConversionError> {
//     i32::from_str(s).map_err(|_| ConversionError::new("Failed to convert string to i32"))
// }

// pub fn str_to_f64(s: &str) -> Result<f64, ConversionError> {
//     f64::from_str(s).map_err(|_| ConversionError::new("Failed to convert string to f64"))
// }

// pub fn str_to_bool(s: &str) -> Result<bool, ConversionError> {
//     match s.to_lowercase().as_str() {
//         "true" | "yes" | "1" | "on" | "enable" | "enabled" => Ok(true),
//         "false" | "no" | "0" | "off" | "disable" | "disabled" => Ok(false),
//         _ => Err(ConversionError::new("Failed to convert string to bool")),
//     }
// }

// pub fn bytes_to_hex(bytes: &[u8]) -> String {
//     bytes.iter()
//         .map(|b| format!("{:02x}", b))
//         .collect()
// }

// pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, ConversionError> {
//     // Ensure the string has an even length
//     if hex.len() % 2 != 0 {
//         return Err(ConversionError::new("Hex string must have even length"));
//     }
    
//     // Process hex chars in pairs
//     let mut result = Vec::with_capacity(hex.len() / 2);
//     let mut chars = hex.chars();
    
//     while let (Some(h), Some(l)) = (chars.next(), chars.next()) {
//         // Convert hex chars to u8
//         let byte = hex_char_to_u8(h)? << 4 | hex_char_to_u8(l)?;
//         result.push(byte);
//     }
    
//     Ok(result)
// }

// fn hex_char_to_u8(c: char) -> Result<u8, ConversionError> {
//     match c {
//         '0'..='9' => Ok(c as u8 - b'0'),
//         'a'..='f' => Ok(c as u8 - b'a' + 10),
//         'A'..='F' => Ok(c as u8 - b'A' + 10),
//         _ => Err(ConversionError::new("Invalid hex character")),
//     }
// }

// pub fn bytes_to_string(bytes: &[u8]) -> Result<String, ConversionError> {
//     String::from_utf8(bytes.to_vec())
//         .map_err(|_| ConversionError::new("Failed to convert bytes to UTF-8 string"))
// }

// pub struct ConversionError {
//     message: String,
// }

// impl ConversionError {
//     pub fn new(message: &str) -> Self {
//         Self {
//             message: message.to_string(),
//         }
//     }
// }