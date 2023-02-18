use serde::{Serialize, Deserialize};
use crate::r4::{period};

 #[derive(Serialize, Deserialize, Debug)]
 pub struct Telecom {    
    #[serde(default)]
    pub system: String,
    #[serde(default)]
    pub value: String,
    #[serde(rename = "use", default)]
    pub telecome_use: String,
    #[serde(default = "default_number")] // default = 0
    pub rank: i32,
    #[serde(default = "period::default_period")]
    pub period: period::Period
 }

 pub fn default_number() -> i32 {
    0
}
