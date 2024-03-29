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

/// struct for typed errors of method [`create_placement_group`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreatePlacementGroupError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_placement_group`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeletePlacementGroupError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_placement_group`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPlacementGroupError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_placement_group_servers`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPlacementGroupServersError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_placement_groups`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListPlacementGroupsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_placement_group`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetPlacementGroupError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_placement_group_servers`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetPlacementGroupServersError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_placement_group`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdatePlacementGroupError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_placement_group_servers`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdatePlacementGroupServersError {
    UnknownValue(serde_json::Value),
}

/// Create a new placement group.
pub async fn create_placement_group(
    configuration: &configuration::Configuration,
    zone: &str,
    create_placement_group_request: crate::models::CreatePlacementGroupRequest,
) -> Result<
    crate::models::ScalewayPeriodInstancePeriodV1PeriodCreatePlacementGroupResponse,
    Error<CreatePlacementGroupError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/instance/v1/zones/{zone}/placement_groups",
        local_var_configuration.base_path,
        zone = crate::apis::urlencode(zone)
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
    local_var_req_builder = local_var_req_builder.json(&create_placement_group_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreatePlacementGroupError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete the given placement group.
pub async fn delete_placement_group(
    configuration: &configuration::Configuration,
    zone: &str,
    placement_group_id: &str,
) -> Result<(), Error<DeletePlacementGroupError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/instance/v1/zones/{zone}/placement_groups/{placement_group_id}",
        local_var_configuration.base_path,
        zone = crate::apis::urlencode(zone),
        placement_group_id = crate::apis::urlencode(placement_group_id)
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
        let local_var_entity: Option<DeletePlacementGroupError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get the given placement group.
pub async fn get_placement_group(
    configuration: &configuration::Configuration,
    zone: &str,
    placement_group_id: &str,
) -> Result<
    crate::models::ScalewayPeriodInstancePeriodV1PeriodGetPlacementGroupResponse,
    Error<GetPlacementGroupError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/instance/v1/zones/{zone}/placement_groups/{placement_group_id}",
        local_var_configuration.base_path,
        zone = crate::apis::urlencode(zone),
        placement_group_id = crate::apis::urlencode(placement_group_id)
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
        let local_var_entity: Option<GetPlacementGroupError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get all servers belonging to the given placement group.
pub async fn get_placement_group_servers(
    configuration: &configuration::Configuration,
    zone: &str,
    placement_group_id: &str,
) -> Result<
    crate::models::ScalewayPeriodInstancePeriodV1PeriodGetPlacementGroupServersResponse,
    Error<GetPlacementGroupServersError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/instance/v1/zones/{zone}/placement_groups/{placement_group_id}/servers",
        local_var_configuration.base_path,
        zone = crate::apis::urlencode(zone),
        placement_group_id = crate::apis::urlencode(placement_group_id)
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
        let local_var_entity: Option<GetPlacementGroupServersError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List all placement groups.
pub async fn list_placement_groups(
    configuration: &configuration::Configuration,
    zone: &str,
    per_page: Option<i32>,
    page: Option<i32>,
    organization: Option<&str>,
    project: Option<&str>,
    tags: Option<&str>,
    name: Option<&str>,
) -> Result<
    crate::models::ScalewayPeriodInstancePeriodV1PeriodListPlacementGroupsResponse,
    Error<ListPlacementGroupsError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/instance/v1/zones/{zone}/placement_groups",
        local_var_configuration.base_path,
        zone = crate::apis::urlencode(zone)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = per_page {
        local_var_req_builder =
            local_var_req_builder.query(&[("per_page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder =
            local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = organization {
        local_var_req_builder =
            local_var_req_builder.query(&[("organization", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = project {
        local_var_req_builder =
            local_var_req_builder.query(&[("project", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = tags {
        local_var_req_builder =
            local_var_req_builder.query(&[("tags", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = name {
        local_var_req_builder =
            local_var_req_builder.query(&[("name", &local_var_str.to_string())]);
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
        let local_var_entity: Option<ListPlacementGroupsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Set all parameters of the given placement group.
pub async fn set_placement_group(
    configuration: &configuration::Configuration,
    zone: &str,
    placement_group_id: &str,
    set_placement_group_request: crate::models::SetPlacementGroupRequest,
) -> Result<
    crate::models::ScalewayPeriodInstancePeriodV1PeriodSetPlacementGroupResponse,
    Error<SetPlacementGroupError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/instance/v1/zones/{zone}/placement_groups/{placement_group_id}",
        local_var_configuration.base_path,
        zone = crate::apis::urlencode(zone),
        placement_group_id = crate::apis::urlencode(placement_group_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

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
    local_var_req_builder = local_var_req_builder.json(&set_placement_group_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SetPlacementGroupError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Set all servers belonging to the given placement group.
pub async fn set_placement_group_servers(
    configuration: &configuration::Configuration,
    zone: &str,
    placement_group_id: &str,
    set_placement_group_servers_request: crate::models::SetPlacementGroupServersRequest,
) -> Result<
    crate::models::ScalewayPeriodInstancePeriodV1PeriodSetPlacementGroupServersResponse,
    Error<SetPlacementGroupServersError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/instance/v1/zones/{zone}/placement_groups/{placement_group_id}/servers",
        local_var_configuration.base_path,
        zone = crate::apis::urlencode(zone),
        placement_group_id = crate::apis::urlencode(placement_group_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

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
    local_var_req_builder = local_var_req_builder.json(&set_placement_group_servers_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SetPlacementGroupServersError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update one or more parameter of the given placement group.
pub async fn update_placement_group(
    configuration: &configuration::Configuration,
    zone: &str,
    placement_group_id: &str,
    update_placement_group_request: crate::models::UpdatePlacementGroupRequest,
) -> Result<
    crate::models::ScalewayPeriodInstancePeriodV1PeriodUpdatePlacementGroupResponse,
    Error<UpdatePlacementGroupError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/instance/v1/zones/{zone}/placement_groups/{placement_group_id}",
        local_var_configuration.base_path,
        zone = crate::apis::urlencode(zone),
        placement_group_id = crate::apis::urlencode(placement_group_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

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
    local_var_req_builder = local_var_req_builder.json(&update_placement_group_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdatePlacementGroupError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update all servers belonging to the given placement group.
pub async fn update_placement_group_servers(
    configuration: &configuration::Configuration,
    zone: &str,
    placement_group_id: &str,
    update_placement_group_servers_request: crate::models::UpdatePlacementGroupServersRequest,
) -> Result<
    crate::models::ScalewayPeriodInstancePeriodV1PeriodUpdatePlacementGroupServersResponse,
    Error<UpdatePlacementGroupServersError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/instance/v1/zones/{zone}/placement_groups/{placement_group_id}/servers",
        local_var_configuration.base_path,
        zone = crate::apis::urlencode(zone),
        placement_group_id = crate::apis::urlencode(placement_group_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

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
    local_var_req_builder = local_var_req_builder.json(&update_placement_group_servers_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdatePlacementGroupServersError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
