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
pub struct CreateSnapshotRequest {
    /// Name of the snapshot
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// UUID of the volume
    #[serde(
        rename = "volume_id",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub volume_id: Option<Option<String>>,
    /// The tags of the snapshot
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Organization ID of the snapshot
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    /// Project ID of the snapshot
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    /// Overrides the volume_type of the snapshot. If omitted, the volume type of the original volume will be used.
    #[serde(rename = "volume_type", skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<VolumeType>,
    /// Bucket name for snapshot imports
    #[serde(
        rename = "bucket",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub bucket: Option<Option<String>>,
    /// Object key for snapshot imports
    #[serde(
        rename = "key",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub key: Option<Option<String>>,
    /// Imported snapshot size, must be a multiple of 512 (in bytes)
    #[serde(
        rename = "size",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub size: Option<Option<i32>>,
}

impl CreateSnapshotRequest {
    pub fn new() -> CreateSnapshotRequest {
        CreateSnapshotRequest {
            name: None,
            volume_id: None,
            tags: None,
            organization: None,
            project: None,
            volume_type: None,
            bucket: None,
            key: None,
            size: None,
        }
    }
}

/// Overrides the volume_type of the snapshot. If omitted, the volume type of the original volume will be used.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VolumeType {
    #[serde(rename = "unknown_volume_type")]
    UnknownVolumeType,
    #[serde(rename = "l_ssd")]
    LSsd,
    #[serde(rename = "b_ssd")]
    BSsd,
    #[serde(rename = "unified")]
    Unified,
}

impl Default for VolumeType {
    fn default() -> VolumeType {
        Self::UnknownVolumeType
    }
}