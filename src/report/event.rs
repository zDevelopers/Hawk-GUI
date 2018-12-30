use chrono::{DateTime, FixedOffset};
use uuid::Uuid;

fn default_event_type() -> EventType { EventType::Blue }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    pub date: DateTime<FixedOffset>,

    #[serde(default = "default_event_type", rename = "type")]
    pub event_type: EventType,

    pub title: String,
    pub description: Option<String>,

    pub icon: EventIcon,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EventType {
    Blue,
    Gold,
    Green,
    Red
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum EventIcon {
    Player { uuid: Uuid },
    Icon { icon_id: String },
    Url { url: String }
}
