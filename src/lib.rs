//! FHIR client built completely with rust
//! currently a libary of resources 


pub mod r4;

pub const BASE_URL:&str = "https://hapi.fhir.org/baseR4/Patient/";


#[cfg(feature ="reqwasm")]
pub mod reqwasm{ 

pub async fn fetch_resource(id: &str) -> Result<String, reqwasm::Error> {
    
        let url = format!("{}/{}", crate::BASE_URL, id);
        //let resp = Request::get(&url).send().await?;
    
        //let body = resp.json::<Fhir::patient::Patient>().await?;

    
        let res = reqwasm::http::Request::get(&url) 
          
        .send()
        .await?
        .text()
        .await?;

     Ok(res)
    }
}

#[cfg(feature ="reqwest")]
pub mod reqwest{
pub async fn fetch_resource(id: &str) -> Result<String, reqwest::Error> {
    
        let url = format!("{}/{}", crate::BASE_URL, id);
        //let resp = Request::get(&url).send().await?;
    
        //let body = resp.json::<Fhir::patient::Patient>().await?;

    
        let res = reqwest::Client::new()
              .get(&url) 
              .send()
              .await?
              .text()
              .await?;
    
        Ok(res)
    }
}


#[cfg(feature ="reqwest-wasm")]
pub mod reqwestwasm{ 
     
     pub async fn fetch_resource(id: &str) -> Result<String, Box<dyn std::error::Error>> {

        let url = format!("{}/{}", crate::BASE_URL, id);
         let res = reqwest::Client::new()
             .get(&url)
             .send()
             .await?
             .text()
             .await?;

             Ok(res)
     
            //  let text = res.text().await?;
            //  let branch_info: Branch = serde_json::from_str(&text).unwrap();
         
            //  Ok(JsValue::from_serde(&branch_info).unwrap())
     }


    }


    
    // pub fn get_patient_by_id(pt_id:&str) -> String {
    
    //     let mut url = BASE_URL.to_owned();
    //     url.push_str(pt_id);
        
    //    // let res = get_request(url).await?;
        
    //     url
        
    //     }
        
        
    
    //     pub async fn post_patient()-> Result<(), reqwest::Error>{
    
    
    //       Ok(())
    //     }
    
    
    

