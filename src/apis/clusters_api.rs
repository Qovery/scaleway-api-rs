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

/// struct for typed errors of method [`create_cluster`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateClusterError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_cluster`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteClusterError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_cluster`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetClusterError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_cluster_kube_config`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetClusterKubeConfigError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_cluster_available_versions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListClusterAvailableVersionsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_clusters`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListClustersError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`reset_cluster_admin_token`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResetClusterAdminTokenError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_cluster`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateClusterError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`upgrade_cluster`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpgradeClusterError {
    UnknownValue(serde_json::Value),
}

/// This method allows to create a new Kubernetes cluster on an account.
pub async fn create_cluster(
    configuration: &configuration::Configuration,
    region: &str,
    create_cluster_request: crate::models::CreateClusterRequest,
) -> Result<crate::models::ScalewayPeriodK8sPeriodV1PeriodCluster, Error<CreateClusterError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/k8s/v1/regions/{region}/clusters",
        local_var_configuration.base_path,
        region = crate::apis::urlencode(region)
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
    local_var_req_builder = local_var_req_builder.json(&create_cluster_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateClusterError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This method allows to delete a specific cluster and all its associated pools and nodes. Note that this method will not delete any Load Balancers or Block Volumes that are associated with the cluster.
pub async fn delete_cluster(
    configuration: &configuration::Configuration,
    region: &str,
    cluster_id: &str,
    with_additional_resources: Option<bool>,
) -> Result<crate::models::ScalewayPeriodK8sPeriodV1PeriodCluster, Error<DeleteClusterError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/k8s/v1/regions/{region}/clusters/{cluster_id}",
        local_var_configuration.base_path,
        region = crate::apis::urlencode(region),
        cluster_id = crate::apis::urlencode(cluster_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = with_additional_resources {
        local_var_req_builder = local_var_req_builder
            .query(&[("with_additional_resources", &local_var_str.to_string())]);
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
        let local_var_entity: Option<DeleteClusterError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This method allows to get details about a specific Kubernetes cluster.
pub async fn get_cluster(
    configuration: &configuration::Configuration,
    region: &str,
    cluster_id: &str,
) -> Result<crate::models::ScalewayPeriodK8sPeriodV1PeriodCluster, Error<GetClusterError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/k8s/v1/regions/{region}/clusters/{cluster_id}",
        local_var_configuration.base_path,
        region = crate::apis::urlencode(region),
        cluster_id = crate::apis::urlencode(cluster_id)
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
        let local_var_entity: Option<GetClusterError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This method allows to download the Kubernetes cluster config file (AKA kubeconfig) for a specific cluster in order to use it with, for instance, `kubectl`. Tips: add `?dl=1` at the end of the URL to directly get the base64 decoded kubeconfig. If not, the kubeconfig will be base64 encoded.
pub async fn get_cluster_kube_config(
    configuration: &configuration::Configuration,
    region: &str,
    cluster_id: &str,
) -> Result<crate::models::ScalewayPeriodStdPeriodFile, Error<GetClusterKubeConfigError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/k8s/v1/regions/{region}/clusters/{cluster_id}/kubeconfig",
        local_var_configuration.base_path,
        region = crate::apis::urlencode(region),
        cluster_id = crate::apis::urlencode(cluster_id)
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
        let local_var_entity: Option<GetClusterKubeConfigError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This method allows to list the versions that a specific Kubernetes cluster is allowed to upgrade to. Note that it will be every patch version greater than the actual one as well a one minor version ahead of the actual one. Upgrades skipping a minor version will not work.
pub async fn list_cluster_available_versions(
    configuration: &configuration::Configuration,
    region: &str,
    cluster_id: &str,
) -> Result<
    crate::models::ScalewayPeriodK8sPeriodV1PeriodListClusterAvailableVersionsResponse,
    Error<ListClusterAvailableVersionsError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/k8s/v1/regions/{region}/clusters/{cluster_id}/available-versions",
        local_var_configuration.base_path,
        region = crate::apis::urlencode(region),
        cluster_id = crate::apis::urlencode(cluster_id)
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
        let local_var_entity: Option<ListClusterAvailableVersionsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This method allows to list all the existing Kubernetes clusters in an account.
pub async fn list_clusters(
    configuration: &configuration::Configuration,
    region: &str,
    organization_id: Option<&str>,
    project_id: Option<&str>,
    order_by: Option<&str>,
    page: Option<i32>,
    page_size: Option<i32>,
    name: Option<&str>,
    status: Option<&str>,
    r#type: Option<&str>,
) -> Result<
    crate::models::ScalewayPeriodK8sPeriodV1PeriodListClustersResponse,
    Error<ListClustersError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/k8s/v1/regions/{region}/clusters",
        local_var_configuration.base_path,
        region = crate::apis::urlencode(region)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = organization_id {
        local_var_req_builder =
            local_var_req_builder.query(&[("organization_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = project_id {
        local_var_req_builder =
            local_var_req_builder.query(&[("project_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = order_by {
        local_var_req_builder =
            local_var_req_builder.query(&[("order_by", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder =
            local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_size {
        local_var_req_builder =
            local_var_req_builder.query(&[("page_size", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = name {
        local_var_req_builder =
            local_var_req_builder.query(&[("name", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = status {
        local_var_req_builder =
            local_var_req_builder.query(&[("status", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = r#type {
        local_var_req_builder =
            local_var_req_builder.query(&[("type", &local_var_str.to_string())]);
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
        let local_var_entity: Option<ListClustersError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This method allows to reset the admin token for a specific Kubernetes cluster. This will invalidate the old admin token (which will not be usable after) and create a new one. Note that the redownload of the kubeconfig will be necessary to keep interacting with the cluster (if the old admin token was used).
pub async fn reset_cluster_admin_token(
    configuration: &configuration::Configuration,
    region: &str,
    cluster_id: &str,
    body: serde_json::Value,
) -> Result<(), Error<ResetClusterAdminTokenError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/k8s/v1/regions/{region}/clusters/{cluster_id}/reset-admin-token",
        local_var_configuration.base_path,
        region = crate::apis::urlencode(region),
        cluster_id = crate::apis::urlencode(cluster_id)
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
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<ResetClusterAdminTokenError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This method allows to update a specific Kubernetes cluster. Note that this method is not made to upgrade a Kubernetes cluster.
pub async fn update_cluster(
    configuration: &configuration::Configuration,
    region: &str,
    cluster_id: &str,
    update_cluster_request: crate::models::UpdateClusterRequest,
) -> Result<crate::models::ScalewayPeriodK8sPeriodV1PeriodCluster, Error<UpdateClusterError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/k8s/v1/regions/{region}/clusters/{cluster_id}",
        local_var_configuration.base_path,
        region = crate::apis::urlencode(region),
        cluster_id = crate::apis::urlencode(cluster_id)
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
    local_var_req_builder = local_var_req_builder.json(&update_cluster_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateClusterError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This method allows to upgrade a specific Kubernetes cluster and/or its associated pools to a specific and supported Kubernetes version.
pub async fn upgrade_cluster(
    configuration: &configuration::Configuration,
    region: &str,
    cluster_id: &str,
    upgrade_cluster_request: crate::models::UpgradeClusterRequest,
) -> Result<crate::models::ScalewayPeriodK8sPeriodV1PeriodCluster, Error<UpgradeClusterError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/k8s/v1/regions/{region}/clusters/{cluster_id}/upgrade",
        local_var_configuration.base_path,
        region = crate::apis::urlencode(region),
        cluster_id = crate::apis::urlencode(cluster_id)
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
    local_var_req_builder = local_var_req_builder.json(&upgrade_cluster_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpgradeClusterError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
