use rocket_contrib::templates::tera::{self, GlobalFn, FilterFn, Result};
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

pub fn is_creature_test(value: Option<Value>, _params: Vec<Value>) -> Result<bool> {
    Ok(value.map(|value| from_value::<DamageCause>(value).unwrap_or(DamageCause::Unknown))
         .map(|cause| cause.is_creature())
         .unwrap_or(false))
}
