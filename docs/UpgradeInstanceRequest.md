# UpgradeInstanceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**node_type** | Option<**String**> | Node type of the instance you want to upgrade to | [optional]
**enable_ha** | Option<**bool**> | Set to true to enable high availability on your instance | [optional]
**volume_size** | Option<**i32**> | Increase your block storage volume size | [optional]
**volume_type** | Option<**String**> | Change your instance storage type | [optional][default to Lssd]
**upgradable_version_id** | Option<**String**> | This will create a new Database Instance with same instance specification as the current one and perform a Database Engine upgrade. (UUID format) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


