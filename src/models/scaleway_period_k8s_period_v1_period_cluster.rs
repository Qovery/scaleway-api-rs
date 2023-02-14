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
pub struct ScalewayPeriodK8sPeriodV1PeriodCluster {
    /// The ID of the cluster
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The type of the cluster
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The name of the cluster
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The status of the cluster
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// The Kubernetes version of the cluster
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// The region in which the cluster is
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The ID of the organization owning the cluster
    #[serde(rename = "organization_id", skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    /// The ID of the project owning the cluster
    #[serde(rename = "project_id", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    /// The tags associated with the cluster
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// The Container Network Interface (CNI) plugin running in the cluster
    #[serde(rename = "cni", skip_serializing_if = "Option::is_none")]
    pub cni: Option<Cni>,
    /// The description of the cluster
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The Kubernetes API server URL of the cluster
    #[serde(rename = "cluster_url", skip_serializing_if = "Option::is_none")]
    pub cluster_url: Option<String>,
    /// The DNS wildcard resovling all the ready nodes of the cluster
    #[serde(rename = "dns_wildcard", skip_serializing_if = "Option::is_none")]
    pub dns_wildcard: Option<String>,
    /// The date at which the cluster was created (RFC 3339 format)
    #[serde(
        rename = "created_at",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub created_at: Option<Option<String>>,
    /// The date at which the cluster was last updated (RFC 3339 format)
    #[serde(
        rename = "updated_at",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub updated_at: Option<Option<String>>,
    #[serde(rename = "autoscaler_config", skip_serializing_if = "Option::is_none")]
    pub autoscaler_config: Option<Box<crate::models::ScalewayK8sV1ClusterAutoscalerConfig>>,
    /// The enablement of the Kubernetes Dashboard in the cluster
    #[serde(rename = "dashboard_enabled", skip_serializing_if = "Option::is_none")]
    pub dashboard_enabled: Option<bool>,
    /// The ingress controller used in the cluster
    #[serde(rename = "ingress", skip_serializing_if = "Option::is_none")]
    pub ingress: Option<Ingress>,
    #[serde(rename = "auto_upgrade", skip_serializing_if = "Option::is_none")]
    pub auto_upgrade: Option<Box<crate::models::ScalewayK8sV1ClusterAutoUpgrade>>,
    /// True if a new Kubernetes version is available
    #[serde(rename = "upgrade_available", skip_serializing_if = "Option::is_none")]
    pub upgrade_available: Option<bool>,
    /// List of enabled feature gates
    #[serde(rename = "feature_gates", skip_serializing_if = "Option::is_none")]
    pub feature_gates: Option<Vec<String>>,
    /// List of enabled admission plugins
    #[serde(rename = "admission_plugins", skip_serializing_if = "Option::is_none")]
    pub admission_plugins: Option<Vec<String>>,
    #[serde(
        rename = "open_id_connect_config",
        skip_serializing_if = "Option::is_none"
    )]
    pub open_id_connect_config: Option<Box<crate::models::ScalewayK8sV1ClusterOpenIdConnectConfig>>,
    /// Additional Subject Alternative Names for the Kubernetes API server certificate
    #[serde(
        rename = "apiserver_cert_sans",
        skip_serializing_if = "Option::is_none"
    )]
    pub apiserver_cert_sans: Option<Vec<String>>,
}

impl ScalewayPeriodK8sPeriodV1PeriodCluster {
    pub fn new() -> ScalewayPeriodK8sPeriodV1PeriodCluster {
        ScalewayPeriodK8sPeriodV1PeriodCluster {
            id: None,
            r#type: None,
            name: None,
            status: None,
            version: None,
            region: None,
            organization_id: None,
            project_id: None,
            tags: None,
            cni: None,
            description: None,
            cluster_url: None,
            dns_wildcard: None,
            created_at: None,
            updated_at: None,
            autoscaler_config: None,
            dashboard_enabled: None,
            ingress: None,
            auto_upgrade: None,
            upgrade_available: None,
            feature_gates: None,
            admission_plugins: None,
            open_id_connect_config: None,
            apiserver_cert_sans: None,
        }
    }
}

/// The status of the cluster
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "creating")]
    Creating,
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "deleting")]
    Deleting,
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "updating")]
    Updating,
    #[serde(rename = "locked")]
    Locked,
    #[serde(rename = "pool_required")]
    PoolRequired,
}

impl Default for Status {
    fn default() -> Status {
        Self::Unknown
    }
}
/// The Container Network Interface (CNI) plugin running in the cluster
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Cni {
    #[serde(rename = "unknown_cni")]
    UnknownCni,
    #[serde(rename = "cilium")]
    Cilium,
    #[serde(rename = "calico")]
    Calico,
    #[serde(rename = "weave")]
    Weave,
    #[serde(rename = "flannel")]
    Flannel,
    #[serde(rename = "kilo")]
    Kilo,
}

impl Default for Cni {
    fn default() -> Cni {
        Self::UnknownCni
    }
}
/// The ingress controller used in the cluster
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Ingress {
    #[serde(rename = "unknown_ingress")]
    UnknownIngress,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "nginx")]
    Nginx,
    #[serde(rename = "traefik")]
    Traefik,
    #[serde(rename = "traefik2")]
    Traefik2,
}

impl Default for Ingress {
    fn default() -> Ingress {
        Self::UnknownIngress
    }
}