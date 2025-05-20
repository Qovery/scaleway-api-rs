# \OffersApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_offer**](OffersApi.md#get_offer) | **GET** /baremetal/v1/zones/{zone}/offers/{offer_id} | Get offer
[**list_offers**](OffersApi.md#list_offers) | **GET** /baremetal/v1/zones/{zone}/offers | List offers



## get_offer

> models::ScalewayPeriodBaremetalPeriodV1PeriodOffer get_offer(zone, offer_id)
Get offer

Get details of an offer identified by its offer ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**offer_id** | **String** | ID of the researched Offer. | [required] |

### Return type

[**models::ScalewayPeriodBaremetalPeriodV1PeriodOffer**](scaleway.baremetal.v1.Offer.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_offers

> models::ScalewayPeriodBaremetalPeriodV1PeriodListOffersResponse list_offers(zone, page, page_size, subscription_period, name)
List offers

List all available Elastic Metal server configurations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone** | **String** | The zone you want to target | [required] |
**page** | Option<**i32**> | Page number. |  |
**page_size** | Option<**i32**> | Number of offers per page. |  |
**subscription_period** | Option<**String**> | Subscription period type to filter offers by. |  |[default to unknown_subscription_period]
**name** | Option<**String**> | Offer name to filter offers by. |  |

### Return type

[**models::ScalewayPeriodBaremetalPeriodV1PeriodListOffersResponse**](scaleway.baremetal.v1.ListOffersResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

