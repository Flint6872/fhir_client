//! FHIR client built completely with rust
//! currently a libary of resources 


pub mod r4;

#[allow(dead_code)]

const BASE_URL:&str = "https://hapi.fhir.org/baseR4/Patient/";


pub fn get_patient_by_id(pt_id:&str) -> String {

    let mut url = BASE_URL.to_owned();
    url.push_str(pt_id);
    
   // let res = get_request(url).await?;
    
    url
    
    }
    
    

//     pub async fn post_patient()-> Result<(), reqwest::Error>{


//       Ok(())
//     }


