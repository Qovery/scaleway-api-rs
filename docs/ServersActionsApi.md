# \ServersActionsApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**reboot_server**](ServersActionsApi.md#reboot_server) | **POST** /baremetal/v1/zones/{zone}/servers/{server_id}/reboot | Reboot a baremetal server
[**start_server**](ServersActionsApi.md#start_server) | **POST** /baremetal/v1/zones/{zone}/servers/{server_id}/start | Start a baremetal server
[**stop_server**](ServersActionsApi.md#stop_server) | **POST** /baremetal/v1/zones/{zone}/servers/{server_id}/stop | Stop a baremetal server



## reboot_server

> crate::models::ScalewayBaremetalV1Server reboot_server(zone, server_id, inline_object5)
Reboot a baremetal server

Reboot the server associated with the given ID, use boot param to reboot in rescue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** | ID of the server to reboot | [required] |
**inline_object5** | [**InlineObject5**](InlineObject5.md) |  | [required] |

### Return type

[**crate::models::ScalewayBaremetalV1Server**](scaleway.baremetal.v1.Server.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_server

> crate::models::ScalewayBaremetalV1Server start_server(zone, server_id, inline_object6)
Start a baremetal server

Start the server associated with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** | ID of the server to start | [required] |
**inline_object6** | [**InlineObject6**](InlineObject6.md) |  | [required] |

### Return type

[**crate::models::ScalewayBaremetalV1Server**](scaleway.baremetal.v1.Server.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_server

> crate::models::ScalewayBaremetalV1Server stop_server(zone, server_id, body)
Stop a baremetal server

Stop the server associated with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** | ID of the server to stop | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::ScalewayBaremetalV1Server**](scaleway.baremetal.v1.Server.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

