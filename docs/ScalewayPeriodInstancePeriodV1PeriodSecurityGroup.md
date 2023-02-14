# ScalewayPeriodInstancePeriodV1PeriodSecurityGroup

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The security groups' unique ID | [optional]
**name** | Option<**String**> | The security groups name | [optional]
**description** | Option<**String**> | The security groups description | [optional]
**enable_default_security** | Option<**bool**> | True if SMTP is blocked on IPv4 and IPv6. This feature is read only, please open a ticket if you need to make it configurable. | [optional]
**inbound_default_policy** | Option<**String**> | The default inbound policy | [optional][default to Accept]
**outbound_default_policy** | Option<**String**> | The default outbound policy | [optional][default to Accept]
**organization** | Option<**String**> | The security groups organization ID | [optional]
**project** | Option<**String**> | The security group project ID | [optional]
**tags** | Option<**Vec<String>**> | The security group tags | [optional]
**organization_default** | Option<**bool**> | True if it is your default security group for this organization ID | [optional]
**project_default** | Option<**bool**> | True if it is your default security group for this project ID | [optional]
**creation_date** | Option<**String**> | The security group creation date (RFC 3339 format) | [optional]
**modification_date** | Option<**String**> | The security group modification date (RFC 3339 format) | [optional]
**servers** | Option<[**Vec<crate::models::ScalewayPeriodInstancePeriodV1PeriodServerSummary>**](scaleway.instance.v1.ServerSummary.md)> | List of servers attached to this security group | [optional]
**stateful** | Option<**bool**> | True if the security group is stateful | [optional]
**state** | Option<**String**> | Security group state | [optional][default to Available]
**zone** | Option<**String**> | The zone in which is the security group | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


