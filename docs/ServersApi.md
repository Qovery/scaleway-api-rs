# \ServersApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_option_server**](ServersApi.md#add_option_server) | **POST** /baremetal/v1/zones/{zone}/servers/{server_id}/options/{option_id} | Add server option
[**create_server**](ServersApi.md#create_server) | **POST** /baremetal/v1/zones/{zone}/servers | Create an elastic metal server
[**delete_option_server**](ServersApi.md#delete_option_server) | **DELETE** /baremetal/v1/zones/{zone}/servers/{server_id}/options/{option_id} | Delete server option
[**delete_server**](ServersApi.md#delete_server) | **DELETE** /baremetal/v1/zones/{zone}/servers/{server_id} | Delete an elastic metal server
[**get_server**](ServersApi.md#get_server) | **GET** /baremetal/v1/zones/{zone}/servers/{server_id} | Get a specific elastic metal server
[**get_server_metrics**](ServersApi.md#get_server_metrics) | **GET** /baremetal/v1/zones/{zone}/servers/{server_id}/metrics | Return server metrics
[**install_server**](ServersApi.md#install_server) | **POST** /baremetal/v1/zones/{zone}/servers/{server_id}/install | Install an elastic metal server
[**list_server_events**](ServersApi.md#list_server_events) | **GET** /baremetal/v1/zones/{zone}/servers/{server_id}/events | List server events
[**list_servers**](ServersApi.md#list_servers) | **GET** /baremetal/v1/zones/{zone}/servers | List elastic metal servers for organization
[**update_ip**](ServersApi.md#update_ip) | **PATCH** /baremetal/v1/zones/{zone}/servers/{server_id}/ips/{ip_id} | Update IP
[**update_server**](ServersApi.md#update_server) | **PATCH** /baremetal/v1/zones/{zone}/servers/{server_id} | Update an elastic metal server



## add_option_server

> crate::models::ScalewayBaremetalV1Server add_option_server(zone, server_id, option_id, add_option_server_request)
Add server option

Add an option to a specific server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** | ID of the server | [required] |
**option_id** | **String** | ID of the option to add | [required] |
**add_option_server_request** | [**AddOptionServerRequest**](AddOptionServerRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayBaremetalV1Server**](scaleway.baremetal.v1.Server.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_server

> crate::models::ScalewayBaremetalV1Server create_server(zone, create_server_request)
Create an elastic metal server

Create a new elastic metal server. Once the server is created, you probably want to install an OS.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**create_server_request** | [**CreateServerRequest**](CreateServerRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayBaremetalV1Server**](scaleway.baremetal.v1.Server.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_option_server

> crate::models::ScalewayBaremetalV1Server delete_option_server(zone, server_id, option_id)
Delete server option

Delete an option from a specific server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** | ID of the server | [required] |
**option_id** | **String** | ID of the option to delete | [required] |

### Return type

[**crate::models::ScalewayBaremetalV1Server**](scaleway.baremetal.v1.Server.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_server

> crate::models::ScalewayBaremetalV1Server delete_server(zone, server_id)
Delete an elastic metal server

Delete the server associated with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** | ID of the server to delete | [required] |

### Return type

[**crate::models::ScalewayBaremetalV1Server**](scaleway.baremetal.v1.Server.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_server

> crate::models::ScalewayBaremetalV1Server get_server(zone, server_id)
Get a specific elastic metal server

Get the server associated with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** | ID of the server | [required] |

### Return type

[**crate::models::ScalewayBaremetalV1Server**](scaleway.baremetal.v1.Server.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_server_metrics

> crate::models::ScalewayBaremetalV1GetServerMetricsResponse get_server_metrics(zone, server_id)
Return server metrics

Give the ping status on the server associated with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** | Server ID to get the metrics | [required] |

### Return type

[**crate::models::ScalewayBaremetalV1GetServerMetricsResponse**](scaleway.baremetal.v1.GetServerMetricsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## install_server

> crate::models::ScalewayBaremetalV1Server install_server(zone, server_id, install_server_request)
Install an elastic metal server

Install an OS on the server associated with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** | Server ID to install | [required] |
**install_server_request** | [**InstallServerRequest**](InstallServerRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayBaremetalV1Server**](scaleway.baremetal.v1.Server.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_server_events

> crate::models::ScalewayBaremetalV1ListServerEventsResponse list_server_events(zone, server_id, page, page_size, order_by)
List server events

List events associated to the given server ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** | ID of the server events searched | [required] |
**page** | Option<**i64**> | Page number |  |[default to 1]
**page_size** | Option<**i64**> | Number of server events per page |  |[default to 20]
**order_by** | Option<**String**> | Order of the server events |  |[default to created_at_asc]

### Return type

[**crate::models::ScalewayBaremetalV1ListServerEventsResponse**](scaleway.baremetal.v1.ListServerEventsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_servers

> crate::models::ScalewayBaremetalV1ListServersResponse list_servers(zone, page, page_size, order_by, tags, status, name, organization_id, project_id, option_id)
List elastic metal servers for organization

List elastic metal servers for organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**page** | Option<**i64**> | Page number |  |[default to 1]
**page_size** | Option<**i64**> | Number of server per page |  |[default to 20]
**order_by** | Option<**String**> | Order of the servers |  |[default to created_at_asc]
**tags** | Option<[**Vec<String>**](String.md)> | Filter by tags |  |
**status** | Option<[**Vec<String>**](String.md)> | Filter by status |  |
**name** | Option<**String**> | Filter by name |  |
**organization_id** | Option<**String**> | Filter by organization ID |  |
**project_id** | Option<**String**> | Filter by project ID |  |
**option_id** | Option<**String**> | Filter by option ID |  |

### Return type

[**crate::models::ScalewayBaremetalV1ListServersResponse**](scaleway.baremetal.v1.ListServersResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_ip

> crate::models::ScalewayBaremetalV1Ip update_ip(zone, server_id, ip_id, update_ip_request)
Update IP

Configure ip associated with the given server ID and ipID. You can use this method to set a reverse dns for an IP.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** | ID of the server | [required] |
**ip_id** | **String** | ID of the IP to update | [required] |
**update_ip_request** | [**UpdateIpRequest**](UpdateIpRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayBaremetalV1Ip**](scaleway.baremetal.v1.IP.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_server

> crate::models::ScalewayBaremetalV1Server update_server(zone, server_id, update_server_request)
Update an elastic metal server

Update the server associated with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** | ID of the server to update | [required] |
**update_server_request** | [**UpdateServerRequest**](UpdateServerRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayBaremetalV1Server**](scaleway.baremetal.v1.Server.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

