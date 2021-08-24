# ScalewayRdbV1NodeType

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Node Type name identifier | [optional]
**stock_status** | Option<**String**> | Current stock status for the Node Type | [optional][default to StockStatus_Unknown]
**description** | Option<**String**> | Current specs of the offer | [optional]
**vcpus** | Option<**f32**> | Number of virtual CPUs | [optional]
**memory** | Option<**f32**> | Quantity of RAM (in bytes) | [optional]
**volume_constraint** | Option<[**crate::models::ScalewayRdbV1NodeTypeVolumeConstraint**](scaleway_rdb_v1_NodeType_volume_constraint.md)> |  | [optional]
**is_bssd_compatible** | Option<**bool**> | The Node Type is compliant with Block Storage | [optional]
**disabled** | Option<**bool**> | The Node Type is currently disabled | [optional]
**beta** | Option<**bool**> | The Node Type is currently in beta | [optional]
**available_volume_types** | Option<[**Vec<crate::models::ScalewayRdbV1NodeTypeVolumeType>**](scaleway.rdb.v1.NodeType.VolumeType.md)> | Available storage options for the Node Type | [optional]
**region** | Option<**String**> | Region the Node Type is in | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


