use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Issuer { 
    #[serde(default)]
    pub display: String
}

pub fn default_issuer() -> Issuer{
    Issuer{display: "".to_string() }
}