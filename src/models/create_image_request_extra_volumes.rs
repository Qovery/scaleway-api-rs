/*
 * Account API
 *
 * # Introduction  The Account API allows you to manage projects. Project is Scaleway’s resource management feature. Designed to help you organize your infrastructure and cloud services, the feature allows resources to be isolated and grouped into specific projects.
 *
 * The version of the OpenAPI document: v2
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreateImageRequestExtraVolumes : Additional volumes of the image

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateImageRequestExtraVolumes {
    #[serde(rename = "<extra_volumeKey>", skip_serializing_if = "Option::is_none")]
    pub less_than_extra_volume_key_greater_than:
        Option<Box<crate::models::ScalewayPeriodInstancePeriodV1PeriodVolumeTemplate>>,
}

impl CreateImageRequestExtraVolumes {
    /// Additional volumes of the image
    pub fn new() -> CreateImageRequestExtraVolumes {
        CreateImageRequestExtraVolumes {
            less_than_extra_volume_key_greater_than: None,
        }
    }
}
