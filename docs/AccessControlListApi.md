# \AccessControlListApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_cluster_acl_rules**](AccessControlListApi.md#add_cluster_acl_rules) | **POST** /k8s/v1/regions/{region}/clusters/{cluster_id}/acls | Add new ACLs
[**delete_acl_rule**](AccessControlListApi.md#delete_acl_rule) | **DELETE** /k8s/v1/regions/{region}/acls/{acl_id} | Delete an existing ACL
[**list_cluster_acl_rules**](AccessControlListApi.md#list_cluster_acl_rules) | **GET** /k8s/v1/regions/{region}/clusters/{cluster_id}/acls | List ACLs
[**set_cluster_acl_rules**](AccessControlListApi.md#set_cluster_acl_rules) | **PUT** /k8s/v1/regions/{region}/clusters/{cluster_id}/acls | Set new ACLs



## add_cluster_acl_rules

> models::ScalewayPeriodK8sPeriodV1PeriodAddClusterAclRulesResponse add_cluster_acl_rules(region, cluster_id, add_cluster_acl_rules_request)
Add new ACLs

Add new ACL rules for a specific cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**cluster_id** | **String** | ID of the cluster whose ACLs will be added. | [required] |
**add_cluster_acl_rules_request** | [**AddClusterAclRulesRequest**](AddClusterAclRulesRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodK8sPeriodV1PeriodAddClusterAclRulesResponse**](scaleway.k8s.v1.AddClusterACLRulesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_acl_rule

> delete_acl_rule(region, acl_id)
Delete an existing ACL

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**acl_id** | **String** | ID of the ACL rule to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_cluster_acl_rules

> models::ScalewayPeriodK8sPeriodV1PeriodListClusterAclRulesResponse list_cluster_acl_rules(region, cluster_id, page, page_size)
List ACLs

List ACLs for a specific cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**cluster_id** | **String** | ID of the cluster whose ACLs will be listed. | [required] |
**page** | Option<**i32**> | Page number for the returned ACLs. |  |
**page_size** | Option<**i32**> | Maximum number of ACLs per page. |  |

### Return type

[**models::ScalewayPeriodK8sPeriodV1PeriodListClusterAclRulesResponse**](scaleway.k8s.v1.ListClusterACLRulesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_cluster_acl_rules

> models::ScalewayPeriodK8sPeriodV1PeriodSetClusterAclRulesResponse set_cluster_acl_rules(region, cluster_id, set_cluster_acl_rules_request)
Set new ACLs

Set new ACL rules for a specific cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**cluster_id** | **String** | ID of the cluster whose ACLs will be set. | [required] |
**set_cluster_acl_rules_request** | [**SetClusterAclRulesRequest**](SetClusterAclRulesRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodK8sPeriodV1PeriodSetClusterAclRulesResponse**](scaleway.k8s.v1.SetClusterACLRulesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

