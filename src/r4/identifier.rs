use serde::{Serialize, Deserialize};
use crate::r4::{coding, period, assigner};

#[derive(Serialize, Deserialize, Debug)]
pub struct Identifier {
    #[serde(rename = "use", default)]
    pub id_use: String,   
    #[serde(rename = "type", default = "coding::default_coding_type")]
    pub id_type: coding::CodingType,
    #[serde(default)]
    pub system: String, //Uri
    #[serde(default)]
    pub value: String,
    #[serde(default = "period::default_period")]
    pub period: period::Period,
    #[serde(default = "assigner::default_assigner")]
    pub assigner:assigner::Assigner
}