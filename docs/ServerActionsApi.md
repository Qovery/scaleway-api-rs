# \ServerActionsApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**reboot_server**](ServerActionsApi.md#reboot_server) | **POST** /baremetal/v1/zones/{zone}/servers/{server_id}/reboot | Reboot an elastic metal server
[**start_server**](ServerActionsApi.md#start_server) | **POST** /baremetal/v1/zones/{zone}/servers/{server_id}/start | Start an elastic metal server
[**stop_server**](ServerActionsApi.md#stop_server) | **POST** /baremetal/v1/zones/{zone}/servers/{server_id}/stop | Stop an elastic metal server



## reboot_server

> crate::models::ScalewayBaremetalV1Server reboot_server(zone, server_id, reboot_server_request)
Reboot an elastic metal server

Reboot the server associated with the given ID, use boot param to reboot in rescue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** | ID of the server to reboot | [required] |
**reboot_server_request** | [**RebootServerRequest**](RebootServerRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayBaremetalV1Server**](scaleway.baremetal.v1.Server.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_server

> crate::models::ScalewayBaremetalV1Server start_server(zone, server_id, reboot_server_request)
Start an elastic metal server

Start the server associated with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** | ID of the server to start | [required] |
**reboot_server_request** | [**RebootServerRequest**](RebootServerRequest.md) |  | [required] |

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
Stop an elastic metal server

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

