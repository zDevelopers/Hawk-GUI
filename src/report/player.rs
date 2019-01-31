use std::collections::HashMap;
use std::rc::Rc;

use uuid::Uuid;

use crate::report::raw::Player as RawPlayer;
use crate::report::raw::Team as RawTeam;
use crate::report::settings::SettingsPlayers;
use crate::report::team::TeamColor;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    pub uuid: Uuid,
    pub name: String,
    pub color: TeamColor,
    pub team: Option<String>,

    pub tag_line: String,
    pub tag_line_secondary: String,
    pub tag_line_details: String,

    pub statistics: Option<PlayerStatistics>,
    pub displayed_statistics: Option<DisplayedPlayerStatistics>
}

impl Player {
    pub fn from_raw(raw_player: &RawPlayer, teams: &Vec<RawTeam>, colors: &HashMap<Uuid, TeamColor>, default_color: &TeamColor, settings: &SettingsPlayers) -> Self {
        Self {
            uuid: raw_player.uuid.clone(),
            name: raw_player.name.clone(),
            color: colors.get(&raw_player.uuid).unwrap_or(default_color).clone(),
            tag_line: raw_player.tag_line.clone().unwrap_or("".to_string()).clone(),
            tag_line_secondary: raw_player.tag_line_secondary.clone().unwrap_or("".to_string()).clone(),
            tag_line_details: raw_player.tag_line_details.clone().unwrap_or("".to_string()).clone(),
            statistics: raw_player.statistics.clone(),
            displayed_statistics: match &raw_player.statistics {
                Some(statistics) => Some(DisplayedPlayerStatistics::calculate_displayed_statistics(statistics, settings)),
                None => None
            },
            team: teams.iter().find(|team| team.players.contains(&raw_player.uuid)).map(|team| team.name.clone())
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerStatistics {
    pub generic: Option<HashMap<String, u32>>,
    pub used: Option<HashMap<String, u32>>,
    pub mined: Option<HashMap<String, u32>>,
    pub picked_up: Option<HashMap<String, u32>>
}

///
/// Stores statistics as they will be displayed according
/// to the settings, separating those visible by default and
/// those hidden under a link because they are not highlighted.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DisplayedPlayerStatistics {
    pub generic: Option<DisplayedStatistics>,
    pub used: Option<DisplayedStatistics>,
    pub mined: Option<DisplayedStatistics>,
    pub picked_up: Option<DisplayedStatistics>
}

///
/// For a single piece of statistics (global, mined, …), stores the visible
/// ones and the by-default-hidden.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DisplayedStatistics {
    pub visible: HashMap<String, u32>,
    pub hidden: HashMap<String, u32>
}

impl DisplayedPlayerStatistics {
    pub fn calculate_displayed_statistics(statistics: &PlayerStatistics, settings: &SettingsPlayers) -> DisplayedPlayerStatistics {
        DisplayedPlayerStatistics {
            generic: match settings.global_statistics {
                true => match &statistics.generic {
                    Some(statistic) => Some(Self::calculate_displayed(statistic, &settings.statistics_whitelist, &settings.statistics_highlight)),
                    None => None
                }
                false => None
            },
            used: match settings.used {
                true => match &statistics.used {
                    Some(statistic) => Some(Self::calculate_displayed(statistic, &settings.used_whitelist, &settings.used_highlight)),
                    None => None
                }
                false => None
            },
            mined: match settings.mined {
                true => match &statistics.mined {
                    Some(statistic) => Some(Self::calculate_displayed(statistic, &settings.mined_whitelist, &settings.mined_highlight)),
                    None => None
                }
                false => None
            },
            picked_up: match settings.picked_up {
                true => match &statistics.picked_up {
                    Some(statistic) => Some(Self::calculate_displayed(statistic, &settings.picked_up_whitelist, &settings.picked_up_highlight)),
                    None => None
                }
                false => None
            }
        }
    }

    fn calculate_displayed(statistics: &HashMap<String, u32>, whitelist: &Vec<String>, highlight: &Vec<String>) -> DisplayedStatistics {
        let mut visible = HashMap::new();
        let mut hidden = HashMap::new();

        statistics.iter()
            .filter(|(stat, _val)| whitelist.is_empty() || whitelist.contains(stat))
            .for_each(|(stat, val)| {
                match highlight.is_empty() || highlight.contains(stat) {
                    true => &mut visible,
                    false => &mut hidden
                }.insert(stat.clone(), *val);
            });

        DisplayedStatistics { visible, hidden }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SimplePlayer {
    pub uuid: Uuid,
    pub name: String,
    pub color: TeamColor,
    pub team: Option<String>
}

impl From<Player> for SimplePlayer {
    fn from(player: Player) -> Self {
        SimplePlayer {
            uuid: player.uuid,
            name: player.name,
            color: player.color,
            team: player.team.clone()
        }
    }
}

impl From<&Player> for SimplePlayer {
    fn from(player: &Player) -> Self {
        SimplePlayer {
            uuid: player.uuid.clone(),
            name: player.name.clone(),
            color: player.color.clone(),
            team: player.team.clone()
        }
    }
}

impl From<Rc<Player>> for SimplePlayer {
    fn from(player: Rc<Player>) -> Self {
        SimplePlayer {
            uuid: player.uuid.clone(),
            name: player.name.clone(),
            color: player.color.clone(),
            team: player.team.clone()
        }
    }
}

impl From<&Rc<Player>> for SimplePlayer {
    fn from(player: &Rc<Player>) -> Self {
        SimplePlayer {
            uuid: player.uuid.clone(),
            name: player.name.clone(),
            color: player.color.clone(),
            team: player.team.clone()
        }
    }
}
