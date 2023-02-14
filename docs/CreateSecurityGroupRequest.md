# CreateSecurityGroupRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Name of the security group | 
**description** | Option<**String**> | Description of the security group | [optional]
**organization** | Option<**String**> | Organization ID the security group belongs to | [optional]
**project** | Option<**String**> | Project ID the security group belong to | [optional]
**tags** | Option<**Vec<String>**> | The tags of the security group | [optional]
**organization_default** | Option<**bool**> | Whether this security group becomes the default security group for new instances | [optional][default to false]
**project_default** | Option<**bool**> | Whether this security group becomes the default security group for new instances | [optional][default to false]
**stateful** | Option<**bool**> | Whether the security group is stateful or not | [optional][default to false]
**inbound_default_policy** | Option<**String**> | Default policy for inbound rules | [optional][default to Accept]
**outbound_default_policy** | Option<**String**> | Default policy for outbound rules | [optional][default to Accept]
**enable_default_security** | Option<**bool**> | True if SMTP is blocked on IPv4 and IPv6. This feature is read only, please open a ticket if you need to make it configurable. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


