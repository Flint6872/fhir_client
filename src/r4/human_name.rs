use serde::{Serialize, Deserialize};
use crate::r4::{period, extension};

#[derive(Serialize, Deserialize, Debug)]
pub struct HumanName {
  #[serde(rename = "use", default)]
    pub human_name_use: String,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub family: String,

    pub _family: Option<extension::ExtensionPointer>,
    #[serde(default)]
    pub given: Vec<String>,
    #[serde(default)]
    pub prefix: Vec<String>,
    #[serde(default)]
    pub suffix: Vec<String>,
    #[serde(default = "period::default_period")]
    pub period: period::Period
 }
