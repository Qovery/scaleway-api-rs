# \ImagesApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_image**](ImagesApi.md#create_image) | **post** /instance/v1/zones/{zone}/images | Create an instance image
[**delete_image**](ImagesApi.md#delete_image) | **delete** /instance/v1/zones/{zone}/images/{image_id} | Delete an instance image
[**delete_image1**](ImagesApi.md#delete_image1) | **delete** /registry/v1/regions/{region}/images/{image_id} | Delete an image
[**get_image**](ImagesApi.md#get_image) | **get** /instance/v1/zones/{zone}/images/{image_id} | Get an instance image
[**get_image1**](ImagesApi.md#get_image1) | **get** /registry/v1/regions/{region}/images/{image_id} | Get a image
[**list_images**](ImagesApi.md#list_images) | **get** /instance/v1/zones/{zone}/images | List instance images
[**list_images1**](ImagesApi.md#list_images1) | **get** /registry/v1/regions/{region}/images | List all your images
[**set_image**](ImagesApi.md#set_image) | **put** /instance/v1/zones/{zone}/images/{id} | Update image
[**update_image**](ImagesApi.md#update_image) | **patch** /registry/v1/regions/{region}/images/{image_id} | Update an existing image



## create_image

> crate::models::ScalewayInstanceV1CreateImageResponse create_image(zone, inline_object)
Create an instance image

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**inline_object** | [**InlineObject**](InlineObject.md) |  | [required] |

### Return type

[**crate::models::ScalewayInstanceV1CreateImageResponse**](scaleway.instance.v1.CreateImageResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_image

> delete_image(zone, image_id)
Delete an instance image

Delete the image with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**image_id** | **String** | UUID of the image you want to delete | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_image1

> crate::models::ScalewayRegistryV1Image delete_image1(region, image_id)
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

> crate::models::ScalewayInstanceV1GetImageResponse get_image(zone, image_id)
Get an instance image

Get details of an image with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**image_id** | **String** | UUID of the image you want to get | [required] |

### Return type

[**crate::models::ScalewayInstanceV1GetImageResponse**](scaleway.instance.v1.GetImageResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_image1

> crate::models::ScalewayRegistryV1Image get_image1(region, image_id)
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

> crate::models::ScalewayInstanceV1ListImagesResponse list_images(zone, organization, per_page, page, name, public, arch, project)
List instance images

List all images available in an account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**organization** | Option<**String**> |  |  |
**per_page** | Option<**f32**> |  |  |
**page** | Option<**f32**> | Page number |  |[default to 1]
**name** | Option<**String**> |  |  |
**public** | Option<**bool**> |  |  |
**arch** | Option<**String**> |  |  |
**project** | Option<**String**> |  |  |

### Return type

[**crate::models::ScalewayInstanceV1ListImagesResponse**](scaleway.instance.v1.ListImagesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_images1

> crate::models::ScalewayRegistryV1ListImagesResponse list_images1(region, page, page_size, order_by, namespace_id, name, organization_id, project_id)
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


## set_image

> crate::models::ScalewayInstanceV1SetImageResponse set_image(zone, id, inline_object1)
Update image

Replace all image properties with an image message.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**id** | **String** |  | [required] |
**inline_object1** | [**InlineObject1**](InlineObject1.md) |  | [required] |

### Return type

[**crate::models::ScalewayInstanceV1SetImageResponse**](scaleway.instance.v1.SetImageResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_image

> crate::models::ScalewayRegistryV1Image update_image(region, image_id, inline_object22)
Update an existing image

Update the image associated with the given id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**image_id** | **String** | Image ID to update | [required] |
**inline_object22** | [**InlineObject22**](InlineObject22.md) |  | [required] |

### Return type

[**crate::models::ScalewayRegistryV1Image**](scaleway.registry.v1.Image.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

