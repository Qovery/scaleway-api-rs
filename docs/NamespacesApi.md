# \NamespacesApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_namespace**](NamespacesApi.md#create_namespace) | **POST** /registry/v1/regions/{region}/namespaces | Create a new namespace
[**delete_namespace**](NamespacesApi.md#delete_namespace) | **DELETE** /registry/v1/regions/{region}/namespaces/{namespace_id} | Delete an existing namespace
[**get_namespace**](NamespacesApi.md#get_namespace) | **GET** /registry/v1/regions/{region}/namespaces/{namespace_id} | Get a namespace
[**list_namespaces**](NamespacesApi.md#list_namespaces) | **GET** /registry/v1/regions/{region}/namespaces | List all your namespaces
[**update_namespace**](NamespacesApi.md#update_namespace) | **PATCH** /registry/v1/regions/{region}/namespaces/{namespace_id} | Update an existing namespace



## create_namespace

> crate::models::ScalewayRegistryV1Namespace create_namespace(region, inline_object29)
Create a new namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**inline_object29** | [**InlineObject29**](InlineObject29.md) |  | [required] |

### Return type

[**crate::models::ScalewayRegistryV1Namespace**](scaleway.registry.v1.Namespace.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_namespace

> crate::models::ScalewayRegistryV1Namespace delete_namespace(region, namespace_id)
Delete an existing namespace

Delete the namespace associated with the given id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**namespace_id** | **String** | The unique ID of the Namespace | [required] |

### Return type

[**crate::models::ScalewayRegistryV1Namespace**](scaleway.registry.v1.Namespace.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_namespace

> crate::models::ScalewayRegistryV1Namespace get_namespace(region, namespace_id)
Get a namespace

Get the namespace associated with the given id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**namespace_id** | **String** | The unique ID of the Namespace | [required] |

### Return type

[**crate::models::ScalewayRegistryV1Namespace**](scaleway.registry.v1.Namespace.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_namespaces

> crate::models::ScalewayRegistryV1ListNamespacesResponse list_namespaces(region, page, page_size, order_by, organization_id, project_id, name)
List all your namespaces

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**page** | Option<**f32**> | A positive integer to choose the page to display |  |[default to 1]
**page_size** | Option<**f32**> | A positive integer lower or equal to 100 to select the number of items to display |  |[default to 20]
**order_by** | Option<**String**> | Field by which to order the display of Images |  |[default to created_at_asc]
**organization_id** | Option<**String**> | Filter by Organization ID |  |
**project_id** | Option<**String**> | Filter by Project ID |  |
**name** | Option<**String**> | Filter by the namespace name (exact match) |  |

### Return type

[**crate::models::ScalewayRegistryV1ListNamespacesResponse**](scaleway.registry.v1.ListNamespacesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_namespace

> crate::models::ScalewayRegistryV1Namespace update_namespace(region, namespace_id, inline_object30)
Update an existing namespace

Update the namespace associated with the given id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**namespace_id** | **String** | Namespace ID to update | [required] |
**inline_object30** | [**InlineObject30**](InlineObject30.md) |  | [required] |

### Return type

[**crate::models::ScalewayRegistryV1Namespace**](scaleway.registry.v1.Namespace.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

