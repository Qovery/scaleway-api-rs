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

/// struct for typed errors of method [`add_option_server`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddOptionServerError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_server`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateServerError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_option_server`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteOptionServerError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_server`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteServerError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_server`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetServerError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_server_metrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetServerMetricsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`install_server`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InstallServerError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_server_events`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListServerEventsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_servers`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListServersError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`migrate_server_to_monthly_offer`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MigrateServerToMonthlyOfferError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_ip`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateIpError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_server`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateServerError {
    UnknownValue(serde_json::Value),
}

/// Add an option, such as Private Networks, to a specific server.
pub async fn add_option_server(
    configuration: &configuration::Configuration,
    zone: &str,
    server_id: &str,
    option_id: &str,
    add_option_server_request: models::AddOptionServerRequest,
) -> Result<models::ScalewayPeriodBaremetalPeriodV1PeriodServer, Error<AddOptionServerError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_zone = zone;
    let p_server_id = server_id;
    let p_option_id = option_id;
    let p_add_option_server_request = add_option_server_request;

    let uri_str = format!(
        "{}/baremetal/v1/zones/{zone}/servers/{server_id}/options/{option_id}",
        configuration.base_path,
        zone = crate::apis::urlencode(p_zone),
        server_id = crate::apis::urlencode(p_server_id),
        option_id = crate::apis::urlencode(p_option_id)
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
    req_builder = req_builder.json(&p_add_option_server_request);

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodBaremetalPeriodV1PeriodServer`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodBaremetalPeriodV1PeriodServer`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AddOptionServerError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Create a new Elastic Metal server. Once the server is created, proceed with the [installation of an OS](#post-3e949e).
pub async fn create_server(
    configuration: &configuration::Configuration,
    zone: &str,
    create_server_request: models::CreateServerRequest,
) -> Result<models::ScalewayPeriodBaremetalPeriodV1PeriodServer, Error<CreateServerError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_zone = zone;
    let p_create_server_request = create_server_request;

    let uri_str = format!(
        "{}/baremetal/v1/zones/{zone}/servers",
        configuration.base_path,
        zone = crate::apis::urlencode(p_zone)
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
    req_builder = req_builder.json(&p_create_server_request);

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodBaremetalPeriodV1PeriodServer`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodBaremetalPeriodV1PeriodServer`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateServerError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Delete an option from a specific server.
pub async fn delete_option_server(
    configuration: &configuration::Configuration,
    zone: &str,
    server_id: &str,
    option_id: &str,
) -> Result<models::ScalewayPeriodBaremetalPeriodV1PeriodServer, Error<DeleteOptionServerError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_zone = zone;
    let p_server_id = server_id;
    let p_option_id = option_id;

    let uri_str = format!(
        "{}/baremetal/v1/zones/{zone}/servers/{server_id}/options/{option_id}",
        configuration.base_path,
        zone = crate::apis::urlencode(p_zone),
        server_id = crate::apis::urlencode(p_server_id),
        option_id = crate::apis::urlencode(p_option_id)
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodBaremetalPeriodV1PeriodServer`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodBaremetalPeriodV1PeriodServer`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteOptionServerError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Delete the server associated with the ID.
pub async fn delete_server(
    configuration: &configuration::Configuration,
    zone: &str,
    server_id: &str,
) -> Result<models::ScalewayPeriodBaremetalPeriodV1PeriodServer, Error<DeleteServerError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_zone = zone;
    let p_server_id = server_id;

    let uri_str = format!(
        "{}/baremetal/v1/zones/{zone}/servers/{server_id}",
        configuration.base_path,
        zone = crate::apis::urlencode(p_zone),
        server_id = crate::apis::urlencode(p_server_id)
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodBaremetalPeriodV1PeriodServer`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodBaremetalPeriodV1PeriodServer`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteServerError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Get full details of an existing Elastic Metal server associated with the ID.
pub async fn get_server(
    configuration: &configuration::Configuration,
    zone: &str,
    server_id: &str,
) -> Result<models::ScalewayPeriodBaremetalPeriodV1PeriodServer, Error<GetServerError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_zone = zone;
    let p_server_id = server_id;

    let uri_str = format!(
        "{}/baremetal/v1/zones/{zone}/servers/{server_id}",
        configuration.base_path,
        zone = crate::apis::urlencode(p_zone),
        server_id = crate::apis::urlencode(p_server_id)
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodBaremetalPeriodV1PeriodServer`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodBaremetalPeriodV1PeriodServer`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetServerError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Get the ping status of the server associated with the ID.
pub async fn get_server_metrics(
    configuration: &configuration::Configuration,
    zone: &str,
    server_id: &str,
) -> Result<
    models::ScalewayPeriodBaremetalPeriodV1PeriodGetServerMetricsResponse,
    Error<GetServerMetricsError>,
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_zone = zone;
    let p_server_id = server_id;

    let uri_str = format!(
        "{}/baremetal/v1/zones/{zone}/servers/{server_id}/metrics",
        configuration.base_path,
        zone = crate::apis::urlencode(p_zone),
        server_id = crate::apis::urlencode(p_server_id)
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodBaremetalPeriodV1PeriodGetServerMetricsResponse`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodBaremetalPeriodV1PeriodGetServerMetricsResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetServerMetricsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Install an Operating System (OS) on the Elastic Metal server with a specific ID.
pub async fn install_server(
    configuration: &configuration::Configuration,
    zone: &str,
    server_id: &str,
    install_server_request: models::InstallServerRequest,
) -> Result<models::ScalewayPeriodBaremetalPeriodV1PeriodServer, Error<InstallServerError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_zone = zone;
    let p_server_id = server_id;
    let p_install_server_request = install_server_request;

    let uri_str = format!(
        "{}/baremetal/v1/zones/{zone}/servers/{server_id}/install",
        configuration.base_path,
        zone = crate::apis::urlencode(p_zone),
        server_id = crate::apis::urlencode(p_server_id)
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
    req_builder = req_builder.json(&p_install_server_request);

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodBaremetalPeriodV1PeriodServer`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodBaremetalPeriodV1PeriodServer`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<InstallServerError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// List event (i.e. start/stop/reboot) associated to the server ID.
pub async fn list_server_events(
    configuration: &configuration::Configuration,
    zone: &str,
    server_id: &str,
    page: Option<i32>,
    page_size: Option<i32>,
    order_by: Option<&str>,
) -> Result<
    models::ScalewayPeriodBaremetalPeriodV1PeriodListServerEventsResponse,
    Error<ListServerEventsError>,
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_zone = zone;
    let p_server_id = server_id;
    let p_page = page;
    let p_page_size = page_size;
    let p_order_by = order_by;

    let uri_str = format!(
        "{}/baremetal/v1/zones/{zone}/servers/{server_id}/events",
        configuration.base_path,
        zone = crate::apis::urlencode(p_zone),
        server_id = crate::apis::urlencode(p_server_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_page_size {
        req_builder = req_builder.query(&[("page_size", &param_value.to_string())]);
    }
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodBaremetalPeriodV1PeriodListServerEventsResponse`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodBaremetalPeriodV1PeriodListServerEventsResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListServerEventsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// List Elastic Metal servers for a specific Organization.
pub async fn list_servers(
    configuration: &configuration::Configuration,
    zone: &str,
    page: Option<i32>,
    page_size: Option<i32>,
    order_by: Option<&str>,
    tags: Option<Vec<String>>,
    status: Option<Vec<String>>,
    name: Option<&str>,
    organization_id: Option<&str>,
    project_id: Option<&str>,
    option_id: Option<&str>,
) -> Result<models::ScalewayPeriodBaremetalPeriodV1PeriodListServersResponse, Error<ListServersError>>
{
    // add a prefix to parameters to efficiently prevent name collisions
    let p_zone = zone;
    let p_page = page;
    let p_page_size = page_size;
    let p_order_by = order_by;
    let p_tags = tags;
    let p_status = status;
    let p_name = name;
    let p_organization_id = organization_id;
    let p_project_id = project_id;
    let p_option_id = option_id;

    let uri_str = format!(
        "{}/baremetal/v1/zones/{zone}/servers",
        configuration.base_path,
        zone = crate::apis::urlencode(p_zone)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_page_size {
        req_builder = req_builder.query(&[("page_size", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_order_by {
        req_builder = req_builder.query(&[("order_by", &param_value.to_string())]);
    }
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
    if let Some(ref param_value) = p_status {
        req_builder = match "multi" {
            "multi" => req_builder.query(
                &param_value
                    .iter()
                    .map(|p| ("status".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "status",
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
    if let Some(ref param_value) = p_organization_id {
        req_builder = req_builder.query(&[("organization_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_project_id {
        req_builder = req_builder.query(&[("project_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_option_id {
        req_builder = req_builder.query(&[("option_id", &param_value.to_string())]);
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodBaremetalPeriodV1PeriodListServersResponse`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodBaremetalPeriodV1PeriodListServersResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListServersError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Migrate server with hourly offer to monthly offer.
pub async fn migrate_server_to_monthly_offer(
    configuration: &configuration::Configuration,
    zone: &str,
    server_id: &str,
) -> Result<
    models::ScalewayPeriodBaremetalPeriodV1PeriodServer,
    Error<MigrateServerToMonthlyOfferError>,
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_zone = zone;
    let p_server_id = server_id;

    let uri_str = format!(
        "{}/baremetal/v1/zones/{zone}/servers/{server_id}/migrate-offer-monthly",
        configuration.base_path,
        zone = crate::apis::urlencode(p_zone),
        server_id = crate::apis::urlencode(p_server_id)
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodBaremetalPeriodV1PeriodServer`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodBaremetalPeriodV1PeriodServer`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MigrateServerToMonthlyOfferError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Configure the IP address associated with the server ID and IP ID. You can use this method to set a reverse DNS for an IP address.
pub async fn update_ip(
    configuration: &configuration::Configuration,
    zone: &str,
    server_id: &str,
    ip_id: &str,
    update_ip_request: models::UpdateIpRequest,
) -> Result<models::ScalewayPeriodBaremetalPeriodV1PeriodIp, Error<UpdateIpError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_zone = zone;
    let p_server_id = server_id;
    let p_ip_id = ip_id;
    let p_update_ip_request = update_ip_request;

    let uri_str = format!(
        "{}/baremetal/v1/zones/{zone}/servers/{server_id}/ips/{ip_id}",
        configuration.base_path,
        zone = crate::apis::urlencode(p_zone),
        server_id = crate::apis::urlencode(p_server_id),
        ip_id = crate::apis::urlencode(p_ip_id)
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
    req_builder = req_builder.json(&p_update_ip_request);

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodBaremetalPeriodV1PeriodIp`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodBaremetalPeriodV1PeriodIp`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateIpError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Update the server associated with the ID. You can update parameters such as the server's name, tags and description. Any parameters left null in the request body are not updated.
pub async fn update_server(
    configuration: &configuration::Configuration,
    zone: &str,
    server_id: &str,
    update_server_request: models::UpdateServerRequest,
) -> Result<models::ScalewayPeriodBaremetalPeriodV1PeriodServer, Error<UpdateServerError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_zone = zone;
    let p_server_id = server_id;
    let p_update_server_request = update_server_request;

    let uri_str = format!(
        "{}/baremetal/v1/zones/{zone}/servers/{server_id}",
        configuration.base_path,
        zone = crate::apis::urlencode(p_zone),
        server_id = crate::apis::urlencode(p_server_id)
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
    req_builder = req_builder.json(&p_update_server_request);

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodBaremetalPeriodV1PeriodServer`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodBaremetalPeriodV1PeriodServer`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateServerError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
