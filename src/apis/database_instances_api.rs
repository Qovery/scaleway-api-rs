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

/// struct for typed errors of method [`apply_instance_maintenance`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApplyInstanceMaintenanceError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`clone_instance`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CloneInstanceError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_instance`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateInstanceError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_instance`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteInstanceError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_instance`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetInstanceError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_instance_certificate`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetInstanceCertificateError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_instance_log`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetInstanceLogError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_instance_metrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetInstanceMetricsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_instance_logs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListInstanceLogsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_instance_logs_details`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListInstanceLogsDetailsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_instances`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListInstancesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`prepare_instance_logs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PrepareInstanceLogsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`purge_instance_logs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PurgeInstanceLogsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`renew_instance_certificate`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RenewInstanceCertificateError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`restart_instance`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RestartInstanceError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_instance`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateInstanceError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`upgrade_instance`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpgradeInstanceError {
    UnknownValue(serde_json::Value),
}

/// Apply maintenance tasks to your Database Instance. This will trigger pending maintenance tasks to start in your Database Instance and can generate service interruption. Maintenance tasks can be applied between `starts_at` and `stops_at` times, and are run directly by Scaleway at `forced_at` timestamp.
pub async fn apply_instance_maintenance(
    configuration: &configuration::Configuration,
    region: &str,
    instance_id: &str,
    body: serde_json::Value,
) -> Result<models::ScalewayPeriodRdbPeriodV1PeriodMaintenance, Error<ApplyInstanceMaintenanceError>>
{
    // add a prefix to parameters to efficiently prevent name collisions
    let p_region = region;
    let p_instance_id = instance_id;
    let p_body = body;

    let uri_str = format!(
        "{}/rdb/v1/regions/{region}/instances/{instance_id}/apply-maintenance",
        configuration.base_path,
        region = crate::apis::urlencode(p_region),
        instance_id = crate::apis::urlencode(p_instance_id)
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodRdbPeriodV1PeriodMaintenance`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodRdbPeriodV1PeriodMaintenance`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ApplyInstanceMaintenanceError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Clone a given Database Instance, specified by the `region` and `instance_id` parameters. The clone feature allows you to create a new Database Instance from an existing one. The clone includes all existing databases, users and permissions. You can create a clone on a Database Instance bigger than your current one.
pub async fn clone_instance(
    configuration: &configuration::Configuration,
    region: &str,
    instance_id: &str,
    clone_instance_request: models::CloneInstanceRequest,
) -> Result<models::ScalewayPeriodRdbPeriodV1PeriodInstance, Error<CloneInstanceError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_region = region;
    let p_instance_id = instance_id;
    let p_clone_instance_request = clone_instance_request;

    let uri_str = format!(
        "{}/rdb/v1/regions/{region}/instances/{instance_id}/clone",
        configuration.base_path,
        region = crate::apis::urlencode(p_region),
        instance_id = crate::apis::urlencode(p_instance_id)
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
    req_builder = req_builder.json(&p_clone_instance_request);

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodRdbPeriodV1PeriodInstance`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodRdbPeriodV1PeriodInstance`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CloneInstanceError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Create a new Database Instance. You must set the `engine`, `user_name`, `password` and `node_type` parameters. Optionally, you can specify the volume type and size.
pub async fn create_instance(
    configuration: &configuration::Configuration,
    region: &str,
    create_instance_request: models::CreateInstanceRequest,
) -> Result<models::ScalewayPeriodRdbPeriodV1PeriodInstance, Error<CreateInstanceError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_region = region;
    let p_create_instance_request = create_instance_request;

    let uri_str = format!(
        "{}/rdb/v1/regions/{region}/instances",
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
    req_builder = req_builder.json(&p_create_instance_request);

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodRdbPeriodV1PeriodInstance`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodRdbPeriodV1PeriodInstance`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateInstanceError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Delete a given Database Instance, specified by the `region` and `instance_id` parameters. Deleting a Database Instance is permanent, and cannot be undone. Note that upon deletion all your data will be lost.
pub async fn delete_instance(
    configuration: &configuration::Configuration,
    region: &str,
    instance_id: &str,
) -> Result<models::ScalewayPeriodRdbPeriodV1PeriodInstance, Error<DeleteInstanceError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_region = region;
    let p_instance_id = instance_id;

    let uri_str = format!(
        "{}/rdb/v1/regions/{region}/instances/{instance_id}",
        configuration.base_path,
        region = crate::apis::urlencode(p_region),
        instance_id = crate::apis::urlencode(p_instance_id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::DELETE, &uri_str);

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodRdbPeriodV1PeriodInstance`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodRdbPeriodV1PeriodInstance`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteInstanceError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Retrieve information about a given Database Instance, specified by the `region` and `instance_id` parameters. Its full details, including name, status, IP address and port, are returned in the response object.
pub async fn get_instance(
    configuration: &configuration::Configuration,
    region: &str,
    instance_id: &str,
) -> Result<models::ScalewayPeriodRdbPeriodV1PeriodInstance, Error<GetInstanceError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_region = region;
    let p_instance_id = instance_id;

    let uri_str = format!(
        "{}/rdb/v1/regions/{region}/instances/{instance_id}",
        configuration.base_path,
        region = crate::apis::urlencode(p_region),
        instance_id = crate::apis::urlencode(p_instance_id)
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodRdbPeriodV1PeriodInstance`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodRdbPeriodV1PeriodInstance`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetInstanceError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Retrieve information about the TLS certificate of a given Database Instance. Details like name and content are returned in the response.
pub async fn get_instance_certificate(
    configuration: &configuration::Configuration,
    region: &str,
    instance_id: &str,
) -> Result<models::ScalewayPeriodStdPeriodFile, Error<GetInstanceCertificateError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_region = region;
    let p_instance_id = instance_id;

    let uri_str = format!(
        "{}/rdb/v1/regions/{region}/instances/{instance_id}/certificate",
        configuration.base_path,
        region = crate::apis::urlencode(p_region),
        instance_id = crate::apis::urlencode(p_instance_id)
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodStdPeriodFile`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodStdPeriodFile`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetInstanceCertificateError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Retrieve information about the logs of a Database Instance. Specify the `instance_log_id` and `region` in your request to get information such as `download_url`, `status`, `expires_at` and `created_at` about your logs in the response.
pub async fn get_instance_log(
    configuration: &configuration::Configuration,
    region: &str,
    instance_log_id: &str,
) -> Result<models::ScalewayPeriodRdbPeriodV1PeriodInstanceLog, Error<GetInstanceLogError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_region = region;
    let p_instance_log_id = instance_log_id;

    let uri_str = format!(
        "{}/rdb/v1/regions/{region}/logs/{instance_log_id}",
        configuration.base_path,
        region = crate::apis::urlencode(p_region),
        instance_log_id = crate::apis::urlencode(p_instance_log_id)
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodRdbPeriodV1PeriodInstanceLog`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodRdbPeriodV1PeriodInstanceLog`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetInstanceLogError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Retrieve the time series metrics of a given Database Instance. You can define the period from which to retrieve metrics by specifying the `start_date` and `end_date`.
pub async fn get_instance_metrics(
    configuration: &configuration::Configuration,
    region: &str,
    instance_id: &str,
    start_date: Option<String>,
    end_date: Option<String>,
    metric_name: Option<&str>,
) -> Result<models::ScalewayPeriodRdbPeriodV1PeriodInstanceMetrics, Error<GetInstanceMetricsError>>
{
    // add a prefix to parameters to efficiently prevent name collisions
    let p_region = region;
    let p_instance_id = instance_id;
    let p_start_date = start_date;
    let p_end_date = end_date;
    let p_metric_name = metric_name;

    let uri_str = format!(
        "{}/rdb/v1/regions/{region}/instances/{instance_id}/metrics",
        configuration.base_path,
        region = crate::apis::urlencode(p_region),
        instance_id = crate::apis::urlencode(p_instance_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_start_date {
        req_builder = req_builder.query(&[("start_date", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_end_date {
        req_builder = req_builder.query(&[("end_date", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_metric_name {
        req_builder = req_builder.query(&[("metric_name", &param_value.to_string())]);
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodRdbPeriodV1PeriodInstanceMetrics`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodRdbPeriodV1PeriodInstanceMetrics`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetInstanceMetricsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// List the available logs of a Database Instance. By default, the logs returned in the list are ordered by creation date in ascending order, though this can be modified via the order_by field.
pub async fn list_instance_logs(
    configuration: &configuration::Configuration,
    region: &str,
    instance_id: &str,
    order_by: Option<&str>,
) -> Result<
    models::ScalewayPeriodRdbPeriodV1PeriodListInstanceLogsResponse,
    Error<ListInstanceLogsError>,
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_region = region;
    let p_instance_id = instance_id;
    let p_order_by = order_by;

    let uri_str = format!(
        "{}/rdb/v1/regions/{region}/instances/{instance_id}/logs",
        configuration.base_path,
        region = crate::apis::urlencode(p_region),
        instance_id = crate::apis::urlencode(p_instance_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_order_by {
        req_builder = req_builder.query(&[("order_by", &param_value.to_string())]);
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodRdbPeriodV1PeriodListInstanceLogsResponse`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodRdbPeriodV1PeriodListInstanceLogsResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListInstanceLogsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// List remote log details. By default, the details returned in the list are ordered by creation date in ascending order, though this can be modified via the order_by field.
pub async fn list_instance_logs_details(
    configuration: &configuration::Configuration,
    region: &str,
    instance_id: &str,
) -> Result<
    models::ScalewayPeriodRdbPeriodV1PeriodListInstanceLogsDetailsResponse,
    Error<ListInstanceLogsDetailsError>,
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_region = region;
    let p_instance_id = instance_id;

    let uri_str = format!(
        "{}/rdb/v1/regions/{region}/instances/{instance_id}/logs-details",
        configuration.base_path,
        region = crate::apis::urlencode(p_region),
        instance_id = crate::apis::urlencode(p_instance_id)
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodRdbPeriodV1PeriodListInstanceLogsDetailsResponse`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodRdbPeriodV1PeriodListInstanceLogsDetailsResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListInstanceLogsDetailsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// List all Database Instances in the specified region, for a given Scaleway Organization or Scaleway Project. By default, the Database Instances returned in the list are ordered by creation date in ascending order, though this can be modified via the order_by field. You can define additional parameters for your query, such as `tags` and `name`. For the `name` parameter, the value you include will be checked against the whole name string to see if it includes the string you put in the parameter.
pub async fn list_instances(
    configuration: &configuration::Configuration,
    region: &str,
    tags: Option<Vec<String>>,
    name: Option<&str>,
    order_by: Option<&str>,
    organization_id: Option<&str>,
    project_id: Option<&str>,
    page: Option<i32>,
    page_size: Option<i32>,
) -> Result<models::ScalewayPeriodRdbPeriodV1PeriodListInstancesResponse, Error<ListInstancesError>>
{
    // add a prefix to parameters to efficiently prevent name collisions
    let p_region = region;
    let p_tags = tags;
    let p_name = name;
    let p_order_by = order_by;
    let p_organization_id = organization_id;
    let p_project_id = project_id;
    let p_page = page;
    let p_page_size = page_size;

    let uri_str = format!(
        "{}/rdb/v1/regions/{region}/instances",
        configuration.base_path,
        region = crate::apis::urlencode(p_region)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_tags {
        req_builder = match "multi" {
            "multi" => req_builder.query(
                &param_value
                    .iter()
                    .map(|p| ("tags".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "tags",
                &param_value
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref param_value) = p_name {
        req_builder = req_builder.query(&[("name", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_order_by {
        req_builder = req_builder.query(&[("order_by", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_organization_id {
        req_builder = req_builder.query(&[("organization_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_project_id {
        req_builder = req_builder.query(&[("project_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_page_size {
        req_builder = req_builder.query(&[("page_size", &param_value.to_string())]);
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodRdbPeriodV1PeriodListInstancesResponse`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodRdbPeriodV1PeriodListInstancesResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListInstancesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Prepare your Database Instance logs. You can define the `start_date` and `end_date` parameters for your query. The download URL is returned in the response. Logs are recorded from 00h00 to 23h59 and then aggregated in a `.log` file once a day. Therefore, even if you specify a timeframe from which you want to get the logs, you will receive logs from the full 24 hours.
pub async fn prepare_instance_logs(
    configuration: &configuration::Configuration,
    region: &str,
    instance_id: &str,
    prepare_instance_logs_request: models::PrepareInstanceLogsRequest,
) -> Result<
    models::ScalewayPeriodRdbPeriodV1PeriodPrepareInstanceLogsResponse,
    Error<PrepareInstanceLogsError>,
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_region = region;
    let p_instance_id = instance_id;
    let p_prepare_instance_logs_request = prepare_instance_logs_request;

    let uri_str = format!(
        "{}/rdb/v1/regions/{region}/instances/{instance_id}/prepare-logs",
        configuration.base_path,
        region = crate::apis::urlencode(p_region),
        instance_id = crate::apis::urlencode(p_instance_id)
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
    req_builder = req_builder.json(&p_prepare_instance_logs_request);

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodRdbPeriodV1PeriodPrepareInstanceLogsResponse`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodRdbPeriodV1PeriodPrepareInstanceLogsResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<PrepareInstanceLogsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Purge a given remote log from a Database Instance. You can specify the `log_name` of the log you wish to clean from your Database Instance.
pub async fn purge_instance_logs(
    configuration: &configuration::Configuration,
    region: &str,
    instance_id: &str,
    purge_instance_logs_request: models::PurgeInstanceLogsRequest,
) -> Result<(), Error<PurgeInstanceLogsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_region = region;
    let p_instance_id = instance_id;
    let p_purge_instance_logs_request = purge_instance_logs_request;

    let uri_str = format!(
        "{}/rdb/v1/regions/{region}/instances/{instance_id}/purge-logs",
        configuration.base_path,
        region = crate::apis::urlencode(p_region),
        instance_id = crate::apis::urlencode(p_instance_id)
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
    req_builder = req_builder.json(&p_purge_instance_logs_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<PurgeInstanceLogsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Renew a TLS for a Database Instance. Renewing a certificate means that you will not be able to connect to your Database Instance using the previous certificate. You will also need to download and update the new certificate for all database clients.
pub async fn renew_instance_certificate(
    configuration: &configuration::Configuration,
    region: &str,
    instance_id: &str,
    body: serde_json::Value,
) -> Result<(), Error<RenewInstanceCertificateError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_region = region;
    let p_instance_id = instance_id;
    let p_body = body;

    let uri_str = format!(
        "{}/rdb/v1/regions/{region}/instances/{instance_id}/renew-certificate",
        configuration.base_path,
        region = crate::apis::urlencode(p_region),
        instance_id = crate::apis::urlencode(p_instance_id)
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
        let entity: Option<RenewInstanceCertificateError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Restart a given Database Instance, specified by the `region` and `instance_id` parameters. The status of the Database Instance returned in the response.
pub async fn restart_instance(
    configuration: &configuration::Configuration,
    region: &str,
    instance_id: &str,
    body: serde_json::Value,
) -> Result<models::ScalewayPeriodRdbPeriodV1PeriodInstance, Error<RestartInstanceError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_region = region;
    let p_instance_id = instance_id;
    let p_body = body;

    let uri_str = format!(
        "{}/rdb/v1/regions/{region}/instances/{instance_id}/restart",
        configuration.base_path,
        region = crate::apis::urlencode(p_region),
        instance_id = crate::apis::urlencode(p_instance_id)
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodRdbPeriodV1PeriodInstance`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodRdbPeriodV1PeriodInstance`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<RestartInstanceError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Update the parameters of a Database Instance, including name, tags and backup schedule details.
pub async fn update_instance(
    configuration: &configuration::Configuration,
    region: &str,
    instance_id: &str,
    update_instance_request: models::UpdateInstanceRequest,
) -> Result<models::ScalewayPeriodRdbPeriodV1PeriodInstance, Error<UpdateInstanceError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_region = region;
    let p_instance_id = instance_id;
    let p_update_instance_request = update_instance_request;

    let uri_str = format!(
        "{}/rdb/v1/regions/{region}/instances/{instance_id}",
        configuration.base_path,
        region = crate::apis::urlencode(p_region),
        instance_id = crate::apis::urlencode(p_instance_id)
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
    req_builder = req_builder.json(&p_update_instance_request);

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodRdbPeriodV1PeriodInstance`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodRdbPeriodV1PeriodInstance`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateInstanceError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Upgrade your current Database Instance specifications like node type, high availability, volume, or the database engine version. Note that upon upgrade the `enable_ha` parameter can only be set to `true`.
pub async fn upgrade_instance(
    configuration: &configuration::Configuration,
    region: &str,
    instance_id: &str,
    upgrade_instance_request: models::UpgradeInstanceRequest,
) -> Result<models::ScalewayPeriodRdbPeriodV1PeriodInstance, Error<UpgradeInstanceError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_region = region;
    let p_instance_id = instance_id;
    let p_upgrade_instance_request = upgrade_instance_request;

    let uri_str = format!(
        "{}/rdb/v1/regions/{region}/instances/{instance_id}/upgrade",
        configuration.base_path,
        region = crate::apis::urlencode(p_region),
        instance_id = crate::apis::urlencode(p_instance_id)
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
    req_builder = req_builder.json(&p_upgrade_instance_request);

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodRdbPeriodV1PeriodInstance`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodRdbPeriodV1PeriodInstance`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpgradeInstanceError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
