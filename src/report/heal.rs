use crate::report::player::SimplePlayer;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Heal {
    pub heal_cause: HealCause,
    pub healed: SimplePlayer,
    pub heal: u16
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HealCause {
    Natural,
    GoldenApple,
    NotchApple,
    HealingPotion,
    Command,
    Unknown
}
