use std::collections::HashMap;

use config::Config;
use results::{DomainResult, UpdateIpResults};

#[cfg(feature = "cloudflare")]
use cloudflare;
#[cfg(feature = "dyndns2")]
use dyndns2;

pub async fn update_domains(
    config: &Config,
    results: &UpdateIpResults,
) -> HashMap<String, DomainResult> {
    let mut domain_results = HashMap::<String, DomainResult>::new();

    // add more services here
    if cfg!(feature = "dyndns2") {
        dyndns2::update_domains(
            &mut domain_results,
            results,
            &config.domain_services.dyndns2,
        )
        .await;
    }
    if cfg!(feature = "cloudflare") {
        cloudflare::update_domains(
            &mut domain_results,
            results,
            &config.domain_services.cloudflare,
        )
        .await;
    }

    domain_results
}