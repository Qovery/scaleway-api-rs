# UpdateClusterRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | This field allows to update the external name of the cluster. The internal name (used for instance in hostname) won't change. | [optional]
**description** | Option<**String**> | The new description of the cluster | [optional]
**tags** | Option<**Vec<String>**> | The new tags associated with the cluster | [optional]
**autoscaler_config** | Option<[**crate::models::UpdateClusterRequestAutoscalerConfig**](UpdateCluster_request_autoscaler_config.md)> |  | [optional]
**enable_dashboard** | Option<**bool**> | The new value of the Kubernetes Dashboard enablement | [optional]
**ingress** | Option<**String**> | The new Ingress Controller for the cluster | [optional][default to UnknownIngress]
**auto_upgrade** | Option<[**crate::models::UpdateClusterRequestAutoUpgrade**](UpdateCluster_request_auto_upgrade.md)> |  | [optional]
**feature_gates** | Option<**Vec<String>**> | List of feature gates to enable | [optional]
**admission_plugins** | Option<**Vec<String>**> | List of admission plugins to enable | [optional]
**open_id_connect_config** | Option<[**crate::models::UpdateClusterRequestOpenIdConnectConfig**](UpdateCluster_request_open_id_connect_config.md)> |  | [optional]
**apiserver_cert_sans** | Option<**Vec<String>**> | Additional Subject Alternative Names for the Kubernetes API server certificate | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


