# CreateClusterRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization_id** | Option<**String**> | The organization ID where the cluster will be created | [optional]
**project_id** | **String** | The project ID where the cluster will be created | 
**r#type** | Option<**String**> | The type of the cluster (possible values are kapsule, multicloud). | [optional]
**name** | **String** | The name of the cluster | 
**description** | Option<**String**> | The description of the cluster | [optional]
**tags** | Option<**Vec<String>**> | The tags associated with the cluster | [optional]
**version** | **String** | The Kubernetes version of the cluster | 
**cni** | **String** | The Container Network Interface (CNI) plugin that will run in the cluster | [default to UnknownCni]
**enable_dashboard** | Option<**bool**> | The enablement of the Kubernetes Dashboard in the cluster | [optional]
**ingress** | Option<**String**> | The Ingress Controller that will run in the cluster | [optional][default to UnknownIngress]
**pools** | Option<[**Vec<crate::models::ScalewayPeriodK8sPeriodV1PeriodCreateClusterRequestPeriodPoolConfig>**](scaleway.k8s.v1.CreateClusterRequest.PoolConfig.md)> | The pools to be created along with the cluster | [optional]
**autoscaler_config** | Option<[**crate::models::CreateClusterRequestAutoscalerConfig**](CreateCluster_request_autoscaler_config.md)> |  | [optional]
**auto_upgrade** | Option<[**crate::models::CreateClusterRequestAutoUpgrade**](CreateCluster_request_auto_upgrade.md)> |  | [optional]
**feature_gates** | Option<**Vec<String>**> | List of feature gates to enable | [optional]
**admission_plugins** | Option<**Vec<String>**> | List of admission plugins to enable | [optional]
**open_id_connect_config** | Option<[**crate::models::CreateClusterRequestOpenIdConnectConfig**](CreateCluster_request_open_id_connect_config.md)> |  | [optional]
**apiserver_cert_sans** | Option<**Vec<String>**> | Additional Subject Alternative Names for the Kubernetes API server certificate | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


