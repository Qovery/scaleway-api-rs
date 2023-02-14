# \ServersApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_option_server**](ServersApi.md#add_option_server) | **POST** /baremetal/v1/zones/{zone}/servers/{server_id}/options/{option_id} | Add server option
[**create_server**](ServersApi.md#create_server) | **POST** /baremetal/v1/zones/{zone}/servers | Create an elastic metal server
[**create_server1**](ServersApi.md#create_server1) | **POST** /instance/v1/zones/{zone}/servers | Create a server
[**delete_option_server**](ServersApi.md#delete_option_server) | **DELETE** /baremetal/v1/zones/{zone}/servers/{server_id}/options/{option_id} | Delete server option
[**delete_server**](ServersApi.md#delete_server) | **DELETE** /baremetal/v1/zones/{zone}/servers/{server_id} | Delete an elastic metal server
[**delete_server1**](ServersApi.md#delete_server1) | **DELETE** /instance/v1/zones/{zone}/servers/{server_id} | Delete a server
[**get_server**](ServersApi.md#get_server) | **GET** /baremetal/v1/zones/{zone}/servers/{server_id} | Get a specific elastic metal server
[**get_server1**](ServersApi.md#get_server1) | **GET** /instance/v1/zones/{zone}/servers/{server_id} | Get a server
[**get_server_metrics**](ServersApi.md#get_server_metrics) | **GET** /baremetal/v1/zones/{zone}/servers/{server_id}/metrics | Return server metrics
[**install_server**](ServersApi.md#install_server) | **POST** /baremetal/v1/zones/{zone}/servers/{server_id}/install | Install an elastic metal server
[**list_server_actions**](ServersApi.md#list_server_actions) | **GET** /instance/v1/zones/{zone}/servers/{server_id}/action | List server actions
[**list_server_events**](ServersApi.md#list_server_events) | **GET** /baremetal/v1/zones/{zone}/servers/{server_id}/events | List server events
[**list_servers**](ServersApi.md#list_servers) | **GET** /baremetal/v1/zones/{zone}/servers | List elastic metal servers for organization
[**list_servers1**](ServersApi.md#list_servers1) | **GET** /instance/v1/zones/{zone}/servers | List all servers
[**server_action**](ServersApi.md#server_action) | **POST** /instance/v1/zones/{zone}/servers/{server_id}/action | Perform action
[**update_ip**](ServersApi.md#update_ip) | **PATCH** /baremetal/v1/zones/{zone}/servers/{server_id}/ips/{ip_id} | Update IP
[**update_server**](ServersApi.md#update_server) | **PATCH** /baremetal/v1/zones/{zone}/servers/{server_id} | Update an elastic metal server
[**update_server1**](ServersApi.md#update_server1) | **PATCH** /instance/v1/zones/{zone}/servers/{server_id} | Update a server



## add_option_server

> crate::models::ScalewayPeriodBaremetalPeriodV1PeriodServer add_option_server(zone, server_id, option_id, add_option_server_request)
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

[**crate::models::ScalewayPeriodBaremetalPeriodV1PeriodServer**](scaleway.baremetal.v1.Server.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_server

> crate::models::ScalewayPeriodBaremetalPeriodV1PeriodServer create_server(zone, create_server_request)
Create an elastic metal server

Create a new elastic metal server. Once the server is created, you probably want to install an OS.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**create_server_request** | [**CreateServerRequest**](CreateServerRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodBaremetalPeriodV1PeriodServer**](scaleway.baremetal.v1.Server.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_server1

> crate::models::ScalewayPeriodInstancePeriodV1PeriodCreateServerResponse create_server1(zone, create_server1_request)
Create a server

The `volumes` key is a dictionary composed of the volume position as key and the volume parameters as value. Depending of the volume parameters, you can achieve different behaviours :  Create a volume from a snapshot of an image : Optional : `volume_type`, `size`, `boot`. If the `size` parameter is not set, the size of the volume will equal the size of the corresponding snapshot of the image.  Attach an existing volume : Required : `id`, `name`. Optional : `boot`.  Create an empty volume : Required : `name`, `volume_type`, `size`. Optional : `organization`, `project`, `boot`.  Create a volume from a snapshot : Required : `base_snapshot`, `name`, `volume_type`. Optional : `organization`, `project`, `boot`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**create_server1_request** | [**CreateServer1Request**](CreateServer1Request.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodInstancePeriodV1PeriodCreateServerResponse**](scaleway.instance.v1.CreateServerResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_option_server

> crate::models::ScalewayPeriodBaremetalPeriodV1PeriodServer delete_option_server(zone, server_id, option_id)
Delete server option

Delete an option from a specific server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** | ID of the server | [required] |
**option_id** | **String** | ID of the option to delete | [required] |

### Return type

[**crate::models::ScalewayPeriodBaremetalPeriodV1PeriodServer**](scaleway.baremetal.v1.Server.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_server

> crate::models::ScalewayPeriodBaremetalPeriodV1PeriodServer delete_server(zone, server_id)
Delete an elastic metal server

Delete the server associated with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** | ID of the server to delete | [required] |

### Return type

[**crate::models::ScalewayPeriodBaremetalPeriodV1PeriodServer**](scaleway.baremetal.v1.Server.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_server1

> delete_server1(zone, server_id)
Delete a server

Delete a server with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_server

> crate::models::ScalewayPeriodBaremetalPeriodV1PeriodServer get_server(zone, server_id)
Get a specific elastic metal server

Get the server associated with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** | ID of the server | [required] |

### Return type

[**crate::models::ScalewayPeriodBaremetalPeriodV1PeriodServer**](scaleway.baremetal.v1.Server.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_server1

> crate::models::ScalewayPeriodInstancePeriodV1PeriodGetServerResponse get_server1(zone, server_id)
Get a server

Get the details of a specified Server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** | UUID of the server you want to get | [required] |

### Return type

[**crate::models::ScalewayPeriodInstancePeriodV1PeriodGetServerResponse**](scaleway.instance.v1.GetServerResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_server_metrics

> crate::models::ScalewayPeriodBaremetalPeriodV1PeriodGetServerMetricsResponse get_server_metrics(zone, server_id)
Return server metrics

Give the ping status on the server associated with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** | Server ID to get the metrics | [required] |

### Return type

[**crate::models::ScalewayPeriodBaremetalPeriodV1PeriodGetServerMetricsResponse**](scaleway.baremetal.v1.GetServerMetricsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## install_server

> crate::models::ScalewayPeriodBaremetalPeriodV1PeriodServer install_server(zone, server_id, install_server_request)
Install an elastic metal server

Install an OS on the server associated with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** | Server ID to install | [required] |
**install_server_request** | [**InstallServerRequest**](InstallServerRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodBaremetalPeriodV1PeriodServer**](scaleway.baremetal.v1.Server.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_server_actions

> crate::models::ScalewayPeriodInstancePeriodV1PeriodListServerActionsResponse list_server_actions(zone, server_id)
List server actions

List all actions that can currently be performed on a server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** |  | [required] |

### Return type

[**crate::models::ScalewayPeriodInstancePeriodV1PeriodListServerActionsResponse**](scaleway.instance.v1.ListServerActionsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_server_events

> crate::models::ScalewayPeriodBaremetalPeriodV1PeriodListServerEventsResponse list_server_events(zone, server_id, page, page_size, order_by)
List server events

List events associated to the given server ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** | ID of the server events searched | [required] |
**page** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Number of server events per page |  |[default to 20]
**order_by** | Option<**String**> | Order of the server events |  |[default to created_at_asc]

### Return type

[**crate::models::ScalewayPeriodBaremetalPeriodV1PeriodListServerEventsResponse**](scaleway.baremetal.v1.ListServerEventsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_servers

> crate::models::ScalewayPeriodBaremetalPeriodV1PeriodListServersResponse list_servers(zone, page, page_size, order_by, tags, status, name, organization_id, project_id, option_id)
List elastic metal servers for organization

List elastic metal servers for organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**page** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Number of server per page |  |[default to 20]
**order_by** | Option<**String**> | Order of the servers |  |[default to created_at_asc]
**tags** | Option<[**Vec<String>**](String.md)> | Filter by tags |  |
**status** | Option<[**Vec<String>**](String.md)> | Filter by status |  |
**name** | Option<**String**> | Filter by name |  |
**organization_id** | Option<**String**> | Filter by organization ID |  |
**project_id** | Option<**String**> | Filter by project ID |  |
**option_id** | Option<**String**> | Filter by option ID |  |

### Return type

[**crate::models::ScalewayPeriodBaremetalPeriodV1PeriodListServersResponse**](scaleway.baremetal.v1.ListServersResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_servers1

> crate::models::ScalewayPeriodInstancePeriodV1PeriodListServersResponse list_servers1(zone, per_page, page, organization, project, name, private_ip, without_ip, commercial_type, state, tags, private_network, order)
List all servers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**per_page** | Option<**i32**> | A positive integer lower or equal to 100 to select the number of items to return |  |
**page** | Option<**i32**> | A positive integer to choose the page to return |  |[default to 1]
**organization** | Option<**String**> | List only servers of this organization ID |  |
**project** | Option<**String**> | List only servers of this project ID |  |
**name** | Option<**String**> | Filter servers by name (for eg. \"server1\" will return \"server100\" and \"server1\" but not \"foo\") |  |
**private_ip** | Option<**String**> | List servers by private_ip (IP address) |  |
**without_ip** | Option<**bool**> | List servers that are not attached to a public IP |  |
**commercial_type** | Option<**String**> | List servers of this commercial type |  |
**state** | Option<**String**> | List servers in this state |  |[default to running]
**tags** | Option<**String**> | List servers with these exact tags (to filter with several tags, use commas to separate them) |  |
**private_network** | Option<**String**> | List servers in this Private Network |  |
**order** | Option<**String**> | Define the order of the returned servers |  |[default to creation_date_desc]

### Return type

[**crate::models::ScalewayPeriodInstancePeriodV1PeriodListServersResponse**](scaleway.instance.v1.ListServersResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## server_action

> crate::models::ScalewayPeriodInstancePeriodV1PeriodServerActionResponse server_action(zone, server_id, server_action_request)
Perform action

Perform power related actions on a server. Be wary that when terminating a server, all the attached volumes (local *and* block storage) are deleted. So, if you want to keep your local volumes, you must use the `archive` action instead of `terminate`. And if you want to keep block-storage volumes, **you must** detach it beforehand you issue the `terminate` call.  For more information, read the [Volumes](#volumes-7e8a39) documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** | UUID of the server | [required] |
**server_action_request** | [**ServerActionRequest**](ServerActionRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodInstancePeriodV1PeriodServerActionResponse**](scaleway.instance.v1.ServerActionResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_ip

> crate::models::ScalewayPeriodBaremetalPeriodV1PeriodIp update_ip(zone, server_id, ip_id, update_ip_request)
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

[**crate::models::ScalewayPeriodBaremetalPeriodV1PeriodIp**](scaleway.baremetal.v1.IP.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_server

> crate::models::ScalewayPeriodBaremetalPeriodV1PeriodServer update_server(zone, server_id, update_server_request)
Update an elastic metal server

Update the server associated with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** | ID of the server to update | [required] |
**update_server_request** | [**UpdateServerRequest**](UpdateServerRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodBaremetalPeriodV1PeriodServer**](scaleway.baremetal.v1.Server.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_server1

> crate::models::ScalewayPeriodInstancePeriodV1PeriodUpdateServerResponse update_server1(zone, server_id, update_server1_request)
Update a server

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** | UUID of the server | [required] |
**update_server1_request** | [**UpdateServer1Request**](UpdateServer1Request.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodInstancePeriodV1PeriodUpdateServerResponse**](scaleway.instance.v1.UpdateServerResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

