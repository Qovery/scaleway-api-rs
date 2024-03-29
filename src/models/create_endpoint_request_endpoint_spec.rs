/*
 * Account API
 *
 * # Introduction  The Account API allows you to manage projects. Project is Scaleway’s resource management feature. Designed to help you organize your infrastructure and cloud services, the feature allows resources to be isolated and grouped into specific projects.
 *
 * The version of the OpenAPI document: v2
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreateEndpointRequestEndpointSpec : Specification of the endpoint you want to create

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateEndpointRequestEndpointSpec {
    /// Load balancer endpoint specifications. Public endpoint for RDB instances which is systematically present. One per RDB instance
    #[serde(rename = "load_balancer", skip_serializing_if = "Option::is_none")]
    pub load_balancer: Option<serde_json::Value>,
    #[serde(rename = "private_network", skip_serializing_if = "Option::is_none")]
    pub private_network:
        Option<Box<crate::models::CreateEndpointRequestEndpointSpecPrivateNetwork>>,
}

impl CreateEndpointRequestEndpointSpec {
    /// Specification of the endpoint you want to create
    pub fn new() -> CreateEndpointRequestEndpointSpec {
        CreateEndpointRequestEndpointSpec {
            load_balancer: None,
            private_network: None,
        }
    }
}
