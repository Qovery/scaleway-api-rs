/*
 * Account API
 *
 * # Introduction  The Account API allows you to manage projects. Project is Scaleway’s resource management feature. Designed to help you organize your infrastructure and cloud services, the feature allows resources to be isolated and grouped into specific projects.
 *
 * The version of the OpenAPI document: v2
 *
 * Generated by: https://openapi-generator.tech
 */

/// ScalewayBaremetalV1OsUser : Define the username requirements to install the OS

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ScalewayBaremetalV1OsUser {
    #[serde(rename = "editable", skip_serializing_if = "Option::is_none")]
    pub editable: Option<bool>,
    #[serde(rename = "required", skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(
        rename = "default_value",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_value: Option<Option<String>>,
}

impl ScalewayBaremetalV1OsUser {
    /// Define the username requirements to install the OS
    pub fn new() -> ScalewayBaremetalV1OsUser {
        ScalewayBaremetalV1OsUser {
            editable: None,
            required: None,
            default_value: None,
        }
    }
}
