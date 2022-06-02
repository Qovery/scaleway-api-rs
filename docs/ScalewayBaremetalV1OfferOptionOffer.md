# ScalewayBaremetalV1OfferOptionOffer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | ID of the option | [optional]
**name** | Option<**String**> | Name of the option | [optional]
**enabled** | Option<**bool**> | If true the option is enabled and included by default in the offer If false the option is available for the offer but not included by default  | [optional]
**subscription_period** | Option<**String**> | Period of subscription for the offer | [optional][default to SubscriptionPeriod_UnknownSubscriptionPeriod]
**price** | Option<[**crate::models::ScalewayBaremetalV1OfferOptionOfferPrice**](scaleway_baremetal_v1_Offer_OptionOffer_price.md)> |  | [optional]
**manageable** | Option<**bool**> | Boolean to know if option could be managed | [optional]
**os_id** | Option<**String**> | ID of the OS linked to the option | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


