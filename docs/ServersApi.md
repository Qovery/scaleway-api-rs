# \ServersApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_option_server**](ServersApi.md#add_option_server) | **POST** /baremetal/v1/zones/{zone}/servers/{server_id}/options/{option_id} | Add server option
[**create_server**](ServersApi.md#create_server) | **POST** /baremetal/v1/zones/{zone}/servers | Create an Elastic Metal server
[**delete_option_server**](ServersApi.md#delete_option_server) | **DELETE** /baremetal/v1/zones/{zone}/servers/{server_id}/options/{option_id} | Delete server option
[**delete_server**](ServersApi.md#delete_server) | **DELETE** /baremetal/v1/zones/{zone}/servers/{server_id} | Delete an Elastic Metal server
[**get_server**](ServersApi.md#get_server) | **GET** /baremetal/v1/zones/{zone}/servers/{server_id} | Get a specific Elastic Metal server
[**get_server_metrics**](ServersApi.md#get_server_metrics) | **GET** /baremetal/v1/zones/{zone}/servers/{server_id}/metrics | Return server metrics
[**install_server**](ServersApi.md#install_server) | **POST** /baremetal/v1/zones/{zone}/servers/{server_id}/install | Install an Elastic Metal server
[**list_server_events**](ServersApi.md#list_server_events) | **GET** /baremetal/v1/zones/{zone}/servers/{server_id}/events | List server events
[**list_servers**](ServersApi.md#list_servers) | **GET** /baremetal/v1/zones/{zone}/servers | List Elastic Metal servers for an Organization
[**migrate_server_to_monthly_offer**](ServersApi.md#migrate_server_to_monthly_offer) | **POST** /baremetal/v1/zones/{zone}/servers/{server_id}/migrate-offer-monthly | Migrate server offer
[**update_ip**](ServersApi.md#update_ip) | **PATCH** /baremetal/v1/zones/{zone}/servers/{server_id}/ips/{ip_id} | Update IP
[**update_server**](ServersApi.md#update_server) | **PATCH** /baremetal/v1/zones/{zone}/servers/{server_id} | Update an Elastic Metal server



## add_option_server

> models::ScalewayPeriodBaremetalPeriodV1PeriodServer add_option_server(zone, server_id, option_id, add_option_server_request)
Add server option

Add an option, such as Private Networks, to a specific server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** | ID of the server. | [required] |
**option_id** | **String** | ID of the option to add. | [required] |
**add_option_server_request** | [**AddOptionServerRequest**](AddOptionServerRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodBaremetalPeriodV1PeriodServer**](scaleway.baremetal.v1.Server.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_server

> models::ScalewayPeriodBaremetalPeriodV1PeriodServer create_server(zone, create_server_request)
Create an Elastic Metal server

Create a new Elastic Metal server. Once the server is created, proceed with the [installation of an OS](#post-3e949e).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**create_server_request** | [**CreateServerRequest**](CreateServerRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodBaremetalPeriodV1PeriodServer**](scaleway.baremetal.v1.Server.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_option_server

> models::ScalewayPeriodBaremetalPeriodV1PeriodServer delete_option_server(zone, server_id, option_id)
Delete server option

Delete an option from a specific server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** | ID of the server. | [required] |
**option_id** | **String** | ID of the option to delete. | [required] |

### Return type

[**models::ScalewayPeriodBaremetalPeriodV1PeriodServer**](scaleway.baremetal.v1.Server.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_server

> models::ScalewayPeriodBaremetalPeriodV1PeriodServer delete_server(zone, server_id)
Delete an Elastic Metal server

Delete the server associated with the ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** | ID of the server to delete. | [required] |

### Return type

[**models::ScalewayPeriodBaremetalPeriodV1PeriodServer**](scaleway.baremetal.v1.Server.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_server

> models::ScalewayPeriodBaremetalPeriodV1PeriodServer get_server(zone, server_id)
Get a specific Elastic Metal server

Get full details of an existing Elastic Metal server associated with the ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** | ID of the server. | [required] |

### Return type

[**models::ScalewayPeriodBaremetalPeriodV1PeriodServer**](scaleway.baremetal.v1.Server.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_server_metrics

> models::ScalewayPeriodBaremetalPeriodV1PeriodGetServerMetricsResponse get_server_metrics(zone, server_id)
Return server metrics

Get the ping status of the server associated with the ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** | Server ID to get the metrics. | [required] |

### Return type

[**models::ScalewayPeriodBaremetalPeriodV1PeriodGetServerMetricsResponse**](scaleway.baremetal.v1.GetServerMetricsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## install_server

> models::ScalewayPeriodBaremetalPeriodV1PeriodServer install_server(zone, server_id, install_server_request)
Install an Elastic Metal server

Install an Operating System (OS) on the Elastic Metal server with a specific ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** | Server ID to install. | [required] |
**install_server_request** | [**InstallServerRequest**](InstallServerRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodBaremetalPeriodV1PeriodServer**](scaleway.baremetal.v1.Server.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_server_events

> models::ScalewayPeriodBaremetalPeriodV1PeriodListServerEventsResponse list_server_events(zone, server_id, page, page_size, order_by)
List server events

List event (i.e. start/stop/reboot) associated to the server ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** | ID of the server events searched. | [required] |
**page** | Option<**i32**> | Page number. |  |
**page_size** | Option<**i32**> | Number of server events per page. |  |
**order_by** | Option<**String**> | Order of the server events. |  |[default to created_at_asc]

### Return type

[**models::ScalewayPeriodBaremetalPeriodV1PeriodListServerEventsResponse**](scaleway.baremetal.v1.ListServerEventsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_servers

> models::ScalewayPeriodBaremetalPeriodV1PeriodListServersResponse list_servers(zone, page, page_size, order_by, tags, status, name, organization_id, project_id, option_id)
List Elastic Metal servers for an Organization

List Elastic Metal servers for a specific Organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**page** | Option<**i32**> | Page number. |  |
**page_size** | Option<**i32**> | Number of servers per page. |  |
**order_by** | Option<**String**> | Order of the servers. |  |[default to created_at_asc]
**tags** | Option<[**Vec<String>**](String.md)> | Tags to filter for. |  |
**status** | Option<[**Vec<String>**](String.md)> | Status to filter for. |  |
**name** | Option<**String**> | Names to filter for. |  |
**organization_id** | Option<**String**> | Organization ID to filter for. |  |
**project_id** | Option<**String**> | Project ID to filter for. |  |
**option_id** | Option<**String**> | Option ID to filter for. |  |

### Return type

[**models::ScalewayPeriodBaremetalPeriodV1PeriodListServersResponse**](scaleway.baremetal.v1.ListServersResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## migrate_server_to_monthly_offer

> models::ScalewayPeriodBaremetalPeriodV1PeriodServer migrate_server_to_monthly_offer(zone, server_id)
Migrate server offer

Migrate server with hourly offer to monthly offer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** | ID of the server. | [required] |

### Return type

[**models::ScalewayPeriodBaremetalPeriodV1PeriodServer**](scaleway.baremetal.v1.Server.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_ip

> models::ScalewayPeriodBaremetalPeriodV1PeriodIp update_ip(zone, server_id, ip_id, update_ip_request)
Update IP

Configure the IP address associated with the server ID and IP ID. You can use this method to set a reverse DNS for an IP address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** | ID of the server. | [required] |
**ip_id** | **String** | ID of the IP to update. | [required] |
**update_ip_request** | [**UpdateIpRequest**](UpdateIpRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodBaremetalPeriodV1PeriodIp**](scaleway.baremetal.v1.IP.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_server

> models::ScalewayPeriodBaremetalPeriodV1PeriodServer update_server(zone, server_id, update_server_request)
Update an Elastic Metal server

Update the server associated with the ID. You can update parameters such as the server's name, tags and description. Any parameters left null in the request body are not updated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** | ID of the server to update. | [required] |
**update_server_request** | [**UpdateServerRequest**](UpdateServerRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodBaremetalPeriodV1PeriodServer**](scaleway.baremetal.v1.Server.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

