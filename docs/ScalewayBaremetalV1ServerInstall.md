# ScalewayBaremetalV1ServerInstall

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**os_id** | Option<**String**> | ID of the OS. | [optional]
**hostname** | Option<**String**> | Host defined during the server installation. | [optional]
**ssh_key_ids** | Option<**Vec<String>**> | SSH public key IDs defined during server installation. | [optional]
**status** | Option<**String**> | Status of the server installation. | [optional][default to Unknown]
**user** | Option<**String**> | User defined in the server installation, or the default user if none were specified. | [optional]
**service_user** | Option<**String**> | Service user defined in the server installation, or the default user if none were specified. | [optional]
**service_url** | Option<**String**> | Address of the installed service. | [optional]
**partitioning_schema** | Option<[**models::ValidatePartitioningSchemaRequestPartitioningSchema**](ValidatePartitioningSchema_request_partitioning_schema.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


