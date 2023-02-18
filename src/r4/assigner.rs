use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Assigner {
    #[serde(default)]
    pub display: String 
}

pub fn default_assigner() -> Assigner{
    Assigner{display: "".to_string()}
}