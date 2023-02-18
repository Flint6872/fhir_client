///        let identifier_id_use = patient.identifier[n].id_use.as_str();
///
///
///        if patient.identifier[n].id_type.coding.len() >0 {
///
///             for k in 0..patient.identifier[n].id_type.coding.len(){
///                
///                 let identifier_type_code = patient.identifier[n].id_type.coding[k].code.as_str();
///                let identifier_type_display = patient.identifier[n].id_type.coding[k].display.as_str();
///                let identifier_type_system = patient.identifier[n].id_type.coding[k].system.as_str();
                
///            }
///        }   

///        let identifier_system = patient.identifier[n].system.as_str();
    
///        let identifier_value = patient.identifier[n].value.as_str();

///        let identifier_period_start = patient.identifier[n].period.period_start.as_str();
///        let identifier_period_end = patient.identifier[n].period.period_end.as_str();
            
///        let identifier_assigner= patient.identifier[n].assigner.display.as_str();

///  



use serde::{Serialize, Deserialize};
use crate::r4::{meta, text, identifier, human_name, telecom, extension, address, contact, managing_organization};

#[derive(Serialize, Deserialize, Debug)]
pub struct Patient {

#[serde(rename = "resourceType", default)]
    pub resource_type: String,
#[serde(rename = "id", default)]
    pub id_resource: String,
    #[serde(default = "meta::default_meta")]
    pub meta: meta::Meta,
    #[serde(default = "text::default_text")]
    pub text: text::Text,
    #[serde(default)]
    pub identifier: Vec<identifier::Identifier>,
    #[serde(default)]
    pub active: bool,
    #[serde(default)]
    pub name: Vec<human_name::HumanName>,
    #[serde(default)]
    pub telecom: Vec<telecom::Telecom>,
    #[serde(default)]
    pub gender: String,
    #[serde(rename = "birthDate", default )]    
    pub birth_date: String,
    #[serde(rename = "_birthDate")]

    pub _birth_date: Option<extension::ExtensionPointer>,

    pub deceased_boolean: Option<bool>,
    #[serde(default)]
    pub address: Vec<address::Address>,
    #[serde(default)]
    pub marital_status: String,
    #[serde(default)]
    pub photo: String,
    #[serde(default )]
    pub contact: Vec<contact::Contact>,
    #[serde(default)]
    pub communication: String,
    #[serde(default)]
    pub link: String,
    #[serde(rename = "managingOrganization",default = "managing_organization::default_managing_organization")]
    pub managing_organization: managing_organization::ManagingOrganization,

    

}

#[allow(dead_code)]
#[allow(unused_variables)]
fn set_patient_variable(patient: Patient){

    let resource_type = patient.resource_type;
    let id_resource = patient.id_resource;
    
    let meta_version_id = patient.meta.version_id;
    let meta_last_updated = patient.meta.last_updated;
    let meta_source = patient.meta.source;

    let text_status = patient.text.status;
    let text_div = patient.text.div;

  
    for n in 0..patient.identifier.len(){
        
        let identifier_id_use = patient.identifier[n].id_use.as_str();


        if patient.identifier[n].id_type.coding.len() >0 {

            for k in 0..patient.identifier[n].id_type.coding.len(){
                
                let identifier_type_code = patient.identifier[n].id_type.coding[k].code.as_str();
                let identifier_type_display = patient.identifier[n].id_type.coding[k].display.as_str();
                let identifier_type_system = patient.identifier[n].id_type.coding[k].system.as_str();
                
            }
        }   

        let identifier_system = patient.identifier[n].system.as_str();
    
        let identifier_value = patient.identifier[n].value.as_str();

        let identifier_period_start = patient.identifier[n].period.period_start.as_str();
        let identifier_period_end = patient.identifier[n].period.period_end.as_str();
            
        let identifier_assigner= patient.identifier[n].assigner.display.as_str();

    }




    // let active = patient.active;
    
    // pub name: Vec<human_name::HumanName>,
 
    // pub telecom: Vec<telecom::Telecom>,

    // let gender = patient.gender;
    // let birth_date = patient.birth_date;

    // let _birth_date: Option<extension::ExtensionPointer>,

    // let deceased_boolean = patient.deceased_boolean;

    // pub address: Vec<address::Address>,

    // let marital_status = patient.marital_status;
    // let photo = patient.photo;
   
    // pub contact: Vec<contact::Contact>,
  
    // let communication = patient.communication;

    // let link = patient.link;
   
    // pub managing_organization: managing_organization::ManagingOrganization,

}

#[allow(dead_code)]
#[allow(unused_variables)]
fn print_identifier(ident: Vec<identifier::Identifier>){
  
    println!("Identifier");
    println!("id len: {:#?}", ident.len());
   
  for n in 0..ident.len(){
  
    if ident[n].id_type.coding.len() >0 {
      println!("use: {:#?}", ident[n].id_use);
  
        for k in 0..ident[n].id_type.coding.len(){
        println!("system: {:#?}", ident[n].id_type.coding[k].system);    
        println!("code: {:#?}", ident[n].id_type.coding[k].code);
        }
    }   
}
}