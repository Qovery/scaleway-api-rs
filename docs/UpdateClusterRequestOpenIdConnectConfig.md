# UpdateClusterRequestOpenIdConnectConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**issuer_url** | Option<**String**> | URL of the provider which allows the API server to discover public signing keys. Only URLs using the `https://` scheme are accepted. This is typically the provider's discovery URL without a path, for example \"https://accounts.google.com\" or \"https://login.salesforce.com\". | [optional]
**client_id** | Option<**String**> | A client ID that all tokens must be issued for. | [optional]
**username_claim** | Option<**String**> | JWT claim to use as the user name. The default is `sub`, which is expected to be the end user's unique identifier. Admins can choose other claims, such as `email` or `name`, depending on their provider. However, claims other than `email` will be prefixed with the issuer URL to prevent name collision. | [optional]
**username_prefix** | Option<**String**> | Prefix prepended to username claims to prevent name collision (such as `system:` users). For example, the value `oidc:` will create usernames like `oidc:jane.doe`. If this flag is not provided and `username_claim` is a value other than `email`, the prefix defaults to `( Issuer URL )#` where `( Issuer URL )` is the value of `issuer_url`. The value `-` can be used to disable all prefixing. | [optional]
**groups_claim** | Option<**Vec<String>**> | JWT claim to use as the user's group. | [optional]
**groups_prefix** | Option<**String**> | Prefix prepended to group claims to prevent name collision (such as `system:` groups). For example, the value `oidc:` will create group names like `oidc:engineering` and `oidc:infra`. | [optional]
**required_claim** | Option<**Vec<String>**> | Multiple key=value pairs describing a required claim in the ID token. If set, the claims are verified to be present in the ID token with a matching value. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


