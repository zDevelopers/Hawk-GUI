use crate::report::player::SimplePlayer;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Team {
    pub name: String,
    pub color: TeamColor,

    pub players: Vec<SimplePlayer>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TeamColor {
    Aqua,
    Black,
    Blue,
    DarkAqua,
    DarkBlue,
    DarkGray,
    DarkGreen,
    DarkPurple,
    Gold,
    Gray,
    Green,
    LightPurple,
    Red,
    White,
    Yellow
}
