# UpdateClusterRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | New external name for the cluster. | [optional]
**description** | Option<**String**> | New description for the cluster. | [optional]
**tags** | Option<**Vec<String>**> | New tags associated with the cluster. | [optional]
**autoscaler_config** | Option<[**models::UpdateClusterRequestAutoscalerConfig**](UpdateCluster_request_autoscaler_config.md)> |  | [optional]
**auto_upgrade** | Option<[**models::UpdateClusterRequestAutoUpgrade**](UpdateCluster_request_auto_upgrade.md)> |  | [optional]
**feature_gates** | Option<**Vec<String>**> | List of feature gates to enable. | [optional]
**admission_plugins** | Option<**Vec<String>**> | List of admission plugins to enable. | [optional]
**open_id_connect_config** | Option<[**models::UpdateClusterRequestOpenIdConnectConfig**](UpdateCluster_request_open_id_connect_config.md)> |  | [optional]
**apiserver_cert_sans** | Option<**Vec<String>**> | Additional Subject Alternative Names for the Kubernetes API server certificate. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


