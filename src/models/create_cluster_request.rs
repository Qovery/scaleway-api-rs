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
pub struct CreateClusterRequest {
    /// The organization ID where the cluster will be created
    #[serde(rename = "organization_id", skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    /// The project ID where the cluster will be created
    #[serde(rename = "project_id")]
    pub project_id: String,
    /// The type of the cluster (possible values are kapsule, multicloud).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The name of the cluster
    #[serde(rename = "name")]
    pub name: String,
    /// The description of the cluster
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The tags associated with the cluster
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// The Kubernetes version of the cluster
    #[serde(rename = "version")]
    pub version: String,
    /// The Container Network Interface (CNI) plugin that will run in the cluster
    #[serde(rename = "cni")]
    pub cni: Cni,
    /// The enablement of the Kubernetes Dashboard in the cluster
    #[serde(rename = "enable_dashboard", skip_serializing_if = "Option::is_none")]
    pub enable_dashboard: Option<bool>,
    /// The Ingress Controller that will run in the cluster
    #[serde(rename = "ingress", skip_serializing_if = "Option::is_none")]
    pub ingress: Option<Ingress>,
    /// The pools to be created along with the cluster
    #[serde(rename = "pools", skip_serializing_if = "Option::is_none")]
    pub pools: Option<
        Vec<crate::models::ScalewayPeriodK8sPeriodV1PeriodCreateClusterRequestPeriodPoolConfig>,
    >,
    #[serde(rename = "autoscaler_config", skip_serializing_if = "Option::is_none")]
    pub autoscaler_config: Option<Box<crate::models::CreateClusterRequestAutoscalerConfig>>,
    #[serde(rename = "auto_upgrade", skip_serializing_if = "Option::is_none")]
    pub auto_upgrade: Option<Box<crate::models::CreateClusterRequestAutoUpgrade>>,
    /// List of feature gates to enable
    #[serde(rename = "feature_gates", skip_serializing_if = "Option::is_none")]
    pub feature_gates: Option<Vec<String>>,
    /// List of admission plugins to enable
    #[serde(rename = "admission_plugins", skip_serializing_if = "Option::is_none")]
    pub admission_plugins: Option<Vec<String>>,
    #[serde(
        rename = "open_id_connect_config",
        skip_serializing_if = "Option::is_none"
    )]
    pub open_id_connect_config: Option<Box<crate::models::CreateClusterRequestOpenIdConnectConfig>>,
    /// Additional Subject Alternative Names for the Kubernetes API server certificate
    #[serde(
        rename = "apiserver_cert_sans",
        skip_serializing_if = "Option::is_none"
    )]
    pub apiserver_cert_sans: Option<Vec<String>>,
}

impl CreateClusterRequest {
    pub fn new(
        project_id: String,
        name: String,
        version: String,
        cni: Cni,
    ) -> CreateClusterRequest {
        CreateClusterRequest {
            organization_id: None,
            project_id,
            r#type: None,
            name,
            description: None,
            tags: None,
            version,
            cni,
            enable_dashboard: None,
            ingress: None,
            pools: None,
            autoscaler_config: None,
            auto_upgrade: None,
            feature_gates: None,
            admission_plugins: None,
            open_id_connect_config: None,
            apiserver_cert_sans: None,
        }
    }
}

/// The Container Network Interface (CNI) plugin that will run in the cluster
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
/// The Ingress Controller that will run in the cluster
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
