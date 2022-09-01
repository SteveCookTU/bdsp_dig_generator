use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Item {
    #[serde(rename = "UgItemID")]
    pub ug_item_id: i32,
    #[serde(rename = "ItemTableID")]
    pub item_table_id: i32,
    #[serde(rename = "TamatableID")]
    pub tamatable_id: i32,
    #[serde(rename = "PedestaltableID")]
    pub pedestaltable_id: i32,
    #[serde(rename = "StonestatueeffectID")]
    pub stonestatueeffect_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct UgItemTable {
    pub table: Vec<Item>,
}
