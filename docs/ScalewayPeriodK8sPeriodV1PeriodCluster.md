# ScalewayPeriodK8sPeriodV1PeriodCluster

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Cluster ID. | [optional]
**r#type** | Option<**String**> | Cluster type. | [optional]
**name** | Option<**String**> | Cluster name. | [optional]
**status** | Option<**String**> | Status of the cluster. | [optional][default to Unknown]
**version** | Option<**String**> | Kubernetes version of the cluster. | [optional]
**region** | Option<**String**> | Region in which the cluster is deployed. | [optional]
**organization_id** | Option<**String**> | ID of the Organization owning the cluster. | [optional]
**project_id** | Option<**String**> | ID of the Project owning the cluster. | [optional]
**tags** | Option<**Vec<String>**> | Tags associated with the cluster. | [optional]
**cni** | Option<**String**> | Container Network Interface (CNI) plugin running in the cluster. | [optional][default to UnknownCni]
**description** | Option<**String**> | Cluster description. | [optional]
**cluster_url** | Option<**String**> | Kubernetes API server URL of the cluster. | [optional]
**dns_wildcard** | Option<**String**> | Wildcard DNS resolving all the ready cluster nodes. | [optional]
**created_at** | Option<**String**> | Date on which the cluster was created. (RFC 3339 format) | [optional]
**updated_at** | Option<**String**> | Date on which the cluster was last updated. (RFC 3339 format) | [optional]
**autoscaler_config** | Option<[**models::ScalewayK8sV1ClusterAutoscalerConfig**](scaleway_k8s_v1_Cluster_autoscaler_config.md)> |  | [optional]
**auto_upgrade** | Option<[**models::ScalewayK8sV1ClusterAutoUpgrade**](scaleway_k8s_v1_Cluster_auto_upgrade.md)> |  | [optional]
**upgrade_available** | Option<**bool**> | Defines whether a new Kubernetes version is available. | [optional]
**feature_gates** | Option<**Vec<String>**> | List of enabled feature gates. | [optional]
**admission_plugins** | Option<**Vec<String>**> | List of enabled admission plugins. | [optional]
**open_id_connect_config** | Option<[**models::ScalewayK8sV1ClusterOpenIdConnectConfig**](scaleway_k8s_v1_Cluster_open_id_connect_config.md)> |  | [optional]
**apiserver_cert_sans** | Option<**Vec<String>**> | Additional Subject Alternative Names for the Kubernetes API server certificate. | [optional]
**private_network_id** | Option<**String**> | Private network ID for internal cluster communication. | [optional]
**commitment_ends_at** | Option<**String**> | Date on which it will be possible to switch to a smaller offer. (RFC 3339 format) | [optional]
**acl_available** | Option<**bool**> | Defines whether ACL is available on the cluster. | [optional]
**iam_nodes_group_id** | Option<**String**> | IAM group that nodes are members of (this field might be empty during early stage of cluster creation). | [optional]
**new_images_enabled** | Option<**bool**> | Defines whether all pools are migrated to new images. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


