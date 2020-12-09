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
    pub minecraft: Option<String>,
    pub settings: settings::Settings,
    pub players: Vec<player::Player>,
    pub teams: Vec<team::Team>,
    pub winners: Vec<player::SimplePlayer>,
    pub damages: Vec<damage::Damage>,
    pub heals: Vec<heal::Heal>,
    pub events: Vec<event::Event>,
    pub aggregates: aggregates::Aggregate,
    pub has_players_without_team: bool,
}

impl Report {
    ///
    /// Processes a raw report given by an user
    /// and returns a ready-to-be-serialized-and-used
    /// structure.
    ///
    pub fn from_raw(raw_report: raw::Report) -> ReportResult<Self> {
        let players_colors = {
            let mut players_colors = HashMap::new();

            for team in &raw_report.teams {
                for player in &team.players {
                    players_colors.insert(*player, team.color);
                }
            }

            players_colors
        };

        let settings = raw_report.settings;
        let teams = raw_report.teams;

        let players: HashMap<Uuid, Rc<player::Player>> = raw_report.players
            .into_iter()
            .map(|player| {
                (
                    player.uuid,
                    Rc::new(player::Player::from_raw(
                        player,
                        &teams,
                        &players_colors,
                        &settings.players,
                    )),
                )
            })
            .collect();

        let begin = raw_report.date;

        let mut damages = raw_report.damages;
        // Damage::from_raw_vec expect damages to be sorted by chronological order.
        damages.sort_by_key(|d| d.date);

        let damages = damage::Damage::from_raw_vec(damages, &players, &begin)?;
        let heals = heal::Heal::from_raw_vec(raw_report.heals, &players, &begin)?;

        let winners = match raw_report.winners {
            Some(winners) if !winners.is_empty() => {
                let mut team_players = winners.iter()
                    .map(|p| players.get(&p)
                        .map(Into::into)
                        .ok_or(errors::InvalidReportError::MissingPlayerReference { uuid: *p }))
                    .collect::<ReportResult<Vec<player::SimplePlayer>>>()?;

                team_players.sort_by(|a, b| a.name.cmp(&b.name));

                team_players
            },
            _ => Self::extract_winners(&players, &damages),
        };

        let aggregates = aggregates::Aggregate::from_raw(&players, &damages, &heals, &begin, &settings.players);

        let mut players_list: Vec<player::Player> = players
            .iter()
            .map(|(_, player)| player.as_ref().clone())
            .collect();

        players_list.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(Report {
            match_uuid: raw_report.match_uuid,
            title: raw_report.title,
            date: raw_report.date,
            minecraft: raw_report.minecraft,
            settings,
            players: players_list,
            teams: team::Team::from_raw_vec(teams, &players)?,
            events: event::Event::from_raw_vec(raw_report.events, &begin),
            aggregates,
            winners,
            damages,
            heals,
            has_players_without_team: players
                .iter()
                .any(|(_uuid, player)| player.team.is_none()),
        })
    }

    fn extract_winners(
        players: &HashMap<Uuid, Rc<player::Player>>,
        damages: &Vec<damage::Damage>,
    ) -> Vec<player::SimplePlayer> {
        let deads: Vec<Uuid> = damages
            .iter()
            .filter(|damage| damage.lethal)
            .map(|damage| damage.damagee.uuid)
            .collect();

        let mut winners: Vec<player::SimplePlayer> = players
            .iter()
            .map(|(_uuid, player)| player)
            .filter(|player| !deads.contains(&player.as_ref().uuid))
            .map(|player| player.as_ref().into())
            .collect();

        winners.sort_by(|a, b| a.name.cmp(&b.name));

        winners
    }
}

pub fn since(now: &DateTime<FixedOffset>, before: &DateTime<FixedOffset>) -> Duration {
    (now.clone() - before.clone())
        .to_std()
        .unwrap_or(Duration::new(0, 0))
}
