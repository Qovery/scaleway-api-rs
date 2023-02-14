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
pub struct ScalewayPeriodInstancePeriodV1PeriodVolumeServer {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "export_uri", skip_serializing_if = "Option::is_none")]
    pub export_uri: Option<String>,
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[serde(rename = "server", skip_serializing_if = "Option::is_none")]
    pub server: Option<Box<crate::models::ScalewayPeriodInstancePeriodV1PeriodServerSummary>>,
    /// (in bytes)
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(rename = "volume_type", skip_serializing_if = "Option::is_none")]
    pub volume_type:
        Option<crate::models::ScalewayPeriodInstancePeriodV1PeriodVolumeServerPeriodVolumeType>,
    /// (RFC 3339 format)
    #[serde(
        rename = "creation_date",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub creation_date: Option<Option<String>>,
    /// (RFC 3339 format)
    #[serde(
        rename = "modification_date",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub modification_date: Option<Option<String>>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<crate::models::ScalewayPeriodInstancePeriodV1PeriodVolumeServerPeriodState>,
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    #[serde(rename = "boot", skip_serializing_if = "Option::is_none")]
    pub boot: Option<bool>,
    #[serde(rename = "zone", skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
}

impl ScalewayPeriodInstancePeriodV1PeriodVolumeServer {
    pub fn new() -> ScalewayPeriodInstancePeriodV1PeriodVolumeServer {
        ScalewayPeriodInstancePeriodV1PeriodVolumeServer {
            id: None,
            name: None,
            export_uri: None,
            organization: None,
            server: None,
            size: None,
            volume_type: None,
            creation_date: None,
            modification_date: None,
            state: None,
            project: None,
            boot: None,
            zone: None,
        }
    }
}
