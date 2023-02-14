# \RulesApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_rules**](RulesApi.md#list_rules) | **GET** /iam/v1alpha1/rules | List rules of an existing policy
[**set_rules**](RulesApi.md#set_rules) | **PUT** /iam/v1alpha1/rules | Set rules of an existing policy



## list_rules

> crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodListRulesResponse list_rules(policy_id, page_size, page)
List rules of an existing policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | Option<**String**> | Id of policy to search | [required] |
**page_size** | Option<**i32**> | Number of results per page. Value must be between 1 and 100 |  |
**page** | Option<**i32**> | Number of page. Value must be greater to 1 |  |

### Return type

[**crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodListRulesResponse**](scaleway.iam.v1alpha1.ListRulesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_rules

> crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodSetRulesResponse set_rules(set_rules_request)
Set rules of an existing policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**set_rules_request** | [**SetRulesRequest**](SetRulesRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodSetRulesResponse**](scaleway.iam.v1alpha1.SetRulesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

