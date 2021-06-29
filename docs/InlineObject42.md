# InlineObject42

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Resource name | 
**description** | **String** | Resource description | 
**tags** | Option<**Vec<String>**> | List of keywords | [optional]
**ssl_compatibility_level** | Option<**String**> | Enforces minimal SSL version (in SSL/TLS offloading context). - `intermediate` General-purpose servers with a variety of clients, recommended for almost all systems (Supports Firefox 27, Android 4.4.2, Chrome 31, Edge, IE 11 on Windows 7, Java 8u31, OpenSSL 1.0.1, Opera 20, and Safari 9). - `modern` Services with clients that support TLS 1.3 and don't need backward compatibility (Firefox 63, Android 10.0, Chrome 70, Edge 75, Java 11, OpenSSL 1.1.1, Opera 57, and Safari 12.1). - `old` Compatible with a number of very old clients, and should be used only as a last resort (Firefox 1, Android 2.3, Chrome 1, Edge 12, IE8 on Windows XP, Java 6, OpenSSL 0.9.8, Opera 5, and Safari 1).  | [optional][default to SslCompatibilityLevel_Unknown]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


