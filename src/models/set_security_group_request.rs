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
pub struct SetSecurityGroupRequest {
    /// The name of the security group
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The tags of the security group
    #[serde(
        rename = "tags",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub tags: Option<Option<Vec<String>>>,
    /// The creation date of the security group (will be ignored) (RFC 3339 format)
    #[serde(
        rename = "creation_date",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub creation_date: Option<Option<String>>,
    /// The modification date of the security group (will be ignored) (RFC 3339 format)
    #[serde(
        rename = "modification_date",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub modification_date: Option<Option<String>>,
    /// The description of the security group
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// True if SMTP is blocked on IPv4 and IPv6. This feature is read only, please open a ticket if you need to make it configurable.
    #[serde(
        rename = "enable_default_security",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_default_security: Option<bool>,
    /// The default inbound policy
    #[serde(
        rename = "inbound_default_policy",
        skip_serializing_if = "Option::is_none"
    )]
    pub inbound_default_policy: Option<InboundDefaultPolicy>,
    /// The default outbound policy
    #[serde(
        rename = "outbound_default_policy",
        skip_serializing_if = "Option::is_none"
    )]
    pub outbound_default_policy: Option<OutboundDefaultPolicy>,
    /// The security groups organization ID
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    /// The security group project ID
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    /// Please use project_default instead
    #[serde(
        rename = "organization_default",
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_default: Option<bool>,
    /// True use this security group for future instances created in this project
    #[serde(rename = "project_default", skip_serializing_if = "Option::is_none")]
    pub project_default: Option<bool>,
    /// The servers attached to this security group
    #[serde(rename = "servers", skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<crate::models::ScalewayPeriodInstancePeriodV1PeriodServerSummary>>,
    /// True to set the security group as stateful
    #[serde(rename = "stateful", skip_serializing_if = "Option::is_none")]
    pub stateful: Option<bool>,
}

impl SetSecurityGroupRequest {
    pub fn new() -> SetSecurityGroupRequest {
        SetSecurityGroupRequest {
            name: None,
            tags: None,
            creation_date: None,
            modification_date: None,
            description: None,
            enable_default_security: None,
            inbound_default_policy: None,
            outbound_default_policy: None,
            organization: None,
            project: None,
            organization_default: None,
            project_default: None,
            servers: None,
            stateful: None,
        }
    }
}

/// The default inbound policy
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
/// The default outbound policy
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
