# UpgradeInstanceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**node_type** | Option<**String**> | Node type of the Database Instance you want to upgrade to. | [optional]
**enable_ha** | Option<**bool**> | Defines whether or not high availability should be enabled on the Database Instance. | [optional]
**volume_size** | Option<**i32**> | Increase your block storage volume size. | [optional]
**volume_type** | Option<**String**> | Change your Database Instance storage type. | [optional][default to Lssd]
**upgradable_version_id** | Option<**String**> | Update your database engine to a newer version. This will create a new Database Instance with same specifications as the current one and perform a Database Engine upgrade. (UUID format) | [optional]
**major_upgrade_workflow** | Option<[**models::UpgradeInstanceRequestMajorUpgradeWorkflow**](UpgradeInstance_request_major_upgrade_workflow.md)> |  | [optional]
**enable_encryption** | Option<**bool**> | Defines whether or not encryption should be enabled on the Database Instance. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


