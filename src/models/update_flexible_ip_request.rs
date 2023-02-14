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
pub struct UpdateFlexibleIpRequest {
    /// Description to associate with the Flexible IP, max 255 characters
    #[serde(
        rename = "description",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<Option<String>>,
    /// Tags to associate with the Flexible IP
    #[serde(
        rename = "tags",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub tags: Option<Option<Vec<String>>>,
    /// Reverse DNS value
    #[serde(
        rename = "reverse",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub reverse: Option<Option<String>>,
}

impl UpdateFlexibleIpRequest {
    pub fn new() -> UpdateFlexibleIpRequest {
        UpdateFlexibleIpRequest {
            description: None,
            tags: None,
            reverse: None,
        }
    }
}
