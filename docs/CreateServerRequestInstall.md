# CreateServerRequestInstall

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**os_id** | Option<**String**> | ID of the OS to installation on the server. | [optional]
**hostname** | Option<**String**> | Hostname of the server. | [optional]
**ssh_key_ids** | Option<**Vec<String>**> | SSH key IDs authorized on the server. | [optional]
**user** | Option<**String**> | User for the installation. | [optional]
**password** | Option<**String**> | Password for the installation. | [optional]
**service_user** | Option<**String**> | Regular user that runs the service to be installed on the server. | [optional]
**service_password** | Option<**String**> | Password used for the service to install. | [optional]
**partitioning_schema** | Option<[**models::ValidatePartitioningSchemaRequestPartitioningSchema**](ValidatePartitioningSchema_request_partitioning_schema.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


