# ScalewayLbV1Certificate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_type** | Option<**String**> | Type of certificate (Let's encrypt or custom) | [optional][default to Type_Letsencryt]
**id** | Option<**String**> | Certificate ID | [optional]
**common_name** | Option<**String**> | Main domain name of certificate | [optional]
**subject_alternative_name** | Option<**Vec<String>**> | Alternative domain names | [optional]
**fingerprint** | Option<**String**> | Identifier (SHA-1) of the certificate | [optional]
**not_valid_before** | Option<**String**> | Validity bounds | [optional]
**not_valid_after** | Option<**String**> | Validity bounds | [optional]
**status** | Option<**String**> | Status of certificate | [optional][default to Status_Pending]
**lb** | Option<[**crate::models::ScalewayLbV1CertificateLb**](scaleway_lb_v1_Certificate_lb.md)> |  | [optional]
**name** | Option<**String**> | Certificate name | [optional]
**created_at** | Option<**String**> | Date at which the certificate was created | [optional]
**updated_at** | Option<**String**> | Date at which the certificate was last updated | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


