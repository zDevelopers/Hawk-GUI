use crate::report::player::SimplePlayer;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Damage {
    pub damage_cause: DamageCause,
    pub weapon: Option<Weapon>,
    pub damager: SimplePlayer,
    pub damagee: SimplePlayer,
    pub damage: u16,
    pub lethal: bool
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DamageCause {
    Player,       // TODO: “moved“ cause
    PlayerSweep,  // TODO: added cause
    Zombie,
    Skeleton,
    Pigman,
    Spider,
    CaveSpider,   // TODO: different name in DamagesLogger
    Creeper,
    Enderman,
    Slime,
    Ghast,
    MagmaCube,
    Blaze,
    Wolf,
    AngryWolf,    // TODO: idem
    Silverfish,
    IronGolem,
    ZombieVillager,
    EnderDragon,
    Wither,
    WitherSkeleton,

    Fire,
    Lava,
    Thunderbolt,
    Cactus,
    TNT,
    Fall,
    Suffocation,
    Drowning,
    Starvation,

    Command,
    Unknown
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Weapon {
    Fists,

    SwordWood,
    SwordStone,
    SwordIron,
    SwordGold,
    SwordDiamond,

    AxeWood,
    AxeStone,
    AxeIron,
    AxeGold,
    AxeDiamond,

    Bow,

    Magic,
    Thorns
}
