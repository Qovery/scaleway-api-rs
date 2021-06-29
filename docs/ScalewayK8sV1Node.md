# ScalewayK8sV1Node

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of the node | [optional]
**pool_id** | Option<**String**> | The pool ID of the node | [optional]
**cluster_id** | Option<**String**> | The cluster ID of the node | [optional]
**region** | Option<**String**> | The cluster region of the node | [optional]
**name** | Option<**String**> | The name of the node | [optional]
**public_ip_v4** | Option<**String**> | The public IPv4 address of the node (IPv4 address) | [optional]
**public_ip_v6** | Option<**String**> | The public IPv6 address of the node (IPv6 address) | [optional]
**conditions** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | These conditions contains the Node Problem Detector conditions, as well as some in house conditions. | [optional]
**status** | Option<**String**> | The status of the node | [optional][default to Status_Unknown]
**created_at** | Option<**String**> | The date at which the node was created | [optional]
**updated_at** | Option<**String**> | The date at which the node was last updated | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


