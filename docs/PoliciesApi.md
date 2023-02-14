# \PoliciesApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_policy**](PoliciesApi.md#create_policy) | **POST** /iam/v1alpha1/policies | Create a new policy
[**delete_policy**](PoliciesApi.md#delete_policy) | **DELETE** /iam/v1alpha1/policies/{policy_id} | Delete a policy
[**get_policy**](PoliciesApi.md#get_policy) | **GET** /iam/v1alpha1/policies/{policy_id} | Get an existing policy
[**list_policies**](PoliciesApi.md#list_policies) | **GET** /iam/v1alpha1/policies | List policies of an organization
[**update_policy**](PoliciesApi.md#update_policy) | **PATCH** /iam/v1alpha1/policies/{policy_id} | Update an existing policy



## create_policy

> crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodPolicy create_policy(create_policy_request)
Create a new policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_policy_request** | [**CreatePolicyRequest**](CreatePolicyRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodPolicy**](scaleway.iam.v1alpha1.Policy.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_policy

> delete_policy(policy_id)
Delete a policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **String** | Id of policy to delete | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_policy

> crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodPolicy get_policy(policy_id)
Get an existing policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **String** | Id of policy to search | [required] |

### Return type

[**crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodPolicy**](scaleway.iam.v1alpha1.Policy.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_policies

> crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodListPoliciesResponse list_policies(organization_id, order_by, page_size, page, editable, user_ids, group_ids, application_ids, no_principal, policy_name)
List policies of an organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | Option<**String**> | ID of organization to filter | [required] |
**order_by** | Option<**String**> | Criteria for sorting results |  |[default to created_at_asc]
**page_size** | Option<**i32**> | Number of results per page. Value must be between 1 and 100 |  |
**page** | Option<**i32**> | Number of page. Value must be greater to 1 |  |
**editable** | Option<**bool**> | Filter out editable policies or not |  |
**user_ids** | Option<[**Vec<String>**](String.md)> | Filter out by a list of user ID |  |
**group_ids** | Option<[**Vec<String>**](String.md)> | Filter out by a list of group ID |  |
**application_ids** | Option<[**Vec<String>**](String.md)> | Filter out by a list of application ID |  |
**no_principal** | Option<**bool**> | True when the policy do not belong to any principal |  |
**policy_name** | Option<**String**> | Name of policy to fetch |  |

### Return type

[**crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodListPoliciesResponse**](scaleway.iam.v1alpha1.ListPoliciesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_policy

> crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodPolicy update_policy(policy_id, update_policy_request)
Update an existing policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **String** | Id of policy to update | [required] |
**update_policy_request** | [**UpdatePolicyRequest**](UpdatePolicyRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodPolicy**](scaleway.iam.v1alpha1.Policy.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

