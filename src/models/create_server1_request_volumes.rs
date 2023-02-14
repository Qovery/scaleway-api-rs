/*
 * Account API
 *
 * # Introduction  The Account API allows you to manage projects. Project is Scaleway’s resource management feature. Designed to help you organize your infrastructure and cloud services, the feature allows resources to be isolated and grouped into specific projects.
 *
 * The version of the OpenAPI document: v2
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreateServer1RequestVolumes : The volumes attached to the server

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateServer1RequestVolumes {
    #[serde(rename = "<volumeKey>", skip_serializing_if = "Option::is_none")]
    pub less_than_volume_key_greater_than:
        Option<Box<crate::models::ScalewayPeriodInstancePeriodV1PeriodVolumeServerTemplate>>,
}

impl CreateServer1RequestVolumes {
    /// The volumes attached to the server
    pub fn new() -> CreateServer1RequestVolumes {
        CreateServer1RequestVolumes {
            less_than_volume_key_greater_than: None,
        }
    }
}
