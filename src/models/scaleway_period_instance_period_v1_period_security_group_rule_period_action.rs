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
pub enum ScalewayPeriodInstancePeriodV1PeriodSecurityGroupRulePeriodAction {
    #[serde(rename = "accept")]
    Accept,
    #[serde(rename = "drop")]
    Drop,
}

impl ToString for ScalewayPeriodInstancePeriodV1PeriodSecurityGroupRulePeriodAction {
    fn to_string(&self) -> String {
        match self {
            Self::Accept => String::from("accept"),
            Self::Drop => String::from("drop"),
        }
    }
}

impl Default for ScalewayPeriodInstancePeriodV1PeriodSecurityGroupRulePeriodAction {
    fn default() -> ScalewayPeriodInstancePeriodV1PeriodSecurityGroupRulePeriodAction {
        Self::Accept
    }
}
