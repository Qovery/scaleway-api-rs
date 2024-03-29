/*
 * Account API
 *
 * # Introduction  The Account API allows you to manage projects. Project is Scaleway’s resource management feature. Designed to help you organize your infrastructure and cloud services, the feature allows resources to be isolated and grouped into specific projects.
 *
 * The version of the OpenAPI document: v2
 *
 * Generated by: https://openapi-generator.tech
 */

/// ScalewayK8sV1ClusterAutoUpgrade : The auto upgrade configuration of the cluster

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ScalewayK8sV1ClusterAutoUpgrade {
    /// Whether or not auto upgrade is enabled for the cluster
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "maintenance_window", skip_serializing_if = "Option::is_none")]
    pub maintenance_window:
        Option<Box<crate::models::CreateClusterRequestAutoUpgradeMaintenanceWindow>>,
}

impl ScalewayK8sV1ClusterAutoUpgrade {
    /// The auto upgrade configuration of the cluster
    pub fn new() -> ScalewayK8sV1ClusterAutoUpgrade {
        ScalewayK8sV1ClusterAutoUpgrade {
            enabled: None,
            maintenance_window: None,
        }
    }
}
