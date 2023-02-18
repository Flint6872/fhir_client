use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Period {
    #[serde(rename = "start", default)]
    pub period_start: String,
    #[serde(rename = "end", default)]
    pub period_end: String
}

pub fn default_period() -> Period{
    Period{period_start: "".to_string(),
           period_end: "".to_string()
          }
}