/*
 * Account API
 *
 * # Introduction  The Account API allows you to manage projects. Project is Scaleway’s resource management feature. Designed to help you organize your infrastructure and cloud services, the feature allows resources to be isolated and grouped into specific projects.
 *
 * The version of the OpenAPI document: v2
 *
 * Generated by: https://openapi-generator.tech
 */

/// ServerActionRequestVolumes : For each volume UUID, the snapshot parameters of the volume. This field should only be specified when performing a backup action.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ServerActionRequestVolumes {
    #[serde(rename = "<volumeKey>", skip_serializing_if = "Option::is_none")]
    pub less_than_volume_key_greater_than: Option<Box<crate::models::ScalewayPeriodInstancePeriodV1PeriodServerActionRequestPeriodVolumeBackupTemplate>>,
}

impl ServerActionRequestVolumes {
    /// For each volume UUID, the snapshot parameters of the volume. This field should only be specified when performing a backup action.
    pub fn new() -> ServerActionRequestVolumes {
        ServerActionRequestVolumes {
            less_than_volume_key_greater_than: None,
        }
    }
}
