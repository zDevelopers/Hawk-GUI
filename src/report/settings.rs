fn default_true() -> bool { true }
fn default_false() -> bool { false }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    #[serde(default = "default_true")]
    pub date: bool,

    #[serde(default = "default_true")]
    pub players_count: bool,

    #[serde(default = "default_true")]
    pub winners: bool,

    pub summary: Option<SettingsSummary>,
    pub damages: Option<SettingsDamages>,
    pub players: Option<SettingsPlayers>,
    pub generator: Option<SettingsGenerator>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SettingsSummary {
    #[serde(default = "default_true")]
    pub enabled: bool,

    #[serde(default = "default_true")]
    pub history: bool,

    #[serde(default = "default_true")]
    pub players: bool,

    #[serde(default = "default_true")]
    pub teams: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SettingsDamages {
    #[serde(default = "default_true")]
    pub enabled: bool,

    #[serde(default = "default_true")]
    pub damages_per_players: bool,

    #[serde(default = "default_true")]
    pub damages_per_team: bool,  // TODO unimplemented

    #[serde(default = "default_true")]
    pub damages_from_mobs: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SettingsPlayers {
    #[serde(default = "default_true")]
    pub enabled: bool,

    #[serde(default = "default_true")]
    pub play_time: bool,

    #[serde(default)]
    pub statistics_whitelist: Vec<String>,

    #[serde(default)]
    pub statistics_highlight: Vec<String>,

    #[serde(default = "default_false")]
    pub used: bool,

    #[serde(default)]
    pub used_whitelist: Vec<String>,

    #[serde(default)]
    pub used_highlight: Vec<String>,

    #[serde(default = "default_true")]
    pub mined: bool,

    #[serde(default)]
    pub mined_whitelist: Vec<String>,

    #[serde(default)]
    pub mined_highlight: Vec<String>,

    #[serde(default = "default_true")]
    pub picked_up: bool,

    #[serde(default)]
    pub picked_up_whitelist: Vec<String>,

    #[serde(default)]
    pub picked_up_highlight: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SettingsGenerator {
    pub name: String,
    pub link: Option<String>
}
