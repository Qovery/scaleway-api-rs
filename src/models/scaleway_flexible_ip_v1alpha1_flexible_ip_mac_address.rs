/*
 * Elastic metal API
 *
 * # Introduction  Elastic metal as a service allows ordering a dedicated server on-demand like a cloud instance. Dedicated servers could be used for large workloads, big data, those requiring more security, ….  ## Technical Limitations  - Elastic metal is available in `fr-par-1`,  `fr-par-2`, `nl-ams-1` zones  - Installation is done by preseed (± 10min) (preseed: complete install from a virtual media)  ## Features  - Install (Server is installed by preseed (preseed: complete install from a virtual media), you must define at least one ssh key to install your server)  - Start/Stop/Reboot  - Rescue Reboot, a rescue image is an operating system image designed to help you diagnose and fix an OS experiencing failures. When your server boot on rescue, you can mount your disks and start diagnosing/fixing your image.  - Billed by minute (The billing start when the server is delivered and stop when the server is deleted)  - IPv6, all servers are available with an IPv6 /128  - ReverseIP, You can configure your reverse IP (IPv4 and IPv6), you must register the server IP in your DNS records before calling the endpoint  - Basic monitoring with ping status  - Flexible IP is available ([documentation](https://developers.scaleway.com/en/products/flexible-ip/api/))  - IP failovers are not available in api v1, use the api v1alpha1  ## FAQ  ### How can I get my SSH key id?  You can find your `$SCW_SECRET_KEY` and your `SCW_DEFAULT_ORGANIZATION_ID` at the following page: https://console.scaleway.com/project/credentials  ### How can I add my server to a private network?  See [our online documentation](https://developers.scaleway.com/en/products/vpc-elasticmetal/api/).
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

/// ScalewayFlexibleIpV1alpha1FlexibleIpMacAddress : MAC address of the Flexible IP

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ScalewayFlexibleIpV1alpha1FlexibleIpMacAddress {
    /// ID of the Flexible IP
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// MAC address of the Virtual MAC
    #[serde(rename = "mac_address", skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    /// Virtual MAC type
    #[serde(rename = "mac_type", skip_serializing_if = "Option::is_none")]
    pub mac_type: Option<MacType>,
    /// Virtual MAC status
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// Date of last update of the Virtual MAC
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Date of creation of the Virtual MAC
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// MAC Addr IP Availability Zone
    #[serde(rename = "zone", skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
}

impl ScalewayFlexibleIpV1alpha1FlexibleIpMacAddress {
    /// MAC address of the Flexible IP
    pub fn new() -> ScalewayFlexibleIpV1alpha1FlexibleIpMacAddress {
        ScalewayFlexibleIpV1alpha1FlexibleIpMacAddress {
            id: None,
            mac_address: None,
            mac_type: None,
            status: None,
            updated_at: None,
            created_at: None,
            zone: None,
        }
    }
}

/// Virtual MAC type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MacType {
    #[serde(rename = "unknown_type")]
    UnknownType,
    #[serde(rename = "vmware")]
    Vmware,
    #[serde(rename = "xen")]
    Xen,
    #[serde(rename = "kvm")]
    Kvm,
}

impl Default for MacType {
    fn default() -> MacType {
        Self::UnknownType
    }
}
/// Virtual MAC status
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "updating")]
    Updating,
    #[serde(rename = "used")]
    Used,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "deleting")]
    Deleting,
}

impl Default for Status {
    fn default() -> Status {
        Self::Unknown
    }
}
