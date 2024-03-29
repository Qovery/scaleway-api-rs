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
pub enum ScalewayPeriodDomainPeriodV2beta1PeriodDnsZonePeriodStatus {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "locked")]
    Locked,
}

impl ToString for ScalewayPeriodDomainPeriodV2beta1PeriodDnsZonePeriodStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Unknown => String::from("unknown"),
            Self::Active => String::from("active"),
            Self::Pending => String::from("pending"),
            Self::Error => String::from("error"),
            Self::Locked => String::from("locked"),
        }
    }
}

impl Default for ScalewayPeriodDomainPeriodV2beta1PeriodDnsZonePeriodStatus {
    fn default() -> ScalewayPeriodDomainPeriodV2beta1PeriodDnsZonePeriodStatus {
        Self::Unknown
    }
}
