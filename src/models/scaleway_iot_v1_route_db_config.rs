/*
 * Elastic metal API
 *
 * # Introduction  Elastic metal as a service allows ordering a dedicated server on-demand like a cloud instance. Dedicated servers could be used for large workloads, big data, those requiring more security, ….  ## Technical Limitations  - Elastic metal is available in `fr-par-1`,  `fr-par-2`, `nl-ams-1` zones  - Installation is done by preseed (± 10min) (preseed: complete install from a virtual media)  ## Features  - Install (Server is installed by preseed (preseed: complete install from a virtual media), you must define at least one ssh key to install your server)  - Start/Stop/Reboot  - Rescue Reboot, a rescue image is an operating system image designed to help you diagnose and fix an OS experiencing failures. When your server boot on rescue, you can mount your disks and start diagnosing/fixing your image.  - Billed by minute (The billing start when the server is delivered and stop when the server is deleted)  - IPv6, all servers are available with an IPv6 /128  - ReverseIP, You can configure your reverse IP (IPv4 and IPv6), you must register the server IP in your DNS records before calling the endpoint  - Basic monitoring with ping status  - Flexible IP is available ([documentation](https://developers.scaleway.com/en/products/flexible-ip/api/))  - IP failovers are not available in api v1, use the api v1alpha1  ## FAQ  ### How can I get my SSH key id?  You can find your `$SCW_SECRET_KEY` and your `SCW_DEFAULT_ORGANIZATION_ID` at the following page: https://console.scaleway.com/project/credentials  ### How can I add my server to a private network?  See [our online documentation](https://developers.scaleway.com/en/products/vpc-elasticmetal/api/).
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

/// ScalewayIotV1RouteDbConfig : When using Database Route, DB-specific configuration fields

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ScalewayIotV1RouteDbConfig {
    /// Database engine the route will connect to. If not specified, will default to 'PostgreSQL'
    #[serde(rename = "engine", skip_serializing_if = "Option::is_none")]
    pub engine: Option<Engine>,
    /// Database host
    #[serde(rename = "host")]
    pub host: String,
    /// Database port
    #[serde(rename = "port")]
    pub port: i64,
    /// Database name
    #[serde(rename = "dbname")]
    pub dbname: String,
    /// Database username. Make sure this account can execute the provided query
    #[serde(rename = "username")]
    pub username: String,
    /// Database password
    #[serde(rename = "password")]
    pub password: String,
    /// SQL query to be executed ($TOPIC and $PAYLOAD variables are available, see documentation)
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}

impl ScalewayIotV1RouteDbConfig {
    /// When using Database Route, DB-specific configuration fields
    pub fn new(
        host: String,
        port: i64,
        dbname: String,
        username: String,
        password: String,
    ) -> ScalewayIotV1RouteDbConfig {
        ScalewayIotV1RouteDbConfig {
            engine: None,
            host,
            port,
            dbname,
            username,
            password,
            query: None,
        }
    }
}

/// Database engine the route will connect to. If not specified, will default to 'PostgreSQL'
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Engine {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "postgresql")]
    Postgresql,
    #[serde(rename = "mysql")]
    Mysql,
}

impl Default for Engine {
    fn default() -> Engine {
        Self::Unknown
    }
}
