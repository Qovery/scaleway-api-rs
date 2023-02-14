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
pub struct CreateSecurityGroupRequest {
    /// Name of the security group
    #[serde(rename = "name")]
    pub name: String,
    /// Description of the security group
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Organization ID the security group belongs to
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    /// Project ID the security group belong to
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    /// The tags of the security group
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Whether this security group becomes the default security group for new instances
    #[serde(
        rename = "organization_default",
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_default: Option<bool>,
    /// Whether this security group becomes the default security group for new instances
    #[serde(rename = "project_default", skip_serializing_if = "Option::is_none")]
    pub project_default: Option<bool>,
    /// Whether the security group is stateful or not
    #[serde(rename = "stateful", skip_serializing_if = "Option::is_none")]
    pub stateful: Option<bool>,
    /// Default policy for inbound rules
    #[serde(
        rename = "inbound_default_policy",
        skip_serializing_if = "Option::is_none"
    )]
    pub inbound_default_policy: Option<InboundDefaultPolicy>,
    /// Default policy for outbound rules
    #[serde(
        rename = "outbound_default_policy",
        skip_serializing_if = "Option::is_none"
    )]
    pub outbound_default_policy: Option<OutboundDefaultPolicy>,
    /// True if SMTP is blocked on IPv4 and IPv6. This feature is read only, please open a ticket if you need to make it configurable.
    #[serde(
        rename = "enable_default_security",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_default_security: Option<Option<bool>>,
}

impl CreateSecurityGroupRequest {
    pub fn new(name: String) -> CreateSecurityGroupRequest {
        CreateSecurityGroupRequest {
            name,
            description: None,
            organization: None,
            project: None,
            tags: None,
            organization_default: None,
            project_default: None,
            stateful: None,
            inbound_default_policy: None,
            outbound_default_policy: None,
            enable_default_security: None,
        }
    }
}

/// Default policy for inbound rules
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InboundDefaultPolicy {
    #[serde(rename = "accept")]
    Accept,
    #[serde(rename = "drop")]
    Drop,
}

impl Default for InboundDefaultPolicy {
    fn default() -> InboundDefaultPolicy {
        Self::Accept
    }
}
/// Default policy for outbound rules
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OutboundDefaultPolicy {
    #[serde(rename = "accept")]
    Accept,
    #[serde(rename = "drop")]
    Drop,
}

impl Default for OutboundDefaultPolicy {
    fn default() -> OutboundDefaultPolicy {
        Self::Accept
    }
}