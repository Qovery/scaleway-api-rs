# \SshKeysApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_ssh_key**](SshKeysApi.md#create_ssh_key) | **POST** /iam/v1alpha1/ssh-keys | Create an SSH key
[**delete_ssh_key**](SshKeysApi.md#delete_ssh_key) | **DELETE** /iam/v1alpha1/ssh-keys/{ssh_key_id} | Delete an SSH key
[**get_ssh_key**](SshKeysApi.md#get_ssh_key) | **GET** /iam/v1alpha1/ssh-keys/{ssh_key_id} | Get an SSH key
[**list_ssh_keys**](SshKeysApi.md#list_ssh_keys) | **GET** /iam/v1alpha1/ssh-keys | List SSH keys
[**update_ssh_key**](SshKeysApi.md#update_ssh_key) | **PATCH** /iam/v1alpha1/ssh-keys/{ssh_key_id} | Update an SSH key



## create_ssh_key

> crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodSshKey create_ssh_key(create_ssh_key_request)
Create an SSH key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_ssh_key_request** | [**CreateSshKeyRequest**](CreateSshKeyRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodSshKey**](scaleway.iam.v1alpha1.SSHKey.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_ssh_key

> delete_ssh_key(ssh_key_id)
Delete an SSH key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ssh_key_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ssh_key

> crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodSshKey get_ssh_key(ssh_key_id)
Get an SSH key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ssh_key_id** | **String** | The ID of the SSH key | [required] |

### Return type

[**crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodSshKey**](scaleway.iam.v1alpha1.SSHKey.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_ssh_keys

> crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodListSshKeysResponse list_ssh_keys(order_by, page, page_size, organization_id, name, project_id, disabled)
List SSH keys

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_by** | Option<**String**> | Sort order of SSH keys |  |[default to created_at_asc]
**page** | Option<**i32**> | Requested page number. Value must be greater or equals to 1 |  |
**page_size** | Option<**i32**> | Number of items per page. Value must be between 1 and 100 |  |
**organization_id** | Option<**String**> | Filter by organization ID |  |
**name** | Option<**String**> | Name of group to find |  |
**project_id** | Option<**String**> | Filter by project ID |  |
**disabled** | Option<**bool**> | Filter out disabled SSH keys or not |  |

### Return type

[**crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodListSshKeysResponse**](scaleway.iam.v1alpha1.ListSSHKeysResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_ssh_key

> crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodSshKey update_ssh_key(ssh_key_id, update_ssh_key_request)
Update an SSH key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ssh_key_id** | **String** |  | [required] |
**update_ssh_key_request** | [**UpdateSshKeyRequest**](UpdateSshKeyRequest.md) |  | [required] |

### Return type

[**crate::models::ScalewayPeriodIamPeriodV1alpha1PeriodSshKey**](scaleway.iam.v1alpha1.SSHKey.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

