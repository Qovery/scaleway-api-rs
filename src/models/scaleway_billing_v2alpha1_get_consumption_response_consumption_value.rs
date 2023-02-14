/*
 * Account API
 *
 * # Introduction  The Account API allows you to manage projects. Project is Scaleway’s resource management feature. Designed to help you organize your infrastructure and cloud services, the feature allows resources to be isolated and grouped into specific projects.
 *
 * The version of the OpenAPI document: v2
 *
 * Generated by: https://openapi-generator.tech
 */

/// ScalewayBillingV2alpha1GetConsumptionResponseConsumptionValue : Monetary value of the consumption

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ScalewayBillingV2alpha1GetConsumptionResponseConsumptionValue {
    #[serde(rename = "currency_code", skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(rename = "units", skip_serializing_if = "Option::is_none")]
    pub units: Option<i32>,
    #[serde(rename = "nanos", skip_serializing_if = "Option::is_none")]
    pub nanos: Option<i32>,
}

impl ScalewayBillingV2alpha1GetConsumptionResponseConsumptionValue {
    /// Monetary value of the consumption
    pub fn new() -> ScalewayBillingV2alpha1GetConsumptionResponseConsumptionValue {
        ScalewayBillingV2alpha1GetConsumptionResponseConsumptionValue {
            currency_code: None,
            units: None,
            nanos: None,
        }
    }
}
