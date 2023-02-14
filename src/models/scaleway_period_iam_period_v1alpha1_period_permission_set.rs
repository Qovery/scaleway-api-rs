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
pub struct ScalewayPeriodIamPeriodV1alpha1PeriodPermissionSet {
    /// Id of permission set
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name of permission set
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Scope of permission set
    #[serde(rename = "scope_type", skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<ScopeType>,
    /// Description of permission set
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Categories of permission set
    #[serde(
        rename = "categories",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub categories: Option<Option<Vec<String>>>,
}

impl ScalewayPeriodIamPeriodV1alpha1PeriodPermissionSet {
    pub fn new() -> ScalewayPeriodIamPeriodV1alpha1PeriodPermissionSet {
        ScalewayPeriodIamPeriodV1alpha1PeriodPermissionSet {
            id: None,
            name: None,
            scope_type: None,
            description: None,
            categories: None,
        }
    }
}

/// Scope of permission set
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ScopeType {
    #[serde(rename = "unknown_scope_type")]
    UnknownScopeType,
    #[serde(rename = "projects")]
    Projects,
    #[serde(rename = "organization")]
    Organization,
    #[serde(rename = "account_root_user")]
    AccountRootUser,
}

impl Default for ScopeType {
    fn default() -> ScopeType {
        Self::UnknownScopeType
    }
}