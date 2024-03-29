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
pub struct ScalewayPeriodInstancePeriodV1PeriodTask {
    /// The unique ID of the task
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The description of the task
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The progress of the task in percent
    #[serde(rename = "progress", skip_serializing_if = "Option::is_none")]
    pub progress: Option<i32>,
    /// The task start date (RFC 3339 format)
    #[serde(
        rename = "started_at",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub started_at: Option<Option<String>>,
    /// The task end date (RFC 3339 format)
    #[serde(
        rename = "terminated_at",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub terminated_at: Option<Option<String>>,
    /// The task status
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "href_from", skip_serializing_if = "Option::is_none")]
    pub href_from: Option<String>,
    #[serde(rename = "href_result", skip_serializing_if = "Option::is_none")]
    pub href_result: Option<String>,
    /// The zone in which is the task
    #[serde(rename = "zone", skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
}

impl ScalewayPeriodInstancePeriodV1PeriodTask {
    pub fn new() -> ScalewayPeriodInstancePeriodV1PeriodTask {
        ScalewayPeriodInstancePeriodV1PeriodTask {
            id: None,
            description: None,
            progress: None,
            started_at: None,
            terminated_at: None,
            status: None,
            href_from: None,
            href_result: None,
            zone: None,
        }
    }
}

/// The task status
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "started")]
    Started,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "retry")]
    Retry,
}

impl Default for Status {
    fn default() -> Status {
        Self::Pending
    }
}
