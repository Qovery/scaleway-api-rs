# \ImagesApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_image**](ImagesApi.md#delete_image) | **DELETE** /registry/v1/regions/{region}/images/{image_id} | Delete an image
[**get_image**](ImagesApi.md#get_image) | **GET** /registry/v1/regions/{region}/images/{image_id} | Get a image
[**list_images**](ImagesApi.md#list_images) | **GET** /registry/v1/regions/{region}/images | List all your images
[**update_image**](ImagesApi.md#update_image) | **PATCH** /registry/v1/regions/{region}/images/{image_id} | Update an existing image



## delete_image

> crate::models::ScalewayRegistryV1Image delete_image(region, image_id)
Delete an image

Delete the image associated with the given id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**image_id** | **String** | The unique ID of the Image | [required] |

### Return type

[**crate::models::ScalewayRegistryV1Image**](scaleway.registry.v1.Image.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_image

> crate::models::ScalewayRegistryV1Image get_image(region, image_id)
Get a image

Get the image associated with the given id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**image_id** | **String** | The unique ID of the Image | [required] |

### Return type

[**crate::models::ScalewayRegistryV1Image**](scaleway.registry.v1.Image.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_images

> crate::models::ScalewayRegistryV1ListImagesResponse list_images(region, page, page_size, order_by, namespace_id, name, organization_id, project_id)
List all your images

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**page** | Option<**f32**> | A positive integer to choose the page to display |  |[default to 1]
**page_size** | Option<**f32**> | A positive integer lower or equal to 100 to select the number of items to display |  |[default to 20]
**order_by** | Option<**String**> | Field by which to order the display of Images |  |[default to created_at_asc]
**namespace_id** | Option<**String**> | Filter by the Namespace ID |  |
**name** | Option<**String**> | Filter by the Image name (exact match) |  |
**organization_id** | Option<**String**> | Filter by Organization ID |  |
**project_id** | Option<**String**> | Filter by Project ID |  |

### Return type

[**crate::models::ScalewayRegistryV1ListImagesResponse**](scaleway.registry.v1.ListImagesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_image

> crate::models::ScalewayRegistryV1Image update_image(region, image_id, inline_object28)
Update an existing image

Update the image associated with the given id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**image_id** | **String** | Image ID to update | [required] |
**inline_object28** | [**InlineObject28**](InlineObject28.md) |  | [required] |

### Return type

[**crate::models::ScalewayRegistryV1Image**](scaleway.registry.v1.Image.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

