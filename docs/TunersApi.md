# \TunersApi

All URIs are relative to *http://localhost:40772/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_tuner**](TunersApi.md#get_tuner) | **GET** /tuners/{index} | 
[**get_tuner_process**](TunersApi.md#get_tuner_process) | **GET** /tuners/{index}/process | Get Tuner Process Info
[**get_tuners**](TunersApi.md#get_tuners) | **GET** /tuners | 
[**kill_tuner_process**](TunersApi.md#kill_tuner_process) | **DELETE** /tuners/{index}/process | Kill Tuner Process



## get_tuner

> crate::models::TunerDevice get_tuner(index)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **i32** |  | [required] |

### Return type

[**crate::models::TunerDevice**](TunerDevice.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tuner_process

> crate::models::TunerProcess get_tuner_process(index)
Get Tuner Process Info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **i32** |  | [required] |

### Return type

[**crate::models::TunerProcess**](TunerProcess.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tuners

> Vec<crate::models::TunerDevice> get_tuners()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::TunerDevice>**](TunerDevice.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kill_tuner_process

> serde_json::Value kill_tuner_process(index)
Kill Tuner Process

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **i32** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

