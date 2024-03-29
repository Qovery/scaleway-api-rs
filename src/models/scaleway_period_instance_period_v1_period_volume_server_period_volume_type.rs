/*
 * Account API
 *
 * # Introduction  The Account API allows you to manage projects. Project is Scaleway’s resource management feature. Designed to help you organize your infrastructure and cloud services, the feature allows resources to be isolated and grouped into specific projects.
 *
 * The version of the OpenAPI document: v2
 *
 * Generated by: https://openapi-generator.tech
 */

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ScalewayPeriodInstancePeriodV1PeriodVolumeServerPeriodVolumeType {
    #[serde(rename = "l_ssd")]
    LSsd,
    #[serde(rename = "b_ssd")]
    BSsd,
}

impl ToString for ScalewayPeriodInstancePeriodV1PeriodVolumeServerPeriodVolumeType {
    fn to_string(&self) -> String {
        match self {
            Self::LSsd => String::from("l_ssd"),
            Self::BSsd => String::from("b_ssd"),
        }
    }
}

impl Default for ScalewayPeriodInstancePeriodV1PeriodVolumeServerPeriodVolumeType {
    fn default() -> ScalewayPeriodInstancePeriodV1PeriodVolumeServerPeriodVolumeType {
        Self::LSsd
    }
}
