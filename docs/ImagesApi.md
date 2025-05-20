# \ImagesApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_image**](ImagesApi.md#delete_image) | **DELETE** /registry/v1/regions/{region}/images/{image_id} | Delete an image
[**get_image**](ImagesApi.md#get_image) | **GET** /registry/v1/regions/{region}/images/{image_id} | Get an image
[**list_images**](ImagesApi.md#list_images) | **GET** /registry/v1/regions/{region}/images | List images
[**update_image**](ImagesApi.md#update_image) | **PATCH** /registry/v1/regions/{region}/images/{image_id} | Update an image



## delete_image

> models::ScalewayPeriodRegistryPeriodV1PeriodImage delete_image(region, image_id)
Delete an image

Delete a given image. You must specify, in the endpoint, the `region` and `image_id` parameters of the image you want to delete.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**image_id** | **String** | UUID of the image. | [required] |

### Return type

[**models::ScalewayPeriodRegistryPeriodV1PeriodImage**](scaleway.registry.v1.Image.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_image

> models::ScalewayPeriodRegistryPeriodV1PeriodImage get_image(region, image_id)
Get an image

Retrieve information about a given container image, specified by its `image_id` and region. Full details about the image, such as `name`, `namespace_id`, `status`, `visibility`, and `size` are returned in the response.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**image_id** | **String** | UUID of the image. | [required] |

### Return type

[**models::ScalewayPeriodRegistryPeriodV1PeriodImage**](scaleway.registry.v1.Image.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_images

> models::ScalewayPeriodRegistryPeriodV1PeriodListImagesResponse list_images(region, page, page_size, order_by, namespace_id, name, organization_id, project_id)
List images

List all images in a specified region. By default, the images listed are ordered by creation date in ascending order. This can be modified via the order_by field. You can also define additional parameters for your query, such as the `namespace_id` and `project_id` parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**page** | Option<**i32**> | A positive integer to choose the page to display. |  |
**page_size** | Option<**i32**> | A positive integer lower or equal to 100 to select the number of items to display. |  |
**order_by** | Option<**String**> | Criteria to use when ordering image listings. Possible values are `created_at_asc`, `created_at_desc`, `name_asc`, `name_desc`, `region`, `status_asc` and `status_desc`. The default value is `created_at_asc`. |  |[default to created_at_asc]
**namespace_id** | Option<**String**> | Filter by the namespace ID. |  |
**name** | Option<**String**> | Filter by the image name (exact match). |  |
**organization_id** | Option<**String**> | Filter by Organization ID. |  |
**project_id** | Option<**String**> | Filter by Project ID. |  |

### Return type

[**models::ScalewayPeriodRegistryPeriodV1PeriodListImagesResponse**](scaleway.registry.v1.ListImagesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_image

> models::ScalewayPeriodRegistryPeriodV1PeriodImage update_image(region, image_id, update_image_request)
Update an image

Update the parameters of a given image, specified by its `image_id` and `region`. You can update the `visibility` parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**image_id** | **String** | ID of the image to update. | [required] |
**update_image_request** | [**UpdateImageRequest**](UpdateImageRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodRegistryPeriodV1PeriodImage**](scaleway.registry.v1.Image.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

