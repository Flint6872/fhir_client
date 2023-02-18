use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CodingType {
    #[serde(default)]
    pub coding: Vec<Coding>,
    #[serde(default)]
    pub text: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Coding {
    #[serde(default = "default_number")]
    pub value: u32,
    #[serde(default)]
    pub unit: String,
    #[serde(default)]
    pub system: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub display: String
}

pub fn default_coding_type() -> CodingType{
    CodingType{ coding:Vec::<Coding>::new(), text: "".to_string() }
}

pub fn default_coding() -> Coding{
    Coding{ value: 0,
    unit: "".to_string(),
    system: "".to_string(),
    code: "".to_string(),
    display: "".to_string()

 }

} pub fn default_number() -> u32 {
    0
}