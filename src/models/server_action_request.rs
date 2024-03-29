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
pub struct ServerActionRequest {
    /// The action to perform on the server
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<Action>,
    /// The name of the backup you want to create. This field should only be specified when performing a backup action.
    #[serde(
        rename = "name",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<Option<String>>,
    #[serde(rename = "volumes", skip_serializing_if = "Option::is_none")]
    pub volumes: Option<crate::models::ServerActionRequestVolumes>,
}

impl ServerActionRequest {
    pub fn new() -> ServerActionRequest {
        ServerActionRequest {
            action: None,
            name: None,
            volumes: None,
        }
    }
}

/// The action to perform on the server
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "poweron")]
    Poweron,
    #[serde(rename = "backup")]
    Backup,
    #[serde(rename = "stop_in_place")]
    StopInPlace,
    #[serde(rename = "poweroff")]
    Poweroff,
    #[serde(rename = "terminate")]
    Terminate,
    #[serde(rename = "reboot")]
    Reboot,
}

impl Default for Action {
    fn default() -> Action {
        Self::Poweron
    }
}
