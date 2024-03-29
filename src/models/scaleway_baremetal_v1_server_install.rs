/*
 * Account API
 *
 * # Introduction  The Account API allows you to manage projects. Project is Scaleway’s resource management feature. Designed to help you organize your infrastructure and cloud services, the feature allows resources to be isolated and grouped into specific projects.
 *
 * The version of the OpenAPI document: v2
 *
 * Generated by: https://openapi-generator.tech
 */

/// ScalewayBaremetalV1ServerInstall : Configuration of installation

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ScalewayBaremetalV1ServerInstall {
    /// ID of the OS
    #[serde(rename = "os_id", skip_serializing_if = "Option::is_none")]
    pub os_id: Option<String>,
    /// Host defined in the server install
    #[serde(rename = "hostname", skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// SSH public key IDs defined in the server install
    #[serde(rename = "ssh_key_ids", skip_serializing_if = "Option::is_none")]
    pub ssh_key_ids: Option<Vec<String>>,
    /// Status of the server install
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// User defined in the server install or the default one if none were specified
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// Service user defined in the server install or the default one if none were specified
    #[serde(rename = "service_user", skip_serializing_if = "Option::is_none")]
    pub service_user: Option<String>,
    /// The address of the installed service
    #[serde(rename = "service_url", skip_serializing_if = "Option::is_none")]
    pub service_url: Option<String>,
}

impl ScalewayBaremetalV1ServerInstall {
    /// Configuration of installation
    pub fn new() -> ScalewayBaremetalV1ServerInstall {
        ScalewayBaremetalV1ServerInstall {
            os_id: None,
            hostname: None,
            ssh_key_ids: None,
            status: None,
            user: None,
            service_user: None,
            service_url: None,
        }
    }
}

/// Status of the server install
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "to_install")]
    ToInstall,
    #[serde(rename = "installing")]
    Installing,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "error")]
    Error,
}

impl Default for Status {
    fn default() -> Status {
        Self::Unknown
    }
}
