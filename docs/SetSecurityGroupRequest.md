# SetSecurityGroupRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the security group | [optional]
**tags** | Option<**Vec<String>**> | The tags of the security group | [optional]
**creation_date** | Option<**String**> | The creation date of the security group (will be ignored) (RFC 3339 format) | [optional]
**modification_date** | Option<**String**> | The modification date of the security group (will be ignored) (RFC 3339 format) | [optional]
**description** | Option<**String**> | The description of the security group | [optional]
**enable_default_security** | Option<**bool**> | True if SMTP is blocked on IPv4 and IPv6. This feature is read only, please open a ticket if you need to make it configurable. | [optional]
**inbound_default_policy** | Option<**String**> | The default inbound policy | [optional][default to Accept]
**outbound_default_policy** | Option<**String**> | The default outbound policy | [optional][default to Accept]
**organization** | Option<**String**> | The security groups organization ID | [optional]
**project** | Option<**String**> | The security group project ID | [optional]
**organization_default** | Option<**bool**> | Please use project_default instead | [optional]
**project_default** | Option<**bool**> | True use this security group for future instances created in this project | [optional]
**servers** | Option<[**Vec<crate::models::ScalewayPeriodInstancePeriodV1PeriodServerSummary>**](scaleway.instance.v1.ServerSummary.md)> | The servers attached to this security group | [optional]
**stateful** | Option<**bool**> | True to set the security group as stateful | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


