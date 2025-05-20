# \AclsApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_instance_acl_rules**](AclsApi.md#add_instance_acl_rules) | **POST** /rdb/v1/regions/{region}/instances/{instance_id}/acls | Add an ACL rule to a Database Instance
[**delete_instance_acl_rules**](AclsApi.md#delete_instance_acl_rules) | **DELETE** /rdb/v1/regions/{region}/instances/{instance_id}/acls | Delete ACL rules of a Database Instance
[**list_instance_acl_rules**](AclsApi.md#list_instance_acl_rules) | **GET** /rdb/v1/regions/{region}/instances/{instance_id}/acls | List ACL rules of a Database Instance
[**set_instance_acl_rules**](AclsApi.md#set_instance_acl_rules) | **PUT** /rdb/v1/regions/{region}/instances/{instance_id}/acls | Set ACL rules for a Database Instance



## add_instance_acl_rules

> models::ScalewayPeriodRdbPeriodV1PeriodAddInstanceAclRulesResponse add_instance_acl_rules(region, instance_id, add_instance_acl_rules_request)
Add an ACL rule to a Database Instance

Add an additional ACL rule to a Database Instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the Database Instance you want to add ACL rules to. | [required] |
**add_instance_acl_rules_request** | [**AddInstanceAclRulesRequest**](AddInstanceAclRulesRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodRdbPeriodV1PeriodAddInstanceAclRulesResponse**](scaleway.rdb.v1.AddInstanceACLRulesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_instance_acl_rules

> models::ScalewayPeriodRdbPeriodV1PeriodDeleteInstanceAclRulesResponse delete_instance_acl_rules(region, instance_id, delete_instance_acl_rules_request)
Delete ACL rules of a Database Instance

Delete one or more ACL rules of a Database Instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the Database Instance you want to delete an ACL rule from. | [required] |
**delete_instance_acl_rules_request** | [**DeleteInstanceAclRulesRequest**](DeleteInstanceAclRulesRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodRdbPeriodV1PeriodDeleteInstanceAclRulesResponse**](scaleway.rdb.v1.DeleteInstanceACLRulesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_instance_acl_rules

> models::ScalewayPeriodRdbPeriodV1PeriodListInstanceAclRulesResponse list_instance_acl_rules(region, instance_id, page, page_size)
List ACL rules of a Database Instance

List the ACL rules for a given Database Instance. The response is an array of ACL objects, each one representing an ACL that denies, allows or redirects traffic based on certain conditions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the Database Instance. | [required] |
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |

### Return type

[**models::ScalewayPeriodRdbPeriodV1PeriodListInstanceAclRulesResponse**](scaleway.rdb.v1.ListInstanceACLRulesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_instance_acl_rules

> models::ScalewayPeriodRdbPeriodV1PeriodSetInstanceAclRulesResponse set_instance_acl_rules(region, instance_id, set_instance_acl_rules_request)
Set ACL rules for a Database Instance

Replace all the ACL rules of a Database Instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the Database Instance where the ACL rules must be set. | [required] |
**set_instance_acl_rules_request** | [**SetInstanceAclRulesRequest**](SetInstanceAclRulesRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodRdbPeriodV1PeriodSetInstanceAclRulesResponse**](scaleway.rdb.v1.SetInstanceACLRulesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

