use std::time::Duration;

use chrono::{DateTime, FixedOffset};
use uuid::Uuid;

use crate::report::raw::Event as RawEvent;
use crate::report::report::since;

#[inline(always)] pub fn default_event_type() -> EventType { EventType::Blue }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    pub date: DateTime<FixedOffset>,
    pub since_beginning: Duration,

    #[serde(default = "default_event_type", rename = "type")]
    pub event_type: EventType,

    pub title: String,
    pub description: Option<String>,

    pub icon: EventIcon,
}

impl Event {
    pub fn from_raw(raw_event: &RawEvent, begin: &DateTime<FixedOffset>) -> Self {
        Event {
            date: raw_event.date.clone(),
            since_beginning: since(&raw_event.date, begin),
            event_type: raw_event.event_type.clone(),
            title: raw_event.title.clone(),
            description: raw_event.description.clone(),
            icon: raw_event.icon.clone()
        }
    }

    pub fn from_raw_vec(raw_events: &Vec<RawEvent>, begin: &DateTime<FixedOffset>) -> Vec<Self> {
        let mut events: Vec<Self> = raw_events.into_iter().map(|raw_event| Self::from_raw(raw_event, begin)).collect();

        events.sort_by(|a, b| a.date.cmp(&b.date));

        events
    }
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
