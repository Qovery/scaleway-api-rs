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
pub struct CreateInstanceFromSnapshotRequest {
    /// Name of the instance created with the snapshot
    #[serde(rename = "instance_name", skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    /// Whether or not High-Availability is enabled on the new instance
    #[serde(rename = "is_ha_cluster", skip_serializing_if = "Option::is_none")]
    pub is_ha_cluster: Option<bool>,
    /// The node type used to restore the snapshot
    #[serde(rename = "node_type", skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
}

impl CreateInstanceFromSnapshotRequest {
    pub fn new() -> CreateInstanceFromSnapshotRequest {
        CreateInstanceFromSnapshotRequest {
            instance_name: None,
            is_ha_cluster: None,
            node_type: None,
        }
    }
}
