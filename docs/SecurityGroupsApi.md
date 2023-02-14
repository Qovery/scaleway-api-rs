# \SecurityGroupsApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_security_group**](SecurityGroupsApi.md#create_security_group) | **POST** /instance/v1/zones/{zone}/security_groups | Create a security group
[**create_security_group_rule**](SecurityGroupsApi.md#create_security_group_rule) | **POST** /instance/v1/zones/{zone}/security_groups/{security_group_id}/rules | Create rule
[**delete_security_group**](SecurityGroupsApi.md#delete_security_group) | **DELETE** /instance/v1/zones/{zone}/security_groups/{security_group_id} | Delete a security group
[**delete_security_group_rule**](SecurityGroupsApi.md#delete_security_group_rule) | **DELETE** /instance/v1/zones/{zone}/security_groups/{security_group_id}/rules/{security_group_rule_id} | Delete rule
[**get_security_group**](SecurityGroupsApi.md#get_security_group) | **GET** /instance/v1/zones/{zone}/security_groups/{security_group_id} | Get a security group
[**get_security_group_rule**](SecurityGroupsApi.md#get_security_group_rule) | **GET** /instance/v1/zones/{zone}/security_groups/{security_group_id}/rules/{security_group_rule_id} | Get rule
[**list_default_security_group_rules**](SecurityGroupsApi.md#list_default_security_group_rules) | **GET** /instance/v1/zones/{zone}/security_groups/default/rules | Get default rules
[**list_security_group_rules**](SecurityGroupsApi.md#list_security_group_rules) | **GET** /instance/v1/zones/{zone}/security_groups/{security_group_id}/rules | List rules
[**list_security_groups**](SecurityGroupsApi.md#list_security_groups) | **GET** /instance/v1/zones/{zone}/security_groups | List security groups
[**set_security_group**](SecurityGroupsApi.md#set_security_group) | **PUT** /instance/v1/zones/{zone}/security_groups/{id} | Update a security group
[**set_security_group_rule**](SecurityGroupsApi.md#set_security_group_rule) | **PUT** /instance/v1/zones/{zone}/security_groups/{security_group_id}/rules/{security_group_rule_id} | Update security group rule
[**set_security_group_rules**](SecurityGroupsApi.md#set_security_group_rules) | **PUT** /instance/v1/zones/{zone}/security_groups/{security_group_id}/rules | Update all the rules of a security group



## create_security_group

> crate::models::ScalewayPeriodInstancePeriodV1PeriodCreateSecurityGroupResponse create_security_group(zone, create_security_group_request)
Create a security group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**create_security_group_request** | [**CreateSecurityGroupRequest**](CreateSecurityGroupRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodInstancePeriodV1PeriodCreateSecurityGroupResponse**](scaleway.instance.v1.CreateSecurityGroupResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_security_group_rule

> crate::models::ScalewayPeriodInstancePeriodV1PeriodCreateSecurityGroupRuleResponse create_security_group_rule(zone, security_group_id, create_security_group_rule_request)
Create rule

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**security_group_id** | **String** | UUID of the security group | [required] |
**create_security_group_rule_request** | [**CreateSecurityGroupRuleRequest**](CreateSecurityGroupRuleRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodInstancePeriodV1PeriodCreateSecurityGroupRuleResponse**](scaleway.instance.v1.CreateSecurityGroupRuleResponse.md)

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

> crate::models::ScalewayPeriodInstancePeriodV1PeriodGetSecurityGroupResponse get_security_group(zone, security_group_id)
Get a security group

Get the details of a Security Group with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**security_group_id** | **String** | UUID of the security group you want to get | [required] |

### Return type

[**crate::models::ScalewayPeriodInstancePeriodV1PeriodGetSecurityGroupResponse**](scaleway.instance.v1.GetSecurityGroupResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_security_group_rule

> crate::models::ScalewayPeriodInstancePeriodV1PeriodGetSecurityGroupRuleResponse get_security_group_rule(zone, security_group_id, security_group_rule_id)
Get rule

Get details of a security group rule with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**security_group_id** | **String** |  | [required] |
**security_group_rule_id** | **String** |  | [required] |

### Return type

[**crate::models::ScalewayPeriodInstancePeriodV1PeriodGetSecurityGroupRuleResponse**](scaleway.instance.v1.GetSecurityGroupRuleResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_default_security_group_rules

> crate::models::ScalewayPeriodInstancePeriodV1PeriodListSecurityGroupRulesResponse list_default_security_group_rules(zone)
Get default rules

Lists the default rules applied to all the security groups.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |

### Return type

[**crate::models::ScalewayPeriodInstancePeriodV1PeriodListSecurityGroupRulesResponse**](scaleway.instance.v1.ListSecurityGroupRulesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_security_group_rules

> crate::models::ScalewayPeriodInstancePeriodV1PeriodListSecurityGroupRulesResponse list_security_group_rules(zone, security_group_id, per_page, page)
List rules

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**security_group_id** | **String** | UUID of the security group | [required] |
**per_page** | Option<**i32**> | A positive integer lower or equal to 100 to select the number of items to return |  |
**page** | Option<**i32**> | A positive integer to choose the page to return |  |[default to 1]

### Return type

[**crate::models::ScalewayPeriodInstancePeriodV1PeriodListSecurityGroupRulesResponse**](scaleway.instance.v1.ListSecurityGroupRulesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_security_groups

> crate::models::ScalewayPeriodInstancePeriodV1PeriodListSecurityGroupsResponse list_security_groups(zone, name, organization, project, tags, project_default, per_page, page)
List security groups

List all security groups available in an account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**name** | Option<**String**> | Name of the security group |  |
**organization** | Option<**String**> | The security group organization ID |  |
**project** | Option<**String**> | The security group project ID |  |
**tags** | Option<**String**> | List security groups with these exact tags (to filter with several tags, use commas to separate them) |  |
**project_default** | Option<**bool**> | Filter security groups with this value for project_default |  |
**per_page** | Option<**i32**> | A positive integer lower or equal to 100 to select the number of items to return |  |
**page** | Option<**i32**> | A positive integer to choose the page to return |  |[default to 1]

### Return type

[**crate::models::ScalewayPeriodInstancePeriodV1PeriodListSecurityGroupsResponse**](scaleway.instance.v1.ListSecurityGroupsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_security_group

> crate::models::ScalewayPeriodInstancePeriodV1PeriodSetSecurityGroupResponse set_security_group(zone, id, set_security_group_request)
Update a security group

Replace all security group properties with a security group message.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**id** | **String** | The ID of the security group (will be ignored) | [required] |
**set_security_group_request** | [**SetSecurityGroupRequest**](SetSecurityGroupRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodInstancePeriodV1PeriodSetSecurityGroupResponse**](scaleway.instance.v1.SetSecurityGroupResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_security_group_rule

> crate::models::ScalewayPeriodInstancePeriodV1PeriodSetSecurityGroupRuleResponse set_security_group_rule(zone, security_group_id, security_group_rule_id, set_security_group_rule_request)
Update security group rule

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**security_group_id** | **String** |  | [required] |
**security_group_rule_id** | **String** |  | [required] |
**set_security_group_rule_request** | [**SetSecurityGroupRuleRequest**](SetSecurityGroupRuleRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodInstancePeriodV1PeriodSetSecurityGroupRuleResponse**](scaleway.instance.v1.SetSecurityGroupRuleResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_security_group_rules

> crate::models::ScalewayPeriodInstancePeriodV1PeriodSetSecurityGroupRulesResponse set_security_group_rules(zone, security_group_id, set_security_group_rules_request)
Update all the rules of a security group

Replaces the rules of the security group with the rules provided. This endpoint supports the update of existing rules, creation of new rules and deletion of existing rules when they are not passed in the request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**security_group_id** | **String** | UUID of the security group to update the rules on | [required] |
**set_security_group_rules_request** | [**SetSecurityGroupRulesRequest**](SetSecurityGroupRulesRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodInstancePeriodV1PeriodSetSecurityGroupRulesResponse**](scaleway.instance.v1.SetSecurityGroupRulesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

