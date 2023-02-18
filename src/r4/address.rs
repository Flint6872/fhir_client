use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
    #[serde(rename = "use", default)]
    pub address_use: String,
    #[serde(default)]
    pub line: Vec<String>,
    #[serde(default)]
    pub city: String,
    #[serde(default)]
    pub state: String,
    #[serde(rename = "postalCode", default)]
    pub postal_code: String,
    #[serde(default)]
    pub country: String
}