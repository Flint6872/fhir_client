use serde::{Serialize, Deserialize};

 #[derive(Serialize, Deserialize, Debug)]
 pub struct  ExtensionPointer {
     #[serde(default)]
     pub extension: Vec<Extension>
 }
 
 #[derive(Serialize, Deserialize, Debug)]
 pub struct  Extension {
     #[serde(default)]
     pub url: String,
     #[serde(rename = "valueDateTime", default)]
     pub value_date_time: String,
     #[serde(rename = "valueString", default)]
     pub value_string:  String
 }