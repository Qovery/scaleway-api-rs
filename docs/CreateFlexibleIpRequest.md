# CreateFlexibleIpRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**project_id** | **String** | ID of the project to associate with the Flexible IP. | 
**description** | Option<**String**> | Flexible IP description (max. of 255 characters). | [optional]
**tags** | Option<**Vec<String>**> | Tags to associate to the flexible IP. | [optional]
**server_id** | Option<**String**> | ID of the server to which the newly created flexible IP will be attached. | [optional]
**reverse** | Option<**String**> | Value of the reverse DNS. | [optional]
**is_ipv6** | Option<**bool**> | Defines whether the flexible IP has an IPv6 address. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


