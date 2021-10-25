# \ServicesApi

All URIs are relative to *http://localhost:40772/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_logo_image**](ServicesApi.md#get_logo_image) | **GET** /services/{id}/logo | 
[**get_service**](ServicesApi.md#get_service) | **GET** /services/{id} | 
[**get_service_by_channel**](ServicesApi.md#get_service_by_channel) | **GET** /channels/{type}/{channel}/services/{id} | 
[**get_service_stream**](ServicesApi.md#get_service_stream) | **GET** /services/{id}/stream | 
[**get_service_stream_by_channel**](ServicesApi.md#get_service_stream_by_channel) | **GET** /channels/{type}/{channel}/services/{id}/stream | 
[**get_services**](ServicesApi.md#get_services) | **GET** /services | 
[**get_services_by_channel**](ServicesApi.md#get_services_by_channel) | **GET** /channels/{type}/{channel}/services | 



## get_logo_image

> get_logo_image(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_service

> crate::models::Service get_service(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**crate::models::Service**](Service.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_service_by_channel

> Vec<crate::models::Service> get_service_by_channel(_type, channel, id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**_type** | **String** |  | [required] |
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


## get_services

> Vec<crate::models::Service> get_services(service_id, network_id, name, _type, channel_type, channel_channel)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | Option<**i32**> |  |  |
**network_id** | Option<**i32**> |  |  |
**name** | Option<**String**> |  |  |
**_type** | Option<**i32**> |  |  |
**channel_type** | Option<**String**> |  |  |
**channel_channel** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::Service>**](Service.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_services_by_channel

> Vec<crate::models::Service> get_services_by_channel(_type, channel)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**_type** | **String** |  | [required] |
**channel** | **String** |  | [required] |

### Return type

[**Vec<crate::models::Service>**](Service.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

