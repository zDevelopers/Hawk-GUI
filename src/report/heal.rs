use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;

use chrono::{DateTime, FixedOffset};
use uuid::Uuid;

use crate::report::errors::{InvalidReportError, ReportResult};
use crate::report::player::{Player, SimplePlayer};
use crate::report::raw::Heal as RawHeal;
use crate::report::report::since;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Heal {
    pub date: DateTime<FixedOffset>,
    pub since_beginning: Duration,
    pub cause: HealCause,
    pub healed: SimplePlayer,
    pub heal: u16,
}

impl Heal {
    pub fn from_raw(
        raw_heal: RawHeal,
        players: &HashMap<Uuid, Rc<Player>>,
        begin: &DateTime<FixedOffset>,
    ) -> ReportResult<Self> {
        match players.get(&raw_heal.healed) {
            Some(healed) => Ok(Heal {
                date: raw_heal.date,
                since_beginning: since(&raw_heal.date, begin),
                cause: raw_heal.cause,
                healed: healed.into(),
                heal: raw_heal.heal,
            }),
            None => Err(InvalidReportError::MissingPlayerReference {
                uuid: raw_heal.healed,
            }),
        }
    }

    pub fn from_raw_vec(
        raw_heals: Vec<RawHeal>,
        players: &HashMap<Uuid, Rc<Player>>,
        begin: &DateTime<FixedOffset>,
    ) -> ReportResult<Vec<Self>> {
        let mut heals = raw_heals
            .into_iter()
            .map(|heal| Self::from_raw(heal, players, begin))
            .collect::<ReportResult<Vec<Self>>>()?;

        heals.sort_by_key(|h| h.date);

        Ok(heals)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HealCause {
    Natural,
    GoldenApple,
    NotchApple,
    HealingPotion,
    Command,
    Unknown,
}
