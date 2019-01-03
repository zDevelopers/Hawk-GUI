#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    #[serde(default = "default_true")]
    pub date: bool,

    #[serde(default = "default_true")]
    pub players_count: bool,

    #[serde(default = "default_true")]
    pub winners: bool,

    #[serde(default = "default_summary_settings")]
    pub summary: SettingsSummary,

    #[serde(default = "default_damages_settings")]
    pub damages: SettingsDamages,

    #[serde(default = "default_players_settings")]
    pub players: SettingsPlayers,

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


#[inline(always)] fn default_true() -> bool { true }
#[inline(always)] fn default_false() -> bool { false }

#[inline(always)]
pub fn default_settings() -> Settings {
    Settings {
        date: true,
        players_count: true,
        winners: true,
        summary: default_summary_settings(),
        damages: default_damages_settings(),
        players: default_players_settings(),
        generator: None
    }
}

#[inline(always)]
fn default_summary_settings() -> SettingsSummary {
    SettingsSummary {
        enabled: true,
        history: true,
        players: true,
        teams: true
    }
}

#[inline(always)]
fn default_damages_settings() -> SettingsDamages {
    SettingsDamages {
        enabled: true,
        damages_per_players: true,
        damages_per_team: true,
        damages_from_mobs: true
    }
}

#[inline(always)]
fn default_players_settings() -> SettingsPlayers {
    SettingsPlayers {
        enabled: true,
        play_time: true,
        statistics_whitelist: vec![],
        statistics_highlight: vec![],
        used: false,
        used_whitelist: vec![],
        used_highlight: vec![],
        mined: true,
        mined_whitelist: vec![],
        mined_highlight: vec![],
        picked_up: true,
        picked_up_whitelist: vec![],
        picked_up_highlight: vec![]
    }
}
