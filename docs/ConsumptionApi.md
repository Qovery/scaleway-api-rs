# \ConsumptionApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_consumption**](ConsumptionApi.md#get_consumption) | **GET** /billing/v2alpha1/consumption | Consumption over the current month



## get_consumption

> crate::models::ScalewayPeriodBillingPeriodV2alpha1PeriodGetConsumptionResponse get_consumption(organization_id)
Consumption over the current month

The consumption represents the amount of money you have spent for the products you have consumed. The value is not computed in real time. The value of the consumption is a monetary value. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | Option<**String**> | Filter by organization ID (UUID format) |  |

### Return type

[**crate::models::ScalewayPeriodBillingPeriodV2alpha1PeriodGetConsumptionResponse**](scaleway.billing.v2alpha1.GetConsumptionResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

