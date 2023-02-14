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
pub struct ScalewayPeriodInstancePeriodV1PeriodVolumeTemplate {
    /// UUID of the volume
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name of the volume
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Disk size of the volume, must be a multiple of 512 (in bytes)
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    /// Type of the volume
    #[serde(rename = "volume_type", skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<VolumeType>,
    /// Organization ID of the volume
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    /// Project ID of the volume
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
}

impl ScalewayPeriodInstancePeriodV1PeriodVolumeTemplate {
    pub fn new() -> ScalewayPeriodInstancePeriodV1PeriodVolumeTemplate {
        ScalewayPeriodInstancePeriodV1PeriodVolumeTemplate {
            id: None,
            name: None,
            size: None,
            volume_type: None,
            organization: None,
            project: None,
        }
    }
}

/// Type of the volume
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