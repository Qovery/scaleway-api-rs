# ScalewayK8sV1Version

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the Kubernetes version | [optional]
**label** | Option<**String**> | The label of the Kubernetes version | [optional]
**region** | Option<**String**> | The region in which this version is available | [optional]
**available_cnis** | Option<[**Vec<crate::models::ScalewayK8sV1Cni>**](scaleway.k8s.v1.CNI.md)> | The supported Container Network Interface (CNI) plugins for this version | [optional]
**available_ingresses** | Option<[**Vec<crate::models::ScalewayK8sV1Ingress>**](scaleway.k8s.v1.Ingress.md)> | The supported Ingress Controllers for this version | [optional]
**available_container_runtimes** | Option<[**Vec<crate::models::ScalewayK8sV1Runtime>**](scaleway.k8s.v1.Runtime.md)> | The supported container runtimes for this version | [optional]
**available_feature_gates** | Option<**Vec<String>**> | The supported feature gates for this version | [optional]
**available_admission_plugins** | Option<**Vec<String>**> | The supported admission plugins for this version | [optional]
**available_kubelet_args** | Option<[**crate::models::ScalewayK8sV1VersionAvailableKubeletArgs**](scaleway_k8s_v1_Version_available_kubelet_args.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


