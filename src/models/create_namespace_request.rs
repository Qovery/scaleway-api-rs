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
pub struct CreateNamespaceRequest {
    /// Define a namespace name
    #[serde(rename = "name")]
    pub name: String,
    /// Define a description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Assign the namespace owner (deprecated)
    #[serde(rename = "organization_id", skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    /// Assign the namespace to a project ID
    #[serde(rename = "project_id", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    /// Define the default visibility policy
    #[serde(rename = "is_public", skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
}

impl CreateNamespaceRequest {
    pub fn new(name: String) -> CreateNamespaceRequest {
        CreateNamespaceRequest {
            name,
            description: None,
            organization_id: None,
            project_id: None,
            is_public: None,
        }
    }
}
