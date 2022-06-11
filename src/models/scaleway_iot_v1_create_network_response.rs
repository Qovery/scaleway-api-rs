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
pub struct ScalewayIotV1CreateNetworkResponse {
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    pub network: Option<Box<crate::models::ScalewayIotV1CreateNetworkResponseNetwork>>,
    /// Endpoint Key to keep secret. This cannot be retrieved later
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

impl ScalewayIotV1CreateNetworkResponse {
    pub fn new() -> ScalewayIotV1CreateNetworkResponse {
        ScalewayIotV1CreateNetworkResponse {
            network: None,
            secret: None,
        }
    }
}
