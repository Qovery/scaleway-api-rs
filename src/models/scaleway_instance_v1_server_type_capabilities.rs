/*
 * Account API
 *
 * # Introduction  The Account API allows you to manage projects. Project is Scaleway’s resource management feature. Designed to help you organize your infrastructure and cloud services, the feature allows resources to be isolated and grouped into specific projects.
 *
 * The version of the OpenAPI document: v2
 *
 * Generated by: https://openapi-generator.tech
 */

/// ScalewayInstanceV1ServerTypeCapabilities : Capabilities

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ScalewayInstanceV1ServerTypeCapabilities {
    /// True if server supports block storage
    #[serde(
        rename = "block_storage",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub block_storage: Option<Option<bool>>,
    /// List of supported boot types
    #[serde(rename = "boot_types", skip_serializing_if = "Option::is_none")]
    pub boot_types: Option<Vec<crate::models::ScalewayPeriodInstancePeriodV1PeriodBootType>>,
}

impl ScalewayInstanceV1ServerTypeCapabilities {
    /// Capabilities
    pub fn new() -> ScalewayInstanceV1ServerTypeCapabilities {
        ScalewayInstanceV1ServerTypeCapabilities {
            block_storage: None,
            boot_types: None,
        }
    }
}