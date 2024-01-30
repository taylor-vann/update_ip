use std::collections::HashMap;

use crate::type_flyweight::{Config, DomainResult, UpdateIpResults};

mod dyndns2;
mod cloudflare;

pub async fn update_domains(
    config: &Config,
    results: &UpdateIpResults,
) -> HashMap<String, DomainResult> {
    // add more services here
    let mut domain_results = HashMap::<String, DomainResult>::new();
    domain_results = dyndns2::update_domains(domain_results, results, config).await;
    domain_results = cloudflare::update_domains(domain_results, results, config).await;
    
    domain_results
}
