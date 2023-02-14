/*
 * Account API
 *
 * # Introduction  The Account API allows you to manage projects. Project is Scaleway’s resource management feature. Designed to help you organize your infrastructure and cloud services, the feature allows resources to be isolated and grouped into specific projects.
 *
 * The version of the OpenAPI document: v2
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AddInstanceAclRulesRequest {
    /// ACLs rules to add to the instance
    #[serde(rename = "rules")]
    pub rules: Vec<crate::models::ScalewayPeriodRdbPeriodV1PeriodAclRuleRequest>,
}

impl AddInstanceAclRulesRequest {
    pub fn new(
        rules: Vec<crate::models::ScalewayPeriodRdbPeriodV1PeriodAclRuleRequest>,
    ) -> AddInstanceAclRulesRequest {
        AddInstanceAclRulesRequest { rules }
    }
}