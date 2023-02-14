# ScalewayPeriodBaremetalPeriodV1PeriodOffer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | ID of the offer | [optional]
**name** | Option<**String**> | Name of the offer | [optional]
**stock** | Option<**String**> | Stock level | [optional][default to Empty]
**bandwidth** | Option<**i32**> | Bandwidth available in bits/s with the offer | [optional]
**commercial_range** | Option<**String**> | Commercial range of the offer | [optional]
**price_per_hour** | Option<[**crate::models::ScalewayBaremetalV1OfferPricePerHour**](scaleway_baremetal_v1_Offer_price_per_hour.md)> |  | [optional]
**price_per_month** | Option<[**crate::models::ScalewayBaremetalV1OfferPricePerMonth**](scaleway_baremetal_v1_Offer_price_per_month.md)> |  | [optional]
**disks** | Option<[**Vec<crate::models::ScalewayPeriodBaremetalPeriodV1PeriodDisk>**](scaleway.baremetal.v1.Disk.md)> | Disks specifications of the offer | [optional]
**enable** | Option<**bool**> | True if the offer is currently available | [optional]
**cpus** | Option<[**Vec<crate::models::ScalewayPeriodBaremetalPeriodV1PeriodCpu>**](scaleway.baremetal.v1.CPU.md)> | CPU specifications of the offer | [optional]
**memories** | Option<[**Vec<crate::models::ScalewayPeriodBaremetalPeriodV1PeriodMemory>**](scaleway.baremetal.v1.Memory.md)> | Memory specifications of the offer | [optional]
**quota_name** | Option<**String**> | Name of the quota associated to the offer | [optional]
**persistent_memories** | Option<[**Vec<crate::models::ScalewayPeriodBaremetalPeriodV1PeriodPersistentMemory>**](scaleway.baremetal.v1.PersistentMemory.md)> | Persistent memory specifications of the offer | [optional]
**raid_controllers** | Option<[**Vec<crate::models::ScalewayPeriodBaremetalPeriodV1PeriodRaidController>**](scaleway.baremetal.v1.RaidController.md)> | Raid controller specifications of the offer | [optional]
**incompatible_os_ids** | Option<**Vec<String>**> | Array of incompatible OS ids | [optional]
**subscription_period** | Option<**String**> | Period of subscription for the offer | [optional][default to UnknownSubscriptionPeriod]
**operation_path** | Option<**String**> | Operation path of the service | [optional]
**fee** | Option<[**crate::models::ScalewayBaremetalV1OfferFee**](scaleway_baremetal_v1_Offer_fee.md)> |  | [optional]
**options** | Option<[**Vec<crate::models::ScalewayPeriodBaremetalPeriodV1PeriodOfferPeriodOptionOffer>**](scaleway.baremetal.v1.Offer.OptionOffer.md)> | Options available on offer | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


