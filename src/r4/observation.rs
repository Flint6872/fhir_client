use serde::{Serialize, Deserialize};
use crate::r4::{text, identifier, coding, reference, reference_range};

#[derive(Serialize, Deserialize, Debug)]
pub struct Observation {
#[serde(rename = "resourceType", default)]
    pub resource_type: String,
#[serde(rename = "id", default)]
    pub id_resource: String,
    #[serde(default = "text::default_text")]
    pub text: text::Text,
    #[serde(default)]
    pub identifier: Vec<identifier::Identifier>,
    #[serde(default)]
    pub status: String,
    #[serde(default = "coding::default_coding_type")]
    pub code: coding::CodingType,
    #[serde(default = "reference::default_reference")]
    pub subject: reference::Reference,
    #[serde(default)]
    pub issued: String,

    #[serde(default)]
    pub performer: Vec<reference::Reference>,

    #[serde(rename = "valueQuantity", default = "coding::default_coding")]
    pub value_quantity: coding::Coding,

    #[serde(default)]
    pub interpretation: Vec<coding::CodingType>,

    #[serde(rename = "referenceRange", default)]
    pub reference_range: Vec<reference_range::ReferenceRange>,
   
}




