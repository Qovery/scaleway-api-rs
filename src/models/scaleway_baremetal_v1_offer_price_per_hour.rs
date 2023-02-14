/*
 * Account API
 *
 * # Introduction  The Account API allows you to manage projects. Project is Scaleway’s resource management feature. Designed to help you organize your infrastructure and cloud services, the feature allows resources to be isolated and grouped into specific projects.
 *
 * The version of the OpenAPI document: v2
 *
 * Generated by: https://openapi-generator.tech
 */

/// ScalewayBaremetalV1OfferPricePerHour : Price of the offer for the next 60 minutes (a server order at 11h32 will be payed until 12h32)

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ScalewayBaremetalV1OfferPricePerHour {
    #[serde(rename = "currency_code", skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(rename = "units", skip_serializing_if = "Option::is_none")]
    pub units: Option<i32>,
    #[serde(rename = "nanos", skip_serializing_if = "Option::is_none")]
    pub nanos: Option<i32>,
}

impl ScalewayBaremetalV1OfferPricePerHour {
    /// Price of the offer for the next 60 minutes (a server order at 11h32 will be payed until 12h32)
    pub fn new() -> ScalewayBaremetalV1OfferPricePerHour {
        ScalewayBaremetalV1OfferPricePerHour {
            currency_code: None,
            units: None,
            nanos: None,
        }
    }
}
