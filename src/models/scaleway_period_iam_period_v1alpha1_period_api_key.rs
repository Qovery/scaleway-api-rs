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
pub struct ScalewayPeriodIamPeriodV1alpha1PeriodApiKey {
    /// Access key of API key
    #[serde(rename = "access_key", skip_serializing_if = "Option::is_none")]
    pub access_key: Option<String>,
    /// Secret key of API Key
    #[serde(
        rename = "secret_key",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub secret_key: Option<Option<String>>,
    /// ID of application bearer
    #[serde(rename = "application_id", skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    /// ID of user bearer
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Description of API key
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Creation date and time of API key (RFC 3339 format)
    #[serde(
        rename = "created_at",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub created_at: Option<Option<String>>,
    /// Last update date and time of API key (RFC 3339 format)
    #[serde(
        rename = "updated_at",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub updated_at: Option<Option<String>>,
    /// Expiration date and time of API key (RFC 3339 format)
    #[serde(
        rename = "expires_at",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub expires_at: Option<Option<String>>,
    /// The default project ID specified for this API key
    #[serde(rename = "default_project_id", skip_serializing_if = "Option::is_none")]
    pub default_project_id: Option<String>,
    /// Whether or not the API key is editable
    #[serde(rename = "editable", skip_serializing_if = "Option::is_none")]
    pub editable: Option<bool>,
    /// IP Address of the device which created the API key
    #[serde(rename = "creation_ip", skip_serializing_if = "Option::is_none")]
    pub creation_ip: Option<String>,
}

impl ScalewayPeriodIamPeriodV1alpha1PeriodApiKey {
    pub fn new() -> ScalewayPeriodIamPeriodV1alpha1PeriodApiKey {
        ScalewayPeriodIamPeriodV1alpha1PeriodApiKey {
            access_key: None,
            secret_key: None,
            application_id: None,
            user_id: None,
            description: None,
            created_at: None,
            updated_at: None,
            expires_at: None,
            default_project_id: None,
            editable: None,
            creation_ip: None,
        }
    }
}