# \StreamApi

All URIs are relative to *http://localhost:40772/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_channel_stream**](StreamApi.md#get_channel_stream) | **GET** /channels/{type}/{channel}/stream | 
[**get_events_stream**](StreamApi.md#get_events_stream) | **GET** /events/stream | 
[**get_log_stream**](StreamApi.md#get_log_stream) | **GET** /log/stream | 
[**get_program_stream**](StreamApi.md#get_program_stream) | **GET** /programs/{id}/stream | 
[**get_service_stream**](StreamApi.md#get_service_stream) | **GET** /services/{id}/stream | 
[**get_service_stream_by_channel**](StreamApi.md#get_service_stream_by_channel) | **GET** /channels/{type}/{channel}/services/{id}/stream | 



## get_channel_stream

> get_channel_stream(_type, channel, x_mirakurun_priority, decode)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**_type** | **String** |  | [required] |
**channel** | **String** |  | [required] |
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


## get_events_stream

> Vec<crate::models::Event> get_events_stream(resource, _type)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**resource** | Option<**String**> |  |  |
**_type** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::Event>**](Event.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_log_stream

> get_log_stream()


### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

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


## get_service_stream

> get_service_stream(id, x_mirakurun_priority, decode)


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


## get_service_stream_by_channel

> get_service_stream_by_channel(_type, channel, id, x_mirakurun_priority, decode)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**_type** | **String** |  | [required] |
**channel** | **String** |  | [required] |
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

