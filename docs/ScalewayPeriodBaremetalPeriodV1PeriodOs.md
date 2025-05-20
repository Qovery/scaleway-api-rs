# ScalewayPeriodBaremetalPeriodV1PeriodOs

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | ID of the OS. | [optional]
**name** | Option<**String**> | Name of the OS. | [optional]
**version** | Option<**String**> | Version of the OS. | [optional]
**logo_url** | Option<**String**> | URL of this OS's logo. | [optional]
**ssh** | Option<[**models::ScalewayBaremetalV1OsSsh**](scaleway_baremetal_v1_OS_ssh.md)> |  | [optional]
**user** | Option<[**models::ScalewayBaremetalV1OsUser**](scaleway_baremetal_v1_OS_user.md)> |  | [optional]
**password** | Option<[**models::ScalewayBaremetalV1OsPassword**](scaleway_baremetal_v1_OS_password.md)> |  | [optional]
**service_user** | Option<[**models::ScalewayBaremetalV1OsServiceUser**](scaleway_baremetal_v1_OS_service_user.md)> |  | [optional]
**service_password** | Option<[**models::ScalewayBaremetalV1OsServicePassword**](scaleway_baremetal_v1_OS_service_password.md)> |  | [optional]
**enabled** | Option<**bool**> | Defines if the operating system is enabled or not. | [optional]
**license_required** | Option<**bool**> | License required (check server options for pricing details). | [optional]
**allowed** | Option<**bool**> | Defines if a specific Organization is allowed to install this OS type. | [optional]
**custom_partitioning_supported** | Option<**bool**> | Defines if custom partitioning is supported by this OS. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


