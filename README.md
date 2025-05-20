<!--- Text to be prepended to generated README.md -->
[![build / test / fmt](https://github.com/Qovery/scaleway-api-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/Qovery/scaleway-api-rs/actions/workflows/rust.yml)
[![crate publish](https://github.com/Qovery/scaleway-api-rs/actions/workflows/publish-crate.yml/badge.svg)](https://github.com/Qovery/scaleway-api-rs/actions/workflows/publish-crate.yml)
[![publish](https://img.shields.io/badge/rust--doc-latest-orange)](https://docs.rs/scaleway_api_rs/latest/scaleway_api_rs/)
[![Crates.io](https://img.shields.io/badge/crates.io-latest-orange)](https://crates.io/crates/scaleway_api_rs)


# Notes

This Scaleway API Crate is created and maintained by [Qovery](https://www.qovery.com) and used in production in the [Qovery Engine](https://github.com/Qovery/engine/).

This project relies on OpenAPI Generator, refers to [UPDATE.md](UPDATE.md) to update generated code.
# Rust API client for scaleway_api_rs

Scaleway Elastic Metal servers are dedicated physical servers that you can order on-demand, like Instances.
You can install an OS or other images on your Elastic Metal server and connect to it via SSH to use it as you require.
You can power off the server when you are not using or delete it from your account once you have finished using it.
Elastic Metal servers are ideal for large workloads, big data, and applications that require increased security and dedicated resources.

(switchcolumn)
<Message type=\"tip\">
  Check out our dedicated APIs to manage [Private Networks](https://www.scaleway.com/en/developers/api/elastic-metal/private-network-api/) and [Flexible IPs](https://www.scaleway.com/en/developers/api/elastic-metal-flexible-ip) for Elastic Metal servers.
</Message>
(switchcolumn)

## Concepts

Refer to our [dedicated concepts](https://www.scaleway.com/en/docs/compute/elastic-metal/concepts/) page to find definitions of the different terms referring to Elastic Metal servers.

## Quickstart

(switchcolumn)
(switchcolumn)

1. **Configure your environment variables.**
    ```bash
    export PROJECT_ID=\"<project-id>\"
    export ACCESS_KEY=\"<access-key>\"
    export SECRET_KEY=\"<secret-key>\"
    export ZONE=\"<availability-zone>\"
    ```
    <Message type=\"note\">
      This is an optional step that seeks to simplify your usage of the Bare Metal API.
    </Message>

2. **Edit the POST request payload** that we will use in the next step to create an Elastic Metal server. Modify the values in the example according to your needs, using the information in the table to help.
    ```json
    {
    \"offer_id\": \"string\",
    \"project_id\": \"string\",
    \"name\": \"string\",
    \"description\": \"string\",
    \"tags\": [
        \"tag1\", \"tag2\"
    ],
    \"install\": {
        \"os_id\": \"string\",
        \"hostname\": \"string\",
        \"ssh_key_ids\": [
        \"string\"
        ],
        \"user\": \"string\",
        \"password\": \"string\",
        \"service_user\": \"string\",
        \"service_password\": \"string\"
    },
    \"option_ids\": [
        \"string\"
    ]
    }
    ```

    | Parameter        | Description                                                        |
    | :--------------- | :----------------------------------------------------------------- |
    | `offer_id`           | **REQUIRED** UUID of the Elastic Metal offer                                         |
    | `project_id`     | **REQUIRED** UUID of the project you want to create your Elastic Metal in.  |
    | `name`           | **REQUIRED** Name of the Elastic Metal server (≠hostname)                                          |
    | `description`     | **REQUIRED** A description of your server (max 255 characters)                             |
    | `tags`  | **OPTIONAL** An array of tags associated with your server   |
    | `os_id`  | The ID of the operating system image to install on the server   |
    | `hostname`  | Hostname of the server   |
    | `ssh_key_ids`  | SSH key IDs authorized on the server   |
    | `user`  | **NULLABLE** A regular user to be configured on the server   |
    | `password`  | **NULLABLE** The password for the user account   |
    | `service_user`  | **NULLABLE** A service user for third party services (user to login in services such as BigBlueButton)  |
    | `service password`  | **NULLABLE** Password for the service user   |
    | `option_ids`  | IDs of options to enable on server  |

    <Message type=\"tip\">
      To find your Project ID you can either use the [IAM API](https://www.scaleway.com/en/developers/api/account#path-projects-list-all-projects-of-an-organization) or the [Scaleway console](https://console.scaleway.com/project/settings):
    </Message>

3. **Run the following command** to create an Elastic Metal server. Make sure you include the payload you edited in the previous step.
    ```bash
    curl -X POST \\
      -H \"Content-Type: application/json\" \\
      -H \"X-Auth-Token: $SECRET_KEY\" https://api.scaleway.com//baremetal/v1/zones/$ZONE/servers \\
      -d '{
        \"offer_id\": \"bd757ca3-a71b-4158-9ef5-39436b6db2a4\",
        \"project_id\": \"cc6d123a-bc09-4e24-b5d9-3310f2104e13\",
        \"name\": \"MyElasticMetal\",
        \"description\": \"My_Elastic_Metal_Server\",
        \"tags\": [
            \"Ubuntu22\", \"Dedicated\"
        ],
        \"install\": {
            \"os_id\": \"96e5f0f2-d216-4de2-8a15-68730d877885\",
            \"hostname\": \"elasticmetal.example.com\",
            \"ssh_key_ids\": [
            \"fa05e77f-66b7-43b9-bc21-4dfe3c5bb624\"
            ],
            \"user\": \"ubuntu\",
            \"password\": \"mySecretPa$$word\"
        \"option_ids\": [
            \"string\"
        ]
      }\"
    ```
4. **List your Elastic Metal servers.**
    ```bash
    curl -X GET \\
      -H \"Content-Type: application/json\" \\
      -H \"X-Auth-Token: $SECRET_KEY\" https://api.scaleway.com/baremetal/v1/zones/$ZONE/servers
    ```

5. **Retrieve your Elastic Metal server IP** from the response.

6. **Connect to your Elastic Metal server** using SSH
    ```bash
    ssh root@my-elastic-metal-server-ip
    ```

(switchcolumn)
<Message type=\"requirement\">
To perform the following steps, you must first ensure that:
  - you have an account and are logged into the [Scaleway console](https://console.scaleway.com/organization)
  - you have created an [API key](https://www.scaleway.com/en/docs/identity-and-access-management/iam/how-to/create-api-keys/) and that the API key has sufficient [IAM permissions](https://www.scaleway.com/en/docs/identity-and-access-management/iam/reference-content/permission-sets/) to perform the actions described on this page.
  - you have [installed `curl`](https://curl.se/download.html)
</Message>
(switchcolumn)

## Technical information

### Features

- Installation (Server is installed by preseed [preseed: complete install from a virtual media], you must define at least one ssh key to install your server)
- Start/Stop/Reboot
- Rescue Reboot, a rescue image is an operating system image designed to help you diagnose and fix OS experiencing failures. When your server boot on rescue, you can mount your disks and start diagnosing/fixing your image.
- Billed by the minute (billing starts when the server is delivered and stops when the server is deleted)
- IPv6, all servers are available with a /128 IPv6 subnet
- ReverseIP, You can configure your reverse IP (IPv4 and IPv6), you must register the server IP in your DNS records before calling the endpoint
- Basic monitoring with ping status
- Flexible IP is available ([documentation](https://www.scaleway.com/en/developers/api/elastic-metal-flexible-ip))

### Availability Zones

Scaleway's infrastructure is spread across different [regions and Availability Zones](https://www.scaleway.com/en/docs/console/my-account/reference-content/products-availability/).

Elastic Metal servers are available in Paris, Amsterdam, and Warsaw regions, with product availability in the following AZs:

| Name      | API ID                           |
|-----------|----------------------------------|
| Paris     | `fr-par-1` `fr-par-2`            |
| Amsterdam | `nl-ams-1` `nl-ams-2`            |
| Warsaw    | `pl-waw-2` `pl-waw-3`            |

## Technical limitations

- Failover IPs are not available in API `v1`, use the API `v1alpha1`
## Going further

For more help using Scaleway Elastic Metal servers, check out the following resources:
- Our [main documentation](https://www.scaleway.com/en/docs/compute/elastic-metal/)
- The #elastic-metal channel on our [Slack Community](https://www.scaleway.com/en/docs/tutorials/scaleway-slack-community/)
- Our [support ticketing system](https://www.scaleway.com/en/docs/console/my-account/how-to/open-a-support-ticket)
### Troubleshooting

#### How is the installation of Elastic Metal servers done?

- The installation of Elastic Metal servers is done by preseed (± 10min) (preseed: complete install from a virtual media)
#### How can I retrieve my secret key and Project ID?

You can find your [secret key](https://console.scaleway.com/iam/api-keys) and your [Project ID](https://console.scaleway.com/project/credentials) in the Scaleway console.

#### How can I add my server to a Private Network?

See [our dedicated documentation](/en/developers/api/elastic-metal-flexible-ip).


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: v1
- Package version: 0.1.0
- Generator version: 7.13.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `scaleway_api_rs` and add the following to `Cargo.toml` under `[dependencies]`:

```
scaleway_api_rs = { path = "./scaleway_api_rs" }
```

## Documentation for API Endpoints

All URIs are relative to *https://api.scaleway.com*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AclsApi* | [**add_instance_acl_rules**](docs/AclsApi.md#add_instance_acl_rules) | **POST** /rdb/v1/regions/{region}/instances/{instance_id}/acls | Add an ACL rule to a Database Instance
*AclsApi* | [**delete_instance_acl_rules**](docs/AclsApi.md#delete_instance_acl_rules) | **DELETE** /rdb/v1/regions/{region}/instances/{instance_id}/acls | Delete ACL rules of a Database Instance
*AclsApi* | [**list_instance_acl_rules**](docs/AclsApi.md#list_instance_acl_rules) | **GET** /rdb/v1/regions/{region}/instances/{instance_id}/acls | List ACL rules of a Database Instance
*AclsApi* | [**set_instance_acl_rules**](docs/AclsApi.md#set_instance_acl_rules) | **PUT** /rdb/v1/regions/{region}/instances/{instance_id}/acls | Set ACL rules for a Database Instance
*AccessControlListApi* | [**add_cluster_acl_rules**](docs/AccessControlListApi.md#add_cluster_acl_rules) | **POST** /k8s/v1/regions/{region}/clusters/{cluster_id}/acls | Add new ACLs
*AccessControlListApi* | [**delete_acl_rule**](docs/AccessControlListApi.md#delete_acl_rule) | **DELETE** /k8s/v1/regions/{region}/acls/{acl_id} | Delete an existing ACL
*AccessControlListApi* | [**list_cluster_acl_rules**](docs/AccessControlListApi.md#list_cluster_acl_rules) | **GET** /k8s/v1/regions/{region}/clusters/{cluster_id}/acls | List ACLs
*AccessControlListApi* | [**set_cluster_acl_rules**](docs/AccessControlListApi.md#set_cluster_acl_rules) | **PUT** /k8s/v1/regions/{region}/clusters/{cluster_id}/acls | Set new ACLs
*BmcAccessApi* | [**get_bmc_access**](docs/BmcAccessApi.md#get_bmc_access) | **GET** /baremetal/v1/zones/{zone}/servers/{server_id}/bmc-access | Get BMC access
*BmcAccessApi* | [**start_bmc_access**](docs/BmcAccessApi.md#start_bmc_access) | **POST** /baremetal/v1/zones/{zone}/servers/{server_id}/bmc-access | Start BMC access
*BmcAccessApi* | [**stop_bmc_access**](docs/BmcAccessApi.md#stop_bmc_access) | **DELETE** /baremetal/v1/zones/{zone}/servers/{server_id}/bmc-access | Stop BMC access
*BackupsApi* | [**create_database_backup**](docs/BackupsApi.md#create_database_backup) | **POST** /rdb/v1/regions/{region}/backups | Create a database backup
*BackupsApi* | [**delete_database_backup**](docs/BackupsApi.md#delete_database_backup) | **DELETE** /rdb/v1/regions/{region}/backups/{database_backup_id} | Delete a database backup
*BackupsApi* | [**export_database_backup**](docs/BackupsApi.md#export_database_backup) | **POST** /rdb/v1/regions/{region}/backups/{database_backup_id}/export | Export a database backup
*BackupsApi* | [**get_database_backup**](docs/BackupsApi.md#get_database_backup) | **GET** /rdb/v1/regions/{region}/backups/{database_backup_id} | Get a database backup
*BackupsApi* | [**list_database_backups**](docs/BackupsApi.md#list_database_backups) | **GET** /rdb/v1/regions/{region}/backups | List database backups
*BackupsApi* | [**restore_database_backup**](docs/BackupsApi.md#restore_database_backup) | **POST** /rdb/v1/regions/{region}/backups/{database_backup_id}/restore | Restore a database backup
*BackupsApi* | [**update_database_backup**](docs/BackupsApi.md#update_database_backup) | **PATCH** /rdb/v1/regions/{region}/backups/{database_backup_id} | Update a database backup
*ClusterTypesApi* | [**list_cluster_types**](docs/ClusterTypesApi.md#list_cluster_types) | **GET** /k8s/v1/regions/{region}/cluster-types | List cluster types
*ClustersApi* | [**create_cluster**](docs/ClustersApi.md#create_cluster) | **POST** /k8s/v1/regions/{region}/clusters | Create a new Cluster
*ClustersApi* | [**delete_cluster**](docs/ClustersApi.md#delete_cluster) | **DELETE** /k8s/v1/regions/{region}/clusters/{cluster_id} | Delete a Cluster
*ClustersApi* | [**get_cluster**](docs/ClustersApi.md#get_cluster) | **GET** /k8s/v1/regions/{region}/clusters/{cluster_id} | Get a Cluster
*ClustersApi* | [**get_cluster_kube_config**](docs/ClustersApi.md#get_cluster_kube_config) | **GET** /k8s/v1/regions/{region}/clusters/{cluster_id}/kubeconfig | Download the kubeconfig for a Cluster
*ClustersApi* | [**list_cluster_available_types**](docs/ClustersApi.md#list_cluster_available_types) | **GET** /k8s/v1/regions/{region}/clusters/{cluster_id}/available-types | List available cluster types for a cluster
*ClustersApi* | [**list_cluster_available_versions**](docs/ClustersApi.md#list_cluster_available_versions) | **GET** /k8s/v1/regions/{region}/clusters/{cluster_id}/available-versions | List available versions for a Cluster
*ClustersApi* | [**list_clusters**](docs/ClustersApi.md#list_clusters) | **GET** /k8s/v1/regions/{region}/clusters | List Clusters
*ClustersApi* | [**reset_cluster_admin_token**](docs/ClustersApi.md#reset_cluster_admin_token) | **POST** /k8s/v1/regions/{region}/clusters/{cluster_id}/reset-admin-token | Reset the admin token of a Cluster
*ClustersApi* | [**set_cluster_type**](docs/ClustersApi.md#set_cluster_type) | **POST** /k8s/v1/regions/{region}/clusters/{cluster_id}/set-type | Change the Cluster type
*ClustersApi* | [**update_cluster**](docs/ClustersApi.md#update_cluster) | **PATCH** /k8s/v1/regions/{region}/clusters/{cluster_id} | Update a Cluster
*ClustersApi* | [**upgrade_cluster**](docs/ClustersApi.md#upgrade_cluster) | **POST** /k8s/v1/regions/{region}/clusters/{cluster_id}/upgrade | Upgrade a Cluster
*DnsZonesApi* | [**clone_dns_zone**](docs/DnsZonesApi.md#clone_dns_zone) | **POST** /domain/v2beta1/dns-zones/{dns_zone}/clone | Clone a DNS zone
*DnsZonesApi* | [**create_dns_zone**](docs/DnsZonesApi.md#create_dns_zone) | **POST** /domain/v2beta1/dns-zones | Create a DNS zone
*DnsZonesApi* | [**create_ssl_certificate**](docs/DnsZonesApi.md#create_ssl_certificate) | **POST** /domain/v2beta1/ssl-certificates | Create or get the DNS zone's TLS certificate
*DnsZonesApi* | [**delete_dns_zone**](docs/DnsZonesApi.md#delete_dns_zone) | **DELETE** /domain/v2beta1/dns-zones/{dns_zone} | Delete a DNS zone
*DnsZonesApi* | [**delete_dns_zone_tsig_key**](docs/DnsZonesApi.md#delete_dns_zone_tsig_key) | **DELETE** /domain/v2beta1/dns-zones/{dns_zone}/tsig-key | Delete the DNS zone's TSIG key
*DnsZonesApi* | [**delete_ssl_certificate**](docs/DnsZonesApi.md#delete_ssl_certificate) | **DELETE** /domain/v2beta1/ssl-certificates/{dns_zone} | Delete a TLS certificate
*DnsZonesApi* | [**get_dns_zone_tsig_key**](docs/DnsZonesApi.md#get_dns_zone_tsig_key) | **GET** /domain/v2beta1/dns-zones/{dns_zone}/tsig-key | Get the DNS zone's TSIG key
*DnsZonesApi* | [**get_ssl_certificate**](docs/DnsZonesApi.md#get_ssl_certificate) | **GET** /domain/v2beta1/ssl-certificates/{dns_zone} | Get a DNS zone's TLS certificate
*DnsZonesApi* | [**list_dns_zones**](docs/DnsZonesApi.md#list_dns_zones) | **GET** /domain/v2beta1/dns-zones | List DNS zones
*DnsZonesApi* | [**list_ssl_certificates**](docs/DnsZonesApi.md#list_ssl_certificates) | **GET** /domain/v2beta1/ssl-certificates | List a user's TLS certificates
*DnsZonesApi* | [**refresh_dns_zone**](docs/DnsZonesApi.md#refresh_dns_zone) | **POST** /domain/v2beta1/dns-zones/{dns_zone}/refresh | Refresh a DNS zone
*DnsZonesApi* | [**update_dns_zone**](docs/DnsZonesApi.md#update_dns_zone) | **PATCH** /domain/v2beta1/dns-zones/{dns_zone} | Update a DNS zone
*DatabaseInstancesApi* | [**apply_instance_maintenance**](docs/DatabaseInstancesApi.md#apply_instance_maintenance) | **POST** /rdb/v1/regions/{region}/instances/{instance_id}/apply-maintenance | Apply Database Instance maintenance
*DatabaseInstancesApi* | [**clone_instance**](docs/DatabaseInstancesApi.md#clone_instance) | **POST** /rdb/v1/regions/{region}/instances/{instance_id}/clone | Clone a Database Instance
*DatabaseInstancesApi* | [**create_instance**](docs/DatabaseInstancesApi.md#create_instance) | **POST** /rdb/v1/regions/{region}/instances | Create a Database Instance
*DatabaseInstancesApi* | [**delete_instance**](docs/DatabaseInstancesApi.md#delete_instance) | **DELETE** /rdb/v1/regions/{region}/instances/{instance_id} | Delete a Database Instance
*DatabaseInstancesApi* | [**get_instance**](docs/DatabaseInstancesApi.md#get_instance) | **GET** /rdb/v1/regions/{region}/instances/{instance_id} | Get a Database Instance
*DatabaseInstancesApi* | [**get_instance_certificate**](docs/DatabaseInstancesApi.md#get_instance_certificate) | **GET** /rdb/v1/regions/{region}/instances/{instance_id}/certificate | Get the TLS certificate of a Database Instance
*DatabaseInstancesApi* | [**get_instance_log**](docs/DatabaseInstancesApi.md#get_instance_log) | **GET** /rdb/v1/regions/{region}/logs/{instance_log_id} | Get given logs of a Database Instance
*DatabaseInstancesApi* | [**get_instance_metrics**](docs/DatabaseInstancesApi.md#get_instance_metrics) | **GET** /rdb/v1/regions/{region}/instances/{instance_id}/metrics | Get Database Instance metrics
*DatabaseInstancesApi* | [**list_instance_logs**](docs/DatabaseInstancesApi.md#list_instance_logs) | **GET** /rdb/v1/regions/{region}/instances/{instance_id}/logs | List available logs of a Database Instance
*DatabaseInstancesApi* | [**list_instance_logs_details**](docs/DatabaseInstancesApi.md#list_instance_logs_details) | **GET** /rdb/v1/regions/{region}/instances/{instance_id}/logs-details | List remote Database Instance logs details
*DatabaseInstancesApi* | [**list_instances**](docs/DatabaseInstancesApi.md#list_instances) | **GET** /rdb/v1/regions/{region}/instances | List Database Instances
*DatabaseInstancesApi* | [**prepare_instance_logs**](docs/DatabaseInstancesApi.md#prepare_instance_logs) | **POST** /rdb/v1/regions/{region}/instances/{instance_id}/prepare-logs | Prepare logs of a Database Instance
*DatabaseInstancesApi* | [**purge_instance_logs**](docs/DatabaseInstancesApi.md#purge_instance_logs) | **POST** /rdb/v1/regions/{region}/instances/{instance_id}/purge-logs | Purge remote Database Instance logs
*DatabaseInstancesApi* | [**renew_instance_certificate**](docs/DatabaseInstancesApi.md#renew_instance_certificate) | **POST** /rdb/v1/regions/{region}/instances/{instance_id}/renew-certificate | Renew the TLS certificate of a Database Instance
*DatabaseInstancesApi* | [**restart_instance**](docs/DatabaseInstancesApi.md#restart_instance) | **POST** /rdb/v1/regions/{region}/instances/{instance_id}/restart | Restart Database Instance
*DatabaseInstancesApi* | [**update_instance**](docs/DatabaseInstancesApi.md#update_instance) | **PATCH** /rdb/v1/regions/{region}/instances/{instance_id} | Update a Database Instance
*DatabaseInstancesApi* | [**upgrade_instance**](docs/DatabaseInstancesApi.md#upgrade_instance) | **POST** /rdb/v1/regions/{region}/instances/{instance_id}/upgrade | Upgrade a Database Instance
*DatabasesApi* | [**create_database**](docs/DatabasesApi.md#create_database) | **POST** /rdb/v1/regions/{region}/instances/{instance_id}/databases | Create a database in a Database Instance
*DatabasesApi* | [**delete_database**](docs/DatabasesApi.md#delete_database) | **DELETE** /rdb/v1/regions/{region}/instances/{instance_id}/databases/{name} | Delete a database in a Database Instance
*DatabasesApi* | [**list_databases**](docs/DatabasesApi.md#list_databases) | **GET** /rdb/v1/regions/{region}/instances/{instance_id}/databases | List databases in a Database Instance
*EndpointsApi* | [**create_endpoint**](docs/EndpointsApi.md#create_endpoint) | **POST** /rdb/v1/regions/{region}/instances/{instance_id}/endpoints | Create a new Database Instance endpoint
*EndpointsApi* | [**delete_endpoint**](docs/EndpointsApi.md#delete_endpoint) | **DELETE** /rdb/v1/regions/{region}/endpoints/{endpoint_id} | Delete a Database Instance endpoint
*EndpointsApi* | [**get_endpoint**](docs/EndpointsApi.md#get_endpoint) | **GET** /rdb/v1/regions/{region}/endpoints/{endpoint_id} | Get a Database Instance endpoint
*EndpointsApi* | [**migrate_endpoint**](docs/EndpointsApi.md#migrate_endpoint) | **POST** /rdb/v1/regions/{region}/endpoints/{endpoint_id}/migrate | Migrate an existing instance endpoint to another instance
*EnginesApi* | [**list_database_engines**](docs/EnginesApi.md#list_database_engines) | **GET** /rdb/v1/regions/{region}/database-engines | List available database engines
*FlexibleIpApi* | [**attach_flexible_ip**](docs/FlexibleIpApi.md#attach_flexible_ip) | **POST** /flexible-ip/v1alpha1/zones/{zone}/fips/attach | Attach an existing flexible IP to a server
*FlexibleIpApi* | [**create_flexible_ip**](docs/FlexibleIpApi.md#create_flexible_ip) | **POST** /flexible-ip/v1alpha1/zones/{zone}/fips | Create a new flexible IP
*FlexibleIpApi* | [**delete_flexible_ip**](docs/FlexibleIpApi.md#delete_flexible_ip) | **DELETE** /flexible-ip/v1alpha1/zones/{zone}/fips/{fip_id} | Delete an existing flexible IP
*FlexibleIpApi* | [**delete_mac_addr**](docs/FlexibleIpApi.md#delete_mac_addr) | **DELETE** /flexible-ip/v1alpha1/zones/{zone}/fips/{fip_id}/mac | Detach a given virtual MAC address from an existing flexible IP
*FlexibleIpApi* | [**detach_flexible_ip**](docs/FlexibleIpApi.md#detach_flexible_ip) | **POST** /flexible-ip/v1alpha1/zones/{zone}/fips/detach | Detach an existing flexible IP from a server
*FlexibleIpApi* | [**duplicate_mac_addr**](docs/FlexibleIpApi.md#duplicate_mac_addr) | **POST** /flexible-ip/v1alpha1/zones/{zone}/fips/{fip_id}/mac/duplicate | Duplicate a virtual MAC address to another flexible IP
*FlexibleIpApi* | [**generate_mac_addr**](docs/FlexibleIpApi.md#generate_mac_addr) | **POST** /flexible-ip/v1alpha1/zones/{zone}/fips/{fip_id}/mac | Generate a virtual MAC address on an existing flexible IP
*FlexibleIpApi* | [**get_flexible_ip**](docs/FlexibleIpApi.md#get_flexible_ip) | **GET** /flexible-ip/v1alpha1/zones/{zone}/fips/{fip_id} | Get an existing flexible IP
*FlexibleIpApi* | [**list_flexible_ips**](docs/FlexibleIpApi.md#list_flexible_ips) | **GET** /flexible-ip/v1alpha1/zones/{zone}/fips | List flexible IPs
*FlexibleIpApi* | [**move_mac_addr**](docs/FlexibleIpApi.md#move_mac_addr) | **POST** /flexible-ip/v1alpha1/zones/{zone}/fips/{fip_id}/mac/move | Relocate an existing virtual MAC address to a different flexible IP
*FlexibleIpApi* | [**update_flexible_ip**](docs/FlexibleIpApi.md#update_flexible_ip) | **PATCH** /flexible-ip/v1alpha1/zones/{zone}/fips/{fip_id} | Update an existing flexible IP
*ImagesApi* | [**delete_image**](docs/ImagesApi.md#delete_image) | **DELETE** /registry/v1/regions/{region}/images/{image_id} | Delete an image
*ImagesApi* | [**get_image**](docs/ImagesApi.md#get_image) | **GET** /registry/v1/regions/{region}/images/{image_id} | Get an image
*ImagesApi* | [**list_images**](docs/ImagesApi.md#list_images) | **GET** /registry/v1/regions/{region}/images | List images
*ImagesApi* | [**update_image**](docs/ImagesApi.md#update_image) | **PATCH** /registry/v1/regions/{region}/images/{image_id} | Update an image
*ImportsExportsApi* | [**export_raw_dns_zone**](docs/ImportsExportsApi.md#export_raw_dns_zone) | **GET** /domain/v2beta1/dns-zones/{dns_zone}/raw | Export a raw DNS zone
*ImportsExportsApi* | [**import_provider_dns_zone**](docs/ImportsExportsApi.md#import_provider_dns_zone) | **POST** /domain/v2beta1/dns-zones/{dns_zone}/import-provider | Import a DNS zone from another provider
*ImportsExportsApi* | [**import_raw_dns_zone**](docs/ImportsExportsApi.md#import_raw_dns_zone) | **POST** /domain/v2beta1/dns-zones/{dns_zone}/raw | Import a raw DNS zone
*InstanceSettingsApi* | [**add_instance_settings**](docs/InstanceSettingsApi.md#add_instance_settings) | **POST** /rdb/v1/regions/{region}/instances/{instance_id}/settings | Add Database Instance advanced settings
*InstanceSettingsApi* | [**delete_instance_settings**](docs/InstanceSettingsApi.md#delete_instance_settings) | **DELETE** /rdb/v1/regions/{region}/instances/{instance_id}/settings | Delete Database Instance advanced settings
*InstanceSettingsApi* | [**set_instance_settings**](docs/InstanceSettingsApi.md#set_instance_settings) | **PUT** /rdb/v1/regions/{region}/instances/{instance_id}/settings | Set Database Instance advanced settings
*NamespacesApi* | [**create_namespace**](docs/NamespacesApi.md#create_namespace) | **POST** /registry/v1/regions/{region}/namespaces | Create a namespace
*NamespacesApi* | [**delete_namespace**](docs/NamespacesApi.md#delete_namespace) | **DELETE** /registry/v1/regions/{region}/namespaces/{namespace_id} | Delete a namespace
*NamespacesApi* | [**get_namespace**](docs/NamespacesApi.md#get_namespace) | **GET** /registry/v1/regions/{region}/namespaces/{namespace_id} | Get a namespace
*NamespacesApi* | [**list_namespaces**](docs/NamespacesApi.md#list_namespaces) | **GET** /registry/v1/regions/{region}/namespaces | List namespaces
*NamespacesApi* | [**update_namespace**](docs/NamespacesApi.md#update_namespace) | **PATCH** /registry/v1/regions/{region}/namespaces/{namespace_id} | Update a namespace
*NodeTypesApi* | [**list_node_types**](docs/NodeTypesApi.md#list_node_types) | **GET** /rdb/v1/regions/{region}/node-types | List available node types
*NodesApi* | [**create_external_node**](docs/NodesApi.md#create_external_node) | **POST** /k8s/v1/regions/{region}/pools/{pool_id}/external-nodes | Create a Kosmos node
*NodesApi* | [**delete_node**](docs/NodesApi.md#delete_node) | **DELETE** /k8s/v1/regions/{region}/nodes/{node_id} | Delete a Node in a Cluster
*NodesApi* | [**get_node**](docs/NodesApi.md#get_node) | **GET** /k8s/v1/regions/{region}/nodes/{node_id} | Get a Node in a Cluster
*NodesApi* | [**list_nodes**](docs/NodesApi.md#list_nodes) | **GET** /k8s/v1/regions/{region}/clusters/{cluster_id}/nodes | List Nodes in a Cluster
*NodesApi* | [**reboot_node**](docs/NodesApi.md#reboot_node) | **POST** /k8s/v1/regions/{region}/nodes/{node_id}/reboot | Reboot a Node in a Cluster
*OsApi* | [**get_os**](docs/OsApi.md#get_os) | **GET** /baremetal/v1/zones/{zone}/os/{os_id} | Get OS with an ID
*OsApi* | [**list_os**](docs/OsApi.md#list_os) | **GET** /baremetal/v1/zones/{zone}/os | List available OSes
*OffersApi* | [**get_offer**](docs/OffersApi.md#get_offer) | **GET** /baremetal/v1/zones/{zone}/offers/{offer_id} | Get offer
*OffersApi* | [**list_offers**](docs/OffersApi.md#list_offers) | **GET** /baremetal/v1/zones/{zone}/offers | List offers
*OptionsApi* | [**get_option**](docs/OptionsApi.md#get_option) | **GET** /baremetal/v1/zones/{zone}/options/{option_id} | Get option
*OptionsApi* | [**list_options**](docs/OptionsApi.md#list_options) | **GET** /baremetal/v1/zones/{zone}/options | List options
*OptionsApi* | [**list_settings**](docs/OptionsApi.md#list_settings) | **GET** /baremetal/v1/zones/{zone}/settings | List all settings
*OptionsApi* | [**update_setting**](docs/OptionsApi.md#update_setting) | **PATCH** /baremetal/v1/zones/{zone}/settings/{setting_id} | Update setting
*PartitioningSchemasApi* | [**get_default_partitioning_schema**](docs/PartitioningSchemasApi.md#get_default_partitioning_schema) | **GET** /baremetal/v1/zones/{zone}/partitioning-schemas/default | Get default partitioning schema
*PartitioningSchemasApi* | [**validate_partitioning_schema**](docs/PartitioningSchemasApi.md#validate_partitioning_schema) | **POST** /baremetal/v1/zones/{zone}/partitioning-schemas/validate | Validate client partitioning schema
*PoolsApi* | [**create_pool**](docs/PoolsApi.md#create_pool) | **POST** /k8s/v1/regions/{region}/clusters/{cluster_id}/pools | Create a new Pool in a Cluster
*PoolsApi* | [**delete_pool**](docs/PoolsApi.md#delete_pool) | **DELETE** /k8s/v1/regions/{region}/pools/{pool_id} | Delete a Pool in a Cluster
*PoolsApi* | [**get_pool**](docs/PoolsApi.md#get_pool) | **GET** /k8s/v1/regions/{region}/pools/{pool_id} | Get a Pool in a Cluster
*PoolsApi* | [**list_pools**](docs/PoolsApi.md#list_pools) | **GET** /k8s/v1/regions/{region}/clusters/{cluster_id}/pools | List Pools in a Cluster
*PoolsApi* | [**migrate_pools_to_new_images**](docs/PoolsApi.md#migrate_pools_to_new_images) | **POST** /k8s/v1/regions/{region}/clusters/{cluster_id}/migrate-pools-to-new-images | Migrate specific pools or all pools of a cluster to new images.
*PoolsApi* | [**update_pool**](docs/PoolsApi.md#update_pool) | **PATCH** /k8s/v1/regions/{region}/pools/{pool_id} | Update a Pool in a Cluster
*PoolsApi* | [**upgrade_pool**](docs/PoolsApi.md#upgrade_pool) | **POST** /k8s/v1/regions/{region}/pools/{pool_id}/upgrade | Upgrade a Pool in a Cluster
*PrivateNetworksApi* | [**create_private_network**](docs/PrivateNetworksApi.md#create_private_network) | **POST** /vpc/v1/zones/{zone}/private-networks | Create a Private Network
*PrivateNetworksApi* | [**delete_private_network**](docs/PrivateNetworksApi.md#delete_private_network) | **DELETE** /vpc/v1/zones/{zone}/private-networks/{private_network_id} | Delete a Private Network
*PrivateNetworksApi* | [**get_private_network**](docs/PrivateNetworksApi.md#get_private_network) | **GET** /vpc/v1/zones/{zone}/private-networks/{private_network_id} | Get a Private Network
*PrivateNetworksApi* | [**list_private_networks**](docs/PrivateNetworksApi.md#list_private_networks) | **GET** /vpc/v1/zones/{zone}/private-networks | List Private Networks
*PrivateNetworksApi* | [**update_private_network**](docs/PrivateNetworksApi.md#update_private_network) | **PATCH** /vpc/v1/zones/{zone}/private-networks/{private_network_id} | Update Private Network
*PrivilegesApi* | [**list_privileges**](docs/PrivilegesApi.md#list_privileges) | **GET** /rdb/v1/regions/{region}/instances/{instance_id}/privileges | List user privileges for a database
*PrivilegesApi* | [**set_privilege**](docs/PrivilegesApi.md#set_privilege) | **PUT** /rdb/v1/regions/{region}/instances/{instance_id}/privileges | Set user privileges for a database
*ReadReplicasApi* | [**create_read_replica**](docs/ReadReplicasApi.md#create_read_replica) | **POST** /rdb/v1/regions/{region}/read-replicas | Create a Read Replica
*ReadReplicasApi* | [**create_read_replica_endpoint**](docs/ReadReplicasApi.md#create_read_replica_endpoint) | **POST** /rdb/v1/regions/{region}/read-replicas/{read_replica_id}/endpoints | Create an endpoint for a Read Replica
*ReadReplicasApi* | [**delete_read_replica**](docs/ReadReplicasApi.md#delete_read_replica) | **DELETE** /rdb/v1/regions/{region}/read-replicas/{read_replica_id} | Delete a Read Replica
*ReadReplicasApi* | [**get_read_replica**](docs/ReadReplicasApi.md#get_read_replica) | **GET** /rdb/v1/regions/{region}/read-replicas/{read_replica_id} | Get a Read Replica
*ReadReplicasApi* | [**promote_read_replica**](docs/ReadReplicasApi.md#promote_read_replica) | **POST** /rdb/v1/regions/{region}/read-replicas/{read_replica_id}/promote | Promote a Read Replica
*ReadReplicasApi* | [**reset_read_replica**](docs/ReadReplicasApi.md#reset_read_replica) | **POST** /rdb/v1/regions/{region}/read-replicas/{read_replica_id}/reset | Resync a Read Replica
*RecordsApi* | [**clear_dns_zone_records**](docs/RecordsApi.md#clear_dns_zone_records) | **DELETE** /domain/v2beta1/dns-zones/{dns_zone}/records | Clear records within a DNS zone
*RecordsApi* | [**list_dns_zone_nameservers**](docs/RecordsApi.md#list_dns_zone_nameservers) | **GET** /domain/v2beta1/dns-zones/{dns_zone}/nameservers | List name servers within a DNS zone
*RecordsApi* | [**list_dns_zone_records**](docs/RecordsApi.md#list_dns_zone_records) | **GET** /domain/v2beta1/dns-zones/{dns_zone}/records | List records within a DNS zone
*RecordsApi* | [**update_dns_zone_nameservers**](docs/RecordsApi.md#update_dns_zone_nameservers) | **PUT** /domain/v2beta1/dns-zones/{dns_zone}/nameservers | Update name servers within a DNS zone
*RecordsApi* | [**update_dns_zone_records**](docs/RecordsApi.md#update_dns_zone_records) | **PATCH** /domain/v2beta1/dns-zones/{dns_zone}/records | Update records within a DNS zone
*ServerActionsApi* | [**reboot_server**](docs/ServerActionsApi.md#reboot_server) | **POST** /baremetal/v1/zones/{zone}/servers/{server_id}/reboot | Reboot an Elastic Metal server
*ServerActionsApi* | [**start_server**](docs/ServerActionsApi.md#start_server) | **POST** /baremetal/v1/zones/{zone}/servers/{server_id}/start | Start an Elastic Metal server
*ServerActionsApi* | [**stop_server**](docs/ServerActionsApi.md#stop_server) | **POST** /baremetal/v1/zones/{zone}/servers/{server_id}/stop | Stop an Elastic Metal server
*ServersApi* | [**add_option_server**](docs/ServersApi.md#add_option_server) | **POST** /baremetal/v1/zones/{zone}/servers/{server_id}/options/{option_id} | Add server option
*ServersApi* | [**create_server**](docs/ServersApi.md#create_server) | **POST** /baremetal/v1/zones/{zone}/servers | Create an Elastic Metal server
*ServersApi* | [**delete_option_server**](docs/ServersApi.md#delete_option_server) | **DELETE** /baremetal/v1/zones/{zone}/servers/{server_id}/options/{option_id} | Delete server option
*ServersApi* | [**delete_server**](docs/ServersApi.md#delete_server) | **DELETE** /baremetal/v1/zones/{zone}/servers/{server_id} | Delete an Elastic Metal server
*ServersApi* | [**get_server**](docs/ServersApi.md#get_server) | **GET** /baremetal/v1/zones/{zone}/servers/{server_id} | Get a specific Elastic Metal server
*ServersApi* | [**get_server_metrics**](docs/ServersApi.md#get_server_metrics) | **GET** /baremetal/v1/zones/{zone}/servers/{server_id}/metrics | Return server metrics
*ServersApi* | [**install_server**](docs/ServersApi.md#install_server) | **POST** /baremetal/v1/zones/{zone}/servers/{server_id}/install | Install an Elastic Metal server
*ServersApi* | [**list_server_events**](docs/ServersApi.md#list_server_events) | **GET** /baremetal/v1/zones/{zone}/servers/{server_id}/events | List server events
*ServersApi* | [**list_servers**](docs/ServersApi.md#list_servers) | **GET** /baremetal/v1/zones/{zone}/servers | List Elastic Metal servers for an Organization
*ServersApi* | [**migrate_server_to_monthly_offer**](docs/ServersApi.md#migrate_server_to_monthly_offer) | **POST** /baremetal/v1/zones/{zone}/servers/{server_id}/migrate-offer-monthly | Migrate server offer
*ServersApi* | [**update_ip**](docs/ServersApi.md#update_ip) | **PATCH** /baremetal/v1/zones/{zone}/servers/{server_id}/ips/{ip_id} | Update IP
*ServersApi* | [**update_server**](docs/ServersApi.md#update_server) | **PATCH** /baremetal/v1/zones/{zone}/servers/{server_id} | Update an Elastic Metal server
*SnapshotsApi* | [**create_instance_from_snapshot**](docs/SnapshotsApi.md#create_instance_from_snapshot) | **POST** /rdb/v1/regions/{region}/snapshots/{snapshot_id}/create-instance | Create a new Database Instance from a snapshot
*SnapshotsApi* | [**create_snapshot**](docs/SnapshotsApi.md#create_snapshot) | **POST** /rdb/v1/regions/{region}/instances/{instance_id}/snapshots | Create a Database Instance snapshot
*SnapshotsApi* | [**delete_snapshot**](docs/SnapshotsApi.md#delete_snapshot) | **DELETE** /rdb/v1/regions/{region}/snapshots/{snapshot_id} | Delete a Database Instance snapshot
*SnapshotsApi* | [**get_snapshot**](docs/SnapshotsApi.md#get_snapshot) | **GET** /rdb/v1/regions/{region}/snapshots/{snapshot_id} | Get a Database Instance snapshot
*SnapshotsApi* | [**list_snapshots**](docs/SnapshotsApi.md#list_snapshots) | **GET** /rdb/v1/regions/{region}/snapshots | List snapshots
*SnapshotsApi* | [**update_snapshot**](docs/SnapshotsApi.md#update_snapshot) | **PATCH** /rdb/v1/regions/{region}/snapshots/{snapshot_id} | Update a Database Instance snapshot
*TagsApi* | [**delete_tag**](docs/TagsApi.md#delete_tag) | **DELETE** /registry/v1/regions/{region}/tags/{tag_id} | Delete a tag
*TagsApi* | [**get_tag**](docs/TagsApi.md#get_tag) | **GET** /registry/v1/regions/{region}/tags/{tag_id} | Get a tag
*TagsApi* | [**list_tags**](docs/TagsApi.md#list_tags) | **GET** /registry/v1/regions/{region}/images/{image_id}/tags | List tags
*UsersApi* | [**create_user**](docs/UsersApi.md#create_user) | **POST** /rdb/v1/regions/{region}/instances/{instance_id}/users | Create a user for a Database Instance
*UsersApi* | [**delete_user**](docs/UsersApi.md#delete_user) | **DELETE** /rdb/v1/regions/{region}/instances/{instance_id}/users/{name} | Delete a user on a Database Instance
*UsersApi* | [**list_users**](docs/UsersApi.md#list_users) | **GET** /rdb/v1/regions/{region}/instances/{instance_id}/users | List users of a Database Instance
*UsersApi* | [**update_user**](docs/UsersApi.md#update_user) | **PATCH** /rdb/v1/regions/{region}/instances/{instance_id}/users/{name} | Update a user on a Database Instance
*VersionsApi* | [**get_dns_zone_version_diff**](docs/VersionsApi.md#get_dns_zone_version_diff) | **GET** /domain/v2beta1/dns-zones/version/{dns_zone_version_id}/diff | Access differences from a specific DNS zone version
*VersionsApi* | [**get_version**](docs/VersionsApi.md#get_version) | **GET** /k8s/v1/regions/{region}/versions/{version_name} | Get a Version
*VersionsApi* | [**list_dns_zone_version_records**](docs/VersionsApi.md#list_dns_zone_version_records) | **GET** /domain/v2beta1/dns-zones/version/{dns_zone_version_id} | List records from a given version of a specific DNS zone
*VersionsApi* | [**list_dns_zone_versions**](docs/VersionsApi.md#list_dns_zone_versions) | **GET** /domain/v2beta1/dns-zones/{dns_zone}/versions | List versions of a DNS zone
*VersionsApi* | [**list_versions**](docs/VersionsApi.md#list_versions) | **GET** /k8s/v1/regions/{region}/versions | List all available Versions
*VersionsApi* | [**restore_dns_zone_version**](docs/VersionsApi.md#restore_dns_zone_version) | **POST** /domain/v2beta1/dns-zones/version/{dns_zone_version_id}/restore | Restore a DNS zone version


## Documentation For Models

 - [AddClusterAclRulesRequest](docs/AddClusterAclRulesRequest.md)
 - [AddInstanceAclRulesRequest](docs/AddInstanceAclRulesRequest.md)
 - [AddInstanceSettingsRequest](docs/AddInstanceSettingsRequest.md)
 - [AddOptionServerRequest](docs/AddOptionServerRequest.md)
 - [AttachFlexibleIpRequest](docs/AttachFlexibleIpRequest.md)
 - [CloneDnsZoneRequest](docs/CloneDnsZoneRequest.md)
 - [CloneInstanceRequest](docs/CloneInstanceRequest.md)
 - [CreateClusterRequest](docs/CreateClusterRequest.md)
 - [CreateClusterRequestAutoUpgrade](docs/CreateClusterRequestAutoUpgrade.md)
 - [CreateClusterRequestAutoUpgradeMaintenanceWindow](docs/CreateClusterRequestAutoUpgradeMaintenanceWindow.md)
 - [CreateClusterRequestAutoscalerConfig](docs/CreateClusterRequestAutoscalerConfig.md)
 - [CreateClusterRequestAutoscalerConfigScaleDownUtilizationThreshold](docs/CreateClusterRequestAutoscalerConfigScaleDownUtilizationThreshold.md)
 - [CreateClusterRequestOpenIdConnectConfig](docs/CreateClusterRequestOpenIdConnectConfig.md)
 - [CreateDatabaseBackupRequest](docs/CreateDatabaseBackupRequest.md)
 - [CreateDatabaseRequest](docs/CreateDatabaseRequest.md)
 - [CreateDnsZoneRequest](docs/CreateDnsZoneRequest.md)
 - [CreateEndpointRequest](docs/CreateEndpointRequest.md)
 - [CreateEndpointRequestEndpointSpec](docs/CreateEndpointRequestEndpointSpec.md)
 - [CreateEndpointRequestEndpointSpecPrivateNetwork](docs/CreateEndpointRequestEndpointSpecPrivateNetwork.md)
 - [CreateFlexibleIpRequest](docs/CreateFlexibleIpRequest.md)
 - [CreateInstanceFromSnapshotRequest](docs/CreateInstanceFromSnapshotRequest.md)
 - [CreateInstanceRequest](docs/CreateInstanceRequest.md)
 - [CreateInstanceRequestEncryption](docs/CreateInstanceRequestEncryption.md)
 - [CreateNamespaceRequest](docs/CreateNamespaceRequest.md)
 - [CreatePoolRequest](docs/CreatePoolRequest.md)
 - [CreatePoolRequestKubeletArgs](docs/CreatePoolRequestKubeletArgs.md)
 - [CreatePoolRequestUpgradePolicy](docs/CreatePoolRequestUpgradePolicy.md)
 - [CreatePrivateNetworkRequest](docs/CreatePrivateNetworkRequest.md)
 - [CreateReadReplicaEndpointRequest](docs/CreateReadReplicaEndpointRequest.md)
 - [CreateReadReplicaRequest](docs/CreateReadReplicaRequest.md)
 - [CreateServerRequest](docs/CreateServerRequest.md)
 - [CreateServerRequestInstall](docs/CreateServerRequestInstall.md)
 - [CreateSnapshotRequest](docs/CreateSnapshotRequest.md)
 - [CreateSslCertificateRequest](docs/CreateSslCertificateRequest.md)
 - [CreateUserRequest](docs/CreateUserRequest.md)
 - [DeleteInstanceAclRulesRequest](docs/DeleteInstanceAclRulesRequest.md)
 - [DeleteInstanceSettingsRequest](docs/DeleteInstanceSettingsRequest.md)
 - [DetachFlexibleIpRequest](docs/DetachFlexibleIpRequest.md)
 - [DuplicateMacAddrRequest](docs/DuplicateMacAddrRequest.md)
 - [GenerateMacAddrRequest](docs/GenerateMacAddrRequest.md)
 - [ImportProviderDnsZoneRequest](docs/ImportProviderDnsZoneRequest.md)
 - [ImportProviderDnsZoneRequestOnlineV1](docs/ImportProviderDnsZoneRequestOnlineV1.md)
 - [ImportRawDnsZoneRequest](docs/ImportRawDnsZoneRequest.md)
 - [ImportRawDnsZoneRequestAxfrSource](docs/ImportRawDnsZoneRequestAxfrSource.md)
 - [ImportRawDnsZoneRequestBindSource](docs/ImportRawDnsZoneRequestBindSource.md)
 - [InstallServerRequest](docs/InstallServerRequest.md)
 - [MigrateEndpointRequest](docs/MigrateEndpointRequest.md)
 - [MigratePoolsToNewImagesRequest](docs/MigratePoolsToNewImagesRequest.md)
 - [MoveMacAddrRequest](docs/MoveMacAddrRequest.md)
 - [PrepareInstanceLogsRequest](docs/PrepareInstanceLogsRequest.md)
 - [PurgeInstanceLogsRequest](docs/PurgeInstanceLogsRequest.md)
 - [RebootServerRequest](docs/RebootServerRequest.md)
 - [RefreshDnsZoneRequest](docs/RefreshDnsZoneRequest.md)
 - [RestoreDatabaseBackupRequest](docs/RestoreDatabaseBackupRequest.md)
 - [ScalewayBaremetalV1GetServerMetricsResponsePings](docs/ScalewayBaremetalV1GetServerMetricsResponsePings.md)
 - [ScalewayBaremetalV1GetServerMetricsResponsePingsMetadata](docs/ScalewayBaremetalV1GetServerMetricsResponsePingsMetadata.md)
 - [ScalewayBaremetalV1OfferFee](docs/ScalewayBaremetalV1OfferFee.md)
 - [ScalewayBaremetalV1OfferOptionOfferLicense](docs/ScalewayBaremetalV1OfferOptionOfferLicense.md)
 - [ScalewayBaremetalV1OfferOptionOfferPrice](docs/ScalewayBaremetalV1OfferOptionOfferPrice.md)
 - [ScalewayBaremetalV1OfferOptionOfferPrivateNetwork](docs/ScalewayBaremetalV1OfferOptionOfferPrivateNetwork.md)
 - [ScalewayBaremetalV1OfferOptionOfferPublicBandwidth](docs/ScalewayBaremetalV1OfferOptionOfferPublicBandwidth.md)
 - [ScalewayBaremetalV1OfferPricePerHour](docs/ScalewayBaremetalV1OfferPricePerHour.md)
 - [ScalewayBaremetalV1OfferPricePerMonth](docs/ScalewayBaremetalV1OfferPricePerMonth.md)
 - [ScalewayBaremetalV1OsPassword](docs/ScalewayBaremetalV1OsPassword.md)
 - [ScalewayBaremetalV1OsServicePassword](docs/ScalewayBaremetalV1OsServicePassword.md)
 - [ScalewayBaremetalV1OsServiceUser](docs/ScalewayBaremetalV1OsServiceUser.md)
 - [ScalewayBaremetalV1OsSsh](docs/ScalewayBaremetalV1OsSsh.md)
 - [ScalewayBaremetalV1OsUser](docs/ScalewayBaremetalV1OsUser.md)
 - [ScalewayBaremetalV1ServerInstall](docs/ScalewayBaremetalV1ServerInstall.md)
 - [ScalewayBaremetalV1ServerRescueServer](docs/ScalewayBaremetalV1ServerRescueServer.md)
 - [ScalewayDomainV2beta1RecordChangeAdd](docs/ScalewayDomainV2beta1RecordChangeAdd.md)
 - [ScalewayDomainV2beta1RecordChangeDelete](docs/ScalewayDomainV2beta1RecordChangeDelete.md)
 - [ScalewayDomainV2beta1RecordChangeSet](docs/ScalewayDomainV2beta1RecordChangeSet.md)
 - [ScalewayDomainV2beta1RecordChangeSetIdFields](docs/ScalewayDomainV2beta1RecordChangeSetIdFields.md)
 - [ScalewayDomainV2beta1RecordGeoIpConfig](docs/ScalewayDomainV2beta1RecordGeoIpConfig.md)
 - [ScalewayDomainV2beta1RecordHttpServiceConfig](docs/ScalewayDomainV2beta1RecordHttpServiceConfig.md)
 - [ScalewayDomainV2beta1RecordViewConfig](docs/ScalewayDomainV2beta1RecordViewConfig.md)
 - [ScalewayDomainV2beta1RecordWeightedConfig](docs/ScalewayDomainV2beta1RecordWeightedConfig.md)
 - [ScalewayFlexibleIpV1alpha1FlexibleIpMacAddress](docs/ScalewayFlexibleIpV1alpha1FlexibleIpMacAddress.md)
 - [ScalewayK8sV1ClusterAutoUpgrade](docs/ScalewayK8sV1ClusterAutoUpgrade.md)
 - [ScalewayK8sV1ClusterAutoscalerConfig](docs/ScalewayK8sV1ClusterAutoscalerConfig.md)
 - [ScalewayK8sV1ClusterOpenIdConnectConfig](docs/ScalewayK8sV1ClusterOpenIdConnectConfig.md)
 - [ScalewayK8sV1CreateClusterRequestPoolConfigUpgradePolicy](docs/ScalewayK8sV1CreateClusterRequestPoolConfigUpgradePolicy.md)
 - [ScalewayK8sV1ExternalNodeNodeLabels](docs/ScalewayK8sV1ExternalNodeNodeLabels.md)
 - [ScalewayK8sV1NodeConditions](docs/ScalewayK8sV1NodeConditions.md)
 - [ScalewayK8sV1PoolUpgradePolicy](docs/ScalewayK8sV1PoolUpgradePolicy.md)
 - [ScalewayK8sV1VersionAvailableKubeletArgs](docs/ScalewayK8sV1VersionAvailableKubeletArgs.md)
 - [ScalewayPeriodBaremetalPeriodV1PeriodBmcAccess](docs/ScalewayPeriodBaremetalPeriodV1PeriodBmcAccess.md)
 - [ScalewayPeriodBaremetalPeriodV1PeriodCpu](docs/ScalewayPeriodBaremetalPeriodV1PeriodCpu.md)
 - [ScalewayPeriodBaremetalPeriodV1PeriodDisk](docs/ScalewayPeriodBaremetalPeriodV1PeriodDisk.md)
 - [ScalewayPeriodBaremetalPeriodV1PeriodGetServerMetricsResponse](docs/ScalewayPeriodBaremetalPeriodV1PeriodGetServerMetricsResponse.md)
 - [ScalewayPeriodBaremetalPeriodV1PeriodGpu](docs/ScalewayPeriodBaremetalPeriodV1PeriodGpu.md)
 - [ScalewayPeriodBaremetalPeriodV1PeriodIp](docs/ScalewayPeriodBaremetalPeriodV1PeriodIp.md)
 - [ScalewayPeriodBaremetalPeriodV1PeriodListOffersResponse](docs/ScalewayPeriodBaremetalPeriodV1PeriodListOffersResponse.md)
 - [ScalewayPeriodBaremetalPeriodV1PeriodListOptionsResponse](docs/ScalewayPeriodBaremetalPeriodV1PeriodListOptionsResponse.md)
 - [ScalewayPeriodBaremetalPeriodV1PeriodListOsResponse](docs/ScalewayPeriodBaremetalPeriodV1PeriodListOsResponse.md)
 - [ScalewayPeriodBaremetalPeriodV1PeriodListServerEventsResponse](docs/ScalewayPeriodBaremetalPeriodV1PeriodListServerEventsResponse.md)
 - [ScalewayPeriodBaremetalPeriodV1PeriodListServersResponse](docs/ScalewayPeriodBaremetalPeriodV1PeriodListServersResponse.md)
 - [ScalewayPeriodBaremetalPeriodV1PeriodListSettingsResponse](docs/ScalewayPeriodBaremetalPeriodV1PeriodListSettingsResponse.md)
 - [ScalewayPeriodBaremetalPeriodV1PeriodMemory](docs/ScalewayPeriodBaremetalPeriodV1PeriodMemory.md)
 - [ScalewayPeriodBaremetalPeriodV1PeriodOffer](docs/ScalewayPeriodBaremetalPeriodV1PeriodOffer.md)
 - [ScalewayPeriodBaremetalPeriodV1PeriodOfferPeriodOptionOffer](docs/ScalewayPeriodBaremetalPeriodV1PeriodOfferPeriodOptionOffer.md)
 - [ScalewayPeriodBaremetalPeriodV1PeriodOption](docs/ScalewayPeriodBaremetalPeriodV1PeriodOption.md)
 - [ScalewayPeriodBaremetalPeriodV1PeriodOs](docs/ScalewayPeriodBaremetalPeriodV1PeriodOs.md)
 - [ScalewayPeriodBaremetalPeriodV1PeriodPersistentMemory](docs/ScalewayPeriodBaremetalPeriodV1PeriodPersistentMemory.md)
 - [ScalewayPeriodBaremetalPeriodV1PeriodRaidController](docs/ScalewayPeriodBaremetalPeriodV1PeriodRaidController.md)
 - [ScalewayPeriodBaremetalPeriodV1PeriodSchema](docs/ScalewayPeriodBaremetalPeriodV1PeriodSchema.md)
 - [ScalewayPeriodBaremetalPeriodV1PeriodSchemaPeriodDisk](docs/ScalewayPeriodBaremetalPeriodV1PeriodSchemaPeriodDisk.md)
 - [ScalewayPeriodBaremetalPeriodV1PeriodSchemaPeriodFilesystem](docs/ScalewayPeriodBaremetalPeriodV1PeriodSchemaPeriodFilesystem.md)
 - [ScalewayPeriodBaremetalPeriodV1PeriodSchemaPeriodFilesystemPeriodFormat](docs/ScalewayPeriodBaremetalPeriodV1PeriodSchemaPeriodFilesystemPeriodFormat.md)
 - [ScalewayPeriodBaremetalPeriodV1PeriodSchemaPeriodPartition](docs/ScalewayPeriodBaremetalPeriodV1PeriodSchemaPeriodPartition.md)
 - [ScalewayPeriodBaremetalPeriodV1PeriodSchemaPeriodPartitionPeriodLabel](docs/ScalewayPeriodBaremetalPeriodV1PeriodSchemaPeriodPartitionPeriodLabel.md)
 - [ScalewayPeriodBaremetalPeriodV1PeriodSchemaPeriodPool](docs/ScalewayPeriodBaremetalPeriodV1PeriodSchemaPeriodPool.md)
 - [ScalewayPeriodBaremetalPeriodV1PeriodSchemaPeriodPoolPeriodType](docs/ScalewayPeriodBaremetalPeriodV1PeriodSchemaPeriodPoolPeriodType.md)
 - [ScalewayPeriodBaremetalPeriodV1PeriodSchemaPeriodRaid](docs/ScalewayPeriodBaremetalPeriodV1PeriodSchemaPeriodRaid.md)
 - [ScalewayPeriodBaremetalPeriodV1PeriodSchemaPeriodRaidPeriodLevel](docs/ScalewayPeriodBaremetalPeriodV1PeriodSchemaPeriodRaidPeriodLevel.md)
 - [ScalewayPeriodBaremetalPeriodV1PeriodSchemaPeriodZfs](docs/ScalewayPeriodBaremetalPeriodV1PeriodSchemaPeriodZfs.md)
 - [ScalewayPeriodBaremetalPeriodV1PeriodServer](docs/ScalewayPeriodBaremetalPeriodV1PeriodServer.md)
 - [ScalewayPeriodBaremetalPeriodV1PeriodServerEvent](docs/ScalewayPeriodBaremetalPeriodV1PeriodServerEvent.md)
 - [ScalewayPeriodBaremetalPeriodV1PeriodServerPeriodOption](docs/ScalewayPeriodBaremetalPeriodV1PeriodServerPeriodOption.md)
 - [ScalewayPeriodBaremetalPeriodV1PeriodSetting](docs/ScalewayPeriodBaremetalPeriodV1PeriodSetting.md)
 - [ScalewayPeriodDomainPeriodV2beta1PeriodDnsZone](docs/ScalewayPeriodDomainPeriodV2beta1PeriodDnsZone.md)
 - [ScalewayPeriodDomainPeriodV2beta1PeriodDnsZonePeriodStatus](docs/ScalewayPeriodDomainPeriodV2beta1PeriodDnsZonePeriodStatus.md)
 - [ScalewayPeriodDomainPeriodV2beta1PeriodDnsZoneVersion](docs/ScalewayPeriodDomainPeriodV2beta1PeriodDnsZoneVersion.md)
 - [ScalewayPeriodDomainPeriodV2beta1PeriodGetDnsZoneTsigKeyResponse](docs/ScalewayPeriodDomainPeriodV2beta1PeriodGetDnsZoneTsigKeyResponse.md)
 - [ScalewayPeriodDomainPeriodV2beta1PeriodGetDnsZoneVersionDiffResponse](docs/ScalewayPeriodDomainPeriodV2beta1PeriodGetDnsZoneVersionDiffResponse.md)
 - [ScalewayPeriodDomainPeriodV2beta1PeriodImportProviderDnsZoneResponse](docs/ScalewayPeriodDomainPeriodV2beta1PeriodImportProviderDnsZoneResponse.md)
 - [ScalewayPeriodDomainPeriodV2beta1PeriodImportRawDnsZoneRequestPeriodTsigKey](docs/ScalewayPeriodDomainPeriodV2beta1PeriodImportRawDnsZoneRequestPeriodTsigKey.md)
 - [ScalewayPeriodDomainPeriodV2beta1PeriodImportRawDnsZoneResponse](docs/ScalewayPeriodDomainPeriodV2beta1PeriodImportRawDnsZoneResponse.md)
 - [ScalewayPeriodDomainPeriodV2beta1PeriodLinkedProduct](docs/ScalewayPeriodDomainPeriodV2beta1PeriodLinkedProduct.md)
 - [ScalewayPeriodDomainPeriodV2beta1PeriodListDnsZoneNameserversResponse](docs/ScalewayPeriodDomainPeriodV2beta1PeriodListDnsZoneNameserversResponse.md)
 - [ScalewayPeriodDomainPeriodV2beta1PeriodListDnsZoneRecordsResponse](docs/ScalewayPeriodDomainPeriodV2beta1PeriodListDnsZoneRecordsResponse.md)
 - [ScalewayPeriodDomainPeriodV2beta1PeriodListDnsZoneVersionRecordsResponse](docs/ScalewayPeriodDomainPeriodV2beta1PeriodListDnsZoneVersionRecordsResponse.md)
 - [ScalewayPeriodDomainPeriodV2beta1PeriodListDnsZoneVersionsResponse](docs/ScalewayPeriodDomainPeriodV2beta1PeriodListDnsZoneVersionsResponse.md)
 - [ScalewayPeriodDomainPeriodV2beta1PeriodListDnsZonesResponse](docs/ScalewayPeriodDomainPeriodV2beta1PeriodListDnsZonesResponse.md)
 - [ScalewayPeriodDomainPeriodV2beta1PeriodListSslCertificatesResponse](docs/ScalewayPeriodDomainPeriodV2beta1PeriodListSslCertificatesResponse.md)
 - [ScalewayPeriodDomainPeriodV2beta1PeriodNameserver](docs/ScalewayPeriodDomainPeriodV2beta1PeriodNameserver.md)
 - [ScalewayPeriodDomainPeriodV2beta1PeriodRawFormat](docs/ScalewayPeriodDomainPeriodV2beta1PeriodRawFormat.md)
 - [ScalewayPeriodDomainPeriodV2beta1PeriodRecord](docs/ScalewayPeriodDomainPeriodV2beta1PeriodRecord.md)
 - [ScalewayPeriodDomainPeriodV2beta1PeriodRecordChange](docs/ScalewayPeriodDomainPeriodV2beta1PeriodRecordChange.md)
 - [ScalewayPeriodDomainPeriodV2beta1PeriodRecordPeriodGeoIpConfigPeriodMatch](docs/ScalewayPeriodDomainPeriodV2beta1PeriodRecordPeriodGeoIpConfigPeriodMatch.md)
 - [ScalewayPeriodDomainPeriodV2beta1PeriodRecordPeriodHttpServiceConfigPeriodStrategy](docs/ScalewayPeriodDomainPeriodV2beta1PeriodRecordPeriodHttpServiceConfigPeriodStrategy.md)
 - [ScalewayPeriodDomainPeriodV2beta1PeriodRecordPeriodType](docs/ScalewayPeriodDomainPeriodV2beta1PeriodRecordPeriodType.md)
 - [ScalewayPeriodDomainPeriodV2beta1PeriodRecordPeriodViewConfigPeriodView](docs/ScalewayPeriodDomainPeriodV2beta1PeriodRecordPeriodViewConfigPeriodView.md)
 - [ScalewayPeriodDomainPeriodV2beta1PeriodRecordPeriodWeightedConfigPeriodWeightedIp](docs/ScalewayPeriodDomainPeriodV2beta1PeriodRecordPeriodWeightedConfigPeriodWeightedIp.md)
 - [ScalewayPeriodDomainPeriodV2beta1PeriodRefreshDnsZoneResponse](docs/ScalewayPeriodDomainPeriodV2beta1PeriodRefreshDnsZoneResponse.md)
 - [ScalewayPeriodDomainPeriodV2beta1PeriodSslCertificate](docs/ScalewayPeriodDomainPeriodV2beta1PeriodSslCertificate.md)
 - [ScalewayPeriodDomainPeriodV2beta1PeriodSslCertificatePeriodStatus](docs/ScalewayPeriodDomainPeriodV2beta1PeriodSslCertificatePeriodStatus.md)
 - [ScalewayPeriodDomainPeriodV2beta1PeriodUpdateDnsZoneNameserversResponse](docs/ScalewayPeriodDomainPeriodV2beta1PeriodUpdateDnsZoneNameserversResponse.md)
 - [ScalewayPeriodDomainPeriodV2beta1PeriodUpdateDnsZoneRecordsResponse](docs/ScalewayPeriodDomainPeriodV2beta1PeriodUpdateDnsZoneRecordsResponse.md)
 - [ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodAttachFlexibleIpsResponse](docs/ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodAttachFlexibleIpsResponse.md)
 - [ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodDetachFlexibleIpsResponse](docs/ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodDetachFlexibleIpsResponse.md)
 - [ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodFlexibleIp](docs/ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodFlexibleIp.md)
 - [ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodFlexibleIpPeriodStatus](docs/ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodFlexibleIpPeriodStatus.md)
 - [ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodListFlexibleIpsResponse](docs/ScalewayPeriodFlexibleIpPeriodV1alpha1PeriodListFlexibleIpsResponse.md)
 - [ScalewayPeriodK8sPeriodV1PeriodAclRule](docs/ScalewayPeriodK8sPeriodV1PeriodAclRule.md)
 - [ScalewayPeriodK8sPeriodV1PeriodAclRuleRequest](docs/ScalewayPeriodK8sPeriodV1PeriodAclRuleRequest.md)
 - [ScalewayPeriodK8sPeriodV1PeriodAddClusterAclRulesResponse](docs/ScalewayPeriodK8sPeriodV1PeriodAddClusterAclRulesResponse.md)
 - [ScalewayPeriodK8sPeriodV1PeriodCluster](docs/ScalewayPeriodK8sPeriodV1PeriodCluster.md)
 - [ScalewayPeriodK8sPeriodV1PeriodClusterType](docs/ScalewayPeriodK8sPeriodV1PeriodClusterType.md)
 - [ScalewayPeriodK8sPeriodV1PeriodCni](docs/ScalewayPeriodK8sPeriodV1PeriodCni.md)
 - [ScalewayPeriodK8sPeriodV1PeriodCreateClusterRequestPeriodPoolConfig](docs/ScalewayPeriodK8sPeriodV1PeriodCreateClusterRequestPeriodPoolConfig.md)
 - [ScalewayPeriodK8sPeriodV1PeriodExternalNode](docs/ScalewayPeriodK8sPeriodV1PeriodExternalNode.md)
 - [ScalewayPeriodK8sPeriodV1PeriodExternalNodePeriodCoreV1Taint](docs/ScalewayPeriodK8sPeriodV1PeriodExternalNodePeriodCoreV1Taint.md)
 - [ScalewayPeriodK8sPeriodV1PeriodListClusterAclRulesResponse](docs/ScalewayPeriodK8sPeriodV1PeriodListClusterAclRulesResponse.md)
 - [ScalewayPeriodK8sPeriodV1PeriodListClusterAvailableTypesResponse](docs/ScalewayPeriodK8sPeriodV1PeriodListClusterAvailableTypesResponse.md)
 - [ScalewayPeriodK8sPeriodV1PeriodListClusterAvailableVersionsResponse](docs/ScalewayPeriodK8sPeriodV1PeriodListClusterAvailableVersionsResponse.md)
 - [ScalewayPeriodK8sPeriodV1PeriodListClusterTypesResponse](docs/ScalewayPeriodK8sPeriodV1PeriodListClusterTypesResponse.md)
 - [ScalewayPeriodK8sPeriodV1PeriodListClustersResponse](docs/ScalewayPeriodK8sPeriodV1PeriodListClustersResponse.md)
 - [ScalewayPeriodK8sPeriodV1PeriodListNodesResponse](docs/ScalewayPeriodK8sPeriodV1PeriodListNodesResponse.md)
 - [ScalewayPeriodK8sPeriodV1PeriodListPoolsResponse](docs/ScalewayPeriodK8sPeriodV1PeriodListPoolsResponse.md)
 - [ScalewayPeriodK8sPeriodV1PeriodListVersionsResponse](docs/ScalewayPeriodK8sPeriodV1PeriodListVersionsResponse.md)
 - [ScalewayPeriodK8sPeriodV1PeriodNode](docs/ScalewayPeriodK8sPeriodV1PeriodNode.md)
 - [ScalewayPeriodK8sPeriodV1PeriodPool](docs/ScalewayPeriodK8sPeriodV1PeriodPool.md)
 - [ScalewayPeriodK8sPeriodV1PeriodRuntime](docs/ScalewayPeriodK8sPeriodV1PeriodRuntime.md)
 - [ScalewayPeriodK8sPeriodV1PeriodSetClusterAclRulesResponse](docs/ScalewayPeriodK8sPeriodV1PeriodSetClusterAclRulesResponse.md)
 - [ScalewayPeriodK8sPeriodV1PeriodVersion](docs/ScalewayPeriodK8sPeriodV1PeriodVersion.md)
 - [ScalewayPeriodRdbPeriodV1PeriodAclRule](docs/ScalewayPeriodRdbPeriodV1PeriodAclRule.md)
 - [ScalewayPeriodRdbPeriodV1PeriodAclRulePeriodAction](docs/ScalewayPeriodRdbPeriodV1PeriodAclRulePeriodAction.md)
 - [ScalewayPeriodRdbPeriodV1PeriodAclRulePeriodDirection](docs/ScalewayPeriodRdbPeriodV1PeriodAclRulePeriodDirection.md)
 - [ScalewayPeriodRdbPeriodV1PeriodAclRulePeriodProtocol](docs/ScalewayPeriodRdbPeriodV1PeriodAclRulePeriodProtocol.md)
 - [ScalewayPeriodRdbPeriodV1PeriodAclRuleRequest](docs/ScalewayPeriodRdbPeriodV1PeriodAclRuleRequest.md)
 - [ScalewayPeriodRdbPeriodV1PeriodAddInstanceAclRulesResponse](docs/ScalewayPeriodRdbPeriodV1PeriodAddInstanceAclRulesResponse.md)
 - [ScalewayPeriodRdbPeriodV1PeriodAddInstanceSettingsResponse](docs/ScalewayPeriodRdbPeriodV1PeriodAddInstanceSettingsResponse.md)
 - [ScalewayPeriodRdbPeriodV1PeriodDatabase](docs/ScalewayPeriodRdbPeriodV1PeriodDatabase.md)
 - [ScalewayPeriodRdbPeriodV1PeriodDatabaseBackup](docs/ScalewayPeriodRdbPeriodV1PeriodDatabaseBackup.md)
 - [ScalewayPeriodRdbPeriodV1PeriodDatabaseEngine](docs/ScalewayPeriodRdbPeriodV1PeriodDatabaseEngine.md)
 - [ScalewayPeriodRdbPeriodV1PeriodDeleteInstanceAclRulesResponse](docs/ScalewayPeriodRdbPeriodV1PeriodDeleteInstanceAclRulesResponse.md)
 - [ScalewayPeriodRdbPeriodV1PeriodDeleteInstanceSettingsResponse](docs/ScalewayPeriodRdbPeriodV1PeriodDeleteInstanceSettingsResponse.md)
 - [ScalewayPeriodRdbPeriodV1PeriodEndpoint](docs/ScalewayPeriodRdbPeriodV1PeriodEndpoint.md)
 - [ScalewayPeriodRdbPeriodV1PeriodEndpointSpec](docs/ScalewayPeriodRdbPeriodV1PeriodEndpointSpec.md)
 - [ScalewayPeriodRdbPeriodV1PeriodEngineSetting](docs/ScalewayPeriodRdbPeriodV1PeriodEngineSetting.md)
 - [ScalewayPeriodRdbPeriodV1PeriodEngineVersion](docs/ScalewayPeriodRdbPeriodV1PeriodEngineVersion.md)
 - [ScalewayPeriodRdbPeriodV1PeriodInstance](docs/ScalewayPeriodRdbPeriodV1PeriodInstance.md)
 - [ScalewayPeriodRdbPeriodV1PeriodInstanceLog](docs/ScalewayPeriodRdbPeriodV1PeriodInstanceLog.md)
 - [ScalewayPeriodRdbPeriodV1PeriodInstanceMetrics](docs/ScalewayPeriodRdbPeriodV1PeriodInstanceMetrics.md)
 - [ScalewayPeriodRdbPeriodV1PeriodInstanceSetting](docs/ScalewayPeriodRdbPeriodV1PeriodInstanceSetting.md)
 - [ScalewayPeriodRdbPeriodV1PeriodListDatabaseBackupsResponse](docs/ScalewayPeriodRdbPeriodV1PeriodListDatabaseBackupsResponse.md)
 - [ScalewayPeriodRdbPeriodV1PeriodListDatabaseEnginesResponse](docs/ScalewayPeriodRdbPeriodV1PeriodListDatabaseEnginesResponse.md)
 - [ScalewayPeriodRdbPeriodV1PeriodListDatabasesResponse](docs/ScalewayPeriodRdbPeriodV1PeriodListDatabasesResponse.md)
 - [ScalewayPeriodRdbPeriodV1PeriodListInstanceAclRulesResponse](docs/ScalewayPeriodRdbPeriodV1PeriodListInstanceAclRulesResponse.md)
 - [ScalewayPeriodRdbPeriodV1PeriodListInstanceLogsDetailsResponse](docs/ScalewayPeriodRdbPeriodV1PeriodListInstanceLogsDetailsResponse.md)
 - [ScalewayPeriodRdbPeriodV1PeriodListInstanceLogsDetailsResponsePeriodInstanceLogDetail](docs/ScalewayPeriodRdbPeriodV1PeriodListInstanceLogsDetailsResponsePeriodInstanceLogDetail.md)
 - [ScalewayPeriodRdbPeriodV1PeriodListInstanceLogsResponse](docs/ScalewayPeriodRdbPeriodV1PeriodListInstanceLogsResponse.md)
 - [ScalewayPeriodRdbPeriodV1PeriodListInstancesResponse](docs/ScalewayPeriodRdbPeriodV1PeriodListInstancesResponse.md)
 - [ScalewayPeriodRdbPeriodV1PeriodListNodeTypesResponse](docs/ScalewayPeriodRdbPeriodV1PeriodListNodeTypesResponse.md)
 - [ScalewayPeriodRdbPeriodV1PeriodListPrivilegesResponse](docs/ScalewayPeriodRdbPeriodV1PeriodListPrivilegesResponse.md)
 - [ScalewayPeriodRdbPeriodV1PeriodListSnapshotsResponse](docs/ScalewayPeriodRdbPeriodV1PeriodListSnapshotsResponse.md)
 - [ScalewayPeriodRdbPeriodV1PeriodListUsersResponse](docs/ScalewayPeriodRdbPeriodV1PeriodListUsersResponse.md)
 - [ScalewayPeriodRdbPeriodV1PeriodMaintenance](docs/ScalewayPeriodRdbPeriodV1PeriodMaintenance.md)
 - [ScalewayPeriodRdbPeriodV1PeriodNodeType](docs/ScalewayPeriodRdbPeriodV1PeriodNodeType.md)
 - [ScalewayPeriodRdbPeriodV1PeriodNodeTypePeriodVolumeType](docs/ScalewayPeriodRdbPeriodV1PeriodNodeTypePeriodVolumeType.md)
 - [ScalewayPeriodRdbPeriodV1PeriodPrepareInstanceLogsResponse](docs/ScalewayPeriodRdbPeriodV1PeriodPrepareInstanceLogsResponse.md)
 - [ScalewayPeriodRdbPeriodV1PeriodPrivilege](docs/ScalewayPeriodRdbPeriodV1PeriodPrivilege.md)
 - [ScalewayPeriodRdbPeriodV1PeriodReadReplica](docs/ScalewayPeriodRdbPeriodV1PeriodReadReplica.md)
 - [ScalewayPeriodRdbPeriodV1PeriodReadReplicaEndpointSpec](docs/ScalewayPeriodRdbPeriodV1PeriodReadReplicaEndpointSpec.md)
 - [ScalewayPeriodRdbPeriodV1PeriodSetInstanceAclRulesResponse](docs/ScalewayPeriodRdbPeriodV1PeriodSetInstanceAclRulesResponse.md)
 - [ScalewayPeriodRdbPeriodV1PeriodSetInstanceSettingsResponse](docs/ScalewayPeriodRdbPeriodV1PeriodSetInstanceSettingsResponse.md)
 - [ScalewayPeriodRdbPeriodV1PeriodSnapshot](docs/ScalewayPeriodRdbPeriodV1PeriodSnapshot.md)
 - [ScalewayPeriodRdbPeriodV1PeriodStorageClass](docs/ScalewayPeriodRdbPeriodV1PeriodStorageClass.md)
 - [ScalewayPeriodRdbPeriodV1PeriodUpgradableVersion](docs/ScalewayPeriodRdbPeriodV1PeriodUpgradableVersion.md)
 - [ScalewayPeriodRdbPeriodV1PeriodUser](docs/ScalewayPeriodRdbPeriodV1PeriodUser.md)
 - [ScalewayPeriodRdbPeriodV1PeriodVolumePeriodType](docs/ScalewayPeriodRdbPeriodV1PeriodVolumePeriodType.md)
 - [ScalewayPeriodRegistryPeriodV1PeriodImage](docs/ScalewayPeriodRegistryPeriodV1PeriodImage.md)
 - [ScalewayPeriodRegistryPeriodV1PeriodListImagesResponse](docs/ScalewayPeriodRegistryPeriodV1PeriodListImagesResponse.md)
 - [ScalewayPeriodRegistryPeriodV1PeriodListNamespacesResponse](docs/ScalewayPeriodRegistryPeriodV1PeriodListNamespacesResponse.md)
 - [ScalewayPeriodRegistryPeriodV1PeriodListTagsResponse](docs/ScalewayPeriodRegistryPeriodV1PeriodListTagsResponse.md)
 - [ScalewayPeriodRegistryPeriodV1PeriodNamespace](docs/ScalewayPeriodRegistryPeriodV1PeriodNamespace.md)
 - [ScalewayPeriodRegistryPeriodV1PeriodTag](docs/ScalewayPeriodRegistryPeriodV1PeriodTag.md)
 - [ScalewayPeriodStdPeriodFile](docs/ScalewayPeriodStdPeriodFile.md)
 - [ScalewayPeriodStdPeriodTimeSeries](docs/ScalewayPeriodStdPeriodTimeSeries.md)
 - [ScalewayPeriodStdPeriodTimeSeriesPeriodPoint](docs/ScalewayPeriodStdPeriodTimeSeriesPeriodPoint.md)
 - [ScalewayPeriodVpcPeriodV1PeriodListPrivateNetworksResponse](docs/ScalewayPeriodVpcPeriodV1PeriodListPrivateNetworksResponse.md)
 - [ScalewayPeriodVpcPeriodV1PeriodPrivateNetwork](docs/ScalewayPeriodVpcPeriodV1PeriodPrivateNetwork.md)
 - [ScalewayRdbV1EndpointPrivateNetwork](docs/ScalewayRdbV1EndpointPrivateNetwork.md)
 - [ScalewayRdbV1EngineSettingFloatMax](docs/ScalewayRdbV1EngineSettingFloatMax.md)
 - [ScalewayRdbV1EngineSettingFloatMin](docs/ScalewayRdbV1EngineSettingFloatMin.md)
 - [ScalewayRdbV1InstanceBackupSchedule](docs/ScalewayRdbV1InstanceBackupSchedule.md)
 - [ScalewayRdbV1InstanceEndpoint](docs/ScalewayRdbV1InstanceEndpoint.md)
 - [ScalewayRdbV1InstanceVolume](docs/ScalewayRdbV1InstanceVolume.md)
 - [ScalewayRdbV1NodeTypeVolumeConstraint](docs/ScalewayRdbV1NodeTypeVolumeConstraint.md)
 - [ScalewayRdbV1ReadReplicaEndpointSpecPrivateNetwork](docs/ScalewayRdbV1ReadReplicaEndpointSpecPrivateNetwork.md)
 - [ScalewayRdbV1SnapshotVolumeType](docs/ScalewayRdbV1SnapshotVolumeType.md)
 - [ScalewayStdTimeSeriesPointInner](docs/ScalewayStdTimeSeriesPointInner.md)
 - [SetClusterAclRulesRequest](docs/SetClusterAclRulesRequest.md)
 - [SetClusterTypeRequest](docs/SetClusterTypeRequest.md)
 - [SetInstanceAclRulesRequest](docs/SetInstanceAclRulesRequest.md)
 - [SetInstanceSettingsRequest](docs/SetInstanceSettingsRequest.md)
 - [SetPrivilegeRequest](docs/SetPrivilegeRequest.md)
 - [StartBmcAccessRequest](docs/StartBmcAccessRequest.md)
 - [UpdateClusterRequest](docs/UpdateClusterRequest.md)
 - [UpdateClusterRequestAutoUpgrade](docs/UpdateClusterRequestAutoUpgrade.md)
 - [UpdateClusterRequestAutoscalerConfig](docs/UpdateClusterRequestAutoscalerConfig.md)
 - [UpdateClusterRequestOpenIdConnectConfig](docs/UpdateClusterRequestOpenIdConnectConfig.md)
 - [UpdateDatabaseBackupRequest](docs/UpdateDatabaseBackupRequest.md)
 - [UpdateDnsZoneNameserversRequest](docs/UpdateDnsZoneNameserversRequest.md)
 - [UpdateDnsZoneRecordsRequest](docs/UpdateDnsZoneRecordsRequest.md)
 - [UpdateDnsZoneRequest](docs/UpdateDnsZoneRequest.md)
 - [UpdateFlexibleIpRequest](docs/UpdateFlexibleIpRequest.md)
 - [UpdateImageRequest](docs/UpdateImageRequest.md)
 - [UpdateInstanceRequest](docs/UpdateInstanceRequest.md)
 - [UpdateInstanceRequestLogsPolicy](docs/UpdateInstanceRequestLogsPolicy.md)
 - [UpdateIpRequest](docs/UpdateIpRequest.md)
 - [UpdateNamespaceRequest](docs/UpdateNamespaceRequest.md)
 - [UpdatePoolRequest](docs/UpdatePoolRequest.md)
 - [UpdatePoolRequestKubeletArgs](docs/UpdatePoolRequestKubeletArgs.md)
 - [UpdatePoolRequestUpgradePolicy](docs/UpdatePoolRequestUpgradePolicy.md)
 - [UpdatePrivateNetworkRequest](docs/UpdatePrivateNetworkRequest.md)
 - [UpdateServerRequest](docs/UpdateServerRequest.md)
 - [UpdateSettingRequest](docs/UpdateSettingRequest.md)
 - [UpdateSnapshotRequest](docs/UpdateSnapshotRequest.md)
 - [UpdateUserRequest](docs/UpdateUserRequest.md)
 - [UpgradeClusterRequest](docs/UpgradeClusterRequest.md)
 - [UpgradeInstanceRequest](docs/UpgradeInstanceRequest.md)
 - [UpgradeInstanceRequestMajorUpgradeWorkflow](docs/UpgradeInstanceRequestMajorUpgradeWorkflow.md)
 - [UpgradePoolRequest](docs/UpgradePoolRequest.md)
 - [ValidatePartitioningSchemaRequest](docs/ValidatePartitioningSchemaRequest.md)
 - [ValidatePartitioningSchemaRequestPartitioningSchema](docs/ValidatePartitioningSchemaRequestPartitioningSchema.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



