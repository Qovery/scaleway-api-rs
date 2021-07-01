/*
 * Instance API
 *
 * # Introduction  ## Endpoints  Scaleway instance API can be reach on  - `https://api.scaleway.com/instance/v1/zones/fr-par-1` - `https://api.scaleway.com/instance/v1/zones/fr-par-2` - `https://api.scaleway.com/instance/v1/zones/nl-ams-1` - `https://api.scaleway.com/instance/v1/zones/pl-waw-1`  Older endpoints are still reachable but should not be used for new projects  - `https://cp-par1.scaleway.com` - `https://cp-ams1.scaleway.com`  <Example>  The following code is an example request to retrieve detailed information about a volume:  ``` % curl -H 'X-Auth-Token: xxxxxxxx-xxxx-xxxxx-xxxx-xxxxxxxxxxxxx' -H 'Content-Type: application/json' https://api.scaleway.com/instance/v1/zones/fr-par-1/volumes/f929fe39-63f8-4be8-a80e-1e9c8ae22a76 -i  HTTP/1.1 200 OK Server: nginx Date: Thu, 22 May 2014 07:55:00 GMT Content-Type: application/json Content-Length: 1345 Connection: keep-alive Strict-Transport-Security: max-age=86400  {   \"volumes\": [     {       \"export_uri\": null,       \"id\": \"f929fe39-63f8-4be8-a80e-1e9c8ae22a76\",       \"name\": \"volume-0-1\",       \"organization\": \"000a115d-2852-4b0a-9ce8-47f1134ba95a\",       \"server\": null,       \"size\": 10000000000,       \"volume_type\": \"l_ssd\"     },     {       \"export_uri\": null,       \"id\": \"0facb6b5-b117-441a-81c1-f28b1d723779\",       \"name\": \"volume-0-2\",       \"organization\": \"000a115d-2852-4b0a-9ce8-47f1134ba95a\",       \"server\": null,       \"size\": 20000000000,       \"volume_type\": \"l_ssd\"     }   ] } ```  </Example>  ## Pagination  Most of listing requests receive a paginated response.  **Paginated request**  Requests against paginated endpoints accept two `query` arguments:  - `page`, a positive integer to choose the page to return. - `per_page`, an positive integer lower or equal to 100 to select the number of   items to return. The default value is `50`.  Paginated endpoints usually also accept filters to search and sort results. These filters are documented along each endpoint documentation.  **Paginated response**  ```bash % curl -H 'X-Auth-Token: <token>' 'https://api.scaleway.com/instance/v1/zones/fr-par-1/images/?page=2&per_page=10' -i HTTP/1.0 200 OK [...] X-Total-Count: 209 [...] ```  The `X-Total-Count` header contains the total number of items for the resource.
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScalewayRegistryV1Image {
    /// The unique ID of the Image
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The Image name, unique in a namespace
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The unique ID of the Namespace the image belongs to
    #[serde(rename = "namespace_id", skip_serializing_if = "Option::is_none")]
    pub namespace_id: Option<String>,
    /// The status of the image
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// Details of the image status
    #[serde(rename = "status_message", skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// A `public` image is pullable from internet without authentication, opposed to a `private` image. `inherit` will use the namespace `is_public` parameter
    #[serde(rename = "visibility", skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,
    /// Image size in bytes, calculated from the size of image layers. One layer used in two tags of the same image is counted once but one layer used in two images is counted twice. (in bytes)
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<f32>,
    /// Creation date
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Last modification date, from the user or the service
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// List of docker tags of the image
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl ScalewayRegistryV1Image {
    pub fn new() -> ScalewayRegistryV1Image {
        ScalewayRegistryV1Image {
            id: None,
            name: None,
            namespace_id: None,
            status: None,
            status_message: None,
            visibility: None,
            size: None,
            created_at: None,
            updated_at: None,
            tags: None,
        }
    }
}

/// The status of the image
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "deleting")]
    Deleting,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "locked")]
    Locked,
}
/// A `public` image is pullable from internet without authentication, opposed to a `private` image. `inherit` will use the namespace `is_public` parameter
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Visibility {
    #[serde(rename = "visibility_unknown")]
    VisibilityUnknown,
    #[serde(rename = "inherit")]
    Inherit,
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "private")]
    Private,
}
