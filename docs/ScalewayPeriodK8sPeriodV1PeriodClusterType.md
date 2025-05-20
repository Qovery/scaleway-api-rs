# ScalewayPeriodK8sPeriodV1PeriodClusterType

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Cluster type name. | [optional]
**availability** | Option<**String**> | Cluster type availability. | [optional][default to Available]
**max_nodes** | Option<**i32**> | Maximum number of nodes supported by the offer. | [optional]
**commitment_delay** | Option<**String**> | Time period during which you can no longer switch to a lower offer. (in seconds) | [optional]
**sla** | Option<**f32**> | Value of the Service Level Agreement of the offer. | [optional]
**resiliency** | Option<**String**> | Resiliency offered by the offer. | [optional][default to UnknownResiliency]
**memory** | Option<**i32**> | Max RAM allowed for the control plane. (in bytes) | [optional]
**dedicated** | Option<**bool**> | Returns information if this offer uses dedicated resources. | [optional]
**audit_logs_supported** | Option<**bool**> | True if the offer allows activation of the audit log functionality. Please note that audit logs are sent to Cockpit. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


