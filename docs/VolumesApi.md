# \VolumesApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_volume**](VolumesApi.md#create_volume) | **POST** /instance/v1/zones/{zone}/volumes | Create a volume
[**delete_volume**](VolumesApi.md#delete_volume) | **DELETE** /instance/v1/zones/{zone}/volumes/{volume_id} | Delete a volume
[**get_volume**](VolumesApi.md#get_volume) | **GET** /instance/v1/zones/{zone}/volumes/{volume_id} | Get a volume
[**list_volumes**](VolumesApi.md#list_volumes) | **GET** /instance/v1/zones/{zone}/volumes | List volumes
[**set_volume**](VolumesApi.md#set_volume) | **PUT** /instance/v1/zones/{zone}/volumes/{id} | Update volume
[**update_volume**](VolumesApi.md#update_volume) | **PATCH** /instance/v1/zones/{zone}/volumes/{volume_id} | Update a volume



## create_volume

> crate::models::ScalewayPeriodInstancePeriodV1PeriodCreateVolumeResponse create_volume(zone, create_volume_request)
Create a volume

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**create_volume_request** | [**CreateVolumeRequest**](CreateVolumeRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodInstancePeriodV1PeriodCreateVolumeResponse**](scaleway.instance.v1.CreateVolumeResponse.md)

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

> crate::models::ScalewayPeriodInstancePeriodV1PeriodGetVolumeResponse get_volume(zone, volume_id)
Get a volume

Get details of a volume with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**volume_id** | **String** | UUID of the volume you want to get | [required] |

### Return type

[**crate::models::ScalewayPeriodInstancePeriodV1PeriodGetVolumeResponse**](scaleway.instance.v1.GetVolumeResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_volumes

> crate::models::ScalewayPeriodInstancePeriodV1PeriodListVolumesResponse list_volumes(zone, volume_type, per_page, page, organization, project, tags, name)
List volumes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**volume_type** | Option<**String**> | Filter by volume type |  |[default to l_ssd]
**per_page** | Option<**i32**> | A positive integer lower or equal to 100 to select the number of items to return |  |
**page** | Option<**i32**> | A positive integer to choose the page to return |  |[default to 1]
**organization** | Option<**String**> | Filter volume by organization ID |  |
**project** | Option<**String**> | Filter volume by project ID |  |
**tags** | Option<**String**> | Filter volumes with these exact tags (to filter with several tags, use commas to separate them) |  |
**name** | Option<**String**> | Filter volume by name (for eg. \"vol\" will return \"myvolume\" but not \"data\") |  |

### Return type

[**crate::models::ScalewayPeriodInstancePeriodV1PeriodListVolumesResponse**](scaleway.instance.v1.ListVolumesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_volume

> crate::models::ScalewayPeriodInstancePeriodV1PeriodSetVolumeResponse set_volume(zone, id, set_volume_request)
Update volume

Replace all volume properties with a volume message.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**id** | **String** | The volumes unique ID | [required] |
**set_volume_request** | [**SetVolumeRequest**](SetVolumeRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodInstancePeriodV1PeriodSetVolumeResponse**](scaleway.instance.v1.SetVolumeResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_volume

> crate::models::ScalewayPeriodInstancePeriodV1PeriodUpdateVolumeResponse update_volume(zone, volume_id, update_volume_request)
Update a volume

Replace name and/or size properties of given ID volume with the given value(s). Any volume name can be changed while, for now, only `b_ssd` volume growing is supported.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**volume_id** | **String** | UUID of the volume | [required] |
**update_volume_request** | [**UpdateVolumeRequest**](UpdateVolumeRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodInstancePeriodV1PeriodUpdateVolumeResponse**](scaleway.instance.v1.UpdateVolumeResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

