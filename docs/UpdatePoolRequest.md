# UpdatePoolRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**autoscaling** | Option<**bool**> | New value for the pool autoscaling enablement. | [optional]
**size** | Option<**i32**> | New desired pool size. | [optional]
**min_size** | Option<**i32**> | New minimum size for the pool. | [optional]
**max_size** | Option<**i32**> | New maximum size for the pool. | [optional]
**autohealing** | Option<**bool**> | New value for the pool autohealing enablement. | [optional]
**tags** | Option<**Vec<String>**> | New tags associated with the pool. | [optional]
**kubelet_args** | Option<[**models::UpdatePoolRequestKubeletArgs**](UpdatePool_request_kubelet_args.md)> |  | [optional]
**upgrade_policy** | Option<[**models::UpdatePoolRequestUpgradePolicy**](UpdatePool_request_upgrade_policy.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


