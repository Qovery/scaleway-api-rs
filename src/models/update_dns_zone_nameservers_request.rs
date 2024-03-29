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
pub struct UpdateDnsZoneNameserversRequest {
    /// The new DNS zone nameservers
    #[serde(rename = "ns")]
    pub ns: Vec<crate::models::ScalewayPeriodDomainPeriodV2beta1PeriodNameserver>,
}

impl UpdateDnsZoneNameserversRequest {
    pub fn new(
        ns: Vec<crate::models::ScalewayPeriodDomainPeriodV2beta1PeriodNameserver>,
    ) -> UpdateDnsZoneNameserversRequest {
        UpdateDnsZoneNameserversRequest { ns }
    }
}
