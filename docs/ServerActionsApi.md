# \ServerActionsApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**reboot_server**](ServerActionsApi.md#reboot_server) | **POST** /baremetal/v1/zones/{zone}/servers/{server_id}/reboot | Reboot an Elastic Metal server
[**start_server**](ServerActionsApi.md#start_server) | **POST** /baremetal/v1/zones/{zone}/servers/{server_id}/start | Start an Elastic Metal server
[**stop_server**](ServerActionsApi.md#stop_server) | **POST** /baremetal/v1/zones/{zone}/servers/{server_id}/stop | Stop an Elastic Metal server



## reboot_server

> models::ScalewayPeriodBaremetalPeriodV1PeriodServer reboot_server(zone, server_id, reboot_server_request)
Reboot an Elastic Metal server

Reboot the Elastic Metal server associated with the ID, use the `boot_type` `rescue` to reboot the server in rescue mode.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** | ID of the server to reboot. | [required] |
**reboot_server_request** | [**RebootServerRequest**](RebootServerRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodBaremetalPeriodV1PeriodServer**](scaleway.baremetal.v1.Server.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_server

> models::ScalewayPeriodBaremetalPeriodV1PeriodServer start_server(zone, server_id, reboot_server_request)
Start an Elastic Metal server

Start the server associated with the ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** | ID of the server to start. | [required] |
**reboot_server_request** | [**RebootServerRequest**](RebootServerRequest.md) |  | [required] |

### Return type

[**models::ScalewayPeriodBaremetalPeriodV1PeriodServer**](scaleway.baremetal.v1.Server.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_server

> models::ScalewayPeriodBaremetalPeriodV1PeriodServer stop_server(zone, server_id, body)
Stop an Elastic Metal server

Stop the server associated with the ID. The server remains allocated to your account and all data remains on the local storage of the server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** | ID of the server to stop. | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**models::ScalewayPeriodBaremetalPeriodV1PeriodServer**](scaleway.baremetal.v1.Server.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

