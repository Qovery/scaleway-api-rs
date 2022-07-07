# CreatePoolRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the pool | 
**node_type** | **String** | The node type is the type of Scaleway Instance wanted for the pool | 
**placement_group_id** | Option<**String**> | The placement group ID in which all the nodes of the pool will be created | [optional]
**autoscaling** | Option<**bool**> | The enablement of the autoscaling feature for the pool | [optional]
**size** | **i64** | The size (number of nodes) of the pool | 
**min_size** | Option<**i64**> | The minimun size of the pool. Note that this fields will be used only when autoscaling is enabled. | [optional]
**max_size** | Option<**i64**> | The maximum size of the pool. Note that this fields will be used only when autoscaling is enabled. | [optional]
**container_runtime** | Option<**String**> | The customization of the container runtime is available for each pool. Note that `docker` is the only supporter runtime at the moment. Others are to be considered experimental.  | [optional][default to ContainerRuntime_UnknownRuntime]
**autohealing** | Option<**bool**> | The enablement of the autohealing feature for the pool | [optional]
**tags** | Option<**Vec<String>**> | The tags associated with the pool | [optional]
**kubelet_args** | Option<[**crate::models::CreatePoolRequestKubeletArgs**](CreatePool_request_kubelet_args.md)> |  | [optional]
**upgrade_policy** | Option<[**crate::models::CreatePoolRequestUpgradePolicy**](CreatePool_request_upgrade_policy.md)> |  | [optional]
**zone** | Option<**String**> | The Zone in which the Pool's node will be spawn in | [optional]
**root_volume_type** | Option<**String**> | The system volume disk type, we provide two different types of volume (`volume_type`):   - `l_ssd` is a local block storage: your system is stored locally on     the hypervisor of your node.   - `b_ssd` is a remote block storage: your system is stored on a     centralised and resilant cluster.  | [optional][default to RootVolumeType_DefaultVolumeType]
**root_volume_size** | Option<**i64**> | The system volume disk size (in bytes) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


