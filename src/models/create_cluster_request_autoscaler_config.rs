/*
 * Elastic metal API
 *
 * # Introduction  Elastic metal as a service allows ordering a dedicated server on-demand like a cloud instance. Dedicated servers could be used for large workloads, big data, those requiring more security, ….  ## Technical Limitations  - Elastic metal is available in `fr-par-1`,  `fr-par-2`, `nl-ams-1` zones  - Installation is done by preseed (± 10min) (preseed: complete install from a virtual media)  ## Features  - Install (Server is installed by preseed (preseed: complete install from a virtual media), you must define at least one ssh key to install your server)  - Start/Stop/Reboot  - Rescue Reboot, a rescue image is an operating system image designed to help you diagnose and fix an OS experiencing failures. When your server boot on rescue, you can mount your disks and start diagnosing/fixing your image.  - Billed by minute (The billing start when the server is delivered and stop when the server is deleted)  - IPv6, all servers are available with an IPv6 /128  - ReverseIP, You can configure your reverse IP (IPv4 and IPv6), you must register the server IP in your DNS records before calling the endpoint  - Basic monitoring with ping status  - Flexible IP is available ([documentation](https://developers.scaleway.com/en/products/flexible-ip/api/))  - IP failovers are not available in api v1, use the api v1alpha1  ## FAQ  ### How can I get my SSH key id?  You can find your `$SCW_SECRET_KEY` and your `SCW_DEFAULT_ORGANIZATION_ID` at the following page: https://console.scaleway.com/project/credentials  ### How can I add my server to a private network?  See [our online documentation](https://developers.scaleway.com/en/products/vpc-elasticmetal/api/).
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreateClusterRequestAutoscalerConfig : This field allows to specify some configuration for the autoscaler, which is an implementation of the [cluster-autoscaler](https://github.com/kubernetes/autoscaler/tree/master/cluster-autoscaler/).

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CreateClusterRequestAutoscalerConfig {
    /// Disable the cluster autoscaler
    #[serde(
        rename = "scale_down_disabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub scale_down_disabled: Option<bool>,
    /// How long after scale up that scale down evaluation resumes
    #[serde(
        rename = "scale_down_delay_after_add",
        skip_serializing_if = "Option::is_none"
    )]
    pub scale_down_delay_after_add: Option<String>,
    /// Type of resource estimator to be used in scale up
    #[serde(rename = "estimator", skip_serializing_if = "Option::is_none")]
    pub estimator: Option<Estimator>,
    /// Type of node group expander to be used in scale up
    #[serde(rename = "expander", skip_serializing_if = "Option::is_none")]
    pub expander: Option<Expander>,
    /// Ignore DaemonSet pods when calculating resource utilization for scaling down
    #[serde(
        rename = "ignore_daemonsets_utilization",
        skip_serializing_if = "Option::is_none"
    )]
    pub ignore_daemonsets_utilization: Option<bool>,
    /// Detect similar node groups and balance the number of nodes between them
    #[serde(
        rename = "balance_similar_node_groups",
        skip_serializing_if = "Option::is_none"
    )]
    pub balance_similar_node_groups: Option<bool>,
    /// Pods with priority below cutoff will be expendable. They can be killed without any consideration during scale down and they don't cause scale up. Pods with null priority (PodPriority disabled) are non expendable.
    #[serde(
        rename = "expendable_pods_priority_cutoff",
        skip_serializing_if = "Option::is_none"
    )]
    pub expendable_pods_priority_cutoff: Option<i64>,
    /// How long a node should be unneeded before it is eligible for scale down
    #[serde(
        rename = "scale_down_unneeded_time",
        skip_serializing_if = "Option::is_none"
    )]
    pub scale_down_unneeded_time: Option<String>,
    #[serde(
        rename = "scale_down_utilization_threshold",
        skip_serializing_if = "Option::is_none"
    )]
    pub scale_down_utilization_threshold: Option<
        Box<crate::models::CreateClusterRequestAutoscalerConfigScaleDownUtilizationThreshold>,
    >,
    /// Maximum number of seconds the cluster autoscaler waits for pod termination when trying to scale down a node
    #[serde(
        rename = "max_graceful_termination_sec",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_graceful_termination_sec: Option<i64>,
}

impl CreateClusterRequestAutoscalerConfig {
    /// This field allows to specify some configuration for the autoscaler, which is an implementation of the [cluster-autoscaler](https://github.com/kubernetes/autoscaler/tree/master/cluster-autoscaler/).
    pub fn new() -> CreateClusterRequestAutoscalerConfig {
        CreateClusterRequestAutoscalerConfig {
            scale_down_disabled: None,
            scale_down_delay_after_add: None,
            estimator: None,
            expander: None,
            ignore_daemonsets_utilization: None,
            balance_similar_node_groups: None,
            expendable_pods_priority_cutoff: None,
            scale_down_unneeded_time: None,
            scale_down_utilization_threshold: None,
            max_graceful_termination_sec: None,
        }
    }
}

/// Type of resource estimator to be used in scale up
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Estimator {
    #[serde(rename = "unknown_estimator")]
    UnknownEstimator,
    #[serde(rename = "binpacking")]
    Binpacking,
}

impl Default for Estimator {
    fn default() -> Estimator {
        Self::UnknownEstimator
    }
}
/// Type of node group expander to be used in scale up
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Expander {
    #[serde(rename = "unknown_expander")]
    UnknownExpander,
    #[serde(rename = "random")]
    Random,
    #[serde(rename = "most_pods")]
    MostPods,
    #[serde(rename = "least_waste")]
    LeastWaste,
    #[serde(rename = "priority")]
    Priority,
    #[serde(rename = "price")]
    Price,
}

impl Default for Expander {
    fn default() -> Expander {
        Self::UnknownExpander
    }
}
