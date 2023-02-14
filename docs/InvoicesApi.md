# \InvoicesApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**download_invoice**](InvoicesApi.md#download_invoice) | **GET** /billing/v2alpha1/invoices/{invoice_id}/download | Download invoice based on his number
[**list_invoices**](InvoicesApi.md#list_invoices) | **GET** /billing/v2alpha1/invoices | List invoices



## download_invoice

> crate::models::ScalewayPeriodStdPeriodFile download_invoice(invoice_id, file_type)
Download invoice based on his number

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invoice_id** | **String** | Invoice ID (UUID format) | [required] |
**file_type** | Option<**String**> | Wanted file type |  |[default to pdf]

### Return type

[**crate::models::ScalewayPeriodStdPeriodFile**](scaleway.std.File.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_invoices

> crate::models::ScalewayPeriodBillingPeriodV2alpha1PeriodListInvoicesResponse list_invoices(organization_id, started_after, started_before, invoice_type, page, page_size, order_by)
List invoices

List all your invoices filtering by start_date and invoice_type

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | Option<**String**> | Filter invoices by organization ID (UUID format) |  |
**started_after** | Option<**String**> | Filter invoices where invoice's start_date is greater or equal to started_after (RFC 3339 format) |  |
**started_before** | Option<**String**> | Filter invoices where invoice's start_date is before started_before (RFC 3339 format) |  |
**invoice_type** | Option<**String**> | Type of the invoice, it can be 'periodic' or 'purchase' |  |[default to unknown_type]
**page** | Option<**i32**> | A positive integer to choose the page to return |  |[default to 1]
**page_size** | Option<**i32**> | A positive integer lower or equal to 100 to select the number of items to return |  |
**order_by** | Option<**String**> | How invoices are ordered in the response |  |[default to invoice_number_desc]

### Return type

[**crate::models::ScalewayPeriodBillingPeriodV2alpha1PeriodListInvoicesResponse**](scaleway.billing.v2alpha1.ListInvoicesResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

