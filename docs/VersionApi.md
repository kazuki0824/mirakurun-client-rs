# \VersionApi

All URIs are relative to *http://localhost:40772/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**check_version**](VersionApi.md#check_version) | **GET** /version | 
[**update_version**](VersionApi.md#update_version) | **PUT** /version/update | 



## check_version

> crate::models::Version check_version()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Version**](Version.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_version

> update_version(force)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**force** | Option<**bool**> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

