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
pub struct ScalewayPeriodDomainPeriodV2beta1PeriodDnsZone {
    #[serde(rename = "domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "subdomain", skip_serializing_if = "Option::is_none")]
    pub subdomain: Option<String>,
    #[serde(rename = "ns", skip_serializing_if = "Option::is_none")]
    pub ns: Option<Vec<String>>,
    #[serde(rename = "ns_default", skip_serializing_if = "Option::is_none")]
    pub ns_default: Option<Vec<String>>,
    #[serde(rename = "ns_master", skip_serializing_if = "Option::is_none")]
    pub ns_master: Option<Vec<String>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::ScalewayPeriodDomainPeriodV2beta1PeriodDnsZonePeriodStatus>,
    #[serde(
        rename = "message",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub message: Option<Option<String>>,
    /// (RFC 3339 format)
    #[serde(
        rename = "updated_at",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub updated_at: Option<Option<String>>,
    #[serde(rename = "project_id", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
}

impl ScalewayPeriodDomainPeriodV2beta1PeriodDnsZone {
    pub fn new() -> ScalewayPeriodDomainPeriodV2beta1PeriodDnsZone {
        ScalewayPeriodDomainPeriodV2beta1PeriodDnsZone {
            domain: None,
            subdomain: None,
            ns: None,
            ns_default: None,
            ns_master: None,
            status: None,
            message: None,
            updated_at: None,
            project_id: None,
        }
    }
}