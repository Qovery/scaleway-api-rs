/*
 * Elastic Metal API
 *
 * Scaleway Elastic Metal servers are dedicated physical servers that you can order on-demand, like Instances. You can install an OS or other images on your Elastic Metal server and connect to it via SSH to use it as you require. You can power off the server when you are not using or delete it from your account once you have finished using it. Elastic Metal servers are ideal for large workloads, big data, and applications that require increased security and dedicated resources.  (switchcolumn) <Message type=\"tip\">   Check out our dedicated APIs to manage [Private Networks](https://www.scaleway.com/en/developers/api/elastic-metal/private-network-api/) and [Flexible IPs](https://www.scaleway.com/en/developers/api/elastic-metal-flexible-ip) for Elastic Metal servers. </Message> (switchcolumn)  ## Concepts  Refer to our [dedicated concepts](https://www.scaleway.com/en/docs/compute/elastic-metal/concepts/) page to find definitions of the different terms referring to Elastic Metal servers.  ## Quickstart  (switchcolumn) (switchcolumn)  1. **Configure your environment variables.**     ```bash     export PROJECT_ID=\"<project-id>\"     export ACCESS_KEY=\"<access-key>\"     export SECRET_KEY=\"<secret-key>\"     export ZONE=\"<availability-zone>\"     ```     <Message type=\"note\">       This is an optional step that seeks to simplify your usage of the Bare Metal API.     </Message>  2. **Edit the POST request payload** that we will use in the next step to create an Elastic Metal server. Modify the values in the example according to your needs, using the information in the table to help.     ```json     {     \"offer_id\": \"string\",     \"project_id\": \"string\",     \"name\": \"string\",     \"description\": \"string\",     \"tags\": [         \"tag1\", \"tag2\"     ],     \"install\": {         \"os_id\": \"string\",         \"hostname\": \"string\",         \"ssh_key_ids\": [         \"string\"         ],         \"user\": \"string\",         \"password\": \"string\",         \"service_user\": \"string\",         \"service_password\": \"string\"     },     \"option_ids\": [         \"string\"     ]     }     ```      | Parameter        | Description                                                        |     | :--------------- | :----------------------------------------------------------------- |     | `offer_id`           | **REQUIRED** UUID of the Elastic Metal offer                                         |     | `project_id`     | **REQUIRED** UUID of the project you want to create your Elastic Metal in.  |     | `name`           | **REQUIRED** Name of the Elastic Metal server (≠hostname)                                          |     | `description`     | **REQUIRED** A description of your server (max 255 characters)                             |     | `tags`  | **OPTIONAL** An array of tags associated with your server   |     | `os_id`  | The ID of the operating system image to install on the server   |     | `hostname`  | Hostname of the server   |     | `ssh_key_ids`  | SSH key IDs authorized on the server   |     | `user`  | **NULLABLE** A regular user to be configured on the server   |     | `password`  | **NULLABLE** The password for the user account   |     | `service_user`  | **NULLABLE** A service user for third party services (user to login in services such as BigBlueButton)  |     | `service password`  | **NULLABLE** Password for the service user   |     | `option_ids`  | IDs of options to enable on server  |      <Message type=\"tip\">       To find your Project ID you can either use the [IAM API](https://www.scaleway.com/en/developers/api/account#path-projects-list-all-projects-of-an-organization) or the [Scaleway console](https://console.scaleway.com/project/settings):     </Message>  3. **Run the following command** to create an Elastic Metal server. Make sure you include the payload you edited in the previous step.     ```bash     curl -X POST \\       -H \"Content-Type: application/json\" \\       -H \"X-Auth-Token: $SECRET_KEY\" https://api.scaleway.com//baremetal/v1/zones/$ZONE/servers \\       -d '{         \"offer_id\": \"bd757ca3-a71b-4158-9ef5-39436b6db2a4\",         \"project_id\": \"cc6d123a-bc09-4e24-b5d9-3310f2104e13\",         \"name\": \"MyElasticMetal\",         \"description\": \"My_Elastic_Metal_Server\",         \"tags\": [             \"Ubuntu22\", \"Dedicated\"         ],         \"install\": {             \"os_id\": \"96e5f0f2-d216-4de2-8a15-68730d877885\",             \"hostname\": \"elasticmetal.example.com\",             \"ssh_key_ids\": [             \"fa05e77f-66b7-43b9-bc21-4dfe3c5bb624\"             ],             \"user\": \"ubuntu\",             \"password\": \"mySecretPa$$word\"         \"option_ids\": [             \"string\"         ]       }\"     ``` 4. **List your Elastic Metal servers.**     ```bash     curl -X GET \\       -H \"Content-Type: application/json\" \\       -H \"X-Auth-Token: $SECRET_KEY\" https://api.scaleway.com/baremetal/v1/zones/$ZONE/servers     ```  5. **Retrieve your Elastic Metal server IP** from the response.  6. **Connect to your Elastic Metal server** using SSH     ```bash     ssh root@my-elastic-metal-server-ip     ```  (switchcolumn) <Message type=\"requirement\"> To perform the following steps, you must first ensure that:   - you have an account and are logged into the [Scaleway console](https://console.scaleway.com/organization)   - you have created an [API key](https://www.scaleway.com/en/docs/identity-and-access-management/iam/how-to/create-api-keys/) and that the API key has sufficient [IAM permissions](https://www.scaleway.com/en/docs/identity-and-access-management/iam/reference-content/permission-sets/) to perform the actions described on this page.   - you have [installed `curl`](https://curl.se/download.html) </Message> (switchcolumn)  ## Technical information  ### Features  - Installation (Server is installed by preseed [preseed: complete install from a virtual media], you must define at least one ssh key to install your server) - Start/Stop/Reboot - Rescue Reboot, a rescue image is an operating system image designed to help you diagnose and fix OS experiencing failures. When your server boot on rescue, you can mount your disks and start diagnosing/fixing your image. - Billed by the minute (billing starts when the server is delivered and stops when the server is deleted) - IPv6, all servers are available with a /128 IPv6 subnet - ReverseIP, You can configure your reverse IP (IPv4 and IPv6), you must register the server IP in your DNS records before calling the endpoint - Basic monitoring with ping status - Flexible IP is available ([documentation](https://www.scaleway.com/en/developers/api/elastic-metal-flexible-ip))  ### Availability Zones  Scaleway's infrastructure is spread across different [regions and Availability Zones](https://www.scaleway.com/en/docs/console/my-account/reference-content/products-availability/).  Elastic Metal servers are available in Paris, Amsterdam, and Warsaw regions, with product availability in the following AZs:  | Name      | API ID                           | |-----------|----------------------------------| | Paris     | `fr-par-1` `fr-par-2`            | | Amsterdam | `nl-ams-1` `nl-ams-2`            | | Warsaw    | `pl-waw-2` `pl-waw-3`            |  ## Technical limitations  - Failover IPs are not available in API `v1`, use the API `v1alpha1` ## Going further  For more help using Scaleway Elastic Metal servers, check out the following resources: - Our [main documentation](https://www.scaleway.com/en/docs/compute/elastic-metal/) - The #elastic-metal channel on our [Slack Community](https://www.scaleway.com/en/docs/tutorials/scaleway-slack-community/) - Our [support ticketing system](https://www.scaleway.com/en/docs/console/my-account/how-to/open-a-support-ticket) ### Troubleshooting  #### How is the installation of Elastic Metal servers done?  - The installation of Elastic Metal servers is done by preseed (± 10min) (preseed: complete install from a virtual media) #### How can I retrieve my secret key and Project ID?  You can find your [secret key](https://console.scaleway.com/iam/api-keys) and your [Project ID](https://console.scaleway.com/project/credentials) in the Scaleway console.  #### How can I add my server to a Private Network?  See [our dedicated documentation](/en/developers/api/elastic-metal-flexible-ip).
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScalewayPeriodBaremetalPeriodV1PeriodOfferPeriodOptionOffer {
    /// ID of the option.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name of the option.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// If true the option is enabled and included by default in the offer. If true the option is enabled and included by default in the offer If false the option is available for the offer but not included by default.
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Period of subscription for the offer.
    #[serde(
        rename = "subscription_period",
        skip_serializing_if = "Option::is_none"
    )]
    pub subscription_period: Option<SubscriptionPeriod>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<Box<models::ScalewayBaremetalV1OfferOptionOfferPrice>>,
    /// Boolean to know if option could be managed.
    #[serde(rename = "manageable", skip_serializing_if = "Option::is_none")]
    pub manageable: Option<bool>,
    /// Deprecated, use LicenseOptionVars.os_id instead.
    #[serde(
        rename = "os_id",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub os_id: Option<Option<String>>,
    #[serde(
        rename = "license",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub license: Option<Option<Box<models::ScalewayBaremetalV1OfferOptionOfferLicense>>>,
    #[serde(
        rename = "public_bandwidth",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub public_bandwidth:
        Option<Option<Box<models::ScalewayBaremetalV1OfferOptionOfferPublicBandwidth>>>,
    #[serde(
        rename = "private_network",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub private_network:
        Option<Option<Box<models::ScalewayBaremetalV1OfferOptionOfferPrivateNetwork>>>,
    /// Remote_access option.
    #[serde(
        rename = "remote_access",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub remote_access: Option<Option<serde_json::Value>>,
    /// Certification option.
    #[serde(
        rename = "certification",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub certification: Option<Option<serde_json::Value>>,
}

impl ScalewayPeriodBaremetalPeriodV1PeriodOfferPeriodOptionOffer {
    pub fn new() -> ScalewayPeriodBaremetalPeriodV1PeriodOfferPeriodOptionOffer {
        ScalewayPeriodBaremetalPeriodV1PeriodOfferPeriodOptionOffer {
            id: None,
            name: None,
            enabled: None,
            subscription_period: None,
            price: None,
            manageable: None,
            os_id: None,
            license: None,
            public_bandwidth: None,
            private_network: None,
            remote_access: None,
            certification: None,
        }
    }
}
/// Period of subscription for the offer.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SubscriptionPeriod {
    #[serde(rename = "unknown_subscription_period")]
    UnknownSubscriptionPeriod,
    #[serde(rename = "hourly")]
    Hourly,
    #[serde(rename = "monthly")]
    Monthly,
}

impl Default for SubscriptionPeriod {
    fn default() -> SubscriptionPeriod {
        Self::UnknownSubscriptionPeriod
    }
}
