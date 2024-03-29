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
pub struct ScalewayPeriodInstancePeriodV1PeriodListImagesResponse {
    /// List of images
    #[serde(rename = "images", skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<crate::models::ScalewayPeriodInstancePeriodV1PeriodImage>>,
}

impl ScalewayPeriodInstancePeriodV1PeriodListImagesResponse {
    pub fn new() -> ScalewayPeriodInstancePeriodV1PeriodListImagesResponse {
        ScalewayPeriodInstancePeriodV1PeriodListImagesResponse { images: None }
    }
}
