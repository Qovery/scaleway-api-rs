# ScalewayPeriodK8sPeriodV1PeriodNode

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Node ID. | [optional]
**pool_id** | Option<**String**> | Pool ID of the node. | [optional]
**cluster_id** | Option<**String**> | Cluster ID of the node. | [optional]
**provider_id** | Option<**String**> | Underlying instance ID. It is prefixed by instance type and location information (see https://pkg.go.dev/k8s.io/api/core/v1#NodeSpec.ProviderID). | [optional]
**region** | Option<**String**> | Cluster region of the node. | [optional]
**name** | Option<**String**> | Name of the node. | [optional]
**public_ip_v4** | Option<**String**> | Public IPv4 address of the node. (IPv4 address) | [optional]
**public_ip_v6** | Option<**String**> | Public IPv6 address of the node. (IPv6 address) | [optional]
**conditions** | Option<[**models::ScalewayK8sV1NodeConditions**](scaleway_k8s_v1_Node_conditions.md)> |  | [optional]
**status** | Option<**String**> | Status of the node. | [optional][default to Unknown]
**error_message** | Option<**String**> | Details of the error, if any occurred when managing the node. | [optional]
**created_at** | Option<**String**> | Date on which the node was created. (RFC 3339 format) | [optional]
**updated_at** | Option<**String**> | Date on which the node was last updated. (RFC 3339 format) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


