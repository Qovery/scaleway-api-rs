/*
 * Account API
 *
 * # Introduction  The Account API allows you to manage projects. Project is Scaleway’s resource management feature. Designed to help you organize your infrastructure and cloud services, the feature allows resources to be isolated and grouped into specific projects.
 *
 * The version of the OpenAPI document: v2
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InstallServerRequest {
    /// ID of the OS to install on the server
    #[serde(rename = "os_id")]
    pub os_id: String,
    /// Hostname of the server
    #[serde(rename = "hostname")]
    pub hostname: String,
    /// SSH key IDs authorized on the server
    #[serde(rename = "ssh_key_ids")]
    pub ssh_key_ids: Vec<String>,
    /// User used for the installation
    #[serde(
        rename = "user",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub user: Option<Option<String>>,
    /// Password used for the installation
    #[serde(
        rename = "password",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub password: Option<Option<String>>,
    /// User used for the service to install
    #[serde(
        rename = "service_user",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub service_user: Option<Option<String>>,
    /// Password used for the service to install
    #[serde(
        rename = "service_password",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub service_password: Option<Option<String>>,
}

impl InstallServerRequest {
    pub fn new(os_id: String, hostname: String, ssh_key_ids: Vec<String>) -> InstallServerRequest {
        InstallServerRequest {
            os_id,
            hostname,
            ssh_key_ids,
            user: None,
            password: None,
            service_user: None,
            service_password: None,
        }
    }
}
