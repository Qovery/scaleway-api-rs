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
pub struct ScalewayPeriodBaremetalPeriodV1PeriodListOffersResponse {
    /// Total count of matching offers
    #[serde(rename = "total_count", skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// Offers that match filters
    #[serde(rename = "offers", skip_serializing_if = "Option::is_none")]
    pub offers: Option<Vec<crate::models::ScalewayPeriodBaremetalPeriodV1PeriodOffer>>,
}

impl ScalewayPeriodBaremetalPeriodV1PeriodListOffersResponse {
    pub fn new() -> ScalewayPeriodBaremetalPeriodV1PeriodListOffersResponse {
        ScalewayPeriodBaremetalPeriodV1PeriodListOffersResponse {
            total_count: None,
            offers: None,
        }
    }
}
