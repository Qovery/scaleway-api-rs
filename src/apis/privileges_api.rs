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

/// struct for typed errors of method [`list_privileges`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListPrivilegesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_privilege`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetPrivilegeError {
    UnknownValue(serde_json::Value),
}

/// List privileges of a user on a database. By default, the details returned in the list are ordered by creation date in ascending order, though this can be modified via the order_by field. You can define additional parameters for your query, such as `database_name` and `user_name`.
pub async fn list_privileges(
    configuration: &configuration::Configuration,
    region: &str,
    instance_id: &str,
    order_by: Option<&str>,
    page: Option<i32>,
    page_size: Option<i32>,
    database_name: Option<&str>,
    user_name: Option<&str>,
) -> Result<models::ScalewayPeriodRdbPeriodV1PeriodListPrivilegesResponse, Error<ListPrivilegesError>>
{
    // add a prefix to parameters to efficiently prevent name collisions
    let p_region = region;
    let p_instance_id = instance_id;
    let p_order_by = order_by;
    let p_page = page;
    let p_page_size = page_size;
    let p_database_name = database_name;
    let p_user_name = user_name;

    let uri_str = format!(
        "{}/rdb/v1/regions/{region}/instances/{instance_id}/privileges",
        configuration.base_path,
        region = crate::apis::urlencode(p_region),
        instance_id = crate::apis::urlencode(p_instance_id)
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
    if let Some(ref param_value) = p_database_name {
        req_builder = req_builder.query(&[("database_name", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_user_name {
        req_builder = req_builder.query(&[("user_name", &param_value.to_string())]);
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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodRdbPeriodV1PeriodListPrivilegesResponse`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodRdbPeriodV1PeriodListPrivilegesResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListPrivilegesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Set the privileges of a user on a database. You must define `database_name`, `user_name` and `permission` in the request body.
pub async fn set_privilege(
    configuration: &configuration::Configuration,
    region: &str,
    instance_id: &str,
    set_privilege_request: models::SetPrivilegeRequest,
) -> Result<models::ScalewayPeriodRdbPeriodV1PeriodPrivilege, Error<SetPrivilegeError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_region = region;
    let p_instance_id = instance_id;
    let p_set_privilege_request = set_privilege_request;

    let uri_str = format!(
        "{}/rdb/v1/regions/{region}/instances/{instance_id}/privileges",
        configuration.base_path,
        region = crate::apis::urlencode(p_region),
        instance_id = crate::apis::urlencode(p_instance_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

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
    req_builder = req_builder.json(&p_set_privilege_request);

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
            ContentType::Text => Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ScalewayPeriodRdbPeriodV1PeriodPrivilege`"))),
            ContentType::Unsupported(unknown_type) => Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ScalewayPeriodRdbPeriodV1PeriodPrivilege`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SetPrivilegeError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
