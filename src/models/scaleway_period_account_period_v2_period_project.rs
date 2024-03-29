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
pub struct ScalewayPeriodAccountPeriodV2PeriodProject {
    /// The ID of the project (UUID format)
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the project
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The organization ID of the project (UUID format)
    #[serde(rename = "organization_id", skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    /// The creation date of the project (RFC 3339 format)
    #[serde(
        rename = "created_at",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub created_at: Option<Option<String>>,
    /// The update date of the project (RFC 3339 format)
    #[serde(
        rename = "updated_at",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub updated_at: Option<Option<String>>,
    /// The description of the project
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl ScalewayPeriodAccountPeriodV2PeriodProject {
    pub fn new() -> ScalewayPeriodAccountPeriodV2PeriodProject {
        ScalewayPeriodAccountPeriodV2PeriodProject {
            id: None,
            name: None,
            organization_id: None,
            created_at: None,
            updated_at: None,
            description: None,
        }
    }
}
