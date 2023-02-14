# ScalewayPeriodBillingPeriodV2alpha1PeriodInvoice

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Invoice id | [optional]
**start_date** | Option<**String**> | Billing period start date (RFC 3339 format) | [optional]
**issued_date** | Option<**String**> | Date when the invoice was sent to the customer (RFC 3339 format) | [optional]
**due_date** | Option<**String**> | Payment date limit, set according Organization's payment term (RFC 3339 format) | [optional]
**total_untaxed** | Option<[**crate::models::ScalewayBillingV2alpha1InvoiceTotalUntaxed**](scaleway_billing_v2alpha1_Invoice_total_untaxed.md)> |  | [optional]
**total_taxed** | Option<[**crate::models::ScalewayBillingV2alpha1InvoiceTotalTaxed**](scaleway_billing_v2alpha1_Invoice_total_taxed.md)> |  | [optional]
**invoice_type** | Option<**String**> | Type of the invoice | [optional][default to UnknownType]
**number** | Option<**i32**> | Invoice number | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


