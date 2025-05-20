# ScalewayPeriodK8sPeriodV1PeriodCreateClusterRequestPeriodPoolConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Name of the pool. | 
**node_type** | **String** | Node type is the type of Scaleway Instance wanted for the pool. Nodes with insufficient memory are not eligible (DEV1-S, PLAY2-PICO, STARDUST). 'external' is a special node type used to provision instances from other cloud providers in a Kosmos Cluster. | 
**placement_group_id** | Option<**String**> | Placement group ID in which all the nodes of the pool will be created, placement groups are limited to 20 instances. | [optional]
**autoscaling** | Option<**bool**> | Defines whether the autoscaling feature is enabled for the pool. | [optional]
**size** | **i32** | Size (number of nodes) of the pool. | 
**min_size** | Option<**i32**> | Defines the minimum size of the pool. Note that this field is only used when autoscaling is enabled on the pool. | [optional]
**max_size** | Option<**i32**> | Defines the maximum size of the pool. Note that this field is only used when autoscaling is enabled on the pool. | [optional]
**container_runtime** | Option<**String**> | Customization of the container runtime is available for each pool. | [optional][default to UnknownRuntime]
**autohealing** | Option<**bool**> | Defines whether the autohealing feature is enabled for the pool. | [optional]
**tags** | Option<**Vec<String>**> | Tags associated with the pool, see [managing tags](https://www.scaleway.com/en/docs/containers/kubernetes/api-cli/managing-tags). | [optional]
**kubelet_args** | Option<[**models::CreatePoolRequestKubeletArgs**](CreatePool_request_kubelet_args.md)> |  | [optional]
**upgrade_policy** | Option<[**models::ScalewayK8sV1CreateClusterRequestPoolConfigUpgradePolicy**](scaleway_k8s_v1_CreateClusterRequest_PoolConfig_upgrade_policy.md)> |  | [optional]
**zone** | Option<**String**> | Zone in which the pool's nodes will be spawned. | [optional]
**root_volume_type** | Option<**String**> | Defines the system volume disk type. Several types of volume (`volume_type`) are provided:. * `l_ssd` is a local block storage which means your system is stored locally on your node's hypervisor. This type is not available for all node types * `sbs-5k` is a remote block storage which means your system is stored on a centralized and resilient cluster with 5k IOPS limits * `sbs-15k` is a faster remote block storage which means your system is stored on a centralized and resilient cluster with 15k IOPS limits * `b_ssd` is the legacy remote block storage which means your system is stored on a centralized and resilient cluster. Consider using `sbs-5k` or `sbs-15k` instead. | [optional][default to DefaultVolumeType]
**root_volume_size** | Option<**i32**> | System volume disk size. (in bytes) | [optional]
**public_ip_disabled** | Option<**bool**> | Defines if the public IP should be removed from Nodes. To use this feature, your Cluster must have an attached Private Network set up with a Public Gateway. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


