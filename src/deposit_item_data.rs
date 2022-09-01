use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DepositItem {
    #[serde(rename = "itemId")]
    pub item_id: u32,
    pub shape: String,
    pub turn: i32,
    #[serde(rename = "offsetSize")]
    pub offset_size: i32,
    #[serde(rename = "offsetX")]
    pub offset_x: i32,
    #[serde(rename = "offsetY")]
    pub offset_y: i32,
    #[serde(rename = "bIsOnly")]
    pub b_is_only: i32,
    #[serde(rename = "bIsRare")]
    pub b_is_rare: i32,
    pub ratio1: i32,
    pub ratio2: i32,
    pub ratio3: i32,
    pub ratio4: i32,
    pub ratio5: i32,
    pub ratio6: i32,
}

#[derive(Serialize, Deserialize)]
pub struct DepositItemData {
    #[serde(rename = "Deposit")]
    pub deposit: Vec<DepositItem>,
}
