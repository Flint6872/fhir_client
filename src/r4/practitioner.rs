use serde::{Serialize, Deserialize};
use crate::r4::{text, identifier, human_name, address, qualification };


#[derive(Serialize, Deserialize, Debug)]
pub struct Practitioner { 
    #[serde(rename = "resourceType", default)]
    pub resource_type: String,
    #[serde(rename = "id", default)]
    pub id_resource: String,
    #[serde(default = "text::default_text")]
    pub text: text::Text,
    #[serde(default)]
    pub identifier: Vec<identifier::Identifier>,
    
    pub active: Option<bool>,
    #[serde(default)]
    pub name: Vec<human_name::HumanName>,
    #[serde(default)]
    pub address: Vec<address::Address>,
    #[serde(default)]
    pub qualification: Vec<qualification::Qualification>,
}