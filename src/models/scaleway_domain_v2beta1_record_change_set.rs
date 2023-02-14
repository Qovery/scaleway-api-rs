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
pub struct ScalewayDomainV2beta1RecordChangeSet {
    /// (UUID format)
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "id_fields", skip_serializing_if = "Option::is_none")]
    pub id_fields: Option<Box<crate::models::ScalewayDomainV2beta1RecordChangeSetIdFields>>,
    #[serde(rename = "records", skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<crate::models::ScalewayPeriodDomainPeriodV2beta1PeriodRecord>>,
}

impl ScalewayDomainV2beta1RecordChangeSet {
    pub fn new() -> ScalewayDomainV2beta1RecordChangeSet {
        ScalewayDomainV2beta1RecordChangeSet {
            id: None,
            id_fields: None,
            records: None,
        }
    }
}
