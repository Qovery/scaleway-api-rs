/*
 * Account API
 *
 * # Introduction  The Account API allows you to manage projects. Project is Scaleway’s resource management feature. Designed to help you organize your infrastructure and cloud services, the feature allows resources to be isolated and grouped into specific projects.
 *
 * The version of the OpenAPI document: v2
 *
 * Generated by: https://openapi-generator.tech
 */

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ScalewayPeriodK8sPeriodV1PeriodCni {
    #[serde(rename = "unknown_cni")]
    UnknownCni,
    #[serde(rename = "cilium")]
    Cilium,
    #[serde(rename = "calico")]
    Calico,
    #[serde(rename = "weave")]
    Weave,
    #[serde(rename = "flannel")]
    Flannel,
    #[serde(rename = "kilo")]
    Kilo,
}

impl ToString for ScalewayPeriodK8sPeriodV1PeriodCni {
    fn to_string(&self) -> String {
        match self {
            Self::UnknownCni => String::from("unknown_cni"),
            Self::Cilium => String::from("cilium"),
            Self::Calico => String::from("calico"),
            Self::Weave => String::from("weave"),
            Self::Flannel => String::from("flannel"),
            Self::Kilo => String::from("kilo"),
        }
    }
}

impl Default for ScalewayPeriodK8sPeriodV1PeriodCni {
    fn default() -> ScalewayPeriodK8sPeriodV1PeriodCni {
        Self::UnknownCni
    }
}
