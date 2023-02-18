use serde::{Serialize, Deserialize};
use crate::r4::{coding, human_name, telecom, period};


#[derive(Serialize, Deserialize, Debug)]
pub struct Contact {
    #[serde(default)]
    pub relationship: Vec<coding::Coding>,

    pub name: Option<human_name::HumanName>,
    #[serde(default)]
    pub telecom: Vec<telecom::Telecom>,
    #[serde(default = "period::default_period")]
    pub period: period::Period
}
