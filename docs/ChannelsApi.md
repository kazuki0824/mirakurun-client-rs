# \ChannelsApi

All URIs are relative to *http://localhost:40772/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_channel**](ChannelsApi.md#get_channel) | **GET** /channels/{type}/{channel} | 
[**get_channel_stream**](ChannelsApi.md#get_channel_stream) | **GET** /channels/{type}/{channel}/stream | 
[**get_channels**](ChannelsApi.md#get_channels) | **GET** /channels | 
[**get_channels_by_type**](ChannelsApi.md#get_channels_by_type) | **GET** /channels/{type} | 
[**get_service_by_channel**](ChannelsApi.md#get_service_by_channel) | **GET** /channels/{type}/{channel}/services/{id} | 
[**get_service_stream_by_channel**](ChannelsApi.md#get_service_stream_by_channel) | **GET** /channels/{type}/{channel}/services/{id}/stream | 
[**get_services_by_channel**](ChannelsApi.md#get_services_by_channel) | **GET** /channels/{type}/{channel}/services | 



## get_channel

> crate::models::Channel get_channel(r#type, channel)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** |  | [required] |
**channel** | **String** |  | [required] |

### Return type

[**crate::models::Channel**](Channel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_channel_stream

> get_channel_stream(r#type, channel, x_mirakurun_priority, decode)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** |  | [required] |
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


## get_channels

> Vec<crate::models::Channel> get_channels(r#type, channel, name)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | Option<**String**> |  |  |
**channel** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::Channel>**](Channel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_channels_by_type

> Vec<crate::models::Channel> get_channels_by_type(r#type, channel, name)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** |  | [required] |
**channel** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::Channel>**](Channel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_service_by_channel

> Vec<crate::models::Service> get_service_by_channel(r#type, channel, id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** |  | [required] |
**channel** | **String** |  | [required] |
**id** | **i32** |  | [required] |

### Return type

[**Vec<crate::models::Service>**](Service.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_service_stream_by_channel

> get_service_stream_by_channel(r#type, channel, id, x_mirakurun_priority, decode)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** |  | [required] |
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


## get_services_by_channel

> Vec<crate::models::Service> get_services_by_channel(r#type, channel)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** |  | [required] |
**channel** | **String** |  | [required] |

### Return type

[**Vec<crate::models::Service>**](Service.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

