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
pub enum ScalewayPeriodInstancePeriodV1PeriodVolumeServerPeriodState {
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

impl ToString for ScalewayPeriodInstancePeriodV1PeriodVolumeServerPeriodState {
    fn to_string(&self) -> String {
        match self {
            Self::Available => String::from("available"),
            Self::Snapshotting => String::from("snapshotting"),
            Self::Error => String::from("error"),
            Self::Fetching => String::from("fetching"),
            Self::Resizing => String::from("resizing"),
            Self::Saving => String::from("saving"),
            Self::Hotsyncing => String::from("hotsyncing"),
        }
    }
}

impl Default for ScalewayPeriodInstancePeriodV1PeriodVolumeServerPeriodState {
    fn default() -> ScalewayPeriodInstancePeriodV1PeriodVolumeServerPeriodState {
        Self::Available
    }
}
