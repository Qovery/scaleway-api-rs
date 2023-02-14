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
pub struct ScalewayPeriodDomainPeriodV2beta1PeriodRecordChange {
    #[serde(rename = "add", skip_serializing_if = "Option::is_none")]
    pub add: Option<Box<crate::models::ScalewayDomainV2beta1RecordChangeAdd>>,
    #[serde(rename = "set", skip_serializing_if = "Option::is_none")]
    pub set: Option<Box<crate::models::ScalewayDomainV2beta1RecordChangeSet>>,
    #[serde(rename = "delete", skip_serializing_if = "Option::is_none")]
    pub delete: Option<Box<crate::models::ScalewayDomainV2beta1RecordChangeDelete>>,
    #[serde(rename = "clear", skip_serializing_if = "Option::is_none")]
    pub clear: Option<serde_json::Value>,
}

impl ScalewayPeriodDomainPeriodV2beta1PeriodRecordChange {
    pub fn new() -> ScalewayPeriodDomainPeriodV2beta1PeriodRecordChange {
        ScalewayPeriodDomainPeriodV2beta1PeriodRecordChange {
            add: None,
            set: None,
            delete: None,
            clear: None,
        }
    }
}
