# \ProgramsApi

All URIs are relative to *http://localhost:40772/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_program**](ProgramsApi.md#get_program) | **GET** /programs/{id} | 
[**get_program_stream**](ProgramsApi.md#get_program_stream) | **GET** /programs/{id}/stream | 
[**get_programs**](ProgramsApi.md#get_programs) | **GET** /programs | 



## get_program

> crate::models::Program get_program(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**crate::models::Program**](Program.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_program_stream

> get_program_stream(id, x_mirakurun_priority, decode)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**x_mirakurun_priority** | Option<**i32**> |  |  |
**decode** | Option<**i32**> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_programs

> Vec<crate::models::Program> get_programs(network_id, service_id, event_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network_id** | Option<**i32**> |  |  |
**service_id** | Option<**i32**> |  |  |
**event_id** | Option<**i32**> |  |  |

### Return type

[**Vec<crate::models::Program>**](Program.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

