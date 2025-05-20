# \TagsApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_tag**](TagsApi.md#delete_tag) | **DELETE** /registry/v1/regions/{region}/tags/{tag_id} | Delete a tag
[**get_tag**](TagsApi.md#get_tag) | **GET** /registry/v1/regions/{region}/tags/{tag_id} | Get a tag
[**list_tags**](TagsApi.md#list_tags) | **GET** /registry/v1/regions/{region}/images/{image_id}/tags | List tags



## delete_tag

> models::ScalewayPeriodRegistryPeriodV1PeriodTag delete_tag(region, tag_id, force)
Delete a tag

Delete a given image tag. You must specify, in the endpoint, the `region` and `tag_id` parameters of the tag you want to delete.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**tag_id** | **String** | UUID of the tag. | [required] |
**force** | Option<**bool**> | If two tags share the same digest the deletion will fail unless this parameter is set to true (deprecated). |  |

### Return type

[**models::ScalewayPeriodRegistryPeriodV1PeriodTag**](scaleway.registry.v1.Tag.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tag

> models::ScalewayPeriodRegistryPeriodV1PeriodTag get_tag(region, tag_id)
Get a tag

Retrieve information about a given image tag, specified by its `tag_id` and region. Full details about the tag, such as `name`, `image_id`, `status`, and `digest` are returned in the response.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**tag_id** | **String** | UUID of the tag. | [required] |

### Return type

[**models::ScalewayPeriodRegistryPeriodV1PeriodTag**](scaleway.registry.v1.Tag.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_tags

> models::ScalewayPeriodRegistryPeriodV1PeriodListTagsResponse list_tags(region, image_id, page, page_size, order_by, name)
List tags

List all tags for a given image, specified by region. By default, the tags listed are ordered by creation date in ascending order. This can be modified via the order_by field. You can also define additional parameters for your query, such as the `name`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**image_id** | **String** | UUID of the image. | [required] |
**page** | Option<**i32**> | A positive integer to choose the page to display. |  |
**page_size** | Option<**i32**> | A positive integer lower or equal to 100 to select the number of items to display. |  |
**order_by** | Option<**String**> | Criteria to use when ordering tag listings. Possible values are `created_at_asc`, `created_at_desc`, `name_asc`, `name_desc`, `region`, `status_asc` and `status_desc`. The default value is `created_at_asc`. |  |[default to created_at_asc]
**name** | Option<**String**> | Filter by the tag name (exact match). |  |

### Return type

[**models::ScalewayPeriodRegistryPeriodV1PeriodListTagsResponse**](scaleway.registry.v1.ListTagsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

