use std::collections::{BTreeMap, HashMap};
use std::rc::Rc;
use std::time::Duration;

use chrono::{DateTime, FixedOffset};
use uuid::Uuid;

use crate::report::damage::{Damage, DamageCause};
use crate::report::heal::Heal;
use crate::report::player::{Player, PlayerStatistics, DisplayedPlayerStatistics, SimplePlayer};
use crate::report::report::since;
use crate::report::settings::SettingsPlayers;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Aggregate {
    pub global_statistics: DisplayedPlayerStatistics,
    pub players_damages: BTreeMap<Uuid, PlayerAlterationsAggregate>,
    pub environmental_damages: EnvironmentalDamagesAggregate,
}

impl Aggregate {
    pub fn from_raw(
        players: &HashMap<Uuid, Rc<Player>>,
        damages: &Vec<Damage>,
        heals: &Vec<Heal>,
        begin: &DateTime<FixedOffset>,
        settings: &SettingsPlayers,
    ) -> Self {
        Aggregate {
            global_statistics: Self::aggregate_global_statistics(
                &players
                    .iter()
                    .filter_map(|(_uuid, player)| (player.as_ref()).statistics.clone())
                    .collect(),
                settings
            ),
            players_damages: Self::aggregate_alterations(players, damages, heals, begin),
            environmental_damages: Self::aggregate_environmental_damages(damages),
        }
    }

    fn aggregate_global_statistics(statistics: &Vec<PlayerStatistics>, settings: &SettingsPlayers) -> DisplayedPlayerStatistics {
        DisplayedPlayerStatistics::calculate_displayed_statistics(&PlayerStatistics {
            generic: Some(Self::aggregate_single_statistic_group(
                &statistics
                    .iter()
                    .filter_map(|statistic| statistic.generic.clone())
                    .collect(),
            )),
            used: Some(Self::aggregate_single_statistic_group(
                &statistics
                    .iter()
                    .filter_map(|statistic| statistic.used.clone())
                    .collect(),
            )),
            mined: Some(Self::aggregate_single_statistic_group(
                &statistics
                    .iter()
                    .filter_map(|statistic| statistic.mined.clone())
                    .collect(),
            )),
            picked_up: Some(Self::aggregate_single_statistic_group(
                &statistics
                    .iter()
                    .filter_map(|statistic| statistic.picked_up.clone())
                    .collect(),
            )),
        }, settings)
    }

    fn aggregate_single_statistic_group(
        statistics: &Vec<BTreeMap<String, u32>>,
    ) -> BTreeMap<String, u32> {
        let mut aggregated = BTreeMap::new();

        statistics
            .iter()
            .cloned()
            .flatten()
            .for_each(|(stat, val)| {
                aggregated.insert(
                    stat.clone(),
                    match aggregated.get(&stat) {
                        Some(prev_value) => prev_value + val,
                        None => val,
                    },
                );
            });

        aggregated
            .into_iter()
            .filter(|(_stat, val)| val != &0u32)
            .collect()
    }

    fn aggregate_alterations(
        players: &HashMap<Uuid, Rc<Player>>,
        damages: &Vec<Damage>,
        heals: &Vec<Heal>,
        begin: &DateTime<FixedOffset>,
    ) -> BTreeMap<Uuid, PlayerAlterationsAggregate> {
        let ranks: BTreeMap<Uuid, u8> = {
            let mut lethal_damages: Vec<&Damage> =
                damages.iter().filter(|damage| damage.lethal).collect();
            lethal_damages.sort_by(|a, b| a.date.cmp(&b.date).reverse());

            let ranked_deads: Vec<Uuid> = lethal_damages
                .iter()
                .map(|damage| damage.damagee.uuid)
                .collect();

            players
                .iter()
                .map(|(uuid, _player)| uuid)
                .filter(|uuid| !ranked_deads.contains(uuid))
                .chain(ranked_deads.iter())
                .enumerate()
                .map(|(index, uuid)| (uuid.clone(), index as u8 + 1))
                .collect()
        };

        players
            .iter()
            .map(|(uuid, _player)| {
                (
                    uuid.clone(),
                    Self::aggregate_player_alterations(
                        uuid,
                        damages,
                        heals,
                        *ranks.get(uuid).unwrap_or(&0u8),
                        begin,
                    ),
                )
            })
            .collect()
    }

