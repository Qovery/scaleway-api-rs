# CreateSecurityGroupRuleRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**protocol** | [**crate::models::ScalewayPeriodInstancePeriodV1PeriodSecurityGroupRulePeriodProtocol**](scaleway.instance.v1.SecurityGroupRule.Protocol.md) |  | 
**direction** | [**crate::models::ScalewayPeriodInstancePeriodV1PeriodSecurityGroupRulePeriodDirection**](scaleway.instance.v1.SecurityGroupRule.Direction.md) |  | 
**action** | [**crate::models::ScalewayPeriodInstancePeriodV1PeriodSecurityGroupRulePeriodAction**](scaleway.instance.v1.SecurityGroupRule.Action.md) |  | 
**ip_range** | **String** | (IP network) | 
**dest_port_from** | Option<**i32**> | The beginning of the range of ports to apply this rule to (inclusive) | [optional]
**dest_port_to** | Option<**i32**> | The end of the range of ports to apply this rule to (inclusive) | [optional]
**position** | Option<**i32**> | The position of this rule in the security group rules list | [optional]
**editable** | Option<**bool**> | Indicates if this rule is editable (will be ignored) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


