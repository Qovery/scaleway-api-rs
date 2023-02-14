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
pub enum ScalewayPeriodInstancePeriodV1PeriodSecurityGroupRulePeriodProtocol {
    #[serde(rename = "TCP")]
    Tcp,
    #[serde(rename = "UDP")]
    Udp,
    #[serde(rename = "ICMP")]
    Icmp,
    #[serde(rename = "ANY")]
    Any,
}

impl ToString for ScalewayPeriodInstancePeriodV1PeriodSecurityGroupRulePeriodProtocol {
    fn to_string(&self) -> String {
        match self {
            Self::Tcp => String::from("TCP"),
            Self::Udp => String::from("UDP"),
            Self::Icmp => String::from("ICMP"),
            Self::Any => String::from("ANY"),
        }
    }
}

impl Default for ScalewayPeriodInstancePeriodV1PeriodSecurityGroupRulePeriodProtocol {
    fn default() -> ScalewayPeriodInstancePeriodV1PeriodSecurityGroupRulePeriodProtocol {
        Self::Tcp
    }
}