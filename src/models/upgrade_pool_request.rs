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
pub struct UpgradePoolRequest {
    /// The new Kubernetes version for the pool
    #[serde(rename = "version")]
    pub version: String,
}

impl UpgradePoolRequest {
    pub fn new(version: String) -> UpgradePoolRequest {
        UpgradePoolRequest { version }
    }
}