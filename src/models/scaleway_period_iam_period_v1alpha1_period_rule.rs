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
pub struct ScalewayPeriodIamPeriodV1alpha1PeriodRule {
    /// Id of rule
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Names of permission sets bound to the rule
    #[serde(
        rename = "permission_set_names",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub permission_set_names: Option<Option<Vec<String>>>,
    /// Permission_set_names have the same scope_type
    #[serde(
        rename = "permission_sets_scope_type",
        skip_serializing_if = "Option::is_none"
    )]
    pub permission_sets_scope_type: Option<PermissionSetsScopeType>,
    /// List of project IDs scoped to the rule
    #[serde(
        rename = "project_ids",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub project_ids: Option<Option<Vec<String>>>,
    /// ID of organization scoped to the rule
    #[serde(rename = "organization_id", skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    /// ID of account root user scoped to the rule
    #[serde(
        rename = "account_root_user_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub account_root_user_id: Option<String>,
}

impl ScalewayPeriodIamPeriodV1alpha1PeriodRule {
    pub fn new() -> ScalewayPeriodIamPeriodV1alpha1PeriodRule {
        ScalewayPeriodIamPeriodV1alpha1PeriodRule {
            id: None,
            permission_set_names: None,
            permission_sets_scope_type: None,
            project_ids: None,
            organization_id: None,
            account_root_user_id: None,
        }
    }
}

/// Permission_set_names have the same scope_type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PermissionSetsScopeType {
    #[serde(rename = "unknown_scope_type")]
    UnknownScopeType,
    #[serde(rename = "projects")]
    Projects,
    #[serde(rename = "organization")]
    Organization,
    #[serde(rename = "account_root_user")]
    AccountRootUser,
}

impl Default for PermissionSetsScopeType {
    fn default() -> PermissionSetsScopeType {
        Self::UnknownScopeType
    }
}
