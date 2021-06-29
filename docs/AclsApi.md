# \AclsApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_acl**](AclsApi.md#create_acl) | **post** /lb/v1/regions/{region}/frontends/{frontend_id}/acls | Create an ACL for a given frontend
[**delete_acl**](AclsApi.md#delete_acl) | **delete** /lb/v1/regions/{region}/acls/{acl_id} | Delete an ACL
[**get_acl**](AclsApi.md#get_acl) | **get** /lb/v1/regions/{region}/acls/{acl_id} | Get an ACL
[**list_acls**](AclsApi.md#list_acls) | **get** /lb/v1/regions/{region}/frontends/{frontend_id}/acls | List ACL for a given frontend
[**update_acl**](AclsApi.md#update_acl) | **put** /lb/v1/regions/{region}/acls/{acl_id} | Update an ACL



## create_acl

> crate::models::ScalewayLbV1Acl create_acl(region, frontend_id, inline_object37)
Create an ACL for a given frontend

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**frontend_id** | **String** | ID of your frontend | [required] |
**inline_object37** | [**InlineObject37**](InlineObject37.md) |  | [required] |

### Return type

[**crate::models::ScalewayLbV1Acl**](scaleway.lb.v1.Acl.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_acl

> delete_acl(region, acl_id)
Delete an ACL

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**acl_id** | **String** | ID of your ACL ressource | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_acl

> crate::models::ScalewayLbV1Acl get_acl(region, acl_id)
Get an ACL

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**acl_id** | **String** | ID of your ACL ressource | [required] |

### Return type

[**crate::models::ScalewayLbV1Acl**](scaleway.lb.v1.Acl.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_acls

> crate::models::ScalewayLbV1ListAclResponse list_acls(region, frontend_id, page, order_by, page_size, name)
List ACL for a given frontend

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**frontend_id** | **String** | ID of your frontend | [required] |
**page** | **f32** | Page number | [required] |[default to 1]
**order_by** | Option<**String**> | You can order the response by created_at asc/desc or name asc/desc |  |[default to created_at_asc]
**page_size** | Option<**f32**> | The number of items to return |  |[default to 20]
**name** | Option<**String**> | Filter acl per name |  |

### Return type

[**crate::models::ScalewayLbV1ListAclResponse**](scaleway.lb.v1.ListAclResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_acl

> crate::models::ScalewayLbV1Acl update_acl(region, acl_id, inline_object29)
Update an ACL

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**acl_id** | **String** | ID of your ACL ressource | [required] |
**inline_object29** | [**InlineObject29**](InlineObject29.md) |  | [required] |

### Return type

[**crate::models::ScalewayLbV1Acl**](scaleway.lb.v1.Acl.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

