# \PlacementGroupsApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_placement_group**](PlacementGroupsApi.md#create_placement_group) | **post** /instance/v1/zones/{zone}/placement_groups | Create a placement group
[**delete_placement_group**](PlacementGroupsApi.md#delete_placement_group) | **delete** /instance/v1/zones/{zone}/placement_groups/{placement_group_id} | Delete the given placement group
[**get_placement_group**](PlacementGroupsApi.md#get_placement_group) | **get** /instance/v1/zones/{zone}/placement_groups/{placement_group_id} | Get a placement group
[**get_placement_group_servers**](PlacementGroupsApi.md#get_placement_group_servers) | **get** /instance/v1/zones/{zone}/placement_groups/{placement_group_id}/servers | Get placement group servers
[**list_placement_groups**](PlacementGroupsApi.md#list_placement_groups) | **get** /instance/v1/zones/{zone}/placement_groups | List placement groups
[**set_placement_group**](PlacementGroupsApi.md#set_placement_group) | **put** /instance/v1/zones/{zone}/placement_groups/{placement_group_id} | Set placement group
[**set_placement_group_servers**](PlacementGroupsApi.md#set_placement_group_servers) | **put** /instance/v1/zones/{zone}/placement_groups/{placement_group_id}/servers | Set placement group servers
[**update_placement_group**](PlacementGroupsApi.md#update_placement_group) | **patch** /instance/v1/zones/{zone}/placement_groups/{placement_group_id} | Update a placement group
[**update_placement_group_servers**](PlacementGroupsApi.md#update_placement_group_servers) | **patch** /instance/v1/zones/{zone}/placement_groups/{placement_group_id}/servers | Update placement group servers



## create_placement_group

> crate::models::ScalewayInstanceV1CreatePlacementGroupResponse create_placement_group(zone, inline_object4)
Create a placement group

Create a new placement group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**inline_object4** | [**InlineObject4**](InlineObject4.md) |  | [required] |

### Return type

[**crate::models::ScalewayInstanceV1CreatePlacementGroupResponse**](scaleway.instance.v1.CreatePlacementGroupResponse.md)

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

> crate::models::ScalewayInstanceV1GetPlacementGroupResponse get_placement_group(zone, placement_group_id)
Get a placement group

Get the given placement group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**placement_group_id** | **String** | UUID of the placement group you want to get | [required] |

### Return type

[**crate::models::ScalewayInstanceV1GetPlacementGroupResponse**](scaleway.instance.v1.GetPlacementGroupResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_placement_group_servers

> crate::models::ScalewayInstanceV1GetPlacementGroupServersResponse get_placement_group_servers(zone, placement_group_id)
Get placement group servers

Get all servers belonging to the given placement group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**placement_group_id** | **String** |  | [required] |

### Return type

[**crate::models::ScalewayInstanceV1GetPlacementGroupServersResponse**](scaleway.instance.v1.GetPlacementGroupServersResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_placement_groups

> crate::models::ScalewayInstanceV1ListPlacementGroupsResponse list_placement_groups(zone, per_page, page, organization, project, name)
List placement groups

List all placement groups.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**per_page** | Option<**f32**> | A positive integer lower or equal to 100 to select the number of items to return |  |
**page** | Option<**f32**> | A positive integer to choose the page to return |  |[default to 1]
**organization** | Option<**String**> | List only placement groups of this organization ID |  |
**project** | Option<**String**> | List only placement groups of this project ID |  |
**name** | Option<**String**> | Filter placement groups by name (for eg. \"cluster1\" will return \"cluster100\" and \"cluster1\" but not \"foo\") |  |

### Return type

[**crate::models::ScalewayInstanceV1ListPlacementGroupsResponse**](scaleway.instance.v1.ListPlacementGroupsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_placement_group

> crate::models::ScalewayInstanceV1SetPlacementGroupResponse set_placement_group(zone, placement_group_id, inline_object5)
Set placement group

Set all parameters of the given placement group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**placement_group_id** | **String** |  | [required] |
**inline_object5** | [**InlineObject5**](InlineObject5.md) |  | [required] |

### Return type

[**crate::models::ScalewayInstanceV1SetPlacementGroupResponse**](scaleway.instance.v1.SetPlacementGroupResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_placement_group_servers

> crate::models::ScalewayInstanceV1SetPlacementGroupServersResponse set_placement_group_servers(zone, placement_group_id, inline_object7)
Set placement group servers

Set all servers belonging to the given placement group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**placement_group_id** | **String** |  | [required] |
**inline_object7** | [**InlineObject7**](InlineObject7.md) |  | [required] |

### Return type

[**crate::models::ScalewayInstanceV1SetPlacementGroupServersResponse**](scaleway.instance.v1.SetPlacementGroupServersResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_placement_group

> crate::models::ScalewayInstanceV1UpdatePlacementGroupResponse update_placement_group(zone, placement_group_id, inline_object6)
Update a placement group

Update one or more parameter of the given placement group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**placement_group_id** | **String** | UUID of the placement group | [required] |
**inline_object6** | [**InlineObject6**](InlineObject6.md) |  | [required] |

### Return type

[**crate::models::ScalewayInstanceV1UpdatePlacementGroupResponse**](scaleway.instance.v1.UpdatePlacementGroupResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_placement_group_servers

> crate::models::ScalewayInstanceV1UpdatePlacementGroupServersResponse update_placement_group_servers(zone, placement_group_id, inline_object8)
Update placement group servers

Update all servers belonging to the given placement group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**placement_group_id** | **String** | UUID of the placement group | [required] |
**inline_object8** | [**InlineObject8**](InlineObject8.md) |  | [required] |

### Return type

[**crate::models::ScalewayInstanceV1UpdatePlacementGroupServersResponse**](scaleway.instance.v1.UpdatePlacementGroupServersResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

