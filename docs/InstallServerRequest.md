# InstallServerRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**os_id** | **String** | ID of the OS to install on the server | 
**hostname** | **String** | Hostname of the server | 
**ssh_key_ids** | **Vec<String>** | SSH key IDs authorized on the server | 
**user** | Option<**String**> | User used for the installation | [optional]
**password** | Option<**String**> | Password used for the installation | [optional]
**service_user** | Option<**String**> | User used for the service to install | [optional]
**service_password** | Option<**String**> | Password used for the service to install | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


