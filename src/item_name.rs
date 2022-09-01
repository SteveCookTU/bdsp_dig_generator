use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ItemName {
    pub str: String,
}

#[derive(Serialize, Deserialize)]
pub struct Item {
    #[serde(rename = "labelIndex")]
    pub label_index: i32,
    #[serde(rename = "wordDataArray")]
    pub word_data_array: Vec<ItemName>,
}

#[derive(Serialize, Deserialize)]
pub struct ItemNameTable {
    #[serde(rename = "labelDataArray")]
    pub label_data_array: Vec<Item>,
}
