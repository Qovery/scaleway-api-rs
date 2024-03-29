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
pub struct CreateServer1Request {
    /// The server name
    #[serde(rename = "name")]
    pub name: String,
    /// Define if a dynamic IP is required for the instance
    #[serde(
        rename = "dynamic_ip_required",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub dynamic_ip_required: Option<Option<bool>>,
    /// Define the server commercial type (i.e. GP1-S)
    #[serde(rename = "commercial_type")]
    pub commercial_type: String,
    /// The server image ID
    #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "volumes", skip_serializing_if = "Option::is_none")]
    pub volumes: Option<crate::models::CreateServer1RequestVolumes>,
    /// True if IPv6 is enabled on the server
    #[serde(rename = "enable_ipv6", skip_serializing_if = "Option::is_none")]
    pub enable_ipv6: Option<bool>,
    /// The ID of the reserved IP to attach to the server
    #[serde(
        rename = "public_ip",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub public_ip: Option<Option<String>>,
    /// The boot type to use
    #[serde(rename = "boot_type", skip_serializing_if = "Option::is_none")]
    pub boot_type: Option<BootType>,
    /// The bootscript ID to use when `boot_type` is set to `bootscript`
    #[serde(
        rename = "bootscript",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub bootscript: Option<Option<String>>,
    /// The server organization ID
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    /// The server project ID
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    /// The server tags
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// The security group ID
    #[serde(
        rename = "security_group",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub security_group: Option<Option<String>>,
    /// Placement group ID if server must be part of a placement group
    #[serde(
        rename = "placement_group",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub placement_group: Option<Option<String>>,
}

impl CreateServer1Request {
    pub fn new(name: String, commercial_type: String) -> CreateServer1Request {
        CreateServer1Request {
            name,
            dynamic_ip_required: None,
            commercial_type,
            image: None,
            volumes: None,
            enable_ipv6: None,
            public_ip: None,
            boot_type: None,
            bootscript: None,
            organization: None,
            project: None,
            tags: None,
            security_group: None,
            placement_group: None,
        }
    }
}

/// The boot type to use
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BootType {
    #[serde(rename = "local")]
    Local,
    #[serde(rename = "bootscript")]
    Bootscript,
    #[serde(rename = "rescue")]
    Rescue,
}

impl Default for BootType {
    fn default() -> BootType {
        Self::Local
    }
}
