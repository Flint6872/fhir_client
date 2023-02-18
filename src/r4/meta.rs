use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    #[serde(rename = "versionId", default)]
    pub version_id: String,
    #[serde(rename = "lastUpdated", default)]
    pub last_updated: String,
    #[serde(default)]
    pub source: String
}

pub fn default_meta() -> Meta{
    Meta{version_id: "".to_string(),
         last_updated: "".to_string(),
         source: "".to_string()
        }
}