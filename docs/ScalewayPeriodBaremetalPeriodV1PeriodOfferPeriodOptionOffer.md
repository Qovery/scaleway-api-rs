# ScalewayPeriodBaremetalPeriodV1PeriodOfferPeriodOptionOffer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | ID of the option. | [optional]
**name** | Option<**String**> | Name of the option. | [optional]
**enabled** | Option<**bool**> | If true the option is enabled and included by default in the offer. If true the option is enabled and included by default in the offer If false the option is available for the offer but not included by default. | [optional]
**subscription_period** | Option<**String**> | Period of subscription for the offer. | [optional][default to UnknownSubscriptionPeriod]
**price** | Option<[**models::ScalewayBaremetalV1OfferOptionOfferPrice**](scaleway_baremetal_v1_Offer_OptionOffer_price.md)> |  | [optional]
**manageable** | Option<**bool**> | Boolean to know if option could be managed. | [optional]
**os_id** | Option<**String**> | Deprecated, use LicenseOptionVars.os_id instead. | [optional]
**license** | Option<[**models::ScalewayBaremetalV1OfferOptionOfferLicense**](scaleway_baremetal_v1_Offer_OptionOffer_license.md)> |  | [optional]
**public_bandwidth** | Option<[**models::ScalewayBaremetalV1OfferOptionOfferPublicBandwidth**](scaleway_baremetal_v1_Offer_OptionOffer_public_bandwidth.md)> |  | [optional]
**private_network** | Option<[**models::ScalewayBaremetalV1OfferOptionOfferPrivateNetwork**](scaleway_baremetal_v1_Offer_OptionOffer_private_network.md)> |  | [optional]
**remote_access** | Option<[**serde_json::Value**](.md)> | Remote_access option. | [optional]
**certification** | Option<[**serde_json::Value**](.md)> | Certification option. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


