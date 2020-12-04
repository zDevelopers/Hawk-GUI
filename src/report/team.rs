use std::collections::HashMap;
use std::rc::Rc;

use uuid::Uuid;

use crate::report::errors::{InvalidReportError, ReportResult};
use crate::report::player::{Player, SimplePlayer};
use crate::report::raw::Team as RawTeam;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Team {
    pub name: String,
    pub color: TeamColor,

    pub players: Vec<SimplePlayer>,
}

impl Team {
    pub fn from_raw(raw_team: RawTeam, players: &HashMap<Uuid, Rc<Player>>) -> ReportResult<Self> {
        let team_players: ReportResult<Vec<SimplePlayer>> = raw_team.players.into_iter()
            .map(|p| players.get(&p)
                .map(Into::into)
                .ok_or(InvalidReportError::MissingPlayerReference { uuid: p }))
            .collect();

        Ok(Team {
            name: raw_team.name,
            color: raw_team.color,
            players: team_players?,
        })
    }

    pub fn from_raw_vec(
        raw_teams: Vec<RawTeam>,
        players: &HashMap<Uuid, Rc<Player>>,
    ) -> ReportResult<Vec<Self>> {
        raw_teams.into_iter().map(|team| Self::from_raw(team, players)).collect()
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TeamColor {
    Aqua,
    Black,
    Blue,
    DarkAqua,
    DarkBlue,
    DarkGray,
    DarkGreen,
    DarkPurple,
    DarkRed,
    Gold,
    Gray,
    Green,
    LightPurple,
    Red,
    White,
    Yellow,

    None,
}

impl Default for TeamColor {
    #[inline]
    fn default() -> Self {
        TeamColor::None
    }
}
