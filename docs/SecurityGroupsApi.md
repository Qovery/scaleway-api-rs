# \SecurityGroupsApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_security_group**](SecurityGroupsApi.md#create_security_group) | **post** /instance/v1/zones/{zone}/security_groups | Create a security group
[**create_security_group_rule**](SecurityGroupsApi.md#create_security_group_rule) | **post** /instance/v1/zones/{zone}/security_groups/{security_group_id}/rules | Create rule
[**delete_security_group**](SecurityGroupsApi.md#delete_security_group) | **delete** /instance/v1/zones/{zone}/security_groups/{security_group_id} | Delete a security group
[**delete_security_group_rule**](SecurityGroupsApi.md#delete_security_group_rule) | **delete** /instance/v1/zones/{zone}/security_groups/{security_group_id}/rules/{security_group_rule_id} | Delete rule
[**get_security_group**](SecurityGroupsApi.md#get_security_group) | **get** /instance/v1/zones/{zone}/security_groups/{security_group_id} | Get a security group
[**get_security_group_rule**](SecurityGroupsApi.md#get_security_group_rule) | **get** /instance/v1/zones/{zone}/security_groups/{security_group_id}/rules/{security_group_rule_id} | Get rule
[**list_security_group_rules**](SecurityGroupsApi.md#list_security_group_rules) | **get** /instance/v1/zones/{zone}/security_groups/{security_group_id}/rules | List rules
[**list_security_groups**](SecurityGroupsApi.md#list_security_groups) | **get** /instance/v1/zones/{zone}/security_groups | List security groups
[**set_security_group**](SecurityGroupsApi.md#set_security_group) | **put** /instance/v1/zones/{zone}/security_groups/{id} | Update a security group
[**set_security_group_rule**](SecurityGroupsApi.md#set_security_group_rule) | **put** /instance/v1/zones/{zone}/security_groups/{security_group_id}/rules/{security_group_rule_id} | Update security group rule



## create_security_group

> crate::models::ScalewayInstanceV1CreateSecurityGroupResponse create_security_group(zone, inline_object9)
Create a security group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**inline_object9** | [**InlineObject9**](InlineObject9.md) |  | [required] |

### Return type

[**crate::models::ScalewayInstanceV1CreateSecurityGroupResponse**](scaleway.instance.v1.CreateSecurityGroupResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_security_group_rule

> crate::models::ScalewayInstanceV1CreateSecurityGroupRuleResponse create_security_group_rule(zone, security_group_id, inline_object11)
Create rule

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**security_group_id** | **String** | UUID of the security group | [required] |
**inline_object11** | [**InlineObject11**](InlineObject11.md) |  | [required] |

### Return type

[**crate::models::ScalewayInstanceV1CreateSecurityGroupRuleResponse**](scaleway.instance.v1.CreateSecurityGroupRuleResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_security_group

> delete_security_group(zone, security_group_id)
Delete a security group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**security_group_id** | **String** | UUID of the security group you want to delete | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_security_group_rule

> delete_security_group_rule(zone, security_group_id, security_group_rule_id)
Delete rule

Delete a security group rule with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**security_group_id** | **String** |  | [required] |
**security_group_rule_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_security_group

> crate::models::ScalewayInstanceV1GetSecurityGroupResponse get_security_group(zone, security_group_id)
Get a security group

Get the details of a Security Group with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**security_group_id** | **String** | UUID of the security group you want to get | [required] |

### Return type

[**crate::models::ScalewayInstanceV1GetSecurityGroupResponse**](scaleway.instance.v1.GetSecurityGroupResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_security_group_rule

> crate::models::ScalewayInstanceV1GetSecurityGroupRuleResponse get_security_group_rule(zone, security_group_id, security_group_rule_id)
Get rule

Get details of a security group rule with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**security_group_id** | **String** |  | [required] |
**security_group_rule_id** | **String** |  | [required] |

### Return type

[**crate::models::ScalewayInstanceV1GetSecurityGroupRuleResponse**](scaleway.instance.v1.GetSecurityGroupRuleResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_security_group_rules

> crate::models::ScalewayInstanceV1ListSecurityGroupRulesResponse list_security_group_rules(zone, security_group_id, per_page, page)
List rules

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**security_group_id** | **String** | UUID of the security group | [required] |
**per_page** | Option<**f32**> | A positive integer lower or equal to 100 to select the number of items to return |  |
**page** | Option<**f32**> | A positive integer to choose the page to return |  |[default to 1]

### Return type

[**crate::models::ScalewayInstanceV1ListSecurityGroupRulesResponse**](scaleway.instance.v1.ListSecurityGroupRulesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_security_groups

> crate::models::ScalewayInstanceV1ListSecurityGroupsResponse list_security_groups(zone, name, organization, project, per_page, page)
List security groups

List all security groups available in an account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**name** | Option<**String**> | Name of the security group |  |
**organization** | Option<**String**> | The security group organization ID |  |
**project** | Option<**String**> | The security group project ID |  |
**per_page** | Option<**f32**> | A positive integer lower or equal to 100 to select the number of items to return |  |
**page** | Option<**f32**> | A positive integer to choose the page to return |  |[default to 1]

### Return type

[**crate::models::ScalewayInstanceV1ListSecurityGroupsResponse**](scaleway.instance.v1.ListSecurityGroupsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_security_group

> crate::models::ScalewayInstanceV1SetSecurityGroupResponse set_security_group(zone, id, inline_object10)
Update a security group

Replace all security group properties with a security group message.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**id** | **String** | The ID of the security group (will be ignored) | [required] |
**inline_object10** | [**InlineObject10**](InlineObject10.md) |  | [required] |

### Return type

[**crate::models::ScalewayInstanceV1SetSecurityGroupResponse**](scaleway.instance.v1.SetSecurityGroupResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_security_group_rule

> crate::models::ScalewayInstanceV1SetSecurityGroupRuleResponse set_security_group_rule(zone, security_group_id, security_group_rule_id, inline_object12)
Update security group rule

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**security_group_id** | **String** |  | [required] |
**security_group_rule_id** | **String** |  | [required] |
**inline_object12** | [**InlineObject12**](InlineObject12.md) |  | [required] |

### Return type

[**crate::models::ScalewayInstanceV1SetSecurityGroupRuleResponse**](scaleway.instance.v1.SetSecurityGroupRuleResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

