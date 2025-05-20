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

/// struct for typed errors of method [`clone_dns_zone`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CloneDnsZoneError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_dns_zone`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateDnsZoneError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_ssl_certificate`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateSslCertificateError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_dns_zone`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteDnsZoneError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_dns_zone_tsig_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteDnsZoneTsigKeyError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_ssl_certificate`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteSslCertificateError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_dns_zone_tsig_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDnsZoneTsigKeyError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_ssl_certificate`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSslCertificateError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_dns_zones`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListDnsZonesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_ssl_certificates`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListSslCertificatesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`refresh_dns_zone`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RefreshDnsZoneError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_dns_zone`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateDnsZoneError {
    UnknownValue(serde_json::Value),
}

/// Clone an existing DNS zone with all its records into a new DNS zone.
pub async fn clone_dns_zone(
    configuration: &configuration::Configuration,
    dns_zone: &str,
    clone_dns_zone_request: models::CloneDnsZoneRequest,
) -> Result<models::ScalewayPeriodDomainPeriodV2beta1PeriodDnsZone, Error<CloneDnsZoneError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_dns_zone = dns_zone;
    let p_clone_dns_zone_request = clone_dns_zone_request;

    let uri_str = format!(
        "{}/domain/v2beta1/dns-zones/{dns_zone}/clone",
        configuration.base_path,
        dns_zone = crate::apis::urlencode(p_dns_zone)
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
    req_builder = req_builder.json(&p_clone_dns_zone_request);

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodDomainPeriodV2beta1PeriodDnsZone`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodDomainPeriodV2beta1PeriodDnsZone`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CloneDnsZoneError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Create a new DNS zone specified by the domain name, the subdomain and the Project ID.
pub async fn create_dns_zone(
    configuration: &configuration::Configuration,
    create_dns_zone_request: models::CreateDnsZoneRequest,
) -> Result<models::ScalewayPeriodDomainPeriodV2beta1PeriodDnsZone, Error<CreateDnsZoneError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_create_dns_zone_request = create_dns_zone_request;

    let uri_str = format!("{}/domain/v2beta1/dns-zones", configuration.base_path);
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
    req_builder = req_builder.json(&p_create_dns_zone_request);

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodDomainPeriodV2beta1PeriodDnsZone`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodDomainPeriodV2beta1PeriodDnsZone`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateDnsZoneError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Create a new TLS certificate or retrieve information about an existing TLS certificate.
pub async fn create_ssl_certificate(
    configuration: &configuration::Configuration,
    create_ssl_certificate_request: models::CreateSslCertificateRequest,
) -> Result<
    models::ScalewayPeriodDomainPeriodV2beta1PeriodSslCertificate,
    Error<CreateSslCertificateError>,
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_create_ssl_certificate_request = create_ssl_certificate_request;

    let uri_str = format!(
        "{}/domain/v2beta1/ssl-certificates",
        configuration.base_path
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
    req_builder = req_builder.json(&p_create_ssl_certificate_request);

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodDomainPeriodV2beta1PeriodSslCertificate`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodDomainPeriodV2beta1PeriodSslCertificate`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateSslCertificateError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Delete a DNS zone and all its records.
pub async fn delete_dns_zone(
    configuration: &configuration::Configuration,
    dns_zone: &str,
    project_id: &str,
) -> Result<serde_json::Value, Error<DeleteDnsZoneError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_dns_zone = dns_zone;
    let p_project_id = project_id;

    let uri_str = format!(
        "{}/domain/v2beta1/dns-zones/{dns_zone}",
        configuration.base_path,
        dns_zone = crate::apis::urlencode(p_dns_zone)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::DELETE, &uri_str);

    req_builder = req_builder.query(&[("project_id", &p_project_id.to_string())]);
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `serde_json::Value`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `serde_json::Value`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteDnsZoneError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Delete an existing TSIG key specified by its DNS zone. Deleting a TSIG key is permanent and cannot be undone.
pub async fn delete_dns_zone_tsig_key(
    configuration: &configuration::Configuration,
    dns_zone: &str,
) -> Result<(), Error<DeleteDnsZoneTsigKeyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_dns_zone = dns_zone;

    let uri_str = format!(
        "{}/domain/v2beta1/dns-zones/{dns_zone}/tsig-key",
        configuration.base_path,
        dns_zone = crate::apis::urlencode(p_dns_zone)
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
        let entity: Option<DeleteDnsZoneTsigKeyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Delete an existing TLS certificate specified by its DNS zone. Deleting a TLS certificate is permanent and cannot be undone.
pub async fn delete_ssl_certificate(
    configuration: &configuration::Configuration,
    dns_zone: &str,
) -> Result<serde_json::Value, Error<DeleteSslCertificateError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_dns_zone = dns_zone;

    let uri_str = format!(
        "{}/domain/v2beta1/ssl-certificates/{dns_zone}",
        configuration.base_path,
        dns_zone = crate::apis::urlencode(p_dns_zone)
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `serde_json::Value`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `serde_json::Value`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteSslCertificateError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Retrieve information about the TSIG key of a given DNS zone to allow AXFR requests.
pub async fn get_dns_zone_tsig_key(
    configuration: &configuration::Configuration,
    dns_zone: &str,
) -> Result<
    models::ScalewayPeriodDomainPeriodV2beta1PeriodGetDnsZoneTsigKeyResponse,
    Error<GetDnsZoneTsigKeyError>,
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_dns_zone = dns_zone;

    let uri_str = format!(
        "{}/domain/v2beta1/dns-zones/{dns_zone}/tsig-key",
        configuration.base_path,
        dns_zone = crate::apis::urlencode(p_dns_zone)
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodDomainPeriodV2beta1PeriodGetDnsZoneTsigKeyResponse`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodDomainPeriodV2beta1PeriodGetDnsZoneTsigKeyResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetDnsZoneTsigKeyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Get the DNS zone's TLS certificate. If you do not have a certificate, the ouptut returns `no certificate found`.
pub async fn get_ssl_certificate(
    configuration: &configuration::Configuration,
    dns_zone: &str,
) -> Result<
    models::ScalewayPeriodDomainPeriodV2beta1PeriodSslCertificate,
    Error<GetSslCertificateError>,
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_dns_zone = dns_zone;

    let uri_str = format!(
        "{}/domain/v2beta1/ssl-certificates/{dns_zone}",
        configuration.base_path,
        dns_zone = crate::apis::urlencode(p_dns_zone)
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodDomainPeriodV2beta1PeriodSslCertificate`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodDomainPeriodV2beta1PeriodSslCertificate`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetSslCertificateError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Retrieve the list of DNS zones you can manage and filter DNS zones associated with specific domain names.
pub async fn list_dns_zones(
    configuration: &configuration::Configuration,
    domain: &str,
    organization_id: Option<&str>,
    project_id: Option<&str>,
    order_by: Option<&str>,
    page: Option<i32>,
    page_size: Option<i32>,
    dns_zone: Option<&str>,
    dns_zones: Option<Vec<String>>,
    created_after: Option<String>,
    created_before: Option<String>,
    updated_after: Option<String>,
    updated_before: Option<String>,
) -> Result<
    models::ScalewayPeriodDomainPeriodV2beta1PeriodListDnsZonesResponse,
    Error<ListDnsZonesError>,
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_domain = domain;
    let p_organization_id = organization_id;
    let p_project_id = project_id;
    let p_order_by = order_by;
    let p_page = page;
    let p_page_size = page_size;
    let p_dns_zone = dns_zone;
    let p_dns_zones = dns_zones;
    let p_created_after = created_after;
    let p_created_before = created_before;
    let p_updated_after = updated_after;
    let p_updated_before = updated_before;

    let uri_str = format!("{}/domain/v2beta1/dns-zones", configuration.base_path);
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
    req_builder = req_builder.query(&[("domain", &p_domain.to_string())]);
    if let Some(ref param_value) = p_dns_zone {
        req_builder = req_builder.query(&[("dns_zone", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_dns_zones {
        req_builder = match "multi" {
            "multi" => req_builder.query(
                &param_value
                    .iter()
                    .map(|p| ("dns_zones".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "dns_zones",
                &param_value
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref param_value) = p_created_after {
        req_builder = req_builder.query(&[("created_after", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_created_before {
        req_builder = req_builder.query(&[("created_before", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_updated_after {
        req_builder = req_builder.query(&[("updated_after", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_updated_before {
        req_builder = req_builder.query(&[("updated_before", &param_value.to_string())]);
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodDomainPeriodV2beta1PeriodListDnsZonesResponse`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodDomainPeriodV2beta1PeriodListDnsZonesResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListDnsZonesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// List all the TLS certificates a user has created, specified by the user's Project ID and the DNS zone.
pub async fn list_ssl_certificates(
    configuration: &configuration::Configuration,
    dns_zone: &str,
    page: Option<i32>,
    page_size: Option<i32>,
    project_id: Option<&str>,
) -> Result<
    models::ScalewayPeriodDomainPeriodV2beta1PeriodListSslCertificatesResponse,
    Error<ListSslCertificatesError>,
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_dns_zone = dns_zone;
    let p_page = page;
    let p_page_size = page_size;
    let p_project_id = project_id;

    let uri_str = format!(
        "{}/domain/v2beta1/ssl-certificates",
        configuration.base_path
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("dns_zone", &p_dns_zone.to_string())]);
    if let Some(ref param_value) = p_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_page_size {
        req_builder = req_builder.query(&[("page_size", &param_value.to_string())]);
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodDomainPeriodV2beta1PeriodListSslCertificatesResponse`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodDomainPeriodV2beta1PeriodListSslCertificatesResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListSslCertificatesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Refresh an SOA DNS zone to reload the records in the DNS zone and update the SOA serial. You can recreate the given DNS zone and its sub DNS zone if needed.
pub async fn refresh_dns_zone(
    configuration: &configuration::Configuration,
    dns_zone: &str,
    refresh_dns_zone_request: models::RefreshDnsZoneRequest,
) -> Result<
    models::ScalewayPeriodDomainPeriodV2beta1PeriodRefreshDnsZoneResponse,
    Error<RefreshDnsZoneError>,
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_dns_zone = dns_zone;
    let p_refresh_dns_zone_request = refresh_dns_zone_request;

    let uri_str = format!(
        "{}/domain/v2beta1/dns-zones/{dns_zone}/refresh",
        configuration.base_path,
        dns_zone = crate::apis::urlencode(p_dns_zone)
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
    req_builder = req_builder.json(&p_refresh_dns_zone_request);

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodDomainPeriodV2beta1PeriodRefreshDnsZoneResponse`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodDomainPeriodV2beta1PeriodRefreshDnsZoneResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<RefreshDnsZoneError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Update the name and/or the Organizations for a DNS zone.
pub async fn update_dns_zone(
    configuration: &configuration::Configuration,
    dns_zone: &str,
    update_dns_zone_request: models::UpdateDnsZoneRequest,
) -> Result<models::ScalewayPeriodDomainPeriodV2beta1PeriodDnsZone, Error<UpdateDnsZoneError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_dns_zone = dns_zone;
    let p_update_dns_zone_request = update_dns_zone_request;

    let uri_str = format!(
        "{}/domain/v2beta1/dns-zones/{dns_zone}",
        configuration.base_path,
        dns_zone = crate::apis::urlencode(p_dns_zone)
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
    req_builder = req_builder.json(&p_update_dns_zone_request);

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodDomainPeriodV2beta1PeriodDnsZone`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodDomainPeriodV2beta1PeriodDnsZone`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateDnsZoneError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
