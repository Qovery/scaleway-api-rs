/*
 * Elastic Metal API
 *
 * Scaleway Elastic Metal servers are dedicated physical servers that you can order on-demand, like Instances. You can install an OS or other images on your Elastic Metal server and connect to it via SSH to use it as you require. You can power off the server when you are not using or delete it from your account once you have finished using it. Elastic Metal servers are ideal for large workloads, big data, and applications that require increased security and dedicated resources.  (switchcolumn) <Message type=\"tip\">   Check out our dedicated APIs to manage [Private Networks](https://www.scaleway.com/en/developers/api/elastic-metal/private-network-api/) and [Flexible IPs](https://www.scaleway.com/en/developers/api/elastic-metal-flexible-ip) for Elastic Metal servers. </Message> (switchcolumn)  ## Concepts  Refer to our [dedicated concepts](https://www.scaleway.com/en/docs/compute/elastic-metal/concepts/) page to find definitions of the different terms referring to Elastic Metal servers.  ## Quickstart  (switchcolumn) (switchcolumn)  1. **Configure your environment variables.**     ```bash     export PROJECT_ID=\"<project-id>\"     export ACCESS_KEY=\"<access-key>\"     export SECRET_KEY=\"<secret-key>\"     export ZONE=\"<availability-zone>\"     ```     <Message type=\"note\">       This is an optional step that seeks to simplify your usage of the Bare Metal API.     </Message>  2. **Edit the POST request payload** that we will use in the next step to create an Elastic Metal server. Modify the values in the example according to your needs, using the information in the table to help.     ```json     {     \"offer_id\": \"string\",     \"project_id\": \"string\",     \"name\": \"string\",     \"description\": \"string\",     \"tags\": [         \"tag1\", \"tag2\"     ],     \"install\": {         \"os_id\": \"string\",         \"hostname\": \"string\",         \"ssh_key_ids\": [         \"string\"         ],         \"user\": \"string\",         \"password\": \"string\",         \"service_user\": \"string\",         \"service_password\": \"string\"     },     \"option_ids\": [         \"string\"     ]     }     ```      | Parameter        | Description                                                        |     | :--------------- | :----------------------------------------------------------------- |     | `offer_id`           | **REQUIRED** UUID of the Elastic Metal offer                                         |     | `project_id`     | **REQUIRED** UUID of the project you want to create your Elastic Metal in.  |     | `name`           | **REQUIRED** Name of the Elastic Metal server (≠hostname)                                          |     | `description`     | **REQUIRED** A description of your server (max 255 characters)                             |     | `tags`  | **OPTIONAL** An array of tags associated with your server   |     | `os_id`  | The ID of the operating system image to install on the server   |     | `hostname`  | Hostname of the server   |     | `ssh_key_ids`  | SSH key IDs authorized on the server   |     | `user`  | **NULLABLE** A regular user to be configured on the server   |     | `password`  | **NULLABLE** The password for the user account   |     | `service_user`  | **NULLABLE** A service user for third party services (user to login in services such as BigBlueButton)  |     | `service password`  | **NULLABLE** Password for the service user   |     | `option_ids`  | IDs of options to enable on server  |      <Message type=\"tip\">       To find your Project ID you can either use the [IAM API](https://www.scaleway.com/en/developers/api/account#path-projects-list-all-projects-of-an-organization) or the [Scaleway console](https://console.scaleway.com/project/settings):     </Message>  3. **Run the following command** to create an Elastic Metal server. Make sure you include the payload you edited in the previous step.     ```bash     curl -X POST \\       -H \"Content-Type: application/json\" \\       -H \"X-Auth-Token: $SECRET_KEY\" https://api.scaleway.com//baremetal/v1/zones/$ZONE/servers \\       -d '{         \"offer_id\": \"bd757ca3-a71b-4158-9ef5-39436b6db2a4\",         \"project_id\": \"cc6d123a-bc09-4e24-b5d9-3310f2104e13\",         \"name\": \"MyElasticMetal\",         \"description\": \"My_Elastic_Metal_Server\",         \"tags\": [             \"Ubuntu22\", \"Dedicated\"         ],         \"install\": {             \"os_id\": \"96e5f0f2-d216-4de2-8a15-68730d877885\",             \"hostname\": \"elasticmetal.example.com\",             \"ssh_key_ids\": [             \"fa05e77f-66b7-43b9-bc21-4dfe3c5bb624\"             ],             \"user\": \"ubuntu\",             \"password\": \"mySecretPa$$word\"         \"option_ids\": [             \"string\"         ]       }\"     ``` 4. **List your Elastic Metal servers.**     ```bash     curl -X GET \\       -H \"Content-Type: application/json\" \\       -H \"X-Auth-Token: $SECRET_KEY\" https://api.scaleway.com/baremetal/v1/zones/$ZONE/servers     ```  5. **Retrieve your Elastic Metal server IP** from the response.  6. **Connect to your Elastic Metal server** using SSH     ```bash     ssh root@my-elastic-metal-server-ip     ```  (switchcolumn) <Message type=\"requirement\"> To perform the following steps, you must first ensure that:   - you have an account and are logged into the [Scaleway console](https://console.scaleway.com/organization)   - you have created an [API key](https://www.scaleway.com/en/docs/identity-and-access-management/iam/how-to/create-api-keys/) and that the API key has sufficient [IAM permissions](https://www.scaleway.com/en/docs/identity-and-access-management/iam/reference-content/permission-sets/) to perform the actions described on this page.   - you have [installed `curl`](https://curl.se/download.html) </Message> (switchcolumn)  ## Technical information  ### Features  - Installation (Server is installed by preseed [preseed: complete install from a virtual media], you must define at least one ssh key to install your server) - Start/Stop/Reboot - Rescue Reboot, a rescue image is an operating system image designed to help you diagnose and fix OS experiencing failures. When your server boot on rescue, you can mount your disks and start diagnosing/fixing your image. - Billed by the minute (billing starts when the server is delivered and stops when the server is deleted) - IPv6, all servers are available with a /128 IPv6 subnet - ReverseIP, You can configure your reverse IP (IPv4 and IPv6), you must register the server IP in your DNS records before calling the endpoint - Basic monitoring with ping status - Flexible IP is available ([documentation](https://www.scaleway.com/en/developers/api/elastic-metal-flexible-ip))  ### Availability Zones  Scaleway's infrastructure is spread across different [regions and Availability Zones](https://www.scaleway.com/en/docs/console/my-account/reference-content/products-availability/).  Elastic Metal servers are available in Paris, Amsterdam, and Warsaw regions, with product availability in the following AZs:  | Name      | API ID                           | |-----------|----------------------------------| | Paris     | `fr-par-1` `fr-par-2`            | | Amsterdam | `nl-ams-1` `nl-ams-2`            | | Warsaw    | `pl-waw-2` `pl-waw-3`            |  ## Technical limitations  - Failover IPs are not available in API `v1`, use the API `v1alpha1` ## Going further  For more help using Scaleway Elastic Metal servers, check out the following resources: - Our [main documentation](https://www.scaleway.com/en/docs/compute/elastic-metal/) - The #elastic-metal channel on our [Slack Community](https://www.scaleway.com/en/docs/tutorials/scaleway-slack-community/) - Our [support ticketing system](https://www.scaleway.com/en/docs/console/my-account/how-to/open-a-support-ticket) ### Troubleshooting  #### How is the installation of Elastic Metal servers done?  - The installation of Elastic Metal servers is done by preseed (± 10min) (preseed: complete install from a virtual media) #### How can I retrieve my secret key and Project ID?  You can find your [secret key](https://console.scaleway.com/iam/api-keys) and your [Project ID](https://console.scaleway.com/project/credentials) in the Scaleway console.  #### How can I add my server to a Private Network?  See [our dedicated documentation](/en/developers/api/elastic-metal-flexible-ip).
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, ContentType, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{de::Error as _, Deserialize, Serialize};

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

/// struct for typed errors of method [`list_cluster_available_types`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListClusterAvailableTypesError {
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

/// struct for typed errors of method [`set_cluster_type`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetClusterTypeError {
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

/// Create a new Kubernetes cluster in a Scaleway region.
pub async fn create_cluster(
    configuration: &configuration::Configuration,
    region: &str,
    create_cluster_request: models::CreateClusterRequest,
) -> Result<models::ScalewayPeriodK8sPeriodV1PeriodCluster, Error<CreateClusterError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_region = region;
    let p_create_cluster_request = create_cluster_request;

    let uri_str = format!(
        "{}/k8s/v1/regions/{region}/clusters",
        configuration.base_path,
        region = crate::apis::urlencode(p_region)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-Auth-Token", value);
    };
    req_builder = req_builder.json(&p_create_cluster_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodK8sPeriodV1PeriodCluster`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodK8sPeriodV1PeriodCluster`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateClusterError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Delete a specific Kubernetes cluster and all its associated pools and nodes, and possibly its associated Load Balancers or Block Volumes.
pub async fn delete_cluster(
    configuration: &configuration::Configuration,
    region: &str,
    cluster_id: &str,
    with_additional_resources: bool,
) -> Result<models::ScalewayPeriodK8sPeriodV1PeriodCluster, Error<DeleteClusterError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_region = region;
    let p_cluster_id = cluster_id;
    let p_with_additional_resources = with_additional_resources;

    let uri_str = format!(
        "{}/k8s/v1/regions/{region}/clusters/{cluster_id}",
        configuration.base_path,
        region = crate::apis::urlencode(p_region),
        cluster_id = crate::apis::urlencode(p_cluster_id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::DELETE, &uri_str);

    req_builder = req_builder.query(&[(
        "with_additional_resources",
        &p_with_additional_resources.to_string(),
    )]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-Auth-Token", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodK8sPeriodV1PeriodCluster`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodK8sPeriodV1PeriodCluster`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteClusterError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Retrieve information about a specific Kubernetes cluster.
pub async fn get_cluster(
    configuration: &configuration::Configuration,
    region: &str,
    cluster_id: &str,
) -> Result<models::ScalewayPeriodK8sPeriodV1PeriodCluster, Error<GetClusterError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_region = region;
    let p_cluster_id = cluster_id;

    let uri_str = format!(
        "{}/k8s/v1/regions/{region}/clusters/{cluster_id}",
        configuration.base_path,
        region = crate::apis::urlencode(p_region),
        cluster_id = crate::apis::urlencode(p_cluster_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-Auth-Token", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodK8sPeriodV1PeriodCluster`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodK8sPeriodV1PeriodCluster`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetClusterError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Download the Kubernetes cluster config file (also known as `kubeconfig`) for a specific cluster to use it with `kubectl`. Tip: add `?dl=1` at the end of the URL to directly retrieve the base64 decoded kubeconfig. If you choose not to, the kubeconfig will be base64 encoded.
pub async fn get_cluster_kube_config(
    configuration: &configuration::Configuration,
    region: &str,
    cluster_id: &str,
    redacted: Option<bool>,
) -> Result<models::ScalewayPeriodStdPeriodFile, Error<GetClusterKubeConfigError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_region = region;
    let p_cluster_id = cluster_id;
    let p_redacted = redacted;

    let uri_str = format!(
        "{}/k8s/v1/regions/{region}/clusters/{cluster_id}/kubeconfig",
        configuration.base_path,
        region = crate::apis::urlencode(p_region),
        cluster_id = crate::apis::urlencode(p_cluster_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_redacted {
        req_builder = req_builder.query(&[("redacted", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-Auth-Token", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodStdPeriodFile`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodStdPeriodFile`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetClusterKubeConfigError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// List the cluster types that a specific Kubernetes cluster is allowed to switch to.
pub async fn list_cluster_available_types(
    configuration: &configuration::Configuration,
    region: &str,
    cluster_id: &str,
) -> Result<
    models::ScalewayPeriodK8sPeriodV1PeriodListClusterAvailableTypesResponse,
    Error<ListClusterAvailableTypesError>,
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_region = region;
    let p_cluster_id = cluster_id;

    let uri_str = format!(
        "{}/k8s/v1/regions/{region}/clusters/{cluster_id}/available-types",
        configuration.base_path,
        region = crate::apis::urlencode(p_region),
        cluster_id = crate::apis::urlencode(p_cluster_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-Auth-Token", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodK8sPeriodV1PeriodListClusterAvailableTypesResponse`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodK8sPeriodV1PeriodListClusterAvailableTypesResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListClusterAvailableTypesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// List the versions that a specific Kubernetes cluster is allowed to upgrade to. Results will include every patch version greater than the current patch, as well as one minor version ahead of the current version. Any upgrade skipping a minor version will not work.
pub async fn list_cluster_available_versions(
    configuration: &configuration::Configuration,
    region: &str,
    cluster_id: &str,
) -> Result<
    models::ScalewayPeriodK8sPeriodV1PeriodListClusterAvailableVersionsResponse,
    Error<ListClusterAvailableVersionsError>,
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_region = region;
    let p_cluster_id = cluster_id;

    let uri_str = format!(
        "{}/k8s/v1/regions/{region}/clusters/{cluster_id}/available-versions",
        configuration.base_path,
        region = crate::apis::urlencode(p_region),
        cluster_id = crate::apis::urlencode(p_cluster_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-Auth-Token", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodK8sPeriodV1PeriodListClusterAvailableVersionsResponse`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodK8sPeriodV1PeriodListClusterAvailableVersionsResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListClusterAvailableVersionsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// List all existing Kubernetes clusters in a specific region.
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
    private_network_id: Option<&str>,
) -> Result<models::ScalewayPeriodK8sPeriodV1PeriodListClustersResponse, Error<ListClustersError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_region = region;
    let p_organization_id = organization_id;
    let p_project_id = project_id;
    let p_order_by = order_by;
    let p_page = page;
    let p_page_size = page_size;
    let p_name = name;
    let p_status = status;
    let p_type = r#type;
    let p_private_network_id = private_network_id;

    let uri_str = format!(
        "{}/k8s/v1/regions/{region}/clusters",
        configuration.base_path,
        region = crate::apis::urlencode(p_region)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_organization_id {
        req_builder = req_builder.query(&[("organization_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_project_id {
        req_builder = req_builder.query(&[("project_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_order_by {
        req_builder = req_builder.query(&[("order_by", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_page_size {
        req_builder = req_builder.query(&[("page_size", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_name {
        req_builder = req_builder.query(&[("name", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_status {
        req_builder = req_builder.query(&[("status", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_type {
        req_builder = req_builder.query(&[("type", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_private_network_id {
        req_builder = req_builder.query(&[("private_network_id", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-Auth-Token", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodK8sPeriodV1PeriodListClustersResponse`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodK8sPeriodV1PeriodListClustersResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListClustersError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Reset the admin token for a specific Kubernetes cluster. This will revoke the old admin token (which will not be usable afterwards) and create a new one. Note that you will need to download the kubeconfig again to keep interacting with the cluster.
pub async fn reset_cluster_admin_token(
    configuration: &configuration::Configuration,
    region: &str,
    cluster_id: &str,
    body: serde_json::Value,
) -> Result<(), Error<ResetClusterAdminTokenError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_region = region;
    let p_cluster_id = cluster_id;
    let p_body = body;

    let uri_str = format!(
        "{}/k8s/v1/regions/{region}/clusters/{cluster_id}/reset-admin-token",
        configuration.base_path,
        region = crate::apis::urlencode(p_region),
        cluster_id = crate::apis::urlencode(p_cluster_id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-Auth-Token", value);
    };
    req_builder = req_builder.json(&p_body);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<ResetClusterAdminTokenError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Change the type of a specific Kubernetes cluster. To see the possible values you can enter for the `type` field, [list available cluster types](#list-available-cluster-types-for-a-cluster).
pub async fn set_cluster_type(
    configuration: &configuration::Configuration,
    region: &str,
    cluster_id: &str,
    set_cluster_type_request: models::SetClusterTypeRequest,
) -> Result<models::ScalewayPeriodK8sPeriodV1PeriodCluster, Error<SetClusterTypeError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_region = region;
    let p_cluster_id = cluster_id;
    let p_set_cluster_type_request = set_cluster_type_request;

    let uri_str = format!(
        "{}/k8s/v1/regions/{region}/clusters/{cluster_id}/set-type",
        configuration.base_path,
        region = crate::apis::urlencode(p_region),
        cluster_id = crate::apis::urlencode(p_cluster_id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-Auth-Token", value);
    };
    req_builder = req_builder.json(&p_set_cluster_type_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodK8sPeriodV1PeriodCluster`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodK8sPeriodV1PeriodCluster`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SetClusterTypeError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Update information on a specific Kubernetes cluster. You can update details such as its name, description, tags and configuration. To upgrade a cluster, you will need to use the dedicated endpoint.
pub async fn update_cluster(
    configuration: &configuration::Configuration,
    region: &str,
    cluster_id: &str,
    update_cluster_request: models::UpdateClusterRequest,
) -> Result<models::ScalewayPeriodK8sPeriodV1PeriodCluster, Error<UpdateClusterError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_region = region;
    let p_cluster_id = cluster_id;
    let p_update_cluster_request = update_cluster_request;

    let uri_str = format!(
        "{}/k8s/v1/regions/{region}/clusters/{cluster_id}",
        configuration.base_path,
        region = crate::apis::urlencode(p_region),
        cluster_id = crate::apis::urlencode(p_cluster_id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-Auth-Token", value);
    };
    req_builder = req_builder.json(&p_update_cluster_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodK8sPeriodV1PeriodCluster`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodK8sPeriodV1PeriodCluster`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateClusterError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Upgrade a specific Kubernetes cluster and possibly its associated pools to a specific and supported Kubernetes version.
pub async fn upgrade_cluster(
    configuration: &configuration::Configuration,
    region: &str,
    cluster_id: &str,
    upgrade_cluster_request: models::UpgradeClusterRequest,
) -> Result<models::ScalewayPeriodK8sPeriodV1PeriodCluster, Error<UpgradeClusterError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_region = region;
    let p_cluster_id = cluster_id;
    let p_upgrade_cluster_request = upgrade_cluster_request;

    let uri_str = format!(
        "{}/k8s/v1/regions/{region}/clusters/{cluster_id}/upgrade",
        configuration.base_path,
        region = crate::apis::urlencode(p_region),
        cluster_id = crate::apis::urlencode(p_cluster_id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-Auth-Token", value);
    };
    req_builder = req_builder.json(&p_upgrade_cluster_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodK8sPeriodV1PeriodCluster`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodK8sPeriodV1PeriodCluster`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpgradeClusterError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
