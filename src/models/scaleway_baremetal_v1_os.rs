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
pub struct ScalewayBaremetalV1Os {
    /// ID of the OS
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name of the OS
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Version of the OS
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// URL of this os's logo
    #[serde(rename = "logo_url", skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    #[serde(rename = "ssh", skip_serializing_if = "Option::is_none")]
    pub ssh: Option<Box<crate::models::ScalewayBaremetalV1OsSsh>>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<crate::models::ScalewayBaremetalV1OsUser>>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<Box<crate::models::ScalewayBaremetalV1OsPassword>>,
    #[serde(rename = "service_user", skip_serializing_if = "Option::is_none")]
    pub service_user: Option<Box<crate::models::ScalewayBaremetalV1OsServiceUser>>,
    #[serde(rename = "service_password", skip_serializing_if = "Option::is_none")]
    pub service_password: Option<Box<crate::models::ScalewayBaremetalV1OsServicePassword>>,
    /// State of OS
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl ScalewayBaremetalV1Os {
    pub fn new() -> ScalewayBaremetalV1Os {
        ScalewayBaremetalV1Os {
            id: None,
            name: None,
            version: None,
            logo_url: None,
            ssh: None,
            user: None,
            password: None,
            service_user: None,
            service_password: None,
            enabled: None,
        }
    }
}
