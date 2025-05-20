# ScalewayPeriodRdbPeriodV1PeriodMaintenance

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**starts_at** | Option<**String**> | Start date of the maintenance window. (RFC 3339 format) | [optional]
**stops_at** | Option<**String**> | End date of the maintenance window. (RFC 3339 format) | [optional]
**closed_at** | Option<**String**> | Closed maintenance date. (RFC 3339 format) | [optional]
**reason** | Option<**String**> | Maintenance information message. | [optional]
**status** | Option<**String**> | Status of the maintenance. | [optional][default to Unknown]
**forced_at** | Option<**String**> | Time when Scaleway-side maintenance will be applied. (RFC 3339 format) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


