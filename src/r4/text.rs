use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Text {
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub div: String
}

pub fn default_text() -> Text{
    Text{status: "".to_string(),
            div: "".to_string()
          }
}