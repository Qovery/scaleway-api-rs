# UpdateDnsZoneRecordsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**changes** | [**Vec<crate::models::ScalewayPeriodDomainPeriodV2beta1PeriodRecordChange>**](scaleway.domain.v2beta1.RecordChange.md) | The changes made to the records | 
**return_all_records** | Option<**bool**> | Whether or not to return all the records | [optional]
**disallow_new_zone_creation** | Option<**bool**> | Forbid the creation of the target zone if not existing (default action is yes) | [optional]
**serial** | Option<**i32**> | Don't use the autoincremenent serial but the provided one (0 to keep the same) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


