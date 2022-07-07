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
pub struct ScalewayRdbV1Snapshot {
    /// UUID of the snapshot
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// UUID of the instance
    #[serde(rename = "instance_id", skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// Name of the snapshot
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Status of the snapshot
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// Size of the snapshot (in bytes)
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// Expiration date (Format ISO 8601)
    #[serde(rename = "expires_at", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Creation date (Format ISO 8601)
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Updated date (Format ISO 8601)
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Name of the instance of the snapshot
    #[serde(rename = "instance_name", skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    /// Source node type
    #[serde(rename = "node_type", skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    /// Region of this snapshot
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

impl ScalewayRdbV1Snapshot {
    pub fn new() -> ScalewayRdbV1Snapshot {
        ScalewayRdbV1Snapshot {
            id: None,
            instance_id: None,
            name: None,
            status: None,
            size: None,
            expires_at: None,
            created_at: None,
            updated_at: None,
            instance_name: None,
            node_type: None,
            region: None,
        }
    }
}

/// Status of the snapshot
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "creating")]
    Creating,
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "restoring")]
    Restoring,
    #[serde(rename = "deleting")]
    Deleting,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "locked")]
    Locked,
}

impl Default for Status {
    fn default() -> Status {
        Self::Unknown
    }
}
