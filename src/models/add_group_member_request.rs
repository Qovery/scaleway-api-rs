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
pub struct AddGroupMemberRequest {
    /// ID of the user to add
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// ID of the application to add
    #[serde(rename = "application_id", skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
}

impl AddGroupMemberRequest {
    pub fn new() -> AddGroupMemberRequest {
        AddGroupMemberRequest {
            user_id: None,
            application_id: None,
        }
    }
}
