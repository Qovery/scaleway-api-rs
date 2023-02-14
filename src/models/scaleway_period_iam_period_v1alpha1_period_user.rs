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
pub struct ScalewayPeriodIamPeriodV1alpha1PeriodUser {
    /// ID of user
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Email of user
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Creation date of user (RFC 3339 format)
    #[serde(
        rename = "created_at",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub created_at: Option<Option<String>>,
    /// Last update date of user (RFC 3339 format)
    #[serde(
        rename = "updated_at",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub updated_at: Option<Option<String>>,
    /// ID of organization
    #[serde(rename = "organization_id", skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    /// Deletion status of user. Owner user cannot be deleted
    #[serde(rename = "deletable", skip_serializing_if = "Option::is_none")]
    pub deletable: Option<bool>,
    /// Last login date (RFC 3339 format)
    #[serde(
        rename = "last_login_at",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_login_at: Option<Option<String>>,
    /// Type of the user
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<RHashType>,
    /// 2FA enabled
    #[serde(rename = "two_factor_enabled", skip_serializing_if = "Option::is_none")]
    pub two_factor_enabled: Option<bool>,
    /// Status of invitation for the user
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}

impl ScalewayPeriodIamPeriodV1alpha1PeriodUser {
    pub fn new() -> ScalewayPeriodIamPeriodV1alpha1PeriodUser {
        ScalewayPeriodIamPeriodV1alpha1PeriodUser {
            id: None,
            email: None,
            created_at: None,
            updated_at: None,
            organization_id: None,
            deletable: None,
            last_login_at: None,
            r#type: None,
            two_factor_enabled: None,
            status: None,
        }
    }
}

/// Type of the user
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "unknown_type")]
    UnknownType,
    #[serde(rename = "guest")]
    Guest,
    #[serde(rename = "owner")]
    Owner,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::UnknownType
    }
}
/// Status of invitation for the user
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "unknown_status")]
    UnknownStatus,
    #[serde(rename = "invitation_pending")]
    InvitationPending,
    #[serde(rename = "activated")]
    Activated,
}

impl Default for Status {
    fn default() -> Status {
        Self::UnknownStatus
    }
}