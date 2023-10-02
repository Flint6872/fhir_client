##fhir_client

Welcome to my Rust learning project.

Currently fhir_client is a library of fhir resources that uses [Reqwest](https://github.com/seanmonstar/reqwest) a powerful Rust HTTP Client to pull json data from the server and
[Serde](https://serde.rs/) to serialize / deserialize the data into structs.

 
'''rust
let id = "591229";
 let res = fhir_client::reqwestwasm::fetch_resource(id).await.unwrap();
    let patient: Fhir::patient::Patient =  serde_json::from_str(&res).unwrap();
'''


## FHIR Servers
**[Hapi FHIR ](https://hapifhir.io/):
Java based FHIR Server

**[Firely ](https://fire.ly/):
.NET based FHIR Server


## EMR FHIR
**[Epic ](https://fhir.epic.com/)
**[Cerner](https://fhir.cerner.com/)





