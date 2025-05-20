# \NamespacesApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_namespace**](NamespacesApi.md#create_namespace) | **POST** /registry/v1/regions/{region}/namespaces | Create a namespace
[**delete_namespace**](NamespacesApi.md#delete_namespace) | **DELETE** /registry/v1/regions/{region}/namespaces/{namespace_id} | Delete a namespace
[**get_namespace**](NamespacesApi.md#get_namespace) | **GET** /registry/v1/regions/{region}/namespaces/{namespace_id} | Get a namespace
[**list_namespaces**](NamespacesApi.md#list_namespaces) | **GET** /registry/v1/regions/{region}/namespaces | List namespaces
[**update_namespace**](NamespacesApi.md#update_namespace) | **PATCH** /registry/v1/regions/{region}/namespaces/{namespace_id} | Update a namespace



## create_namespace

> models::ScalewayPeriodRegistryPeriodV1PeriodNamespace create_namespace(region, create_namespace_request)
Create a namespace

Create a new Container Registry namespace. You must specify the namespace name and region in which you want it to be created. Optionally, you can specify the `project_id` and `is_public` in the request payload.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**create_namespace_request** | [**CreateNamespaceRequest**](CreateNamespaceRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodRegistryPeriodV1PeriodNamespace**](scaleway.registry.v1.Namespace.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_namespace

> models::ScalewayPeriodRegistryPeriodV1PeriodNamespace delete_namespace(region, namespace_id)
Delete a namespace

Delete a given namespace. You must specify, in the endpoint, the `region` and `namespace_id` parameters of the namespace you want to delete.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**namespace_id** | **String** | UUID of the namespace. | [required] |

### Return type

[**models::ScalewayPeriodRegistryPeriodV1PeriodNamespace**](scaleway.registry.v1.Namespace.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_namespace

> models::ScalewayPeriodRegistryPeriodV1PeriodNamespace get_namespace(region, namespace_id)
Get a namespace

Retrieve information about a given namespace, specified by its `namespace_id` and region. Full details about the namespace, such as `description`, `project_id`, `status`, `endpoint`, `is_public`, `size`, and `image_count` are returned in the response.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**namespace_id** | **String** | UUID of the namespace. | [required] |

### Return type

[**models::ScalewayPeriodRegistryPeriodV1PeriodNamespace**](scaleway.registry.v1.Namespace.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_namespaces

> models::ScalewayPeriodRegistryPeriodV1PeriodListNamespacesResponse list_namespaces(region, page, page_size, order_by, organization_id, project_id, name)
List namespaces

List all namespaces in a specified region. By default, the namespaces listed are ordered by creation date in ascending order. This can be modified via the order_by field. You can also define additional parameters for your query, such as the `instance_id` and `project_id` parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**page** | Option<**i32**> | A positive integer to choose the page to display. |  |
**page_size** | Option<**i32**> | A positive integer lower or equal to 100 to select the number of items to display. |  |
**order_by** | Option<**String**> | Criteria to use when ordering namespace listings. Possible values are `created_at_asc`, `created_at_desc`, `name_asc`, `name_desc`, `region`, `status_asc` and `status_desc`. The default value is `created_at_asc`. |  |[default to created_at_asc]
**organization_id** | Option<**String**> | Filter by Organization ID. |  |
**project_id** | Option<**String**> | Filter by Project ID. |  |
**name** | Option<**String**> | Filter by the namespace name (exact match). |  |

### Return type

[**models::ScalewayPeriodRegistryPeriodV1PeriodListNamespacesResponse**](scaleway.registry.v1.ListNamespacesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_namespace

> models::ScalewayPeriodRegistryPeriodV1PeriodNamespace update_namespace(region, namespace_id, update_namespace_request)
Update a namespace

Update the parameters of a given namespace, specified by its `namespace_id` and `region`. You can update the `description` and `is_public` parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**namespace_id** | **String** | ID of the namespace to update. | [required] |
**update_namespace_request** | [**UpdateNamespaceRequest**](UpdateNamespaceRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodRegistryPeriodV1PeriodNamespace**](scaleway.registry.v1.Namespace.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

