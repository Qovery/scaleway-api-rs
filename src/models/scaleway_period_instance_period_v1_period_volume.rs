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
pub struct ScalewayPeriodInstancePeriodV1PeriodVolume {
    /// The volume unique ID
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The volume name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Show the volume NBD export URI
    #[serde(rename = "export_uri", skip_serializing_if = "Option::is_none")]
    pub export_uri: Option<String>,
    /// The volume disk size (in bytes)
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    /// The volume type
    #[serde(rename = "volume_type", skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<VolumeType>,
    /// The volume creation date (RFC 3339 format)
    #[serde(
        rename = "creation_date",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub creation_date: Option<Option<String>>,
    /// The volume modification date (RFC 3339 format)
    #[serde(
        rename = "modification_date",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub modification_date: Option<Option<String>>,
    /// The volume organization ID
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    /// The volume project ID
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    /// The volume tags
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "server", skip_serializing_if = "Option::is_none")]
    pub server: Option<Box<crate::models::SetVolumeRequestServer>>,
    /// The volume state
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    /// The zone in which is the volume
    #[serde(rename = "zone", skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
}

impl ScalewayPeriodInstancePeriodV1PeriodVolume {
    pub fn new() -> ScalewayPeriodInstancePeriodV1PeriodVolume {
        ScalewayPeriodInstancePeriodV1PeriodVolume {
            id: None,
            name: None,
            export_uri: None,
            size: None,
            volume_type: None,
            creation_date: None,
            modification_date: None,
            organization: None,
            project: None,
            tags: None,
            server: None,
            state: None,
            zone: None,
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
/// The volume state
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "available")]
    Available,
    #[serde(rename = "snapshotting")]
    Snapshotting,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "fetching")]
    Fetching,
    #[serde(rename = "resizing")]
    Resizing,
    #[serde(rename = "saving")]
    Saving,
    #[serde(rename = "hotsyncing")]
    Hotsyncing,
}

impl Default for State {
    fn default() -> State {
        Self::Available
    }
}
