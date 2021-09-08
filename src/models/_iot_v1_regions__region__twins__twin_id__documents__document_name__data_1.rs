/*
 * Bare metal API
 *
 * # Introduction  Bare metal as a service allows ordering a dedicated server on-demand like a cloud instance. Dedicated servers could be used for large workloads, big data, those requiring more security, ….  This is the `v1` documentation, the `v1alpha1` version is available [here](/en/products/baremetal/api/v1alpha1).  ## Technical Limitations  - Bare metal is only available in `fr-par-2` zone  - Installation is done by preseed (± 10min) (preseed: complete install from a virtual media)  - The list of OS is limited, you can install your own using the following tutorial: https://www.scaleway.com/en/docs/bare-metal-server-installation-kvm-over-ip/  ## Features  - Install (Server is installed by preseed (preseed: complete install from a virtual media), you must define at least one ssh key to install your server)  - Start/Stop/Reboot  - Rescue Reboot, a rescue image is an operating system image designed to help you diagnose and fix an OS experiencing failures. When your server boot on rescue, you can mount your disks and start diagnosing/fixing your image.  - BMC access: Baseboard Management Controller (BMC) allows you to remotely access the low-level parameters of your dedicated server. For instance, your KVM-IP management console could be accessed with it.  - Billed by minute (The billing start when the server is delivered and stop when the server is deleted)  - IPv6, all servers are available with an IPv6 /128  - ReverseIP, You can configure your reverse IP (IPv4 and IPv6), you must register the server IP in your DNS records before calling the endpoint  - Basic monitoring with ping status  - IP failovers are not available in api v1, use the api v1alpha1  ## FAQ  ### How can I get my ssh key id ?  You can find your `$SCW_SECRET_KEY` and your `SCW_DEFAULT_ORGANIZATION_ID` at the following page: https://console.scaleway.com/project/credentials
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

/// IotV1RegionsRegionTwinsTwinIdDocumentsDocumentNameData1 : A json data that will be applied on the document's current data. Patching rules: * The patch goes recursively through the patch objects. * If the patch object property is null, then it is removed from the final object. * If the patch object property is a value (number, strings, bool, arrays), it is replaced. * If the patch object property is an object, the previous rules will be applied recursively on it.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IotV1RegionsRegionTwinsTwinIdDocumentsDocumentNameData1 {
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    pub fields: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl IotV1RegionsRegionTwinsTwinIdDocumentsDocumentNameData1 {
    /// A json data that will be applied on the document's current data. Patching rules: * The patch goes recursively through the patch objects. * If the patch object property is null, then it is removed from the final object. * If the patch object property is a value (number, strings, bool, arrays), it is replaced. * If the patch object property is an object, the previous rules will be applied recursively on it.
    pub fn new() -> IotV1RegionsRegionTwinsTwinIdDocumentsDocumentNameData1 {
        IotV1RegionsRegionTwinsTwinIdDocumentsDocumentNameData1 { fields: None }
    }
}
