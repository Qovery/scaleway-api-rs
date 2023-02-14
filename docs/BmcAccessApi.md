# \BmcAccessApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_bmc_access**](BmcAccessApi.md#get_bmc_access) | **GET** /baremetal/v1/zones/{zone}/servers/{server_id}/bmc-access | Get BMC (Baseboard Management Controller) access for a given elastic metal server
[**start_bmc_access**](BmcAccessApi.md#start_bmc_access) | **POST** /baremetal/v1/zones/{zone}/servers/{server_id}/bmc-access | Start BMC (Baseboard Management Controller) access for a given elastic metal server
[**stop_bmc_access**](BmcAccessApi.md#stop_bmc_access) | **DELETE** /baremetal/v1/zones/{zone}/servers/{server_id}/bmc-access | Stop BMC (Baseboard Management Controller) access for a given elastic metal server



## get_bmc_access

> crate::models::ScalewayPeriodBaremetalPeriodV1PeriodBmcAccess get_bmc_access(zone, server_id)
Get BMC (Baseboard Management Controller) access for a given elastic metal server

Get the BMC (Baseboard Management Controller) access associated with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** | ID of the server | [required] |

### Return type

[**crate::models::ScalewayPeriodBaremetalPeriodV1PeriodBmcAccess**](scaleway.baremetal.v1.BMCAccess.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_bmc_access

> crate::models::ScalewayPeriodBaremetalPeriodV1PeriodBmcAccess start_bmc_access(zone, server_id, start_bmc_access_request)
Start BMC (Baseboard Management Controller) access for a given elastic metal server

Start BMC (Baseboard Management Controller) access associated with the given ID. The BMC (Baseboard Management Controller) access is available one hour after the installation of the server. You need first to create an option Remote Access. You will find the ID and the price with a call to listOffers (https://developers.scaleway.com/en/products/baremetal/api/#get-78db92). Then you can add the option https://developers.scaleway.com/en/products/baremetal/api/#post-b14abd. Do not forget to delete the Option.  After start BMC, you need to Get Remote Access to get the login/password https://developers.scaleway.com/en/products/baremetal/api/#get-cefc0f. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** | ID of the server | [required] |
**start_bmc_access_request** | [**StartBmcAccessRequest**](StartBmcAccessRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodBaremetalPeriodV1PeriodBmcAccess**](scaleway.baremetal.v1.BMCAccess.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_bmc_access

> stop_bmc_access(zone, server_id)
Stop BMC (Baseboard Management Controller) access for a given elastic metal server

Stop BMC (Baseboard Management Controller) access associated with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**server_id** | **String** | ID of the server | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

