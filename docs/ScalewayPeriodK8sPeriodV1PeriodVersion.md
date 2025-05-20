# ScalewayPeriodK8sPeriodV1PeriodVersion

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the Kubernetes version. | [optional]
**label** | Option<**String**> | Label of the Kubernetes version. | [optional]
**region** | Option<**String**> | Region in which this version is available. | [optional]
**available_cnis** | Option<[**Vec<models::ScalewayPeriodK8sPeriodV1PeriodCni>**](scaleway.k8s.v1.CNI.md)> | Supported Container Network Interface (CNI) plugins for this version. | [optional]
**available_container_runtimes** | Option<[**Vec<models::ScalewayPeriodK8sPeriodV1PeriodRuntime>**](scaleway.k8s.v1.Runtime.md)> | Supported container runtimes for this version. | [optional]
**available_feature_gates** | Option<**Vec<String>**> | Supported feature gates for this version. | [optional]
**available_admission_plugins** | Option<**Vec<String>**> | Supported admission plugins for this version. | [optional]
**available_kubelet_args** | Option<[**models::ScalewayK8sV1VersionAvailableKubeletArgs**](scaleway_k8s_v1_Version_available_kubelet_args.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