    fn aggregate_player_alterations(
        player: &Uuid,
        damages: &Vec<Damage>,
        heals: &Vec<Heal>,
        rank: u8,
        begin: &DateTime<FixedOffset>,
    ) -> PlayerAlterationsAggregate {
        let mut damages_taken: Vec<Damage> = damages
            .iter()
            .cloned()
            .filter(|damage| &damage.damagee.uuid == player)
            .collect();
        let mut damages_caused: Vec<Damage> = damages
            .iter()
            .cloned()
            .filter(|damage| match &damage.cause {
                DamageCause::Player(cause) => &cause.player.uuid == player,
                _ => false
            })
            .collect();
        let mut heals: Vec<Heal> = heals
            .iter()
            .cloned()
            .filter(|heal| &heal.healed.uuid == player)
            .collect();

        damages_taken.sort_by_key(|d| d.date);
        damages_caused.sort_by_key(|d| d.date);
        heals.sort_by_key(|h| h.date);

        PlayerAlterationsAggregate {
            damages_taken_total: (&damages_taken)
                .iter()
                .fold(0u32, |acc, damage| damage.damage as u32 + acc),
            damages_caused_total: (&damages_caused)
                .iter()
                .fold(0u32, |acc, damage| damage.damage as u32 + acc),
            heals_total: (&heals)
                .iter()
                .fold(0u32, |acc, heal| heal.heal as u32 + acc),

            damages_taken,
            damages_caused,
            heals,

            kills: damages
                .iter()
                .filter(|damage| match &damage.cause {
                    DamageCause::Player(cause) => &cause.player.uuid == player && damage.lethal,
                    _ => false
                })
                .map(|damage| damage.damagee.clone())
                .collect(),

            killed_by: damages
                .iter()
                .filter(|damage| &damage.damagee.uuid == player && damage.lethal)
                .map(|damage| damage.cause.clone())
                .last(),

            game_duration: match damages
                .iter()
                .filter(|damage| &damage.damagee.uuid == player && damage.lethal)
                .map(|damage| &damage.date)
                .last()
            {
                Some(date) => since(date, begin),
                None => damages
                    .iter()
                    .last()
                    .map(|damage| since(&damage.date, begin))
                    .unwrap_or(Duration::from_secs(0)),
            },

            rank,
        }
    }

    fn aggregate_environmental_damages(damages: &Vec<Damage>) -> EnvironmentalDamagesAggregate {
        let mut aggregated = EnvironmentalDamagesAggregate {
            entities: BTreeMap::new(),
            environment: BTreeMap::new()
        };

        damages
            .iter()
            .filter(|damage| !matches!(damage.cause, DamageCause::Player { .. }))
            .for_each(|damage| {
                let aggregate = match damage.cause {
                    DamageCause::Entity(_) => &mut aggregated.entities,
                    _ => &mut aggregated.environment
                };

                let key = match &damage.cause {
                   DamageCause::Entity(cause) => cause.entity.clone(),
                    _ => damage.cause.to_string()
                };

                aggregate.insert(
                    key.clone(),
                    match aggregate.get(&key.clone()) {
                        Some(aggregate) => aggregate + damage.damage as u32,
                        None => damage.damage as u32,
                    },
                );
            });

        aggregated
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerAlterationsAggregate {
    pub damages_taken: Vec<Damage>,
    pub damages_taken_total: u32,
    pub damages_caused: Vec<Damage>,
    pub damages_caused_total: u32,
    pub heals: Vec<Heal>,
    pub heals_total: u32,
    pub kills: Vec<SimplePlayer>,
    pub killed_by: Option<DamageCause>,
    pub game_duration: Duration,
    pub rank: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnvironmentalDamagesAggregate {
    pub entities: BTreeMap<String, u32>,
    pub environment: BTreeMap<String, u32>
}
