# \ProjectsApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_project**](ProjectsApi.md#create_project) | **POST** /account/v2/projects | Create project
[**delete_project**](ProjectsApi.md#delete_project) | **DELETE** /account/v2/projects/{project_id} | Delete project
[**get_project**](ProjectsApi.md#get_project) | **GET** /account/v2/projects/{project_id} | Get project
[**list_projects**](ProjectsApi.md#list_projects) | **GET** /account/v2/projects | List projects
[**update_project**](ProjectsApi.md#update_project) | **PATCH** /account/v2/projects/{project_id} | Update project



## create_project

> crate::models::ScalewayPeriodAccountPeriodV2PeriodProject create_project(create_project_request)
Create project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_project_request** | [**CreateProjectRequest**](CreateProjectRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodAccountPeriodV2PeriodProject**](scaleway.account.v2.Project.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_project

> delete_project(project_id)
Delete project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The project ID of the project (UUID format) | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project

> crate::models::ScalewayPeriodAccountPeriodV2PeriodProject get_project(project_id)
Get project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The project ID of the project (UUID format) | [required] |

### Return type

[**crate::models::ScalewayPeriodAccountPeriodV2PeriodProject**](scaleway.account.v2.Project.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_projects

> crate::models::ScalewayPeriodAccountPeriodV2PeriodListProjectsResponse list_projects(organization_id, name, page, page_size, order_by, project_ids)
List projects

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | Option<**String**> | The organization ID of the project (UUID format) |  |
**name** | Option<**String**> | The name of the project |  |
**page** | Option<**i32**> | The page number for the returned projects |  |[default to 1]
**page_size** | Option<**i32**> | The maximum number of project per page |  |[default to 20]
**order_by** | Option<**String**> | The sort order of the returned projects |  |[default to created_at_asc]
**project_ids** | Option<[**Vec<String>**](String.md)> | Filter out by a list of project ID |  |

### Return type

[**crate::models::ScalewayPeriodAccountPeriodV2PeriodListProjectsResponse**](scaleway.account.v2.ListProjectsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_project

> crate::models::ScalewayPeriodAccountPeriodV2PeriodProject update_project(project_id, update_project_request)
Update project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **String** | The project ID of the project (UUID format) | [required] |
**update_project_request** | [**UpdateProjectRequest**](UpdateProjectRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodAccountPeriodV2PeriodProject**](scaleway.account.v2.Project.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

