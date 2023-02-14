# ScalewayPeriodK8sPeriodV1PeriodPool

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of the pool | [optional]
**cluster_id** | Option<**String**> | The cluster ID of the pool | [optional]
**created_at** | Option<**String**> | The date at which the pool was created (RFC 3339 format) | [optional]
**updated_at** | Option<**String**> | The date at which the pool was last updated (RFC 3339 format) | [optional]
**name** | Option<**String**> | The name of the pool | [optional]
**status** | Option<**String**> | The status of the pool | [optional][default to Unknown]
**version** | Option<**String**> | The version of the pool | [optional]
**node_type** | **String** | The node type is the type of Scaleway Instance wanted for the pool. Nodes with insufficient memory are not eligible (DEV1-S, PLAY2-PICO, STARDUST). 'external' is a special node type used to provision instances from other cloud providers. | 
**autoscaling** | Option<**bool**> | The enablement of the autoscaling feature for the pool | [optional]
**size** | **i32** | The size (number of nodes) of the pool | 
**min_size** | Option<**i32**> | The minimum size of the pool. Note that this field will be used only when autoscaling is enabled. | [optional]
**max_size** | Option<**i32**> | The maximum size of the pool. Note that this field will be used only when autoscaling is enabled. | [optional]
**container_runtime** | Option<**String**> | The customization of the container runtime is available for each pool. Note that `docker` is deprecated since 1.20 and will be removed in 1.24.  | [optional][default to UnknownRuntime]
**autohealing** | Option<**bool**> | The enablement of the autohealing feature for the pool | [optional]
**tags** | Option<**Vec<String>**> | The tags associated with the pool | [optional]
**placement_group_id** | Option<**String**> | The placement group ID in which all the nodes of the pool will be created | [optional]
**kubelet_args** | Option<[**crate::models::CreatePoolRequestKubeletArgs**](CreatePool_request_kubelet_args.md)> |  | [optional]
**upgrade_policy** | Option<[**crate::models::ScalewayK8sV1PoolUpgradePolicy**](scaleway_k8s_v1_Pool_upgrade_policy.md)> |  | [optional]
**zone** | Option<**String**> | The Zone in which the Pool's node will be spawn in | [optional]
**root_volume_type** | Option<**String**> | The system volume disk type, we provide two different types of volume (`volume_type`):   - `l_ssd` is a local block storage: your system is stored locally on     the hypervisor of your node.   - `b_ssd` is a remote block storage: your system is stored on a     centralised and resilient cluster.  | [optional][default to DefaultVolumeType]
**root_volume_size** | Option<**i32**> | The system volume disk size (in bytes) | [optional]
**region** | Option<**String**> | The cluster region of the pool | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


