use serde::{Serialize, Deserialize};
use crate::r4::{identifier, coding, period, issuer };


#[derive(Serialize, Deserialize, Debug)]
pub struct Qualification { 
    #[serde(default)]
    pub identifier: Vec<identifier::Identifier>,
    #[serde(default = "coding::default_coding_type")]
    pub code: coding::CodingType,
    #[serde(default = "period::default_period")]
    pub period: period::Period,
    #[serde(default = "issuer::default_issuer")]
    pub issuer: issuer::Issuer
}

