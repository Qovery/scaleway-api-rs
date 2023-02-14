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
pub struct ScalewayPeriodRdbPeriodV1PeriodSnapshot {
    /// UUID of the snapshot
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// UUID of the instance
    #[serde(rename = "instance_id", skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// Name of the snapshot
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Status of the snapshot
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// Size of the snapshot (in bytes)
    #[serde(
        rename = "size",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub size: Option<Option<i32>>,
    /// Expiration date (Format ISO 8601) (RFC 3339 format)
    #[serde(
        rename = "expires_at",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub expires_at: Option<Option<String>>,
    /// Creation date (Format ISO 8601) (RFC 3339 format)
    #[serde(
        rename = "created_at",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub created_at: Option<Option<String>>,
    /// Updated date (Format ISO 8601) (RFC 3339 format)
    #[serde(
        rename = "updated_at",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub updated_at: Option<Option<String>>,
    /// Name of the instance of the snapshot
    #[serde(rename = "instance_name", skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    /// Source node type
    #[serde(rename = "node_type", skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    /// Region of this snapshot
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

impl ScalewayPeriodRdbPeriodV1PeriodSnapshot {
    pub fn new() -> ScalewayPeriodRdbPeriodV1PeriodSnapshot {
        ScalewayPeriodRdbPeriodV1PeriodSnapshot {
            id: None,
            instance_id: None,
            name: None,
            status: None,
            size: None,
            expires_at: None,
            created_at: None,
            updated_at: None,
            instance_name: None,
            node_type: None,
            region: None,
        }
    }
}

/// Status of the snapshot
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "creating")]
    Creating,
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "restoring")]
    Restoring,
    #[serde(rename = "deleting")]
    Deleting,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "locked")]
    Locked,
}

impl Default for Status {
    fn default() -> Status {
        Self::Unknown
    }
}
