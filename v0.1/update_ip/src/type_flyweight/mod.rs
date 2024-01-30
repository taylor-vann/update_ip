// this module should be an external lib
// Types should be accessible to other applications
// However, they are tightly coupled to a version of serde

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path;

// add domain services here
// beware of hydra
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DomainServices {
    pub dyndns2: Option<Vec<Dyndns2>>,
}

// ip services are accounted for by response type
// beware of hydra
pub type IpServices = Vec<(String, String)>;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Config {
    pub results_filepath: path::PathBuf,
    pub ip_services: IpServices,
    pub domain_services: DomainServices,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ResponseJson {
    pub status_code: u16,
    pub body: String,
    pub headers: HashMap<String, String>,
    pub timestamp: u128,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct IpServiceResult {
    pub prev_address: Option<String>,
    pub address: Option<String>,
    pub service: Option<String>,
    pub errors: Vec<String>,
    pub response: Option<ResponseJson>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DomainResult {
    pub hostname: String,
    pub errors: Vec<String>,
    pub response: Option<ResponseJson>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UpdateIpResults {
    pub ip_service_result: IpServiceResult,
    pub domain_service_results: HashMap<String, DomainResult>,
}

impl IpServiceResult {
    pub fn new() -> IpServiceResult {
        IpServiceResult {
            prev_address: None,
            address: None,
            service: None,
            errors: Vec::new(),
            response: None,
        }
    }
}

impl DomainResult {
    pub fn new(hostname: &String) -> DomainResult {
        DomainResult {
            hostname: hostname.clone(),
            errors: Vec::<String>::new(),
            response: None,
        }
    }
}

impl UpdateIpResults {
    pub fn new() -> UpdateIpResults {
        UpdateIpResults {
            ip_service_result: IpServiceResult::new(),
            domain_service_results: HashMap::<String, DomainResult>::new(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Dyndns2 {
    pub service_uri: String,
    pub hostname: String,
    pub username: String,
    pub password: String,
}

/*
curl --request PUT \
  --url https://api.cloudflare.com/client/v4/zones/zone_id/dns_records/dns_record_id \
  --header 'Content-Type: application/json' \
  --header 'X-Auth-Email: ' \
  --data '{
  "content": "198.51.100.4",
  "name": "example.com",
  "proxied": false,
  "type": "A",
  "comment": "Domain verification record",
  "tags": [
    "owner:dns-team"
  ],
  "ttl": 3600
}'
*/

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Cloudflare {
		pub hostname: String,
    pub api_token: String,
    pub zone_id: String,
    pub dns_record_id: String,
}
