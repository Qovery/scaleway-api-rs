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
pub struct UpdateSshKeyRequest {
    /// Name of the SSH key. Max length is 1000
    #[serde(
        rename = "name",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<Option<String>>,
    /// Enable or disable the SSH key
    #[serde(
        rename = "disabled",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled: Option<Option<bool>>,
}

impl UpdateSshKeyRequest {
    pub fn new() -> UpdateSshKeyRequest {
        UpdateSshKeyRequest {
            name: None,
            disabled: None,
        }
    }
}
