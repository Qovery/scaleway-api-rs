# \VolumesApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_volume**](VolumesApi.md#create_volume) | **post** /instance/v1/zones/{zone}/volumes | Create a volume
[**delete_volume**](VolumesApi.md#delete_volume) | **delete** /instance/v1/zones/{zone}/volumes/{volume_id} | Delete a volume
[**get_volume**](VolumesApi.md#get_volume) | **get** /instance/v1/zones/{zone}/volumes/{volume_id} | Get a volume
[**list_volumes**](VolumesApi.md#list_volumes) | **get** /instance/v1/zones/{zone}/volumes | List volumes
[**set_volume**](VolumesApi.md#set_volume) | **put** /instance/v1/zones/{zone}/volumes/{id} | Update volume
[**update_volume**](VolumesApi.md#update_volume) | **patch** /instance/v1/zones/{zone}/volumes/{volume_id} | Update a volume



## create_volume

> crate::models::ScalewayInstanceV1CreateVolumeResponse create_volume(zone, inline_object19)
Create a volume

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**inline_object19** | [**InlineObject19**](InlineObject19.md) |  | [required] |

### Return type

[**crate::models::ScalewayInstanceV1CreateVolumeResponse**](scaleway.instance.v1.CreateVolumeResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_volume

> delete_volume(zone, volume_id)
Delete a volume

Delete the volume with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**volume_id** | **String** | UUID of the volume you want to delete | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_volume

> crate::models::ScalewayInstanceV1GetVolumeResponse get_volume(zone, volume_id)
Get a volume

Get details of a volume with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**volume_id** | **String** | UUID of the volume you want to get | [required] |

### Return type

[**crate::models::ScalewayInstanceV1GetVolumeResponse**](scaleway.instance.v1.GetVolumeResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_volumes

> crate::models::ScalewayInstanceV1ListVolumesResponse list_volumes(zone, volume_type, per_page, page, organization, project, name)
List volumes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**volume_type** | Option<**String**> | Filter by volume type |  |[default to l_ssd]
**per_page** | Option<**f32**> | A positive integer lower or equal to 100 to select the number of items to return |  |
**page** | Option<**f32**> | A positive integer to choose the page to return |  |[default to 1]
**organization** | Option<**String**> | Filter volume by organization ID |  |
**project** | Option<**String**> | Filter volume by project ID |  |
**name** | Option<**String**> | Filter volume by name (for eg. \"vol\" will return \"myvolume\" but not \"data\") |  |

### Return type

[**crate::models::ScalewayInstanceV1ListVolumesResponse**](scaleway.instance.v1.ListVolumesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_volume

> crate::models::ScalewayInstanceV1SetVolumeResponse set_volume(zone, id, inline_object20)
Update volume

Replace all volume properties with a volume message.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**id** | **String** | The volumes unique ID | [required] |
**inline_object20** | [**InlineObject20**](InlineObject20.md) |  | [required] |

### Return type

[**crate::models::ScalewayInstanceV1SetVolumeResponse**](scaleway.instance.v1.SetVolumeResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_volume

> crate::models::ScalewayInstanceV1UpdateVolumeResponse update_volume(zone, volume_id, inline_object21)
Update a volume

Replace name and/or size properties of given ID volume with the given value(s). Any volume name can be changed while, for now, only `b_ssd` volume growing is supported.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**volume_id** | **String** | UUID of the volume | [required] |
**inline_object21** | [**InlineObject21**](InlineObject21.md) |  | [required] |

### Return type

[**crate::models::ScalewayInstanceV1UpdateVolumeResponse**](scaleway.instance.v1.UpdateVolumeResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

