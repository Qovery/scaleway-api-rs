# ScalewayIotV1RouteDbConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**engine** | Option<**String**> | Database engine the route will connect to. If not specified, will default to 'PostgreSQL' | [optional][default to Engine_Unknown]
**host** | **String** | Database host | 
**port** | **i64** | Database port | 
**dbname** | **String** | Database name | 
**username** | **String** | Database username. Make sure this account can execute the provided query | 
**password** | **String** | Database password | 
**query** | Option<**String**> | SQL query to be executed ($TOPIC and $PAYLOAD variables are available, see documentation) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


