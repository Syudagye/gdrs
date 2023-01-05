use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomLevel {
    #[serde(rename = "k1")]
    level_id: Option<i64>,

    #[serde(rename = "k2")]
    level_name: String,

    #[serde(rename = "k3")]
    description: Option<String>,

    #[serde(rename = "k4")]
    level_string: String,

    #[serde(rename = "k5")]
    creator_name: String,

    #[serde(rename = "k6")]
    creator_user_id: Option<i64>,

    #[serde(rename = "k7")]
    level_difficulty: Option<i64>,

    #[serde(rename = "k8")]
    official_song_id: Option<i64>,

    #[serde(rename = "k9")]
    rating: Option<i64>,

    // TODO: implement all fields
}
