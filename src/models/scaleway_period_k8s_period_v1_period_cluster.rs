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
pub struct ScalewayPeriodK8sPeriodV1PeriodCluster {
    /// Cluster ID.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Cluster type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Cluster name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Status of the cluster.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// Kubernetes version of the cluster.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Region in which the cluster is deployed.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// ID of the Organization owning the cluster.
    #[serde(rename = "organization_id", skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    /// ID of the Project owning the cluster.
    #[serde(rename = "project_id", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    /// Tags associated with the cluster.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Container Network Interface (CNI) plugin running in the cluster.
    #[serde(rename = "cni", skip_serializing_if = "Option::is_none")]
    pub cni: Option<Cni>,
    /// Cluster description.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Kubernetes API server URL of the cluster.
    #[serde(rename = "cluster_url", skip_serializing_if = "Option::is_none")]
    pub cluster_url: Option<String>,
    /// Wildcard DNS resolving all the ready cluster nodes.
    #[serde(rename = "dns_wildcard", skip_serializing_if = "Option::is_none")]
    pub dns_wildcard: Option<String>,
    /// Date on which the cluster was created. (RFC 3339 format)
    #[serde(
        rename = "created_at",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub created_at: Option<Option<String>>,
    /// Date on which the cluster was last updated. (RFC 3339 format)
    #[serde(
        rename = "updated_at",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub updated_at: Option<Option<String>>,
    #[serde(rename = "autoscaler_config", skip_serializing_if = "Option::is_none")]
    pub autoscaler_config: Option<Box<models::ScalewayK8sV1ClusterAutoscalerConfig>>,
    #[serde(rename = "auto_upgrade", skip_serializing_if = "Option::is_none")]
    pub auto_upgrade: Option<Box<models::ScalewayK8sV1ClusterAutoUpgrade>>,
    /// Defines whether a new Kubernetes version is available.
    #[serde(rename = "upgrade_available", skip_serializing_if = "Option::is_none")]
    pub upgrade_available: Option<bool>,
    /// List of enabled feature gates.
    #[serde(rename = "feature_gates", skip_serializing_if = "Option::is_none")]
    pub feature_gates: Option<Vec<String>>,
    /// List of enabled admission plugins.
    #[serde(rename = "admission_plugins", skip_serializing_if = "Option::is_none")]
    pub admission_plugins: Option<Vec<String>>,
    #[serde(
        rename = "open_id_connect_config",
        skip_serializing_if = "Option::is_none"
    )]
    pub open_id_connect_config: Option<Box<models::ScalewayK8sV1ClusterOpenIdConnectConfig>>,
    /// Additional Subject Alternative Names for the Kubernetes API server certificate.
    #[serde(
        rename = "apiserver_cert_sans",
        skip_serializing_if = "Option::is_none"
    )]
    pub apiserver_cert_sans: Option<Vec<String>>,
    /// Private network ID for internal cluster communication.
    #[serde(
        rename = "private_network_id",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub private_network_id: Option<Option<String>>,
    /// Date on which it will be possible to switch to a smaller offer. (RFC 3339 format)
    #[serde(
        rename = "commitment_ends_at",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub commitment_ends_at: Option<Option<String>>,
    /// Defines whether ACL is available on the cluster.
    #[serde(rename = "acl_available", skip_serializing_if = "Option::is_none")]
    pub acl_available: Option<bool>,
    /// IAM group that nodes are members of (this field might be empty during early stage of cluster creation).
    #[serde(rename = "iam_nodes_group_id", skip_serializing_if = "Option::is_none")]
    pub iam_nodes_group_id: Option<String>,
    /// Defines whether all pools are migrated to new images.
    #[serde(rename = "new_images_enabled", skip_serializing_if = "Option::is_none")]
    pub new_images_enabled: Option<bool>,
}

impl ScalewayPeriodK8sPeriodV1PeriodCluster {
    pub fn new() -> ScalewayPeriodK8sPeriodV1PeriodCluster {
        ScalewayPeriodK8sPeriodV1PeriodCluster {
            id: None,
            r#type: None,
            name: None,
            status: None,
            version: None,
            region: None,
            organization_id: None,
            project_id: None,
            tags: None,
            cni: None,
            description: None,
            cluster_url: None,
            dns_wildcard: None,
            created_at: None,
            updated_at: None,
            autoscaler_config: None,
            auto_upgrade: None,
            upgrade_available: None,
            feature_gates: None,
            admission_plugins: None,
            open_id_connect_config: None,
            apiserver_cert_sans: None,
            private_network_id: None,
            commitment_ends_at: None,
            acl_available: None,
            iam_nodes_group_id: None,
            new_images_enabled: None,
        }
    }
}
/// Status of the cluster.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "creating")]
    Creating,
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "deleting")]
    Deleting,
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "updating")]
    Updating,
    #[serde(rename = "locked")]
    Locked,
    #[serde(rename = "pool_required")]
    PoolRequired,
}

impl Default for Status {
    fn default() -> Status {
        Self::Unknown
    }
}
/// Container Network Interface (CNI) plugin running in the cluster.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Cni {
    #[serde(rename = "unknown_cni")]
    UnknownCni,
    #[serde(rename = "cilium")]
    Cilium,
    #[serde(rename = "calico")]
    Calico,
    #[serde(rename = "weave")]
    Weave,
    #[serde(rename = "flannel")]
    Flannel,
    #[serde(rename = "kilo")]
    Kilo,
    #[serde(rename = "none")]
    None,
}

impl Default for Cni {
    fn default() -> Cni {
        Self::UnknownCni
    }
}
