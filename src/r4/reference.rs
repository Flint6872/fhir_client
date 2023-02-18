use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Reference {
    #[serde(default)]
    pub reference: String,
    #[serde(default)]
    pub display: String,
}

pub fn default_reference() -> Reference{
    Reference {reference: "".to_string(),
                display: "".to_string()}
}
