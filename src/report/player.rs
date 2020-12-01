use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;

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
    pub displayed_statistics: Option<DisplayedPlayerStatistics>,
}

impl Player {
    pub fn from_raw(
        raw_player: &RawPlayer,
        teams: &Vec<RawTeam>,
        colors: &HashMap<Uuid, TeamColor>,
        default_color: &TeamColor,
        settings: &SettingsPlayers,
    ) -> Self {
        Self {
            uuid: raw_player.uuid.clone(),
            name: raw_player.name.clone(),
            color: colors
                .get(&raw_player.uuid)
                .unwrap_or(default_color)
                .clone(),
            tag_line: raw_player
                .tag_line
                .clone()
                .unwrap_or("".to_string())
                .clone(),
            tag_line_secondary: raw_player
                .tag_line_secondary
                .clone()
                .unwrap_or("".to_string())
                .clone(),
            tag_line_details: raw_player
                .tag_line_details
                .clone()
                .unwrap_or("".to_string())
                .clone(),
            statistics: raw_player.statistics.clone(),
            displayed_statistics: match &raw_player.statistics {
                Some(statistics) => Some(
                    DisplayedPlayerStatistics::calculate_displayed_statistics(statistics, settings),
                ),
                None => None,
            },
            team: teams
                .iter()
                .find(|team| team.players.contains(&raw_player.uuid))
                .map(|team| team.name.clone()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerStatistics {
    pub generic: Option<HashMap<String, u32>>,
    pub used: Option<HashMap<String, u32>>,
    pub mined: Option<HashMap<String, u32>>,
    pub picked_up: Option<HashMap<String, u32>>,
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
    pub picked_up: Option<DisplayedStatistics>,
}

///
/// For a single piece of statistics (global, mined, …), stores the visible
/// ones and the by-default-hidden.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DisplayedStatistics {
    pub visible: Vec<Statistic>,
    pub hidden: Vec<Statistic>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Statistic {
    Duration {
        id: String,
        raw: u32,
        duration: Duration,
    },
    Distance {
        id: String,
        raw: u32,
        distance: f32,
        unit: String,
    },
    Hearts {
        id: String,
        raw: u32,
        hearts: f32,
    },
    Times {
        id: String,
        raw: u32,
    },
    Number {
        id: String,
        raw: u32,
    },
}

impl DisplayedPlayerStatistics {
    pub fn calculate_displayed_statistics(
        statistics: &PlayerStatistics,
        settings: &SettingsPlayers,
    ) -> DisplayedPlayerStatistics {
        DisplayedPlayerStatistics {
            generic: match settings.global_statistics {
                true => match &statistics.generic {
                    Some(statistic) => Some(Self::calculate_displayed(
                        statistic,
                        &settings.statistics_whitelist,
                        &settings.statistics_highlight,
                    )),
                    None => None,
                },
                false => None,
            },
            used: match settings.used {
                true => match &statistics.used {
                    Some(statistic) => Some(Self::calculate_displayed(
                        statistic,
                        &settings.used_whitelist,
                        &settings.used_highlight,
                    )),
                    None => None,
                },
                false => None,
            },
            mined: match settings.mined {
                true => match &statistics.mined {
                    Some(statistic) => Some(Self::calculate_displayed(
                        statistic,
                        &settings.mined_whitelist,
                        &settings.mined_highlight,
                    )),
                    None => None,
                },
                false => None,
            },
            picked_up: match settings.picked_up {
                true => match &statistics.picked_up {
                    Some(statistic) => Some(Self::calculate_displayed(
                        statistic,
                        &settings.picked_up_whitelist,
                        &settings.picked_up_highlight,
                    )),
                    None => None,
                },
                false => None,
            },
        }
    }

    fn calculate_displayed(
        statistics: &HashMap<String, u32>,
        whitelist: &Vec<String>,
        highlight: &Vec<String>,
    ) -> DisplayedStatistics {
        let mut visible = Vec::new();
        let mut hidden = Vec::new();

        statistics.iter()
            .filter(|(stat, val)| (whitelist.is_empty() || whitelist.contains(stat)) && **val != 0)
            .for_each(|(stat, val)| {
                match highlight.is_empty() || highlight.contains(stat) {
                    true => &mut visible,
                    false => &mut hidden
                }.push(match stat.replace("minecraft.custom:", "").to_lowercase().as_str() {
                    "minecraft.play_one_minute" |
                    "minecraft.time_since_death" |
                    "minecraft.sneak_time" |
                    "minecraft.time_since_rest" => Statistic::Duration {
                        id: stat.clone(),
                        raw: *val,
                        duration: Duration::from_secs((*val / 20).into())
                    },

                    "minecraft.walk_one_cm" |
                    "minecraft.crouch_one_cm" |
                    "minecraft.sprint_one_cm" |
                    "minecraft.swim_one_cm" |
                    "minecraft.fall_one_cm" |
                    "minecraft.climb_one_cm" |
                    "minecraft.fly_one_cm" |
                    "minecraft.walk_under_water_one_cm" |
                    "minecraft.dive_one_cm" |  // legacy
                    "minecraft.minecart_one_cm" |
                    "minecraft.boat_one_cm" |
                    "minecraft.pig_one_cm" |
                    "minecraft.horse_one_cm" |
                    "minecraft.aviate_one_cm" |
                    "minecraft.walk_on_water_one_cm" => {
                        let meters = *val as f32 / 100_f32;
                        if meters < 1000_f32 {
                            Statistic::Distance {
                                id: stat.clone(),
                                raw: *val,
                                distance: meters,
                                unit: String::from("m")
                            }
                        }
                        else {
                            Statistic::Distance {
                                id: stat.clone(),
                                raw: *val,
                                distance: meters / 1000_f32,
                                unit: String::from("km")
                            }
                        }
                    },

                    "minecraft.damage_dealt" |
                    "minecraft.damage_dealt_absorbed" |
                    "minecraft.damage_dealt_resisted" |
                    "minecraft.damage_taken" |
                    "minecraft.damage_absorbed" |
                    "minecraft.damage_resisted" |
                    "minecraft.damage_blocked_by_shield" => Statistic::Hearts {
                        id: stat.clone(),
                        raw: *val,
                        hearts: (*val as f32 / 20_f32).into()  // These are stored in tenths of life points, i.e. 20th of hearts
                    },

                    "minecraft.interact_with_brewingstand" |
                    "minecraft.interact_with_beacon" |
                    "minecraft.interact_with_crafting_table" |
                    "minecraft.interact_with_furnace" |
                    "minecraft.interact_with_blast_furnace" |
                    "minecraft.interact_with_campfire" |
                    "minecraft.interact_with_cartography_table" |
                    "minecraft.interact_with_lectern" |
                    "minecraft.interact_with_loom" |
                    "minecraft.interact_with_smoker" |
                    "minecraft.interact_with_stonecutter" |
                    "minecraft.inspect_dispenser" |
                    "minecraft.inspect_dropper" |
                    "minecraft.inspect_hopper" |
                    "minecraft.open_chest" |
                    "minecraft.trigger_trapped_chest" |
                    "minecraft.open_enderchest" |
                    "minecraft.play_noteblock" |
                    "minecraft.tune_noteblock" |
                    "minecraft.eat" |  // legacy
                    "minecraft.talked_to_villager" |
                    "minecraft.traded_with_villager" => Statistic::Times { id: stat.clone(), raw: *val },

                    _ => Statistic::Number { id: stat.clone(), raw: *val }
                });
            });

        // We want to sort the statistics by type (Duration -> Distance -> Hearts -> Times -> Number),
        // then by name for the Durations/Hearts and by number for the three others.
        Self::sort_statistics_list(&mut visible);
        Self::sort_statistics_list(&mut hidden);

        DisplayedStatistics { visible, hidden }
    }

    fn sort_statistics_list(stats_list: &mut Vec<Statistic>) -> () {
        let mut durations = Vec::new();
        let mut distances = Vec::new();
        let mut hearts = Vec::new();
        let mut times = Vec::new();
        let mut numbers = Vec::new();

        stats_list.iter().for_each(|stat| {
            match &stat {
                Statistic::Duration { .. } => &mut durations,
                Statistic::Distance { .. } => &mut distances,
                Statistic::Hearts { .. } => &mut hearts,
                Statistic::Times { .. } => &mut times,
                Statistic::Number { .. } => &mut numbers,
            }
            .push(stat.clone())
        });

        Self::sort_statistics_same_type_list(&mut durations);
        Self::sort_statistics_same_type_list(&mut distances);
        Self::sort_statistics_same_type_list(&mut hearts);
        Self::sort_statistics_same_type_list(&mut times);
        Self::sort_statistics_same_type_list(&mut numbers);

        stats_list.clear();

        stats_list.extend(durations);
        stats_list.extend(distances);
        stats_list.extend(hearts);
        stats_list.extend(times);
        stats_list.extend(numbers);
    }

    fn sort_statistics_same_type_list(stats_list: &mut Vec<Statistic>) {
        let default_str = &String::new();
        stats_list.sort_by(|stat_1, stat_2| match stat_1 {
            Statistic::Duration { .. } | Statistic::Hearts { .. } => match stat_1 {
                Statistic::Duration { id, .. } => id,
                Statistic::Hearts { id, .. } => id,
                _ => default_str,
            }
            .cmp(&match stat_2 {
                Statistic::Duration { id, .. } => id.clone(),
                Statistic::Hearts { id, .. } => id.clone(),
                _ => String::new(),
            }),
            _ => match stat_1 {
                Statistic::Distance { raw, .. } => *raw,
                Statistic::Times { raw, .. } => *raw,
                Statistic::Number { raw, .. } => *raw,
                _ => 0u32,
            }
            .cmp(match stat_2 {
                Statistic::Distance { raw, .. } => raw,
                Statistic::Times { raw, .. } => raw,
                Statistic::Number { raw, .. } => raw,
                _ => &0u32,
            })
            .reverse(),
        });
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub struct SimplePlayer {
    pub uuid: Uuid,
    pub name: String,
    pub color: TeamColor,
    pub team: Option<String>,
}

impl From<Player> for SimplePlayer {
    fn from(player: Player) -> Self {
        SimplePlayer {
            uuid: player.uuid,
            name: player.name,
            color: player.color,
            team: player.team.clone(),
        }
    }
}

impl From<&Player> for SimplePlayer {
    fn from(player: &Player) -> Self {
        SimplePlayer {
            uuid: player.uuid.clone(),
            name: player.name.clone(),
            color: player.color.clone(),
            team: player.team.clone(),
        }
    }
}

impl From<Rc<Player>> for SimplePlayer {
    fn from(player: Rc<Player>) -> Self {
        SimplePlayer {
            uuid: player.uuid.clone(),
            name: player.name.clone(),
            color: player.color.clone(),
            team: player.team.clone(),
        }
    }
}

impl From<&Rc<Player>> for SimplePlayer {
    fn from(player: &Rc<Player>) -> Self {
        SimplePlayer {
            uuid: player.uuid.clone(),
            name: player.name.clone(),
            color: player.color.clone(),
            team: player.team.clone(),
        }
    }
}
