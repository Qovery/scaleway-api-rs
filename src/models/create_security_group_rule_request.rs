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
pub struct CreateSecurityGroupRuleRequest {
    #[serde(rename = "protocol")]
    pub protocol:
        crate::models::ScalewayPeriodInstancePeriodV1PeriodSecurityGroupRulePeriodProtocol,
    #[serde(rename = "direction")]
    pub direction:
        crate::models::ScalewayPeriodInstancePeriodV1PeriodSecurityGroupRulePeriodDirection,
    #[serde(rename = "action")]
    pub action: crate::models::ScalewayPeriodInstancePeriodV1PeriodSecurityGroupRulePeriodAction,
    /// (IP network)
    #[serde(rename = "ip_range")]
    pub ip_range: String,
    /// The beginning of the range of ports to apply this rule to (inclusive)
    #[serde(
        rename = "dest_port_from",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub dest_port_from: Option<Option<i32>>,
    /// The end of the range of ports to apply this rule to (inclusive)
    #[serde(
        rename = "dest_port_to",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub dest_port_to: Option<Option<i32>>,
    /// The position of this rule in the security group rules list
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    /// Indicates if this rule is editable (will be ignored)
    #[serde(rename = "editable", skip_serializing_if = "Option::is_none")]
    pub editable: Option<bool>,
}

impl CreateSecurityGroupRuleRequest {
    pub fn new(
        protocol: crate::models::ScalewayPeriodInstancePeriodV1PeriodSecurityGroupRulePeriodProtocol,
        direction: crate::models::ScalewayPeriodInstancePeriodV1PeriodSecurityGroupRulePeriodDirection,
        action: crate::models::ScalewayPeriodInstancePeriodV1PeriodSecurityGroupRulePeriodAction,
        ip_range: String,
    ) -> CreateSecurityGroupRuleRequest {
        CreateSecurityGroupRuleRequest {
            protocol,
            direction,
            action,
            ip_range,
            dest_port_from: None,
            dest_port_to: None,
            position: None,
            editable: None,
        }
    }
}
