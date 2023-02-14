/*
 * Account API
 *
 * # Introduction  The Account API allows you to manage projects. Project is Scaleway’s resource management feature. Designed to help you organize your infrastructure and cloud services, the feature allows resources to be isolated and grouped into specific projects.
 *
 * The version of the OpenAPI document: v2
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateSslCertificateRequest {
    #[serde(rename = "dns_zone", skip_serializing_if = "Option::is_none")]
    pub dns_zone: Option<String>,
    #[serde(
        rename = "alternative_dns_zones",
        skip_serializing_if = "Option::is_none"
    )]
    pub alternative_dns_zones: Option<Vec<String>>,
}

impl CreateSslCertificateRequest {
    pub fn new() -> CreateSslCertificateRequest {
        CreateSslCertificateRequest {
            dns_zone: None,
            alternative_dns_zones: None,
        }
    }
}