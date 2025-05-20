# ScalewayPeriodBaremetalPeriodV1PeriodServerPeriodOption

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | ID of the option. | [optional]
**name** | Option<**String**> | Name of the option. | [optional]
**status** | Option<**String**> | Status of the option on this server. | [optional][default to OptionStatusUnknown]
**manageable** | Option<**bool**> | Defines whether the option can be managed (added or removed). | [optional]
**expires_at** | Option<**String**> | Auto expiration date for compatible options. (RFC 3339 format) | [optional]
**license** | Option<[**models::ScalewayBaremetalV1OfferOptionOfferLicense**](scaleway_baremetal_v1_Offer_OptionOffer_license.md)> |  | [optional]
**public_bandwidth** | Option<[**models::ScalewayBaremetalV1OfferOptionOfferPublicBandwidth**](scaleway_baremetal_v1_Offer_OptionOffer_public_bandwidth.md)> |  | [optional]
**private_network** | Option<[**models::ScalewayBaremetalV1OfferOptionOfferPrivateNetwork**](scaleway_baremetal_v1_Offer_OptionOffer_private_network.md)> |  | [optional]
**remote_access** | Option<[**serde_json::Value**](.md)> | Remote_access option. | [optional]
**certification** | Option<[**serde_json::Value**](.md)> | Certification option. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


