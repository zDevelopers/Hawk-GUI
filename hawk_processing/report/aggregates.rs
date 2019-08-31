use std::collections::HashMap;
use std::rc::Rc;

use uuid::Uuid;

use crate::report::damage::{Damage, DamageCause};
use crate::report::heal::Heal;
use crate::report::player::{Player, PlayerStatistics, SimplePlayer};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Aggregate {
    pub global_statistics: PlayerStatistics,
    pub players_damages: HashMap<Uuid, PlayerAlterationsAggregate>,
    pub environmental_damages: HashMap<DamageCause, u32>,
}

impl Aggregate {
    pub fn from_raw(players: &HashMap<Uuid, Rc<Player>>, damages: &Vec<Damage>, heals: &Vec<Heal>) -> Self {
        Aggregate {
            global_statistics: Self::aggregate_global_statistics(&players.iter().filter_map(|(_uuid, player)| (player.as_ref()).statistics.clone()).collect()),
            players_damages: Self::aggregate_alterations(players, damages, heals),
            environmental_damages: Self::aggregate_environmental_damages(damages)
        }
    }

    fn aggregate_global_statistics(statistics: &Vec<PlayerStatistics>) -> PlayerStatistics {
        PlayerStatistics {
            generic:   Some(Self::aggregate_single_statistic_group(&statistics.iter().filter_map(|statistic| statistic.generic.clone()).collect())),
            used:      Some(Self::aggregate_single_statistic_group(&statistics.iter().filter_map(|statistic| statistic.used.clone()).collect())),
            mined:     Some(Self::aggregate_single_statistic_group(&statistics.iter().filter_map(|statistic| statistic.mined.clone()).collect())),
            picked_up: Some(Self::aggregate_single_statistic_group(&statistics.iter().filter_map(|statistic| statistic.picked_up.clone()).collect()))
        }
    }

    fn aggregate_single_statistic_group(statistics: &Vec<HashMap<String, u32>>) -> HashMap<String, u32> {
        let mut aggregated = HashMap::new();

        statistics.iter().cloned().flatten().for_each(|(stat, val)| {
            aggregated.insert(stat.clone(), match aggregated.get(&stat) {
                Some(prev_value) => prev_value + val,
                None => val
            });
        });

        aggregated.into_iter().filter(|(_stat, val)| val != &0u32).collect()
    }

    fn aggregate_alterations(players: &HashMap<Uuid, Rc<Player>>, damages: &Vec<Damage>, heals: &Vec<Heal>) -> HashMap<Uuid, PlayerAlterationsAggregate> {
        let ranks: HashMap<Uuid, u8> = {
            let mut lethal_damages: Vec<&Damage> = damages.iter().filter(|damage| damage.lethal).collect();
            lethal_damages.sort_by(|a, b| a.date.cmp(&b.date).reverse());

            let ranked_deads: Vec<Uuid> = lethal_damages.iter().map(|damage| damage.damagee.uuid).collect();

            players.iter()
                .map(|(uuid, _player)| uuid)
                .filter(|uuid| !ranked_deads.contains(uuid))
                .chain(ranked_deads.iter())
                .enumerate()
                .map(|(index, uuid)| (uuid.clone(), index as u8 + 1))
                .collect()
        };

        players.iter().map(|(uuid, _player)| (
            uuid.clone(),
            Self::aggregate_player_alterations(uuid, damages, heals, *ranks.get(uuid).unwrap_or(&0u8))
        )).collect()
    }

    fn aggregate_player_alterations(player: &Uuid, damages: &Vec<Damage>, heals: &Vec<Heal>, rank: u8) -> PlayerAlterationsAggregate {
        let mut damages_taken: Vec<Damage> = damages.iter().cloned().filter(|damage| &damage.damagee.uuid == player).collect();
        let mut damages_caused: Vec<Damage> = damages.iter().cloned().filter(|damage| match &damage.damager {
            Some(damager) => &damager.uuid == player,
            None => false
        }).collect();
        let mut heals: Vec<Heal> = heals.iter().cloned().filter(|heal| &heal.healed.uuid == player).collect();

        damages_taken.sort_by(|a, b| a.date.cmp(&b.date));
        damages_caused.sort_by(|a, b| a.date.cmp(&b.date));
        heals.sort_by(|a, b| a.date.cmp(&b.date));

        PlayerAlterationsAggregate {
            damages_taken_total: (&damages_taken).iter().fold(0u32, |acc, damage| damage.damage as u32 + acc),
            damages_caused_total: (&damages_caused).iter().fold(0u32, |acc, damage| damage.damage as u32 + acc),
            heals_total: (&heals).iter().fold(0u32, |acc, heal| heal.heal as u32 + acc),

            damages_taken,
            damages_caused,
            heals,

            kills: damages.iter()
                    .filter(|damage| match &damage.damager {
                        Some(damager) => &damager.uuid == player && damage.lethal,
                        None => false
                    })
                    .map(|damage| damage.damagee.clone())
                    .collect(),

            killed_by: damages.iter()
                    .filter(|damage| &damage.damagee.uuid == player && damage.lethal)
                    .map(|damage| match &damage.damager {
                        Some(damager) => PlayerKiller::Player { player: damager.clone() },
                        None => PlayerKiller::Other { cause: damage.cause.clone() }
                    })
                    .last(),

            rank
        }
    }

    fn aggregate_environmental_damages(damages: &Vec<Damage>) -> HashMap<DamageCause, u32> {
        let mut aggregated = HashMap::new();

        damages.iter()
            .filter(|damage| damage.damager.is_none())
            .for_each(|damage| {
                aggregated.insert(damage.cause.clone(), match aggregated.get(&damage.cause) {
                    Some(aggregate) => aggregate + damage.damage as u32,
                    None => damage.damage as u32
                });
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
    pub killed_by: Option<PlayerKiller>,
    pub rank: u8
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum PlayerKiller {
    Player { player: SimplePlayer },
    Other { cause: DamageCause }
}
