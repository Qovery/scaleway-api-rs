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
pub enum ScalewayPeriodRdbPeriodV1PeriodAclRulePeriodAction {
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "deny")]
    Deny,
}

impl ToString for ScalewayPeriodRdbPeriodV1PeriodAclRulePeriodAction {
    fn to_string(&self) -> String {
        match self {
            Self::Allow => String::from("allow"),
            Self::Deny => String::from("deny"),
        }
    }
}

impl Default for ScalewayPeriodRdbPeriodV1PeriodAclRulePeriodAction {
    fn default() -> ScalewayPeriodRdbPeriodV1PeriodAclRulePeriodAction {
        Self::Allow
    }
}
