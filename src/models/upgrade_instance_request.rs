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
pub struct UpgradeInstanceRequest {
    /// Node type of the instance you want to upgrade to
    #[serde(rename = "node_type", skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    /// Set to true to enable high availability on your instance
    #[serde(rename = "enable_ha", skip_serializing_if = "Option::is_none")]
    pub enable_ha: Option<bool>,
    /// Increase your block storage volume size
    #[serde(rename = "volume_size", skip_serializing_if = "Option::is_none")]
    pub volume_size: Option<i32>,
    /// Change your instance storage type
    #[serde(rename = "volume_type", skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<VolumeType>,
    /// This will create a new Database Instance with same instance specification as the current one and perform a Database Engine upgrade. (UUID format)
    #[serde(
        rename = "upgradable_version_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub upgradable_version_id: Option<String>,
}

impl UpgradeInstanceRequest {
    pub fn new() -> UpgradeInstanceRequest {
        UpgradeInstanceRequest {
            node_type: None,
            enable_ha: None,
            volume_size: None,
            volume_type: None,
            upgradable_version_id: None,
        }
    }
}

/// Change your instance storage type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VolumeType {
    #[serde(rename = "lssd")]
    Lssd,
    #[serde(rename = "bssd")]
    Bssd,
}

impl Default for VolumeType {
    fn default() -> VolumeType {
        Self::Lssd
    }
}
