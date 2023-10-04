# fhir_client

Welcome to my Rust learning project.

Currently fhir_client is a library of limited FHIR resources that uses [Reqwest](https://github.com/seanmonstar/reqwest) a powerful Rust HTTP Client to pull json data from the server and
[Serde](https://serde.rs/) to serialize / deserialize the data into structs.
 
```rust
pub async fn fetch_resource(fhir_resource: &str, id: &str) -> Result<String, reqwest::Error> {
   
        let url = format!("{}/{}/{}", BASE_URL, fhir_resource, id);
        
        let res = reqwest::Client::new()
              .get(&url)
              .send()
              .await?
              .text()
              .await?;
   
        Ok(res)
    }
let patient: Fhir::patient::Patient = serde_json::from_str(&res).unwrap();
```

## FHIR Servers

* [Hapi FHIR ](https://hapifhir.io/):
Java based FHIR Server

* [Firely ](https://fire.ly/):
.NET based FHIR Server

## EMR on FHIR

- [Epic ](https://fhir.epic.com/)
- [Cerner](https://fhir.cerner.com/)

## Rust on FHIR
- [fhirbolt](https://github.com/lschmierer/fhirbolt/tree/main):Fhirbolt is an experimental suite of libraries for working with FHIR resources. It currently provides serialization and deserialization of JSON and XML resources for the Rust programming language.

- [fhir_sdk](https://github.com/FlixCoder/fhir-sdk):
This is a FHIR library in its early stages. The models are generated from the FHIR StructureDefinitions 
