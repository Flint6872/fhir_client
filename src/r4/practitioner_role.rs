use serde::{Serialize, Deserialize};
use crate::r4::{text, identifier, period, coding, telecom, reference};


#[derive(Serialize, Deserialize, Debug)]
pub struct PractitionerRole {
#[serde(rename = "resourceType", default)]
    pub resource_type: String,
#[serde(rename = "id", default)]
    pub id_resource: String,
    #[serde(default = "text::default_text")]
    pub text: text::Text,
    #[serde(default)]
    pub identifier: Vec<identifier::Identifier>,
    
    pub active: Option<bool>,
    #[serde(default = "period::default_period")]
    pub period: period::Period,
    #[serde(default = "reference::default_reference")]
    pub practitioner: reference::Reference,
    #[serde(default = "reference::default_reference")]
    pub organization: reference::Reference,
    #[serde(default)]
    pub code: Vec<coding::CodingType>,
    #[serde(default)]
    pub specialty: Vec<coding::CodingType>,
    #[serde(default)]
    pub location: Vec<Reference>,
    #[serde(rename ="healthcareService" ,default)]
    pub healthcare_service: Vec<Reference>,
    #[serde(default)]
    pub telecom: Vec<telecom::Telecom>,
    #[serde(rename ="availableTime", default)]
    pub available_time: Vec<AvailableTime>,
    #[serde(rename ="notAvailable", default)]
    pub not_available: Vec<NotAvailable>,
    #[serde(rename ="availabilityExceptions", default)]
    pub availability_exceptions: String,
    #[serde(default)]
    pub endpoint: Vec<Reference>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AvailableTime {
    #[serde(rename ="daysOfWeek",default)]
    pub days_of_week: Vec<String>,
    #[serde(rename ="availableStartTime",default)]
    pub available_start_time: String,
    #[serde(rename ="availableEndTime",default)]
    pub available_end_time: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NotAvailable {
    #[serde(default)]
    pub description: String,
    #[serde(default ="period::default_period")]
    pub during: period::Period
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reference {
    #[serde(default)]
    pub referance: String,
    #[serde(default)]
    pub display: String,
}

