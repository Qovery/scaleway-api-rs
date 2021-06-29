# \AlertSubscribersApi

All URIs are relative to *https://api.scaleway.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_subscriber**](AlertSubscribersApi.md#create_subscriber) | **post** /lb/v1/regions/{region}/subscribers | Create a subscriber, webhook or email
[**delete_subscriber**](AlertSubscribersApi.md#delete_subscriber) | **delete** /lb/v1/regions/{region}/lb/subscriber/{subscriber_id} | Delete a subscriber
[**get_subscriber**](AlertSubscribersApi.md#get_subscriber) | **get** /lb/v1/regions/{region}/subscribers/{subscriber_id} | Get a subscriber
[**list_subscriber**](AlertSubscribersApi.md#list_subscriber) | **get** /lb/v1/regions/{region}/subscribers | List all subscriber
[**subscribe_to_lb**](AlertSubscribersApi.md#subscribe_to_lb) | **post** /lb/v1/regions/{region}/lb/{lb_id}/subscribe | Subscribe a subscriber to a given load balancer
[**unsubscribe_from_lb**](AlertSubscribersApi.md#unsubscribe_from_lb) | **delete** /lb/v1/regions/{region}/lb/{lb_id}/unsubscribe | Unsubscribe a subscriber from a given load balancer
[**update_subscriber**](AlertSubscribersApi.md#update_subscriber) | **put** /lb/v1/regions/{region}/subscribers/{subscriber_id} | Update a subscriber



## create_subscriber

> crate::models::ScalewayLbV1Subscriber create_subscriber(region, inline_object50)
Create a subscriber, webhook or email

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**inline_object50** | [**InlineObject50**](InlineObject50.md) |  | [required] |

### Return type

[**crate::models::ScalewayLbV1Subscriber**](scaleway.lb.v1.Subscriber.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_subscriber

> delete_subscriber(region, subscriber_id)
Delete a subscriber

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**subscriber_id** | **String** | Subscriber ID | [required] |

### Return type

 (empty response body)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subscriber

> crate::models::ScalewayLbV1Subscriber get_subscriber(region, subscriber_id)
Get a subscriber

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**subscriber_id** | **String** | Subscriber ID | [required] |

### Return type

[**crate::models::ScalewayLbV1Subscriber**](scaleway.lb.v1.Subscriber.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_subscriber

> crate::models::ScalewayLbV1ListSubscriberResponse list_subscriber(region, page, name, order_by, page_size, organization_id, project_id)
List all subscriber

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**page** | **f32** | Page number | [required] |[default to 1]
**name** | Option<**String**> | Use this to search by name | [required] |
**order_by** | Option<**String**> | You can order the response by created_at asc/desc or name asc/desc |  |[default to created_at_asc]
**page_size** | Option<**f32**> | The number of items to return |  |[default to 20]
**organization_id** | Option<**String**> | Filter Subscribers by organization ID |  |
**project_id** | Option<**String**> | Filter Subscribers by project ID |  |

### Return type

[**crate::models::ScalewayLbV1ListSubscriberResponse**](scaleway.lb.v1.ListSubscriberResponse.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscribe_to_lb

> crate::models::ScalewayLbV1Lb subscribe_to_lb(region, lb_id, inline_object40)
Subscribe a subscriber to a given load balancer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**lb_id** | **String** | Load balancer ID | [required] |
**inline_object40** | [**InlineObject40**](InlineObject40.md) |  | [required] |

### Return type

[**crate::models::ScalewayLbV1Lb**](scaleway.lb.v1.Lb.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unsubscribe_from_lb

> crate::models::ScalewayLbV1Lb unsubscribe_from_lb(region, lb_id)
Unsubscribe a subscriber from a given load balancer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**lb_id** | **String** | Load balancer ID | [required] |

### Return type

[**crate::models::ScalewayLbV1Lb**](scaleway.lb.v1.Lb.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_subscriber

> crate::models::ScalewayLbV1Subscriber update_subscriber(region, subscriber_id, inline_object51)
Update a subscriber

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The region you want to target | [required] |
**subscriber_id** | **String** | Assign the resource to a project IDs | [required] |
**inline_object51** | [**InlineObject51**](InlineObject51.md) |  | [required] |

### Return type

[**crate::models::ScalewayLbV1Subscriber**](scaleway.lb.v1.Subscriber.md)

### Authorization

[scaleway](../README.md#scaleway)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

