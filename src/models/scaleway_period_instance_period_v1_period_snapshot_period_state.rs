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
pub enum ScalewayPeriodInstancePeriodV1PeriodSnapshotPeriodState {
    #[serde(rename = "available")]
    Available,
    #[serde(rename = "snapshotting")]
    Snapshotting,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "invalid_data")]
    InvalidData,
    #[serde(rename = "importing")]
    Importing,
    #[serde(rename = "exporting")]
    Exporting,
}

impl ToString for ScalewayPeriodInstancePeriodV1PeriodSnapshotPeriodState {
    fn to_string(&self) -> String {
        match self {
            Self::Available => String::from("available"),
            Self::Snapshotting => String::from("snapshotting"),
            Self::Error => String::from("error"),
            Self::InvalidData => String::from("invalid_data"),
            Self::Importing => String::from("importing"),
            Self::Exporting => String::from("exporting"),
        }
    }
}

impl Default for ScalewayPeriodInstancePeriodV1PeriodSnapshotPeriodState {
    fn default() -> ScalewayPeriodInstancePeriodV1PeriodSnapshotPeriodState {
        Self::Available
    }
}
