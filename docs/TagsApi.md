# \TagsApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_tag**](TagsApi.md#delete_tag) | **DELETE** /registry/v1/regions/{region}/tags/{tag_id} | Delete a tag
[**get_tag**](TagsApi.md#get_tag) | **GET** /registry/v1/regions/{region}/tags/{tag_id} | Get a tag
[**list_tags**](TagsApi.md#list_tags) | **GET** /registry/v1/regions/{region}/images/{image_id}/tags | List all your tags



## delete_tag

> crate::models::ScalewayRegistryV1Tag delete_tag(region, tag_id, force)
Delete a tag

Delete the tag associated with the given id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**tag_id** | **String** | The unique ID of the tag | [required] |
**force** | Option<**bool**> | If two tags share the same digest the deletion will fail unless this parameter is set to true |  |

### Return type

[**crate::models::ScalewayRegistryV1Tag**](scaleway.registry.v1.Tag.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tag

> crate::models::ScalewayRegistryV1Tag get_tag(region, tag_id)
Get a tag

Get the tag associated with the given id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**tag_id** | **String** | The unique ID of the Tag | [required] |

### Return type

[**crate::models::ScalewayRegistryV1Tag**](scaleway.registry.v1.Tag.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_tags

> crate::models::ScalewayRegistryV1ListTagsResponse list_tags(region, image_id, page, page_size, order_by, name)
List all your tags

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**image_id** | **String** | The unique ID of the image | [required] |
**page** | Option<**f32**> | A positive integer to choose the page to display |  |[default to 1]
**page_size** | Option<**f32**> | A positive integer lower or equal to 100 to select the number of items to display |  |[default to 20]
**order_by** | Option<**String**> | Field by which to order the display of Images |  |[default to created_at_asc]
**name** | Option<**String**> | Filter by the tag name (exact match) |  |

### Return type

[**crate::models::ScalewayRegistryV1ListTagsResponse**](scaleway.registry.v1.ListTagsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

