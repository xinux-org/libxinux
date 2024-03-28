use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Deserializer};

// Custom function to deserialize Unix timestamps
pub fn deserialize_unix_timestamp<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let timestamp = i64::deserialize(deserializer)?;
    Utc.timestamp_opt(timestamp, 0)
        .single()
        .ok_or_else(|| serde::de::Error::custom("Invalid Unix timestamp"))
}
