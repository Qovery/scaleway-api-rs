# CreateClusterRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization_id** | Option<**String**> | Organization ID in which the cluster will be created. | [optional]
**project_id** | Option<**String**> | Project ID in which the cluster will be created. | [optional]
**r#type** | Option<**String**> | Type of the cluster. See [list available cluster types](#list-available-cluster-types-for-a-cluster) for a list of valid types. | [optional]
**name** | **String** | Cluster name. | 
**description** | Option<**String**> | Cluster description. | [optional]
**tags** | Option<**Vec<String>**> | Tags associated with the cluster. | [optional]
**version** | **String** | Kubernetes version of the cluster. | 
**cni** | **String** | Container Network Interface (CNI) plugin running in the cluster. | [default to UnknownCni]
**pools** | Option<[**Vec<models::ScalewayPeriodK8sPeriodV1PeriodCreateClusterRequestPeriodPoolConfig>**](scaleway.k8s.v1.CreateClusterRequest.PoolConfig.md)> | Pools created along with the cluster. | [optional]
**autoscaler_config** | Option<[**models::CreateClusterRequestAutoscalerConfig**](CreateCluster_request_autoscaler_config.md)> |  | [optional]
**auto_upgrade** | Option<[**models::CreateClusterRequestAutoUpgrade**](CreateCluster_request_auto_upgrade.md)> |  | [optional]
**feature_gates** | Option<**Vec<String>**> | List of feature gates to enable. | [optional]
**admission_plugins** | Option<**Vec<String>**> | List of admission plugins to enable. | [optional]
**open_id_connect_config** | Option<[**models::CreateClusterRequestOpenIdConnectConfig**](CreateCluster_request_open_id_connect_config.md)> |  | [optional]
**apiserver_cert_sans** | Option<**Vec<String>**> | Additional Subject Alternative Names for the Kubernetes API server certificate. | [optional]
**private_network_id** | Option<**String**> | Private network ID for internal cluster communication (cannot be changed later). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


