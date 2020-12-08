#[inline(always)]
fn default_one_u8() -> u8 {
    1
}

#[inline(always)]
fn default_one_u32() -> u32 {
    1
}

/// A Minecraft item.
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub struct Item {
    /// The Minecraft item identifier.
    pub id: String,

    /// How many of them?
    #[serde(rename = "Count", default = "default_one_u8")]
    pub count: u8,

    /// Item metadata (if any)
    pub tag: Option<ItemTag>
}

/// Non-exhaustive item tags (the ones we need to know about) as of
/// https://minecraft.gamepedia.com/Player.dat_format#Item_structure .
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "PascalCase")]
pub struct ItemTag {
    pub unbreakable: Option<u8>,  // Technically a boolean but stored as 0/1
    pub enchantments: Option<Vec<EnchantmentTag>>,
    pub stored_enchantments: Option<Vec<EnchantmentTag>>,
    pub custom_potion_effects: Option<Vec<PotionEffectTag>>,
    pub potion: Option<String>,
    #[serde(rename = "display")]
    pub display: Option<DisplayTag>
}

/// An enchantment applied to an item.
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub struct EnchantmentTag {
    pub id: String,
    #[serde(default = "default_one_u32")]
    pub lvl: u32  // As silly as this could sound, the maximal enchantment level (for 1.14+) is 2,147,483,647
}

/// A potion effect (ignoring fields we don't care about, i.e. display-related ones).
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "PascalCase")]
pub struct PotionEffectTag {
    pub id: u8,
    pub amplifier: i8,
    #[serde(default = "default_one_u32")]
    pub duration: u32
}

/// The display tag of an item
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "PascalCase")]
pub struct DisplayTag {
    pub name: Option<String>,
    pub lore: Option<Vec<String>>
}
