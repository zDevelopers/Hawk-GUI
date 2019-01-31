use std::time::Duration;

use inflector::Inflector;
use rocket_contrib::templates::tera::{self, GlobalFn, FilterFn, Result};
use roman;
use serde_json::value::{from_value, to_value, Value};
use uuid;

use crate::minecraft::parse_color_codes;

use crate::report::damage::{DamageCause, Weapon};
use crate::report::heal::HealCause;

pub fn make_head_function() -> GlobalFn {
    Box::new(move |args| -> tera::Result<Value> {
        let uuid = match args.get("uuid") {
            Some(uuid_str) => match from_value::<uuid::Uuid>(uuid_str.clone()) {
                Ok(uuid) => uuid,
                Err(_) => uuid::Uuid::nil() // FIXME Err(format!("Function `head` received uuid={} but `uuid` can only be a valid UUID",uuid_str).into::<Error>())?
            },
            None => uuid::Uuid::nil() // Err(format!("Function `head` was called without a `uuid` argument").into())?
        };

        let size = match args.get("size") {
            Some(size) => match from_value::<u8>(size.clone()) {
                Ok(size) => size,
                Err(_) => 16 // Err(format!("Function `head` received size={} but `size` can only be an integer", size).into())?
            },
            None => 16
        };

        // TODO Ok(to_value(uri!(get_head: uuid, size).to_string()).unwrap())
        Ok(to_value(format!("/head/{uuid}/{size}", uuid = uuid, size = size)).unwrap())
    })
}

pub fn make_minecraft_filter() -> FilterFn {
    |input, _args| Ok(to_value(parse_color_codes(input.as_str().unwrap_or("").to_string())).unwrap())
}

pub fn make_css_class_filter() -> FilterFn {
    |input, _args| Ok(to_value(input.as_str().unwrap_or("").to_lowercase().replace("_", "-")).unwrap())
}

pub fn make_icon_filter() -> FilterFn {
    |input, _args| Ok(to_value(match from_value::<DamageCause>(input.clone()) {
        Ok(damage_cause) => match damage_cause {
            DamageCause::Player => "",
            DamageCause::Zombie => "entity-zombie-small",
            DamageCause::Skeleton => "entity-skeleton-small",
            DamageCause::Pigman => "entity-zombie-pigman-small",
            DamageCause::Witch => "entity-witch-small",  // FIXME missing
            DamageCause::Spider => "entity-spider-small",
            DamageCause::CaveSpider => "entity-cave-spider-small",
            DamageCause::Creeper => "entity-creeper-small",
            DamageCause::Enderman => "entity-enderman-small",
            DamageCause::Slime => "entity-slime-small",
            DamageCause::Ghast => "entity-ghast-small",
            DamageCause::MagmaCube => "entity-magma-cube-small",
            DamageCause::Blaze => "entity-blaze-small",
            DamageCause::Wolf => "entity-wolf-small",
            DamageCause::AngryWolf => "entity-angry-wolf-small",
            DamageCause::Silverfish => "entity-silverfish-small",
            DamageCause::IronGolem => "entity-iron-golem-small",
            DamageCause::ZombieVillager => "entity-zombie-villager-small",
            DamageCause::EnderDragon => "entity-ender-dragon-small",
            DamageCause::Wither => "entity-wither-small",
            DamageCause::WitherSkeleton => "entity-wither-skeleton-small",
            DamageCause::Fire => "block-fire-small",
            DamageCause::Lava => "block-lava-small",
            DamageCause::Thunderbolt => "entity-lightning-small",
            DamageCause::Cactus => "block-cactus-small",
            DamageCause::TNT => "block-tnt-small",
            DamageCause::Fall => "block-stone-small",
            DamageCause::Suffocation => "block-sand-small",
            DamageCause::Drowning => "block-water-small",
            DamageCause::Starvation => "item-rotten-flesh-small",
            DamageCause::Command => "block-command-block-small",
            DamageCause::Unknown => "entity-unknown-small",
        },
        Err(_) => match from_value::<Weapon>(input.clone()) {
            Ok(weapon) => match weapon {
                Weapon::Fists => "",
                Weapon::SwordWood => "item-wood-sword-small",
                Weapon::SwordStone => "item-stone-sword-small",
                Weapon::SwordIron => "item-iron-sword-small",
                Weapon::SwordGold => "item-gold-sword-small",
                Weapon::SwordDiamond => "item-diamond-sword-small",
                Weapon::AxeWood => "item-wood-axe-small",
                Weapon::AxeStone => "item-stone-axe-small",
                Weapon::AxeIron => "item-iron-axe-small",
                Weapon::AxeGold => "item-gold-axe-small",
                Weapon::AxeDiamond => "item-diamond-axe-small",
                Weapon::Bow => "item-bow-pulling-small",
                Weapon::Magic => "item-potion-bottle-splash-small",
                Weapon::Thorns => "item-diamond-chestplate-small",
                Weapon::Unknown => "",
            },
            Err(_) => match from_value::<HealCause>(input.clone()) {
                Ok(heal_cause) => match heal_cause {
                    HealCause::Natural => "item-potato-baked-small",
                    HealCause::GoldenApple => "item-apple-golden-small",
                    HealCause::NotchApple => "item-apple-golden-small",
                    HealCause::HealingPotion => "item-potion-bottle-splash-small",
                    HealCause::Command => "block-command-block-small",
                    HealCause::Unknown => "entity-unknown-small",
                },
                Err(_) => ""
            }
        }
    }).unwrap())
}

