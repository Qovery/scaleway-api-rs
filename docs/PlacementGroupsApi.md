# \PlacementGroupsApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_placement_group**](PlacementGroupsApi.md#create_placement_group) | **POST** /instance/v1/zones/{zone}/placement_groups | Create a placement group
[**delete_placement_group**](PlacementGroupsApi.md#delete_placement_group) | **DELETE** /instance/v1/zones/{zone}/placement_groups/{placement_group_id} | Delete the given placement group
[**get_placement_group**](PlacementGroupsApi.md#get_placement_group) | **GET** /instance/v1/zones/{zone}/placement_groups/{placement_group_id} | Get a placement group
[**get_placement_group_servers**](PlacementGroupsApi.md#get_placement_group_servers) | **GET** /instance/v1/zones/{zone}/placement_groups/{placement_group_id}/servers | Get placement group servers
[**list_placement_groups**](PlacementGroupsApi.md#list_placement_groups) | **GET** /instance/v1/zones/{zone}/placement_groups | List placement groups
[**set_placement_group**](PlacementGroupsApi.md#set_placement_group) | **PUT** /instance/v1/zones/{zone}/placement_groups/{placement_group_id} | Set placement group
[**set_placement_group_servers**](PlacementGroupsApi.md#set_placement_group_servers) | **PUT** /instance/v1/zones/{zone}/placement_groups/{placement_group_id}/servers | Set placement group servers
[**update_placement_group**](PlacementGroupsApi.md#update_placement_group) | **PATCH** /instance/v1/zones/{zone}/placement_groups/{placement_group_id} | Update a placement group
[**update_placement_group_servers**](PlacementGroupsApi.md#update_placement_group_servers) | **PATCH** /instance/v1/zones/{zone}/placement_groups/{placement_group_id}/servers | Update placement group servers



## create_placement_group

> crate::models::ScalewayPeriodInstancePeriodV1PeriodCreatePlacementGroupResponse create_placement_group(zone, create_placement_group_request)
Create a placement group

Create a new placement group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**create_placement_group_request** | [**CreatePlacementGroupRequest**](CreatePlacementGroupRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodInstancePeriodV1PeriodCreatePlacementGroupResponse**](scaleway.instance.v1.CreatePlacementGroupResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_placement_group

> delete_placement_group(zone, placement_group_id)
Delete the given placement group

Delete the given placement group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**placement_group_id** | **String** | UUID of the placement group you want to delete | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_placement_group

> crate::models::ScalewayPeriodInstancePeriodV1PeriodGetPlacementGroupResponse get_placement_group(zone, placement_group_id)
Get a placement group

Get the given placement group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**placement_group_id** | **String** | UUID of the placement group you want to get | [required] |

### Return type

[**crate::models::ScalewayPeriodInstancePeriodV1PeriodGetPlacementGroupResponse**](scaleway.instance.v1.GetPlacementGroupResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_placement_group_servers

> crate::models::ScalewayPeriodInstancePeriodV1PeriodGetPlacementGroupServersResponse get_placement_group_servers(zone, placement_group_id)
Get placement group servers

Get all servers belonging to the given placement group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**placement_group_id** | **String** |  | [required] |

### Return type

[**crate::models::ScalewayPeriodInstancePeriodV1PeriodGetPlacementGroupServersResponse**](scaleway.instance.v1.GetPlacementGroupServersResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_placement_groups

> crate::models::ScalewayPeriodInstancePeriodV1PeriodListPlacementGroupsResponse list_placement_groups(zone, per_page, page, organization, project, tags, name)
List placement groups

List all placement groups.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**per_page** | Option<**i32**> | A positive integer lower or equal to 100 to select the number of items to return |  |
**page** | Option<**i32**> | A positive integer to choose the page to return |  |[default to 1]
**organization** | Option<**String**> | List only placement groups of this organization ID |  |
**project** | Option<**String**> | List only placement groups of this project ID |  |
**tags** | Option<**String**> | List placement groups with these exact tags (to filter with several tags, use commas to separate them) |  |
**name** | Option<**String**> | Filter placement groups by name (for eg. \"cluster1\" will return \"cluster100\" and \"cluster1\" but not \"foo\") |  |

### Return type

[**crate::models::ScalewayPeriodInstancePeriodV1PeriodListPlacementGroupsResponse**](scaleway.instance.v1.ListPlacementGroupsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_placement_group

> crate::models::ScalewayPeriodInstancePeriodV1PeriodSetPlacementGroupResponse set_placement_group(zone, placement_group_id, set_placement_group_request)
Set placement group

Set all parameters of the given placement group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**placement_group_id** | **String** |  | [required] |
**set_placement_group_request** | [**SetPlacementGroupRequest**](SetPlacementGroupRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodInstancePeriodV1PeriodSetPlacementGroupResponse**](scaleway.instance.v1.SetPlacementGroupResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_placement_group_servers

> crate::models::ScalewayPeriodInstancePeriodV1PeriodSetPlacementGroupServersResponse set_placement_group_servers(zone, placement_group_id, set_placement_group_servers_request)
Set placement group servers

Set all servers belonging to the given placement group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**placement_group_id** | **String** |  | [required] |
**set_placement_group_servers_request** | [**SetPlacementGroupServersRequest**](SetPlacementGroupServersRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodInstancePeriodV1PeriodSetPlacementGroupServersResponse**](scaleway.instance.v1.SetPlacementGroupServersResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_placement_group

> crate::models::ScalewayPeriodInstancePeriodV1PeriodUpdatePlacementGroupResponse update_placement_group(zone, placement_group_id, update_placement_group_request)
Update a placement group

Update one or more parameter of the given placement group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**placement_group_id** | **String** | UUID of the placement group | [required] |
**update_placement_group_request** | [**UpdatePlacementGroupRequest**](UpdatePlacementGroupRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodInstancePeriodV1PeriodUpdatePlacementGroupResponse**](scaleway.instance.v1.UpdatePlacementGroupResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_placement_group_servers

> crate::models::ScalewayPeriodInstancePeriodV1PeriodUpdatePlacementGroupServersResponse update_placement_group_servers(zone, placement_group_id, update_placement_group_servers_request)
Update placement group servers

Update all servers belonging to the given placement group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**placement_group_id** | **String** | UUID of the placement group | [required] |
**update_placement_group_servers_request** | [**UpdatePlacementGroupServersRequest**](UpdatePlacementGroupServersRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodInstancePeriodV1PeriodUpdatePlacementGroupServersResponse**](scaleway.instance.v1.UpdatePlacementGroupServersResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

