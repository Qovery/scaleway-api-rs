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

/// struct for typed errors of method [`attach_flexible_ip`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AttachFlexibleIpError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_flexible_ip`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateFlexibleIpError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_flexible_ip`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteFlexibleIpError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_mac_addr`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteMacAddrError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`detach_flexible_ip`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DetachFlexibleIpError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`duplicate_mac_addr`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DuplicateMacAddrError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`generate_mac_addr`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenerateMacAddrError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_flexible_ip`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFlexibleIpError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_flexible_ips`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListFlexibleIpsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`move_mac_addr`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MoveMacAddrError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_flexible_ip`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateFlexibleIpError {
    UnknownValue(serde_json::Value),
}

/// Attach an existing flexible IP to a specified Elastic Metal server.
pub async fn attach_flexible_ip(
    configuration: &configuration::Configuration,
    zone: &str,
    attach_flexible_ip_request: models::AttachFlexibleIpRequest,
) -> Result<
    models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodAttachFlexibleIpsResponse,
    Error<AttachFlexibleIpError>,
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_zone = zone;
    let p_attach_flexible_ip_request = attach_flexible_ip_request;

    let uri_str = format!(
        "{}/flexible-ip/v1alpha1/zones/{zone}/fips/attach",
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
    req_builder = req_builder.json(&p_attach_flexible_ip_request);

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodAttachFlexibleIpsResponse`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodAttachFlexibleIpsResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AttachFlexibleIpError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Generate a new flexible IP within a given zone, specifying its configuration including Project ID and description.
pub async fn create_flexible_ip(
    configuration: &configuration::Configuration,
    zone: &str,
    create_flexible_ip_request: models::CreateFlexibleIpRequest,
) -> Result<
    models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodFlexibleIp,
    Error<CreateFlexibleIpError>,
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_zone = zone;
    let p_create_flexible_ip_request = create_flexible_ip_request;

    let uri_str = format!(
        "{}/flexible-ip/v1alpha1/zones/{zone}/fips",
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
    req_builder = req_builder.json(&p_create_flexible_ip_request);

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodFlexibleIp`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodFlexibleIp`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateFlexibleIpError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Delete an existing flexible IP, specified by its ID and zone. Note that deleting a flexible IP is permanent and cannot be undone.
pub async fn delete_flexible_ip(
    configuration: &configuration::Configuration,
    zone: &str,
    fip_id: &str,
) -> Result<(), Error<DeleteFlexibleIpError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_zone = zone;
    let p_fip_id = fip_id;

    let uri_str = format!(
        "{}/flexible-ip/v1alpha1/zones/{zone}/fips/{fip_id}",
        configuration.base_path,
        zone = crate::apis::urlencode(p_zone),
        fip_id = crate::apis::urlencode(p_fip_id)
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

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteFlexibleIpError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Detach a given MAC (Media Access Control) address from an existing flexible IP.
pub async fn delete_mac_addr(
    configuration: &configuration::Configuration,
    zone: &str,
    fip_id: &str,
) -> Result<(), Error<DeleteMacAddrError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_zone = zone;
    let p_fip_id = fip_id;

    let uri_str = format!(
        "{}/flexible-ip/v1alpha1/zones/{zone}/fips/{fip_id}/mac",
        configuration.base_path,
        zone = crate::apis::urlencode(p_zone),
        fip_id = crate::apis::urlencode(p_fip_id)
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

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteMacAddrError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Detach an existing flexible IP from a specified Elastic Metal server.
pub async fn detach_flexible_ip(
    configuration: &configuration::Configuration,
    zone: &str,
    detach_flexible_ip_request: models::DetachFlexibleIpRequest,
) -> Result<
    models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodDetachFlexibleIpsResponse,
    Error<DetachFlexibleIpError>,
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_zone = zone;
    let p_detach_flexible_ip_request = detach_flexible_ip_request;

    let uri_str = format!(
        "{}/flexible-ip/v1alpha1/zones/{zone}/fips/detach",
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
    req_builder = req_builder.json(&p_detach_flexible_ip_request);

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodDetachFlexibleIpsResponse`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodDetachFlexibleIpsResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DetachFlexibleIpError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Duplicate a virtual MAC address from a given flexible IP to another flexible IP attached to the same server.
pub async fn duplicate_mac_addr(
    configuration: &configuration::Configuration,
    zone: &str,
    fip_id: &str,
    duplicate_mac_addr_request: models::DuplicateMacAddrRequest,
) -> Result<
    models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodFlexibleIp,
    Error<DuplicateMacAddrError>,
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_zone = zone;
    let p_fip_id = fip_id;
    let p_duplicate_mac_addr_request = duplicate_mac_addr_request;

    let uri_str = format!(
        "{}/flexible-ip/v1alpha1/zones/{zone}/fips/{fip_id}/mac/duplicate",
        configuration.base_path,
        zone = crate::apis::urlencode(p_zone),
        fip_id = crate::apis::urlencode(p_fip_id)
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
    req_builder = req_builder.json(&p_duplicate_mac_addr_request);

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodFlexibleIp`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodFlexibleIp`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DuplicateMacAddrError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Generate a virtual MAC (Media Access Control) address on an existing flexible IP.
pub async fn generate_mac_addr(
    configuration: &configuration::Configuration,
    zone: &str,
    fip_id: &str,
    generate_mac_addr_request: models::GenerateMacAddrRequest,
) -> Result<
    models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodFlexibleIp,
    Error<GenerateMacAddrError>,
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_zone = zone;
    let p_fip_id = fip_id;
    let p_generate_mac_addr_request = generate_mac_addr_request;

    let uri_str = format!(
        "{}/flexible-ip/v1alpha1/zones/{zone}/fips/{fip_id}/mac",
        configuration.base_path,
        zone = crate::apis::urlencode(p_zone),
        fip_id = crate::apis::urlencode(p_fip_id)
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
    req_builder = req_builder.json(&p_generate_mac_addr_request);

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodFlexibleIp`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodFlexibleIp`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenerateMacAddrError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Retrieve information about an existing flexible IP, specified by its ID and zone. Its full details, including Project ID, description and status, are returned in the response object.
pub async fn get_flexible_ip(
    configuration: &configuration::Configuration,
    zone: &str,
    fip_id: &str,
) -> Result<models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodFlexibleIp, Error<GetFlexibleIpError>>
{
    // add a prefix to parameters to efficiently prevent name collisions
    let p_zone = zone;
    let p_fip_id = fip_id;

    let uri_str = format!(
        "{}/flexible-ip/v1alpha1/zones/{zone}/fips/{fip_id}",
        configuration.base_path,
        zone = crate::apis::urlencode(p_zone),
        fip_id = crate::apis::urlencode(p_fip_id)
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodFlexibleIp`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodFlexibleIp`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetFlexibleIpError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// List all flexible IPs within a given zone.
pub async fn list_flexible_ips(
    configuration: &configuration::Configuration,
    zone: &str,
    order_by: Option<&str>,
    page: Option<i32>,
    page_size: Option<i32>,
    tags: Option<Vec<String>>,
    status: Option<Vec<models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodFlexibleIpPeriodStatus>>,
    server_ids: Option<Vec<String>>,
    organization_id: Option<&str>,
    project_id: Option<&str>,
) -> Result<
    models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodListFlexibleIpsResponse,
    Error<ListFlexibleIpsError>,
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_zone = zone;
    let p_order_by = order_by;
    let p_page = page;
    let p_page_size = page_size;
    let p_tags = tags;
    let p_status = status;
    let p_server_ids = server_ids;
    let p_organization_id = organization_id;
    let p_project_id = project_id;

    let uri_str = format!(
        "{}/flexible-ip/v1alpha1/zones/{zone}/fips",
        configuration.base_path,
        zone = crate::apis::urlencode(p_zone)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_order_by {
        req_builder = req_builder.query(&[("order_by", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_page_size {
        req_builder = req_builder.query(&[("page_size", &param_value.to_string())]);
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
    if let Some(ref param_value) = p_server_ids {
        req_builder = match "multi" {
            "multi" => req_builder.query(
                &param_value
                    .iter()
                    .map(|p| ("server_ids".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "server_ids",
                &param_value
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref param_value) = p_organization_id {
        req_builder = req_builder.query(&[("organization_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_project_id {
        req_builder = req_builder.query(&[("project_id", &param_value.to_string())]);
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodListFlexibleIpsResponse`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodListFlexibleIpsResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListFlexibleIpsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Relocate a virtual MAC (Media Access Control) address from an existing flexible IP to a different flexible IP.
pub async fn move_mac_addr(
    configuration: &configuration::Configuration,
    zone: &str,
    fip_id: &str,
    move_mac_addr_request: models::MoveMacAddrRequest,
) -> Result<models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodFlexibleIp, Error<MoveMacAddrError>>
{
    // add a prefix to parameters to efficiently prevent name collisions
    let p_zone = zone;
    let p_fip_id = fip_id;
    let p_move_mac_addr_request = move_mac_addr_request;

    let uri_str = format!(
        "{}/flexible-ip/v1alpha1/zones/{zone}/fips/{fip_id}/mac/move",
        configuration.base_path,
        zone = crate::apis::urlencode(p_zone),
        fip_id = crate::apis::urlencode(p_fip_id)
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
    req_builder = req_builder.json(&p_move_mac_addr_request);

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodFlexibleIp`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodFlexibleIp`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MoveMacAddrError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Update the parameters of an existing flexible IP, specified by its ID and zone. These parameters include tags and description.
pub async fn update_flexible_ip(
    configuration: &configuration::Configuration,
    zone: &str,
    fip_id: &str,
    update_flexible_ip_request: models::UpdateFlexibleIpRequest,
) -> Result<
    models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodFlexibleIp,
    Error<UpdateFlexibleIpError>,
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_zone = zone;
    let p_fip_id = fip_id;
    let p_update_flexible_ip_request = update_flexible_ip_request;

    let uri_str = format!(
        "{}/flexible-ip/v1alpha1/zones/{zone}/fips/{fip_id}",
        configuration.base_path,
        zone = crate::apis::urlencode(p_zone),
        fip_id = crate::apis::urlencode(p_fip_id)
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
    req_builder = req_builder.json(&p_update_flexible_ip_request);

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodFlexibleIp`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodFlexibleIp`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateFlexibleIpError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
