/*
 * Elastic metal API
 *
 * # Introduction  Elastic metal as a service allows ordering a dedicated server on-demand like a cloud instance. Dedicated servers could be used for large workloads, big data, those requiring more security, ….  ## Technical Limitations  - Elastic metal is available in `fr-par-1`,  `fr-par-2`, `nl-ams-1` zones  - Installation is done by preseed (± 10min) (preseed: complete install from a virtual media)  ## Features  - Install (Server is installed by preseed (preseed: complete install from a virtual media), you must define at least one ssh key to install your server)  - Start/Stop/Reboot  - Rescue Reboot, a rescue image is an operating system image designed to help you diagnose and fix an OS experiencing failures. When your server boot on rescue, you can mount your disks and start diagnosing/fixing your image.  - Billed by minute (The billing start when the server is delivered and stop when the server is deleted)  - IPv6, all servers are available with an IPv6 /128  - ReverseIP, You can configure your reverse IP (IPv4 and IPv6), you must register the server IP in your DNS records before calling the endpoint  - Basic monitoring with ping status  - Flexible IP is available ([documentation](https://developers.scaleway.com/en/products/flexible-ip/api/))  - IP failovers are not available in api v1, use the api v1alpha1  ## FAQ  ### How can I get my SSH key id?  You can find your `$SCW_SECRET_KEY` and your `SCW_DEFAULT_ORGANIZATION_ID` at the following page: https://console.scaleway.com/project/credentials  ### How can I add my server to a private network?  See [our online documentation](https://developers.scaleway.com/en/products/vpc-elasticmetal/api/).
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ScalewayFlexibleIpV1alpha1FlexibleIp {
    /// ID of the Flexible IP
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Organization ID the Flexible IP is attached to
    #[serde(rename = "organization_id", skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    /// Project ID the Flexible IP is attached to
    #[serde(rename = "project_id", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    /// Description of the Flexible IP
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Tags associated with the Flexible IP
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Date of last update of the Flexible IP
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Date of creation of the Flexible IP
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// - ready : Flexible IP is created and ready to be attached to a server or to have a virtual MAC generated. - updating: Flexible IP is being attached to a server or a virtual MAC operation is ongoing - attached: Flexible IP is attached to a server - error: a Flexible IP operation resulted in an error - detaching: Flexible IP is being detached from a server - locked: Flexible IP resource is locked
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// IP of the Flexible IP (IP address)
    #[serde(rename = "ip_address", skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "mac_address", skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<Box<crate::models::ScalewayFlexibleIpV1alpha1FlexibleIpMacAddress>>,
    /// ID of the server linked to the Flexible IP
    #[serde(rename = "server_id", skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    /// Reverse DNS value
    #[serde(rename = "reverse", skip_serializing_if = "Option::is_none")]
    pub reverse: Option<String>,
    /// Flexible IP Availability Zone
    #[serde(rename = "zone", skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
}

impl ScalewayFlexibleIpV1alpha1FlexibleIp {
    pub fn new() -> ScalewayFlexibleIpV1alpha1FlexibleIp {
        ScalewayFlexibleIpV1alpha1FlexibleIp {
            id: None,
            organization_id: None,
            project_id: None,
            description: None,
            tags: None,
            updated_at: None,
            created_at: None,
            status: None,
            ip_address: None,
            mac_address: None,
            server_id: None,
            reverse: None,
            zone: None,
        }
    }
}

/// - ready : Flexible IP is created and ready to be attached to a server or to have a virtual MAC generated. - updating: Flexible IP is being attached to a server or a virtual MAC operation is ongoing - attached: Flexible IP is attached to a server - error: a Flexible IP operation resulted in an error - detaching: Flexible IP is being detached from a server - locked: Flexible IP resource is locked
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "updating")]
    Updating,
    #[serde(rename = "attached")]
    Attached,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "detaching")]
    Detaching,
    #[serde(rename = "locked")]
    Locked,
}

impl Default for Status {
    fn default() -> Status {
        Self::Unknown
    }
}
