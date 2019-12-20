use std::time::Instant;
use chrono::{DateTime, Utc};

pub fn get_current_percent_between(start: &DateTime<Utc>, end: &DateTime<Utc>) -> f64 {
    let now = Utc::now();
    if start.timestamp_millis() < now.timestamp_millis() && start.timestamp_millis() < end.timestamp_millis() {
        let full_duration = end.signed_duration_since(start.clone());

        let remaining_duration = end.signed_duration_since(now);
        (full_duration.num_seconds() as f64 - remaining_duration.num_seconds() as f64)/ full_duration.num_seconds() as f64
    } else {
        0.0
    }

}