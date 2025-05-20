# ScalewayPeriodRdbPeriodV1PeriodReadReplica

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | UUID of the Read Replica. (UUID format) | [optional]
**endpoints** | Option<[**Vec<models::ScalewayPeriodRdbPeriodV1PeriodEndpoint>**](scaleway.rdb.v1.Endpoint.md)> | Display Read Replica connection information. | [optional]
**status** | Option<**String**> | Read replica status. | [optional][default to Unknown]
**region** | Option<**String**> | Region the Read Replica is in. | [optional]
**same_zone** | Option<**bool**> | Whether the replica is in the same availability zone as the main instance nodes or not. | [optional]
**instance_id** | Option<**String**> | UUID of the Database Instance. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


