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
pub struct SetPlacementGroupRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[serde(rename = "policy_mode", skip_serializing_if = "Option::is_none")]
    pub policy_mode:
        Option<crate::models::ScalewayPeriodInstancePeriodV1PeriodPlacementGroupPeriodPolicyMode>,
    #[serde(rename = "policy_type", skip_serializing_if = "Option::is_none")]
    pub policy_type:
        Option<crate::models::ScalewayPeriodInstancePeriodV1PeriodPlacementGroupPeriodPolicyType>,
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    #[serde(
        rename = "tags",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub tags: Option<Option<crate::models::ScalewayPeriodStdPeriodStringsValue>>,
}

impl SetPlacementGroupRequest {
    pub fn new() -> SetPlacementGroupRequest {
        SetPlacementGroupRequest {
            name: None,
            organization: None,
            policy_mode: None,
            policy_type: None,
            project: None,
            tags: None,
        }
    }
}
