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
pub enum ScalewayPeriodDomainPeriodV2beta1PeriodRawFormat {
    #[serde(rename = "unknown_raw_format")]
    UnknownRawFormat,
    #[serde(rename = "bind")]
    Bind,
}

impl ToString for ScalewayPeriodDomainPeriodV2beta1PeriodRawFormat {
    fn to_string(&self) -> String {
        match self {
            Self::UnknownRawFormat => String::from("unknown_raw_format"),
            Self::Bind => String::from("bind"),
        }
    }
}

impl Default for ScalewayPeriodDomainPeriodV2beta1PeriodRawFormat {
    fn default() -> ScalewayPeriodDomainPeriodV2beta1PeriodRawFormat {
        Self::UnknownRawFormat
    }
}