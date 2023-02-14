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
pub struct ScalewayPeriodBillingPeriodV2alpha1PeriodGetConsumptionResponsePeriodConsumption {
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value:
        Option<Box<crate::models::ScalewayBillingV2alpha1GetConsumptionResponseConsumptionValue>>,
    /// Description of the consumption
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Project ID of the consumption
    #[serde(rename = "project_id", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    /// Category of the consumption
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// The unique identifier of the product
    #[serde(rename = "operation_path", skip_serializing_if = "Option::is_none")]
    pub operation_path: Option<String>,
}

impl ScalewayPeriodBillingPeriodV2alpha1PeriodGetConsumptionResponsePeriodConsumption {
    pub fn new() -> ScalewayPeriodBillingPeriodV2alpha1PeriodGetConsumptionResponsePeriodConsumption
    {
        ScalewayPeriodBillingPeriodV2alpha1PeriodGetConsumptionResponsePeriodConsumption {
            value: None,
            description: None,
            project_id: None,
            category: None,
            operation_path: None,
        }
    }
}
