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
pub struct CloneInstanceRequest {
    /// Name of the clone instance
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Node type of the clone
    #[serde(
        rename = "node_type",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub node_type: Option<Option<String>>,
}

impl CloneInstanceRequest {
    pub fn new() -> CloneInstanceRequest {
        CloneInstanceRequest {
            name: None,
            node_type: None,
        }
    }
}
