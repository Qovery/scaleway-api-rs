/*
 * Account API
 *
 * # Introduction  The Account API allows you to manage projects. Project is Scaleway’s resource management feature. Designed to help you organize your infrastructure and cloud services, the feature allows resources to be isolated and grouped into specific projects.
 *
 * The version of the OpenAPI document: v2
 *
 * Generated by: https://openapi-generator.tech
 */

/// UpdateClusterRequestAutoUpgrade : The new auto upgrade configuration of the cluster. Note that all fields need to be set.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateClusterRequestAutoUpgrade {
    /// Whether or not auto upgrade is enabled for the cluster
    #[serde(
        rename = "enable",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable: Option<Option<bool>>,
    #[serde(rename = "maintenance_window", skip_serializing_if = "Option::is_none")]
    pub maintenance_window:
        Option<Box<crate::models::CreateClusterRequestAutoUpgradeMaintenanceWindow>>,
}

impl UpdateClusterRequestAutoUpgrade {
    /// The new auto upgrade configuration of the cluster. Note that all fields need to be set.
    pub fn new() -> UpdateClusterRequestAutoUpgrade {
        UpdateClusterRequestAutoUpgrade {
            enable: None,
            maintenance_window: None,
        }
    }
}