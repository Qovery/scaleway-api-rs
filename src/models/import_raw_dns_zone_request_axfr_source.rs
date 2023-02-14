/*
 * Account API
 *
 * # Introduction  The Account API allows you to manage projects. Project is Scaleway’s resource management feature. Designed to help you organize your infrastructure and cloud services, the feature allows resources to be isolated and grouped into specific projects.
 *
 * The version of the OpenAPI document: v2
 *
 * Generated by: https://openapi-generator.tech
 */

/// ImportRawDnsZoneRequestAxfrSource : Import from the nameserver given with tsig use or not

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ImportRawDnsZoneRequestAxfrSource {
    #[serde(rename = "name_server", skip_serializing_if = "Option::is_none")]
    pub name_server: Option<String>,
    #[serde(rename = "tsig_key", skip_serializing_if = "Option::is_none")]
    pub tsig_key: Option<Box<crate::models::ScalewayPeriodDomainPeriodV2beta1PeriodImportRawDnsZoneRequestPeriodTsigKey>>,
}

impl ImportRawDnsZoneRequestAxfrSource {
    /// Import from the nameserver given with tsig use or not
    pub fn new() -> ImportRawDnsZoneRequestAxfrSource {
        ImportRawDnsZoneRequestAxfrSource {
            name_server: None,
            tsig_key: None,
        }
    }
}