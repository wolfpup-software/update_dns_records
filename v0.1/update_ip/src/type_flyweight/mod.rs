use serde::{Deserialize, Serialize};

// beware of hydra
pub type IpServices = Vec<(String, String)>;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ResponseJson {
    pub status_code: u16,
    pub body: String,
    pub headers: Vec<(String, String)>,
    pub timestamp: u128,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct IpServiceResult {
    pub address: Option<String>,
    pub service: Option<String>,
    pub address_changed: bool,
    pub errors: Vec<String>,
    pub response: Option<ResponseJson>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DomainResult {
    pub domain: String,
    pub retry: bool,
    pub errors: Vec<String>,
    pub response: Option<ResponseJson>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UpdateIpResults {
    pub ip_service_result: IpServiceResult,
    pub domain_service_results: Vec<DomainResult>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Squarespace {
    pub hostname: String,
    pub username: String,
    pub password: String,
}

// add domain services here
// beware of hydra
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DomainServices {
    pub squarespace: Option<Vec<Squarespace>>,
}
