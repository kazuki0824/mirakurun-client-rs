# \EventsApi

All URIs are relative to *http://localhost:40772/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_events**](EventsApi.md#get_events) | **GET** /events | 
[**get_events_stream**](EventsApi.md#get_events_stream) | **GET** /events/stream | 



## get_events

> Vec<crate::models::Event> get_events()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Event>**](Event.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

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

