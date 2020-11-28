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
    pub fn from_raw(raw_team: &RawTeam, players: &HashMap<Uuid, Rc<Player>>) -> ReportResult<Self> {
        let team_players = {
            let mut team_players: Vec<SimplePlayer> = Vec::new();

            for player in &raw_team.players {
                match players.get(&player) {
                    Some(player) => team_players.push(player.as_ref().into()),
                    None => {
                        return Err(InvalidReportError::MissingPlayerReference { uuid: *player })
                    }
                }
            }

            team_players
        };

        Ok(Team {
            name: raw_team.name.clone(),
            color: raw_team.color.clone(),
            players: team_players,
        })
    }

    pub fn from_raw_vec(
        raw_teams: &Vec<RawTeam>,
        players: &HashMap<Uuid, Rc<Player>>,
    ) -> ReportResult<Vec<Self>> {
        let mut teams = Vec::new();

        for team in raw_teams {
            teams.push(Self::from_raw(team, players)?);
        }

        Ok(teams)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
