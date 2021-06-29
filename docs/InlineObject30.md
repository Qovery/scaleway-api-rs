# InlineObject30

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> |  | [optional]
**forward_protocol** | Option<[**crate::models::ScalewayLbV1Protocol**](scaleway.lb.v1.Protocol.md)> |  | [optional]
**forward_port** | Option<**f32**> |  | [optional]
**forward_port_algorithm** | Option<[**crate::models::ScalewayLbV1ForwardPortAlgorithm**](scaleway.lb.v1.ForwardPortAlgorithm.md)> |  | [optional]
**sticky_sessions** | Option<[**crate::models::ScalewayLbV1StickySessionsType**](scaleway.lb.v1.StickySessionsType.md)> |  | [optional]
**sticky_sessions_cookie_name** | Option<**String**> |  | [optional]
**send_proxy_v2** | Option<**bool**> |  | [optional]
**timeout_server** | Option<**f32**> | (in milliseconds) | [optional]
**timeout_connect** | Option<**f32**> | (in milliseconds) | [optional]
**timeout_tunnel** | Option<**f32**> | (in milliseconds) | [optional]
**on_marked_down_action** | Option<[**crate::models::ScalewayLbV1OnMarkedDownAction**](scaleway.lb.v1.OnMarkedDownAction.md)> |  | [optional]
**proxy_protocol** | Option<[**crate::models::ScalewayLbV1ProxyProtocol**](scaleway.lb.v1.ProxyProtocol.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


