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
pub struct UpdateDnsZoneRecordsRequest {
    /// The changes made to the records
    #[serde(rename = "changes")]
    pub changes: Vec<crate::models::ScalewayPeriodDomainPeriodV2beta1PeriodRecordChange>,
    /// Whether or not to return all the records
    #[serde(
        rename = "return_all_records",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub return_all_records: Option<Option<bool>>,
    /// Forbid the creation of the target zone if not existing (default action is yes)
    #[serde(
        rename = "disallow_new_zone_creation",
        skip_serializing_if = "Option::is_none"
    )]
    pub disallow_new_zone_creation: Option<bool>,
    /// Don't use the autoincremenent serial but the provided one (0 to keep the same)
    #[serde(
        rename = "serial",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub serial: Option<Option<i32>>,
}

impl UpdateDnsZoneRecordsRequest {
    pub fn new(
        changes: Vec<crate::models::ScalewayPeriodDomainPeriodV2beta1PeriodRecordChange>,
    ) -> UpdateDnsZoneRecordsRequest {
        UpdateDnsZoneRecordsRequest {
            changes,
            return_all_records: None,
            disallow_new_zone_creation: None,
            serial: None,
        }
    }
}
