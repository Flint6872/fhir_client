use serde::{Serialize, Deserialize};
use crate::r4::{coding};

#[derive(Serialize, Deserialize, Debug)]
pub struct ReferenceRange {
    #[serde(default = "coding::default_coding")]
    pub low: coding::Coding,
    #[serde( default = "coding::default_coding")]
    pub high: coding::Coding,
    #[serde(rename = "type", default = "coding::default_coding_type")]
    pub ref_type: coding::CodingType 

}