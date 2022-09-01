use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Box {
    #[serde(rename = "type")]
    pub r#type: u32,
    #[serde(rename = "boxId")]
    pub box_id: u32,
    pub shape: String,
    pub ratio1: i32,
    pub ratio2: i32,
}

#[derive(Serialize, Deserialize)]
pub struct StoneBoxData {
    #[serde(rename = "Box")]
    pub r#box: Vec<Box>,
}
