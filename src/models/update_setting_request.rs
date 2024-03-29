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
pub struct UpdateSettingRequest {
    /// Enable/Disable the setting
    #[serde(
        rename = "enabled",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub enabled: Option<Option<bool>>,
}

impl UpdateSettingRequest {
    pub fn new() -> UpdateSettingRequest {
        UpdateSettingRequest { enabled: None }
    }
}
