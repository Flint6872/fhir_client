use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ManagingOrganization {
    #[serde(default)]
    pub reference: String,
    #[serde(default)]
    nonsenes: String // added to have display only show
}

pub fn default_managing_organization() -> ManagingOrganization{
    ManagingOrganization{reference: "".to_string(),
            nonsenes: "".to_string()}
}