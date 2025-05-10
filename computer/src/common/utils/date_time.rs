// Time utilities for all devices
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Get the current Unix timestamp in seconds
pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Get the current Unix timestamp in milliseconds
pub fn current_timestamp_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

/// Format a Unix timestamp into a human-readable string (UTC)
pub fn format_timestamp(timestamp: u64) -> String {
    // This is a simple implementation without using an external date/time library
    // In a real system, you'd use chrono or a similar library
    
    // Convert to days, hours, minutes, seconds
    const SECONDS_PER_MINUTE: u64 = 60;
    const SECONDS_PER_HOUR: u64 = 60 * SECONDS_PER_MINUTE;
    const SECONDS_PER_DAY: u64 = 24 * SECONDS_PER_HOUR;
    
    // Unix epoch starts at 1970-01-01
    let days_since_epoch = timestamp / SECONDS_PER_DAY;
    
    // Basic algorithm to convert days since epoch to date
    // Note: This doesn't handle leap years correctly for simplicity
    let year = 1970 + (days_since_epoch / 365);
    let day_of_year = days_since_epoch % 365;
    
    const DAYS_IN_MONTH: [u64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut month = 0;
    let mut day = day_of_year;
    
    for (i, &days) in DAYS_IN_MONTH.iter().enumerate() {
        if day < days {
            month = i + 1;
            break;
        }
        day -= days;
    }
    
    // Adjust day to be 1-based
    day += 1;
    
    // Time of day
    let seconds_of_day = timestamp % SECONDS_PER_DAY;
    let hour = seconds_of_day / SECONDS_PER_HOUR;
    let minute = (seconds_of_day % SECONDS_PER_HOUR) / SECONDS_PER_MINUTE;
    let second = seconds_of_day % SECONDS_PER_MINUTE;
    
    format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02} UTC", 
        year, month, day, hour, minute, second)
}

/// Get the elapsed time since a past timestamp in milliseconds
pub fn elapsed_ms_since(past_timestamp_ms: u128) -> u128 {
    let now = current_timestamp_ms();
    if now > past_timestamp_ms {
        now - past_timestamp_ms
    } else {
        0
    }
}

/// Add a duration to a timestamp
pub fn add_duration(timestamp: u64, duration: Duration) -> u64 {
    timestamp.saturating_add(duration.as_secs())
}

/// Calculate the time difference between two timestamps in seconds
pub fn time_diff(timestamp1: u64, timestamp2: u64) -> i64 {
    timestamp1 as i64 - timestamp2 as i64
}

/// Check if a timestamp has expired
pub fn is_expired(timestamp: u64, duration: Duration) -> bool {
    let now = current_timestamp();
    now > add_duration(timestamp, duration)
}

/// Get a timestamp for a future time by adding seconds to now
pub fn future_timestamp(seconds_from_now: u64) -> u64 {
    add_duration(current_timestamp(), Duration::from_secs(seconds_from_now))
}

/// Calculate uptime from a start timestamp
pub fn uptime(start_timestamp: u64) -> Duration {
    let now = current_timestamp();
    if now > start_timestamp {
        Duration::from_secs(now - start_timestamp)
    } else {
        Duration::from_secs(0)
    }
}

/// Format a duration as a human-readable string
pub fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    
    let days = total_seconds / (24 * 3600);
    let hours = (total_seconds % (24 * 3600)) / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    
    if days > 0 {
        format!("{}d {}h {}m {}s", days, hours, minutes, seconds)
    } else if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } else {
        format!("{}s", seconds)
    }
}


// ============================================================


// use chrono::{DateTime, Duration, Utc};

// pub fn now() -> DateTime<Utc> {
//     Utc::now()
// }

// pub fn format_duration(duration: Duration) -> String {
//     let seconds = duration.num_seconds();
//     let minutes = seconds / 60;
//     let hours = minutes / 60;
//     let days = hours / 24;
    
//     if days > 0 {
//         format!("{}d {}h {}m {}s", days, hours % 24, minutes % 60, seconds % 60)
    
//     } else if hours > 0 {
//         format!("{}h {}m {}s", hours, minutes % 60, seconds % 60)
//     } else if minutes > 0 {
//         format!("{}m {}s", minutes, seconds % 60)
//     } else {
//         format!("{}s", seconds)
//     }
// }

// pub fn is_expired(expiration: &DateTime<Utc>) -> bool {
//     Utc::now() > *expiration
// }

// pub fn add_timeout(duration: Duration) -> DateTime<Utc> {
//     Utc::now() + duration
// }

// pub fn time_until(target: &DateTime<Utc>) -> Duration {
//     if *target > Utc::now() {
//         *target - Utc::now()
//     } else {
//         Duration::zero()
//     }
// }

// pub fn format_timestamp(dt: &DateTime<Utc>, format: TimestampFormat) -> String {
//     match format {
//         TimestampFormat::ISO8601 => dt.to_rfc3339(),
//         TimestampFormat::RFC2822 => dt.to_rfc2822(),
//         TimestampFormat::Compact => dt.format("%Y%m%d%H%M%S").to_string(),
//         TimestampFormat::Human => dt.format("%Y-%m-%d %H:%M:%S").to_string(),
//     }
// }

// pub enum TimestampFormat {
//     ISO8601,
//     RFC2822,
//     Compact,
//     Human,
// }