// FIXME I18N
pub fn make_name_filter() -> FilterFn {
    |input, _args| Ok(to_value(match from_value::<DamageCause>(input.clone()) {
        Ok(damage_cause) => match damage_cause {
            DamageCause::Player => "Joueur",
            DamageCause::Zombie => "Zombie",
            DamageCause::Skeleton => "Squelette",
            DamageCause::Pigman => "Cochon Zombie",
            DamageCause::Witch => "Sorcière",
            DamageCause::Spider => "Araignée",
            DamageCause::CaveSpider => "Araignée des cavernes",
            DamageCause::Creeper => "Creeper",
            DamageCause::Enderman => "Enderman",
            DamageCause::Slime => "Slime",
            DamageCause::Ghast => "Ghast",
            DamageCause::MagmaCube => "Cube de Magma",
            DamageCause::Blaze => "Blaze",
            DamageCause::Wolf => "Chien",
            DamageCause::AngryWolf => "Loup énervé",
            DamageCause::Silverfish => "Poisson d'argent",
            DamageCause::IronGolem => "Golem de Fer",
            DamageCause::ZombieVillager => "Villageois zombie",
            DamageCause::EnderDragon => "Dragon",
            DamageCause::Wither => "Wither",
            DamageCause::WitherSkeleton => "Wither Squelette",
            DamageCause::Fire => "Feu",
            DamageCause::Lava => "Lave",
            DamageCause::Thunderbolt => "Foudre",
            DamageCause::Cactus => "Cactus",
            DamageCause::TNT => "Trinitroluène",
            DamageCause::Fall => "Chute",
            DamageCause::Suffocation => "Suffocation",
            DamageCause::Drowning => "Noyade",
            DamageCause::Starvation => "Faim",
            DamageCause::Command => "Commande",
            DamageCause::Unknown => "Inconnu",
        },
        Err(_) => match from_value::<Weapon>(input.clone()) {
            Ok(weapon) => match weapon {
                Weapon::Fists => "Poings",
                Weapon::SwordWood => "Épée en bois",
                Weapon::SwordStone => "Épée en pierre",
                Weapon::SwordIron => "Épée en fer",
                Weapon::SwordGold => "Épée en or",
                Weapon::SwordDiamond => "Épée de diamant",
                Weapon::AxeWood => "Hache en bois",
                Weapon::AxeStone => "Hache en pierre",
                Weapon::AxeIron => "Hache en fer",
                Weapon::AxeGold => "Hache en or",
                Weapon::AxeDiamond => "Hache de diamant",
                Weapon::Bow => "Arc",
                Weapon::Magic => "Magie",
                Weapon::Thorns => "Épines d'armure",
                Weapon::Unknown => "Inconnu",
            },
            Err(_) => match from_value::<HealCause>(input.clone()) {
                Ok(heal_cause) => match heal_cause {
                    HealCause::Natural => "Régénération naturelle",
                    HealCause::GoldenApple => "Pomme d'or",
                    HealCause::NotchApple => "Pomme d'or enchantée",
                    HealCause::HealingPotion => "Potion de soin",
                    HealCause::Command => "Commande",
                    HealCause::Unknown => "Inconnu",
                },
                Err(_) => input.as_str().unwrap_or("")
            }
        }
    }).unwrap())
}

pub fn make_enchantment_filter() -> FilterFn {
    |input, args| Ok(to_value(format!(
        "{enchantment}{level}",
        enchantment = match input.as_str() {
            Some(input) => match input {
                "sweeping" => "Sweeping Edge".to_string(),
                _ => input.to_title_case()
            },
            None => input.to_string(),
        },
        level = match args.get("level") {
            Some(level) => match from_value::<u32>(level.clone()) {
                Ok(level) => match level {
                    1 => "".to_string(),
                    _ => match roman::to(level as i32) {
                        Some(roman_level) => format!(" {}", roman_level),
                        None => format!("{}", level),
                    }
                },
                Err(_) => format!(" {}", level.to_string()),
            },
            None => "".to_string(),
        }
    )).unwrap())
}

pub fn make_duration_filter() -> FilterFn {
    |input, _args| Ok(to_value(match from_value::<Duration>(input) {
        Ok(duration) => {
            let total_secs = duration.as_secs();
            let days = total_secs / 86400;
            let hours = (total_secs - days * 86400) / 3600;
            let minutes = (total_secs - (days * 86400) - (hours * 3600)) / 60;
            let seconds = total_secs - (days * 86400) - (hours * 3600) - (minutes * 60);

            format!(
                "{days}{hours}{minutes}{seconds}",
                days = if days != 0 { format!("{}j\u{00A0}", days) } else { "".to_string() },  // FIXME i18n
                hours = if hours != 0 { format!("{:02}:", hours) } else { "".to_string() },
                minutes = format!("{:02}:", minutes),
                seconds = format!("{:02}", seconds)
            )
        },
        Err(_) => "00:00".to_string(),
    }).unwrap())
}

pub fn is_creature_test(value: Option<Value>, _params: Vec<Value>) -> Result<bool> {
    Ok(value.map(|value| from_value::<DamageCause>(value).unwrap_or(DamageCause::Unknown))
         .map(|cause| cause.is_creature())
         .unwrap_or(false))
}
