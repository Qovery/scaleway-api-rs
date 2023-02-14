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
pub struct CreateVolumeRequest {
    /// The volume name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The volume organization ID
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    /// The volume project ID
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    /// The volume tags
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// The volume type
    #[serde(rename = "volume_type", skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<VolumeType>,
    /// The volume disk size, must be a multiple of 512 (in bytes)
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    /// The ID of the volume on which this volume will be based
    #[serde(rename = "base_volume", skip_serializing_if = "Option::is_none")]
    pub base_volume: Option<String>,
    /// The ID of the snapshot on which this volume will be based
    #[serde(rename = "base_snapshot", skip_serializing_if = "Option::is_none")]
    pub base_snapshot: Option<String>,
}

impl CreateVolumeRequest {
    pub fn new() -> CreateVolumeRequest {
        CreateVolumeRequest {
            name: None,
            organization: None,
            project: None,
            tags: None,
            volume_type: None,
            size: None,
            base_volume: None,
            base_snapshot: None,
        }
    }
}

/// The volume type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VolumeType {
    #[serde(rename = "l_ssd")]
    LSsd,
    #[serde(rename = "b_ssd")]
    BSsd,
    #[serde(rename = "unified")]
    Unified,
}

impl Default for VolumeType {
    fn default() -> VolumeType {
        Self::LSsd
    }
}
