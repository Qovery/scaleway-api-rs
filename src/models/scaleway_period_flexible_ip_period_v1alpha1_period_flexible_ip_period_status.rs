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
pub enum ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodFlexibleIpPeriodStatus {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "updating")]
    Updating,
    #[serde(rename = "attached")]
    Attached,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "detaching")]
    Detaching,
    #[serde(rename = "locked")]
    Locked,
}

impl ToString for ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodFlexibleIpPeriodStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Unknown => String::from("unknown"),
            Self::Ready => String::from("ready"),
            Self::Updating => String::from("updating"),
            Self::Attached => String::from("attached"),
            Self::Error => String::from("error"),
            Self::Detaching => String::from("detaching"),
            Self::Locked => String::from("locked"),
        }
    }
}

impl Default for ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodFlexibleIpPeriodStatus {
    fn default() -> ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodFlexibleIpPeriodStatus {
        Self::Unknown
    }
}
