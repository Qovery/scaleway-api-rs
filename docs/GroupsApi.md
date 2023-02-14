# \GroupsApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_group_member**](GroupsApi.md#add_group_member) | **POST** /iam/v1alpha1/groups/{group_id}/add-member | Add a user of an application to a group
[**create_group**](GroupsApi.md#create_group) | **POST** /iam/v1alpha1/groups | Create a new group
[**delete_group**](GroupsApi.md#delete_group) | **DELETE** /iam/v1alpha1/groups/{group_id} | Delete a group
[**get_group**](GroupsApi.md#get_group) | **GET** /iam/v1alpha1/groups/{group_id} | Get a group
[**list_groups**](GroupsApi.md#list_groups) | **GET** /iam/v1alpha1/groups | List groups
[**remove_group_member**](GroupsApi.md#remove_group_member) | **POST** /iam/v1alpha1/groups/{group_id}/remove-member | Remove a user or an application from a group
[**set_group_members**](GroupsApi.md#set_group_members) | **PUT** /iam/v1alpha1/groups/{group_id}/members | Overwrite users and applications of a group
[**update_group**](GroupsApi.md#update_group) | **PATCH** /iam/v1alpha1/groups/{group_id} | Update a group



## add_group_member

> crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodGroup add_group_member(group_id, add_group_member_request)
Add a user of an application to a group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of group | [required] |
**add_group_member_request** | [**AddGroupMemberRequest**](AddGroupMemberRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodGroup**](scaleway.iam.v1alpha1.Group.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_group

> crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodGroup create_group(create_group_request)
Create a new group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_group_request** | [**CreateGroupRequest**](CreateGroupRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodGroup**](scaleway.iam.v1alpha1.Group.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_group

> delete_group(group_id)
Delete a group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of group to delete | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group

> crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodGroup get_group(group_id)
Get a group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of group | [required] |

### Return type

[**crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodGroup**](scaleway.iam.v1alpha1.Group.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_groups

> crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodListGroupsResponse list_groups(order_by, page, page_size, organization_id, name, application_ids, user_ids, group_ids)
List groups

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_by** | Option<**String**> | Sort order of groups |  |[default to created_at_asc]
**page** | Option<**i32**> | Requested page number. Value must be greater or equals to 1 |  |
**page_size** | Option<**i32**> | Number of items per page. Value must be between 1 and 100 |  |
**organization_id** | Option<**String**> | Filter by organization ID |  |
**name** | Option<**String**> | Name of group to find |  |
**application_ids** | Option<[**Vec<String>**](String.md)> | Filter out by a list of application ID |  |
**user_ids** | Option<[**Vec<String>**](String.md)> | Filter out by a list of user ID |  |
**group_ids** | Option<[**Vec<String>**](String.md)> | Filter out by a list of group ID |  |

### Return type

[**crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodListGroupsResponse**](scaleway.iam.v1alpha1.ListGroupsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_group_member

> crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodGroup remove_group_member(group_id, remove_group_member_request)
Remove a user or an application from a group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of group | [required] |
**remove_group_member_request** | [**RemoveGroupMemberRequest**](RemoveGroupMemberRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodGroup**](scaleway.iam.v1alpha1.Group.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_group_members

> crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodGroup set_group_members(group_id, set_group_members_request)
Overwrite users and applications of a group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** |  | [required] |
**set_group_members_request** | [**SetGroupMembersRequest**](SetGroupMembersRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodGroup**](scaleway.iam.v1alpha1.Group.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_group

> crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodGroup update_group(group_id, update_group_request)
Update a group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of group to update | [required] |
**update_group_request** | [**UpdateGroupRequest**](UpdateGroupRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodGroup**](scaleway.iam.v1alpha1.Group.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

