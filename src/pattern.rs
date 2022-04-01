use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Pattern {
    // index ?
    // patterns: Vec<String>,
    pub patterns: String,
    pub count: usize,
    #[serde(deserialize_with = "deserialize_samples")]
    pub samples: Vec<Sample>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Sample {
    pub predict: i32,
    pub date: DateTime<Utc>,
    pub rawlog: String,
}

fn deserialize_samples<'de, D>(deserializer: D) -> Result<Vec<Sample>, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = String::deserialize(deserializer)?;
    serde_json::from_str(&buf).map_err(serde::de::Error::custom)
}
