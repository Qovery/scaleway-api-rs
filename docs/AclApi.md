# \AclApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_instance_acl_rules**](AclApi.md#add_instance_acl_rules) | **POST** /rdb/v1/regions/{region}/instances/{instance_id}/acls | Add an ACL instance to a given instance
[**delete_instance_acl_rules**](AclApi.md#delete_instance_acl_rules) | **DELETE** /rdb/v1/regions/{region}/instances/{instance_id}/acls | Delete ACL rules of a given instance
[**list_instance_acl_rules**](AclApi.md#list_instance_acl_rules) | **GET** /rdb/v1/regions/{region}/instances/{instance_id}/acls | List ACL rules of a given instance
[**set_instance_acl_rules**](AclApi.md#set_instance_acl_rules) | **PUT** /rdb/v1/regions/{region}/instances/{instance_id}/acls | Set ACL rules for a given instance



## add_instance_acl_rules

> crate::models::ScalewayPeriodRdbPeriodV1PeriodAddInstanceAclRulesResponse add_instance_acl_rules(region, instance_id, add_instance_acl_rules_request)
Add an ACL instance to a given instance

Add an additional ACL rule to a database instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance you want to add acl rules to | [required] |
**add_instance_acl_rules_request** | [**AddInstanceAclRulesRequest**](AddInstanceAclRulesRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodRdbPeriodV1PeriodAddInstanceAclRulesResponse**](scaleway.rdb.v1.AddInstanceACLRulesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_instance_acl_rules

> crate::models::ScalewayPeriodRdbPeriodV1PeriodDeleteInstanceAclRulesResponse delete_instance_acl_rules(region, instance_id, delete_instance_acl_rules_request)
Delete ACL rules of a given instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance you want to delete an ACL rules from | [required] |
**delete_instance_acl_rules_request** | [**DeleteInstanceAclRulesRequest**](DeleteInstanceAclRulesRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodRdbPeriodV1PeriodDeleteInstanceAclRulesResponse**](scaleway.rdb.v1.DeleteInstanceACLRulesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_instance_acl_rules

> crate::models::ScalewayPeriodRdbPeriodV1PeriodListInstanceAclRulesResponse list_instance_acl_rules(region, instance_id, page, page_size)
List ACL rules of a given instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance | [required] |
**page** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 20]

### Return type

[**crate::models::ScalewayPeriodRdbPeriodV1PeriodListInstanceAclRulesResponse**](scaleway.rdb.v1.ListInstanceACLRulesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_instance_acl_rules

> crate::models::ScalewayPeriodRdbPeriodV1PeriodSetInstanceAclRulesResponse set_instance_acl_rules(region, instance_id, set_instance_acl_rules_request)
Set ACL rules for a given instance

Replace all the ACL rules of a database instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance where the ACL rules has to be set | [required] |
**set_instance_acl_rules_request** | [**SetInstanceAclRulesRequest**](SetInstanceAclRulesRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodRdbPeriodV1PeriodSetInstanceAclRulesResponse**](scaleway.rdb.v1.SetInstanceACLRulesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

