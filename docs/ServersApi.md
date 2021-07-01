# \ServersApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_server**](ServersApi.md#create_server) | **post** /instance/v1/zones/{zone}/servers | Create a server
[**create_server1**](ServersApi.md#create_server1) | **post** /apple-silicon/v1alpha1/zones/{zone}/servers | Create a server
[**delete_server**](ServersApi.md#delete_server) | **delete** /instance/v1/zones/{zone}/servers/{server_id} | Delete a server
[**delete_server1**](ServersApi.md#delete_server1) | **delete** /apple-silicon/v1alpha1/zones/{zone}/servers/{server_id} | Delete a server
[**get_server**](ServersApi.md#get_server) | **get** /instance/v1/zones/{zone}/servers/{server_id} | Get a server
[**get_server1**](ServersApi.md#get_server1) | **get** /apple-silicon/v1alpha1/zones/{zone}/servers/{server_id} | Get a server
[**list_server_actions**](ServersApi.md#list_server_actions) | **get** /instance/v1/zones/{zone}/servers/{server_id}/action | List server actions
[**list_servers**](ServersApi.md#list_servers) | **get** /instance/v1/zones/{zone}/servers | List all servers
[**list_servers1**](ServersApi.md#list_servers1) | **get** /apple-silicon/v1alpha1/zones/{zone}/servers | List all servers
[**reboot_server**](ServersApi.md#reboot_server) | **post** /apple-silicon/v1alpha1/zones/{zone}/servers/{server_id}/reboot | Reboot a server
[**reinstall_server**](ServersApi.md#reinstall_server) | **post** /apple-silicon/v1alpha1/zones/{zone}/servers/{server_id}/reinstall | Reinstall a server
[**server_action**](ServersApi.md#server_action) | **post** /instance/v1/zones/{zone}/servers/{server_id}/action | Perform action
[**update_server**](ServersApi.md#update_server) | **patch** /instance/v1/zones/{zone}/servers/{server_id} | Update a server
[**update_server1**](ServersApi.md#update_server1) | **patch** /apple-silicon/v1alpha1/zones/{zone}/servers/{server_id} | Update a server



## create_server

> crate::models::ScalewayInstanceV1CreateServerResponse create_server(zone, inline_object13)
Create a server

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**inline_object13** | [**InlineObject13**](InlineObject13.md) |  | [required] |

### Return type

[**crate::models::ScalewayInstanceV1CreateServerResponse**](scaleway.instance.v1.CreateServerResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_server1

> crate::models::ScalewayAppleSiliconV1alpha1Server create_server1(zone, inline_object33)
Create a server

Create a server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The global you want to target | [required] |
**inline_object33** | [**InlineObject33**](InlineObject33.md) |  | [required] |

### Return type

[**crate::models::ScalewayAppleSiliconV1alpha1Server**](scaleway.apple_silicon.v1alpha1.Server.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_server

> delete_server(zone, server_id)
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


## delete_server1

> delete_server1(zone, server_id)
Delete a server

Delete a server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The global you want to target | [required] |
**server_id** | **String** | UUID of the server you want to delete | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_server

> crate::models::ScalewayInstanceV1GetServerResponse get_server(zone, server_id)
Get a server

Get the details of a specified Server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** | UUID of the server you want to get | [required] |

### Return type

[**crate::models::ScalewayInstanceV1GetServerResponse**](scaleway.instance.v1.GetServerResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_server1

> crate::models::ScalewayAppleSiliconV1alpha1Server get_server1(zone, server_id)
Get a server

Get a server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The global you want to target | [required] |
**server_id** | **String** | UUID of the server you want to get | [required] |

### Return type

[**crate::models::ScalewayAppleSiliconV1alpha1Server**](scaleway.apple_silicon.v1alpha1.Server.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_server_actions

> crate::models::ScalewayInstanceV1ListServerActionsResponse list_server_actions(zone, server_id)
List server actions

Liste all actions that can currently be performed on a server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** |  | [required] |

### Return type

[**crate::models::ScalewayInstanceV1ListServerActionsResponse**](scaleway.instance.v1.ListServerActionsResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_servers

> crate::models::ScalewayInstanceV1ListServersResponse list_servers(zone, per_page, page, organization, project, name, private_ip, without_ip, commercial_type, state, tags, private_network, order)
List all servers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**per_page** | Option<**f32**> | A positive integer lower or equal to 100 to select the number of items to return |  |
**page** | Option<**f32**> | A positive integer to choose the page to return |  |[default to 1]
**organization** | Option<**String**> | List only servers of this organization ID |  |
**project** | Option<**String**> | List only servers of this project ID |  |
**name** | Option<**String**> | Filter servers by name (for eg. \"server1\" will return \"server100\" and \"server1\" but not \"foo\") |  |
**private_ip** | Option<**String**> | List servers by private_ip (IP address) |  |
**without_ip** | Option<**bool**> | List servers that are not attached to a public IP |  |
**commercial_type** | Option<**String**> | List servers of this commercial type |  |
**state** | Option<**String**> | List servers in this state |  |[default to running]
**tags** | Option<**String**> | List servers with these exact tags |  |
**private_network** | Option<**String**> | List servers in this Private Network |  |
**order** | Option<**String**> | Define the order of the returned servers |  |[default to creation_date_desc]

### Return type

[**crate::models::ScalewayInstanceV1ListServersResponse**](scaleway.instance.v1.ListServersResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_servers1

> crate::models::ScalewayAppleSiliconV1alpha1ListServersResponse list_servers1(zone, order_by, project_id, organization_id, page, page_size)
List all servers

List all servers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The global you want to target | [required] |
**order_by** | Option<**String**> | The sort order of the returned servers |  |[default to created_at_asc]
**project_id** | Option<**String**> | List only servers of this project ID |  |
**organization_id** | Option<**String**> | List only servers of this organization ID |  |
**page** | Option<**f32**> | A positive integer to choose the page to return |  |[default to 1]
**page_size** | Option<**f32**> | A positive integer lower or equal to 100 to select the number of items to return |  |

### Return type

[**crate::models::ScalewayAppleSiliconV1alpha1ListServersResponse**](scaleway.apple_silicon.v1alpha1.ListServersResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reboot_server

> crate::models::ScalewayAppleSiliconV1alpha1Server reboot_server(zone, server_id, body)
Reboot a server

Reboot a server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The global you want to target | [required] |
**server_id** | **String** | UUID of the server you want to reboot | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::ScalewayAppleSiliconV1alpha1Server**](scaleway.apple_silicon.v1alpha1.Server.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reinstall_server

> crate::models::ScalewayAppleSiliconV1alpha1Server reinstall_server(zone, server_id, body)
Reinstall a server

Reinstall a server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The global you want to target | [required] |
**server_id** | **String** | UUID of the server you want to reinstall | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::ScalewayAppleSiliconV1alpha1Server**](scaleway.apple_silicon.v1alpha1.Server.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## server_action

> crate::models::ScalewayInstanceV1ServerActionResponse server_action(zone, server_id, inline_object15)
Perform action

Perform power related actions on a server. Be wary that when terminating a server, all the attached volumes (local *and* block storage) are deleted. So, if you want to keep your local volumes, you must use the `archive` action instead of `terminate`. And if you want to keep block-storage volumes, **you must** detach it beforehand you issue the `terminate` call.  For more information, read the [Volumes](#volumes-7e8a39) documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** | UUID of the server | [required] |
**inline_object15** | [**InlineObject15**](InlineObject15.md) |  | [required] |

### Return type

[**crate::models::ScalewayInstanceV1ServerActionResponse**](scaleway.instance.v1.ServerActionResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_server

> crate::models::ScalewayInstanceV1UpdateServerResponse update_server(zone, server_id, inline_object14)
Update a server

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** | UUID of the server | [required] |
**inline_object14** | [**InlineObject14**](InlineObject14.md) |  | [required] |

### Return type

[**crate::models::ScalewayInstanceV1UpdateServerResponse**](scaleway.instance.v1.UpdateServerResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_server1

> crate::models::ScalewayAppleSiliconV1alpha1Server update_server1(zone, server_id, inline_object34)
Update a server

Update a server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The global you want to target | [required] |
**server_id** | **String** | UUID of the server you want to update | [required] |
**inline_object34** | [**InlineObject34**](InlineObject34.md) |  | [required] |

### Return type

[**crate::models::ScalewayAppleSiliconV1alpha1Server**](scaleway.apple_silicon.v1alpha1.Server.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

