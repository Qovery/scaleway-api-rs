# UpdateClusterRequestOpenIdConnectConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**issuer_url** | Option<**String**> | URL of the provider which allows the API server to discover public signing keys. Only URLs which use the `https://` scheme are accepted. This is typically the provider's discovery URL without a path, for example \"https://accounts.google.com\" or \"https://login.salesforce.com\". This URL should point to the level below .well-known/openid-configuration.  | [optional]
**client_id** | Option<**String**> | A client id that all tokens must be issued for | [optional]
**username_claim** | Option<**String**> | JWT claim to use as the user name. By default `sub`, which is expected to be a unique identifier of the end user. Admins can choose other claims, such as `email` or `name`, depending on their provider. However, claims other than `email` will be prefixed with the issuer URL to prevent naming clashes with other plugins.  | [optional]
**username_prefix** | Option<**String**> | Prefix prepended to username claims to prevent clashes with existing names (such as `system:` users). For example, the value `oidc:` will create usernames like `oidc:jane.doe`. If this flag isn't provided and `username_claim` is a value other than `email` the prefix defaults to `( Issuer URL )#` where `( Issuer URL )` is the value of `issuer_url`. The value `-` can be used to disable all prefixing.  | [optional]
**groups_claim** | Option<**Vec<String>**> | JWT claim to use as the user's group | [optional]
**groups_prefix** | Option<**String**> | Prefix prepended to group claims to prevent clashes with existing names (such as `system:` groups). For example, the value `oidc:` will create group names like `oidc:engineering` and `oidc:infra`.  | [optional]
**required_claim** | Option<**Vec<String>**> | Multiple key=value pairs that describes a required claim in the ID Token. If set, the claims are verified to be present in the ID Token with a matching value.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


