/*
 * Elastic metal API
 *
 * # Introduction  Elastic metal as a service allows ordering a dedicated server on-demand like a cloud instance. Dedicated servers could be used for large workloads, big data, those requiring more security, ….  ## Technical Limitations  - Elastic metal is available in `fr-par-1`,  `fr-par-2`, `nl-ams-1` zones  - Installation is done by preseed (± 10min) (preseed: complete install from a virtual media)  ## Features  - Install (Server is installed by preseed (preseed: complete install from a virtual media), you must define at least one ssh key to install your server)  - Start/Stop/Reboot  - Rescue Reboot, a rescue image is an operating system image designed to help you diagnose and fix an OS experiencing failures. When your server boot on rescue, you can mount your disks and start diagnosing/fixing your image.  - Billed by minute (The billing start when the server is delivered and stop when the server is deleted)  - IPv6, all servers are available with an IPv6 /128  - ReverseIP, You can configure your reverse IP (IPv4 and IPv6), you must register the server IP in your DNS records before calling the endpoint  - Basic monitoring with ping status  - Flexible IP is available ([documentation](https://developers.scaleway.com/en/products/flexible-ip/api/))  - IP failovers are not available in api v1, use the api v1alpha1  ## FAQ  ### How can I get my SSH key id?  You can find your `$SCW_SECRET_KEY` and your `SCW_DEFAULT_ORGANIZATION_ID` at the following page: https://console.scaleway.com/project/credentials  ### How can I add my server to a private network?  See [our online documentation](https://developers.scaleway.com/en/products/vpc-elasticmetal/api/).
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for typed errors of method [`get_offer`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOfferError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_offers`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListOffersError {
    UnknownValue(serde_json::Value),
}

/// Return specific offer for the given ID.
pub async fn get_offer(
    configuration: &configuration::Configuration,
    zone: &str,
    offer_id: &str,
) -> Result<crate::models::ScalewayBaremetalV1Offer, Error<GetOfferError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;
    // GDU
    let local_var_uri_str = format!(
        "{}/baremetal/v1/zones/{zone}/offers/{offer_id}",
        local_var_configuration.base_path,
        zone = crate::apis::urlencode(zone),
        offer_id = crate::apis::urlencode(offer_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-Auth-Token", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetOfferError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List all available server offers.
pub async fn list_offers(
    configuration: &configuration::Configuration,
    zone: &str,
    page: Option<i64>,
    page_size: Option<i64>,
    subscription_period: Option<&str>,
) -> Result<crate::models::ScalewayBaremetalV1ListOffersResponse, Error<ListOffersError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;
    // GDU
    let local_var_uri_str = format!(
        "{}/baremetal/v1/zones/{zone}/offers",
        local_var_configuration.base_path,
        zone = crate::apis::urlencode(zone)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = page {
        local_var_req_builder =
            local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_size {
        local_var_req_builder =
            local_var_req_builder.query(&[("page_size", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = subscription_period {
        local_var_req_builder =
            local_var_req_builder.query(&[("subscription_period", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-Auth-Token", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ListOffersError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
