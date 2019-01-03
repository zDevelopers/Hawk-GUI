use std::collections::HashMap;
use std::rc::Rc;
use std::result::Result::Ok;
use std::time::Duration;

use chrono::{DateTime, FixedOffset};
use uuid::Uuid;

use crate::report::*;
use crate::report::errors::ReportResult;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Report {
    pub match_uuid: Uuid,
    pub title: String,
    pub date: DateTime<FixedOffset>,
    pub settings: settings::Settings,
    pub players: Vec<player::Player>,
    pub teams: Vec<team::Team>,
    pub damages: Vec<damage::Damage>,
    pub heals: Vec<heal::Heal>,
    pub events: Vec<event::Event>,
    pub aggregates: aggregates::Aggregate,
}

impl Report {
    ///
    /// Processes a raw report given by an user
    /// and returns a ready-to-be-serialized-and-used
    /// structure.
    ///
    pub fn from_raw<'a>(raw_report: raw::Report) -> ReportResult<Self> {
        let players_colors = {
            let mut players_colors = HashMap::new();

            for team in &raw_report.teams {
                for player in &team.players {
                    players_colors.insert(player.clone(), team.color.clone());
                }
            }

            players_colors
        };

        let players: HashMap<Uuid, Rc<player::Player>> = (&raw_report.players).into_iter()
            .map(|player| (
                player.uuid,
                Rc::new(player::Player::from_raw(player, &players_colors, &raw::default_team_color()))
            ))
            .collect();

        let begin = &raw_report.date;

        let damages = damage::Damage::from_raw_vec(&raw_report.damages, &players, &begin)?;
        let heals = heal::Heal::from_raw_vec(&raw_report.heals, &players, &begin)?;

        Ok(Report {
            match_uuid: raw_report.match_uuid.clone(),
            title: raw_report.title.clone(),
            date: raw_report.date.clone(),
            settings: raw_report.settings.clone(),
            players: players.iter().map(|(_, player)| (*player.as_ref()).clone()).collect(),
            teams: team::Team::from_raw_vec(&raw_report.teams, &players)?,
            events: event::Event::from_raw_vec(&raw_report.events, &begin),
            aggregates: aggregates::Aggregate::from_raw(&players, &damages, &heals),
            damages,
            heals,
        })
    }
}

pub fn since(now: &DateTime<FixedOffset>, before: &DateTime<FixedOffset>) -> Duration {
    (now.clone() - before.clone()).to_std().unwrap_or(Duration::new(0, 0))
}
