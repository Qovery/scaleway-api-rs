# CreateReadReplicaRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**instance_id** | **String** | UUID of the Database Instance you want to create a Read Replica from. (UUID format) | 
**endpoint_spec** | Option<[**Vec<models::ScalewayPeriodRdbPeriodV1PeriodReadReplicaEndpointSpec>**](scaleway.rdb.v1.ReadReplicaEndpointSpec.md)> | Specification of the endpoint you want to create. | [optional]
**same_zone** | Option<**bool**> | Defines whether to create the replica in the same availability zone as the main instance nodes or not. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


