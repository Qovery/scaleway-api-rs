# \ACLApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_instance_acl_rules**](ACLApi.md#add_instance_acl_rules) | **POST** /rdb/v1/regions/{region}/instances/{instance_id}/acls | Add an ACL instance to a given instance
[**delete_instance_acl_rules**](ACLApi.md#delete_instance_acl_rules) | **DELETE** /rdb/v1/regions/{region}/instances/{instance_id}/acls | Delete ACL rules of a given instance
[**list_instance_acl_rules**](ACLApi.md#list_instance_acl_rules) | **GET** /rdb/v1/regions/{region}/instances/{instance_id}/acls | List ACL rules of a given instance
[**set_instance_acl_rules**](ACLApi.md#set_instance_acl_rules) | **PUT** /rdb/v1/regions/{region}/instances/{instance_id}/acls | Set ACL rules for a given instance



## add_instance_acl_rules

> crate::models::ScalewayRdbV1AddInstanceAclRulesResponse add_instance_acl_rules(region, instance_id, inline_object13)
Add an ACL instance to a given instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance you want to add acl rules to | [required] |
**inline_object13** | [**InlineObject13**](InlineObject13.md) |  | [required] |

### Return type

[**crate::models::ScalewayRdbV1AddInstanceAclRulesResponse**](scaleway.rdb.v1.AddInstanceACLRulesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_instance_acl_rules

> crate::models::ScalewayRdbV1DeleteInstanceAclRulesResponse delete_instance_acl_rules(region, instance_id, inline_object14)
Delete ACL rules of a given instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance you want to delete an ACL rules from | [required] |
**inline_object14** | [**InlineObject14**](InlineObject14.md) |  | [required] |

### Return type

[**crate::models::ScalewayRdbV1DeleteInstanceAclRulesResponse**](scaleway.rdb.v1.DeleteInstanceACLRulesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_instance_acl_rules

> crate::models::ScalewayRdbV1ListInstanceAclRulesResponse list_instance_acl_rules(region, instance_id, page, page_size)
List ACL rules of a given instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance | [required] |
**page** | Option<**f32**> | Page number |  |[default to 1]
**page_size** | Option<**f32**> | Page size |  |[default to 20]

### Return type

[**crate::models::ScalewayRdbV1ListInstanceAclRulesResponse**](scaleway.rdb.v1.ListInstanceACLRulesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_instance_acl_rules

> crate::models::ScalewayRdbV1SetInstanceAclRulesResponse set_instance_acl_rules(region, instance_id, inline_object12)
Set ACL rules for a given instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**instance_id** | **String** | UUID of the instance where the ACL rules has to be set | [required] |
**inline_object12** | [**InlineObject12**](InlineObject12.md) |  | [required] |

### Return type

[**crate::models::ScalewayRdbV1SetInstanceAclRulesResponse**](scaleway.rdb.v1.SetInstanceACLRulesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

