# UpdateDnsZoneRecordsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**changes** | [**Vec<models::ScalewayPeriodDomainPeriodV2beta1PeriodRecordChange>**](scaleway.domain.v2beta1.RecordChange.md) | Changes made to the records. | 
**return_all_records** | Option<**bool**> | Specifies whether or not to return all the records. | [optional]
**disallow_new_zone_creation** | Option<**bool**> | Disable the creation of the target zone if it does not exist. Target zone creation is disabled by default. | [optional]
**serial** | Option<**i32**> | Use the provided serial (0) instead of the auto-increment serial. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


