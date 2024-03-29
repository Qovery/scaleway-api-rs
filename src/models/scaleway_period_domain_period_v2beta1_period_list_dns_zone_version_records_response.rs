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
pub struct ScalewayPeriodDomainPeriodV2beta1PeriodListDnsZoneVersionRecordsResponse {
    /// The total number of DNS zones versions records
    #[serde(rename = "total_count", skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    #[serde(rename = "records", skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<crate::models::ScalewayPeriodDomainPeriodV2beta1PeriodRecord>>,
}

impl ScalewayPeriodDomainPeriodV2beta1PeriodListDnsZoneVersionRecordsResponse {
    pub fn new() -> ScalewayPeriodDomainPeriodV2beta1PeriodListDnsZoneVersionRecordsResponse {
        ScalewayPeriodDomainPeriodV2beta1PeriodListDnsZoneVersionRecordsResponse {
            total_count: None,
            records: None,
        }
    }
}
