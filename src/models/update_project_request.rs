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
pub struct UpdateProjectRequest {
    /// The name of the project
    #[serde(
        rename = "name",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<Option<String>>,
    /// The description of the project
    #[serde(
        rename = "description",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<Option<String>>,
}

impl UpdateProjectRequest {
    pub fn new() -> UpdateProjectRequest {
        UpdateProjectRequest {
            name: None,
            description: None,
        }
    }
}
