/*
 * Account API
 *
 * # Introduction  The Account API allows you to manage projects. Project is Scaleway’s resource management feature. Designed to help you organize your infrastructure and cloud services, the feature allows resources to be isolated and grouped into specific projects.
 *
 * The version of the OpenAPI document: v2
 *
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for typed errors of method [`get_bmc_access`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBmcAccessError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`start_bmc_access`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StartBmcAccessError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`stop_bmc_access`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StopBmcAccessError {
    UnknownValue(serde_json::Value),
}

/// Get the BMC (Baseboard Management Controller) access associated with the given ID.
pub async fn get_bmc_access(
    configuration: &configuration::Configuration,
    zone: &str,
    server_id: &str,
) -> Result<crate::models::ScalewayPeriodBaremetalPeriodV1PeriodBmcAccess, Error<GetBmcAccessError>>
{
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/baremetal/v1/zones/{zone}/servers/{server_id}/bmc-access",
        local_var_configuration.base_path,
        zone = crate::apis::urlencode(zone),
        server_id = crate::apis::urlencode(server_id)
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
        let local_var_entity: Option<GetBmcAccessError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Start BMC (Baseboard Management Controller) access associated with the given ID. The BMC (Baseboard Management Controller) access is available one hour after the installation of the server. You need first to create an option Remote Access. You will find the ID and the price with a call to listOffers (https://developers.scaleway.com/en/products/baremetal/api/#get-78db92). Then you can add the option https://developers.scaleway.com/en/products/baremetal/api/#post-b14abd. Do not forget to delete the Option.  After start BMC, you need to Get Remote Access to get the login/password https://developers.scaleway.com/en/products/baremetal/api/#get-cefc0f.
pub async fn start_bmc_access(
    configuration: &configuration::Configuration,
    zone: &str,
    server_id: &str,
    start_bmc_access_request: crate::models::StartBmcAccessRequest,
) -> Result<crate::models::ScalewayPeriodBaremetalPeriodV1PeriodBmcAccess, Error<StartBmcAccessError>>
{
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/baremetal/v1/zones/{zone}/servers/{server_id}/bmc-access",
        local_var_configuration.base_path,
        zone = crate::apis::urlencode(zone),
        server_id = crate::apis::urlencode(server_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

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
    local_var_req_builder = local_var_req_builder.json(&start_bmc_access_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<StartBmcAccessError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Stop BMC (Baseboard Management Controller) access associated with the given ID.
pub async fn stop_bmc_access(
    configuration: &configuration::Configuration,
    zone: &str,
    server_id: &str,
) -> Result<(), Error<StopBmcAccessError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/baremetal/v1/zones/{zone}/servers/{server_id}/bmc-access",
        local_var_configuration.base_path,
        zone = crate::apis::urlencode(zone),
        server_id = crate::apis::urlencode(server_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

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
        Ok(())
    } else {
        let local_var_entity: Option<StopBmcAccessError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
