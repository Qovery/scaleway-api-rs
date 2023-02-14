# ScalewayPeriodInstancePeriodV1PeriodSetSecurityGroupRulesRequestPeriodRule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | UUID of the security rule to update. If no value is provided, a new rule will be created | [optional]
**action** | Option<**String**> | Action to apply when the rule matches a packet | [optional][default to Accept]
**protocol** | Option<**String**> | Protocol family this rule applies to | [optional][default to Tcp]
**direction** | Option<**String**> | Direction the rule applies to | [optional][default to Inbound]
**ip_range** | Option<**String**> | The range of IP address this rules applies to (IP network) | [optional]
**dest_port_from** | Option<**i32**> | Beginning of the range of ports this rule applies to (inclusive). This value will be set to null if protocol is ICMP or ANY | [optional]
**dest_port_to** | Option<**i32**> | End of the range of ports this rule applies to (inclusive). This value will be set to null if protocol is ICMP or ANY, or if it is equal to dest_port_from | [optional]
**position** | Option<**i32**> | Position of this rule in the security group rules list. If several rules are passed with the same position, the resulting order is undefined | [optional]
**editable** | Option<**bool**> | Indicates if this rule is editable. Rules with the value false will be ignored | [optional]
**zone** | Option<**String**> | Zone of the rule. This field is ignored | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


