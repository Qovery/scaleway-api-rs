# ScalewayPeriodK8sPeriodV1PeriodCluster

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of the cluster | [optional]
**r#type** | Option<**String**> | The type of the cluster | [optional]
**name** | Option<**String**> | The name of the cluster | [optional]
**status** | Option<**String**> | The status of the cluster | [optional][default to Unknown]
**version** | Option<**String**> | The Kubernetes version of the cluster | [optional]
**region** | Option<**String**> | The region in which the cluster is | [optional]
**organization_id** | Option<**String**> | The ID of the organization owning the cluster | [optional]
**project_id** | Option<**String**> | The ID of the project owning the cluster | [optional]
**tags** | Option<**Vec<String>**> | The tags associated with the cluster | [optional]
**cni** | Option<**String**> | The Container Network Interface (CNI) plugin running in the cluster | [optional][default to UnknownCni]
**description** | Option<**String**> | The description of the cluster | [optional]
**cluster_url** | Option<**String**> | The Kubernetes API server URL of the cluster | [optional]
**dns_wildcard** | Option<**String**> | The DNS wildcard resovling all the ready nodes of the cluster | [optional]
**created_at** | Option<**String**> | The date at which the cluster was created (RFC 3339 format) | [optional]
**updated_at** | Option<**String**> | The date at which the cluster was last updated (RFC 3339 format) | [optional]
**autoscaler_config** | Option<[**crate::models::ScalewayK8sV1ClusterAutoscalerConfig**](scaleway_k8s_v1_Cluster_autoscaler_config.md)> |  | [optional]
**dashboard_enabled** | Option<**bool**> | The enablement of the Kubernetes Dashboard in the cluster | [optional]
**ingress** | Option<**String**> | The ingress controller used in the cluster | [optional][default to UnknownIngress]
**auto_upgrade** | Option<[**crate::models::ScalewayK8sV1ClusterAutoUpgrade**](scaleway_k8s_v1_Cluster_auto_upgrade.md)> |  | [optional]
**upgrade_available** | Option<**bool**> | True if a new Kubernetes version is available | [optional]
**feature_gates** | Option<**Vec<String>**> | List of enabled feature gates | [optional]
**admission_plugins** | Option<**Vec<String>**> | List of enabled admission plugins | [optional]
**open_id_connect_config** | Option<[**crate::models::ScalewayK8sV1ClusterOpenIdConnectConfig**](scaleway_k8s_v1_Cluster_open_id_connect_config.md)> |  | [optional]
**apiserver_cert_sans** | Option<**Vec<String>**> | Additional Subject Alternative Names for the Kubernetes API server certificate | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


