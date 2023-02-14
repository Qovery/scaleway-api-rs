# UpdatePoolRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**autoscaling** | Option<**bool**> | The new value for the enablement of autoscaling for the pool | [optional]
**size** | Option<**i32**> | The new size for the pool | [optional]
**min_size** | Option<**i32**> | The new minimun size for the pool | [optional]
**max_size** | Option<**i32**> | The new maximum size for the pool | [optional]
**autohealing** | Option<**bool**> | The new value for the enablement of autohealing for the pool | [optional]
**tags** | Option<**Vec<String>**> | The new tags associated with the pool | [optional]
**kubelet_args** | Option<[**crate::models::UpdatePoolRequestKubeletArgs**](UpdatePool_request_kubelet_args.md)> |  | [optional]
**upgrade_policy** | Option<[**crate::models::CreatePoolRequestUpgradePolicy**](CreatePool_request_upgrade_policy.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


