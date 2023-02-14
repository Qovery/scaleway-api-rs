# CreatePolicyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Name of policy to create (max length is 64 chars) | 
**description** | Option<**String**> | Description of policy to create (max length is 200 chars) | [optional]
**organization_id** | Option<**String**> | ID of organization | [optional]
**rules** | Option<[**Vec<crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodRuleSpecs>**](scaleway.iam.v1alpha1.RuleSpecs.md)> | Rules of the policy to create | [optional]
**user_id** | Option<**String**> | ID of user, owner of the policy | [optional]
**group_id** | Option<**String**> | ID of group, owner of the policy | [optional]
**application_id** | Option<**String**> | ID of application, owner of the policy | [optional]
**no_principal** | Option<**bool**> | True when the policy do not belong to any principal | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


