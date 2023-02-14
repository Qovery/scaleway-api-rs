# ScalewayPeriodIamPeriodV1alpha1PeriodUser

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | ID of user | [optional]
**email** | Option<**String**> | Email of user | [optional]
**created_at** | Option<**String**> | Creation date of user (RFC 3339 format) | [optional]
**updated_at** | Option<**String**> | Last update date of user (RFC 3339 format) | [optional]
**organization_id** | Option<**String**> | ID of organization | [optional]
**deletable** | Option<**bool**> | Deletion status of user. Owner user cannot be deleted | [optional]
**last_login_at** | Option<**String**> | Last login date (RFC 3339 format) | [optional]
**r#type** | Option<**String**> | Type of the user | [optional][default to UnknownType]
**two_factor_enabled** | Option<**bool**> | 2FA enabled | [optional]
**status** | Option<**String**> | Status of invitation for the user | [optional][default to UnknownStatus]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


