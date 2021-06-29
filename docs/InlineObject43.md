# InlineObject43

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Resource name | 
**forward_protocol** | **String** | Backend protocol. TCP or HTTP | [default to ForwardProtocol_Tcp]
**forward_port** | **f32** | User sessions will be forwarded to this port of backend servers | 
**forward_port_algorithm** | **String** | Load balancing algorithm | [default to ForwardPortAlgorithm_Roundrobin]
**sticky_sessions** | **String** | Enables cookie-based session persistence | [default to StickySessions_None]
**sticky_sessions_cookie_name** | Option<**String**> | Cookie name for for sticky sessions | [optional]
**health_check** | [**crate::models::LbV1RegionsRegionLbsLbIdBackendsHealthCheck**](_lb_v1_regions__region__lbs__lb_id__backends_health_check.md) |  | 
**server_ip** | **Vec<String>** | Backend server IP addresses list (IPv4 or IPv6) | 
**send_proxy_v2** | Option<**bool**> | Deprecated in favor of proxy_protocol field ! | [optional]
**timeout_server** | Option<**f32**> | Maximum server connection inactivity time (in milliseconds) | [optional]
**timeout_connect** | Option<**f32**> | Maximum initical server connection establishment time (in milliseconds) | [optional]
**timeout_tunnel** | Option<**f32**> | Maximum tunnel inactivity time (in milliseconds) | [optional]
**on_marked_down_action** | Option<**String**> | Modify what occurs when a backend server is marked down | [optional][default to OnMarkedDownAction_OnMarkedDownActionNone]
**proxy_protocol** | Option<**String**> | The PROXY protocol informs the other end about the incoming connection, so that it can know the client's address or the public address it accessed to, whatever the upper layer protocol.  * `proxy_protocol_none` Disable proxy protocol. * `proxy_protocol_v1` Version one (text format). * `proxy_protocol_v2` Version two (binary format). * `proxy_protocol_v2_ssl` Version two with SSL connection. * `proxy_protocol_v2_ssl_cn` Version two with SSL connection and common name information.  | [optional][default to ProxyProtocol_Unknown]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


