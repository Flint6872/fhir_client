pub mod patient;
pub mod meta;
pub mod text;
pub mod identifier;
pub mod coding;
pub mod period;
pub mod assigner;
pub mod human_name;
pub mod extension;
pub mod telecom;
pub mod address;
pub mod contact;
pub mod managing_organization;
pub mod practitioner;
pub mod qualification;
pub mod issuer;
pub mod practitioner_role;
//pub mod available_time;
//pub mod available_end_time;
pub mod reference;
pub mod observation;
pub mod reference_range;


pub use self::patient::Patient;
pub use self::practitioner::Practitioner;
pub use self::practitioner_role::PractitionerRole;
pub use self::observation::Observation;