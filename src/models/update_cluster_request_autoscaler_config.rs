/*
 * Account API
 *
 * # Introduction  The Account API allows you to manage projects. Project is Scaleway’s resource management feature. Designed to help you organize your infrastructure and cloud services, the feature allows resources to be isolated and grouped into specific projects.
 *
 * The version of the OpenAPI document: v2
 *
 * Generated by: https://openapi-generator.tech
 */

/// UpdateClusterRequestAutoscalerConfig : This field allows to update some configuration for the autoscaler, which is an implementation of the [cluster-autoscaler](https://github.com/kubernetes/autoscaler/tree/master/cluster-autoscaler/).

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateClusterRequestAutoscalerConfig {
    /// Disable the cluster autoscaler
    #[serde(
        rename = "scale_down_disabled",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub scale_down_disabled: Option<Option<bool>>,
    /// How long after scale up that scale down evaluation resumes
    #[serde(
        rename = "scale_down_delay_after_add",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub scale_down_delay_after_add: Option<Option<String>>,
    /// Type of resource estimator to be used in scale up
    #[serde(rename = "estimator", skip_serializing_if = "Option::is_none")]
    pub estimator: Option<Estimator>,
    /// Type of node group expander to be used in scale up
    #[serde(rename = "expander", skip_serializing_if = "Option::is_none")]
    pub expander: Option<Expander>,
    /// Ignore DaemonSet pods when calculating resource utilization for scaling down
    #[serde(
        rename = "ignore_daemonsets_utilization",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub ignore_daemonsets_utilization: Option<Option<bool>>,
    /// Detect similar node groups and balance the number of nodes between them
    #[serde(
        rename = "balance_similar_node_groups",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub balance_similar_node_groups: Option<Option<bool>>,
    /// Pods with priority below cutoff will be expendable. They can be killed without any consideration during scale down and they don't cause scale up. Pods with null priority (PodPriority disabled) are non expendable.
    #[serde(
        rename = "expendable_pods_priority_cutoff",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub expendable_pods_priority_cutoff: Option<Option<i32>>,
    /// How long a node should be unneeded before it is eligible for scale down
    #[serde(
        rename = "scale_down_unneeded_time",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub scale_down_unneeded_time: Option<Option<String>>,
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
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_graceful_termination_sec: Option<Option<i32>>,
}

impl UpdateClusterRequestAutoscalerConfig {
    /// This field allows to update some configuration for the autoscaler, which is an implementation of the [cluster-autoscaler](https://github.com/kubernetes/autoscaler/tree/master/cluster-autoscaler/).
    pub fn new() -> UpdateClusterRequestAutoscalerConfig {
        UpdateClusterRequestAutoscalerConfig {
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
