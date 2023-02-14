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
pub struct DeleteInstanceSettingsRequest {
    /// Settings names to delete
    #[serde(rename = "setting_names")]
    pub setting_names: Vec<String>,
}

impl DeleteInstanceSettingsRequest {
    pub fn new(setting_names: Vec<String>) -> DeleteInstanceSettingsRequest {
        DeleteInstanceSettingsRequest { setting_names }
    }
}