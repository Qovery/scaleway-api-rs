/*
 * Account API
 *
 * # Introduction  The Account API allows you to manage projects. Project is Scaleway’s resource management feature. Designed to help you organize your infrastructure and cloud services, the feature allows resources to be isolated and grouped into specific projects.
 *
 * The version of the OpenAPI document: v2
 *
 * Generated by: https://openapi-generator.tech
 */

/// ScalewayBaremetalV1GetServerMetricsResponsePings : Timeseries of ping on the server

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ScalewayBaremetalV1GetServerMetricsResponsePings {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "points", skip_serializing_if = "Option::is_none")]
    pub points: Option<Vec<crate::models::ScalewayPeriodStdPeriodTimeSeriesPeriodPoint>>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::models::ScalewayBaremetalV1GetServerMetricsResponsePingsMetadata>,
}

impl ScalewayBaremetalV1GetServerMetricsResponsePings {
    /// Timeseries of ping on the server
    pub fn new() -> ScalewayBaremetalV1GetServerMetricsResponsePings {
        ScalewayBaremetalV1GetServerMetricsResponsePings {
            name: None,
            points: None,
            metadata: None,
        }
    }
}
