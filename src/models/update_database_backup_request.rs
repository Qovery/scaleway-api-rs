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
pub struct UpdateDatabaseBackupRequest {
    /// Name of the Database Backup
    #[serde(
        rename = "name",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<Option<String>>,
    /// Expiration date (Format ISO 8601) (RFC 3339 format)
    #[serde(
        rename = "expires_at",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub expires_at: Option<Option<String>>,
}

impl UpdateDatabaseBackupRequest {
    pub fn new() -> UpdateDatabaseBackupRequest {
        UpdateDatabaseBackupRequest {
            name: None,
            expires_at: None,
        }
    }
}
