# LbV1RegionsRegionAclsAclIdMatch

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ip_subnet** | Option<**Vec<String>**> | A list of IPs or CIDR v4/v6 addresses of the client of the session to match | [optional]
**http_filter** | Option<**String**> | The HTTP filter to match. This filter is supported only if your backend supports HTTP forwarding. It extracts the request's URL path, which starts at the first slash and ends before the question mark (without the host part).  | [optional][default to HttpFilter_AclHttpFilterNone]
**http_filter_value** | Option<**Vec<String>**> | A list of possible values to match for the given HTTP filter | [optional]
**invert** | Option<**bool**> | If set to `true`, the ACL matching condition will be of type \"UNLESS\" | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


