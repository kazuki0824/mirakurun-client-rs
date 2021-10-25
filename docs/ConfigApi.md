# \ConfigApi

All URIs are relative to *http://localhost:40772/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**channel_scan**](ConfigApi.md#channel_scan) | **PUT** /config/channels/scan | Channel Scan
[**get_channels_config**](ConfigApi.md#get_channels_config) | **GET** /config/channels | 
[**get_server_config**](ConfigApi.md#get_server_config) | **GET** /config/server | 
[**get_tuners_config**](ConfigApi.md#get_tuners_config) | **GET** /config/tuners | 
[**update_channels_config**](ConfigApi.md#update_channels_config) | **PUT** /config/channels | 
[**update_server_config**](ConfigApi.md#update_server_config) | **PUT** /config/server | 
[**update_tuners_config**](ConfigApi.md#update_tuners_config) | **PUT** /config/tuners | 



## channel_scan

> channel_scan(dry_run, _type, min_ch, max_ch, min_sub_ch, max_sub_ch, use_sub_ch, scan_mode, set_disabled_on_add, refresh)
Channel Scan

Entry rewriting specifications: - The scan is performed on a range of channels of the specified type and the entries for those channels, if any, are saved in the configuration file. - If the channel to be scanned is described in the configuration file and is enabled, the scan will not be performed for that channel and the entries described will remain intact. If you do not want to keep the entries, use the `refresh` option. - All entries outside the channel range of the specified type will be deleted. - All entries of a type other than the specified type will remain.  About BS Subchannel Style: - Only when scanning BS, you can specify the channel number in the subchannel style (e.g. BS01_0). To specify the channel number, use minSubCh and maxSubCh in addition to minCh and maxCh. - The subchannel number parameters (minSubCh, maxSubCh) are used only if the type is BS and are ignored otherwise. - Subchannel style scans scan in the following range:     From `BS${minCh}_${minSubCh}` to `BS${maxCh}_${maxSubCh}` - In the subchannel style, minCh and maxCh are zero padded to two digits. minSubCh and maxSubCh are not padded. - BS \"non\" subchannel style scans and GR scans are basically the same. Note that if you scan the wrong channel range, the GR channel will be registered as BS and the BS channel will be registered as GR. This problem does not occur because CS scan uses a character string with `CS` added as a channel number prefix.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dry_run** | Option<**bool**> | dry run. If `true`, the scanned result will not be saved. |  |[default to false]
**_type** | Option<**String**> | Specifies the channel type to scan. |  |[default to GR]
**min_ch** | Option<**i32**> | Specifies the minimum number of channel numbers to scan. |  |
**max_ch** | Option<**i32**> | Specifies the maximum number of channel numbers to scan. |  |
**min_sub_ch** | Option<**i32**> | Specifies the minimum number of subchannel numbers to scan. This parameter is only used if the type is `BS` and the bs_subch_style is `true`. |  |
**max_sub_ch** | Option<**i32**> | Specifies the maximum number of subchannel numbers to scan. This parameter is only used if the type is `BS` and the bs_subch_style is `true`. |  |
**use_sub_ch** | Option<**bool**> | Specify true to specify the channel in the subchannel style. Only used for BS scans. (e.g. BS01_0) |  |[default to true]
**scan_mode** | Option<**String**> | To specify the service explictly, use the `Service` mode.  _Default value (GR)_: Channel _Default value (BS/CS)_: Service |  |
**set_disabled_on_add** | Option<**bool**> | If `true`, set disable on add channel.  _Default value (GR)_: false _Default value (BS/CS)_: true |  |
**refresh** | Option<**bool**> | If `true`, update the existing settings without inheriting them. However, non-scanned types of channels will always be inherited. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_channels_config

> Vec<crate::models::ConfigChannelsItem> get_channels_config()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::ConfigChannelsItem>**](ConfigChannelsItem.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_server_config

> crate::models::ConfigServer get_server_config()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ConfigServer**](ConfigServer.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tuners_config

> Vec<crate::models::ConfigTunersItem> get_tuners_config()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::ConfigTunersItem>**](ConfigTunersItem.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_channels_config

> Vec<crate::models::ConfigChannelsItem> update_channels_config(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**Vec<crate::models::ConfigChannelsItem>**](ConfigChannelsItem.md)> |  |  |

### Return type

[**Vec<crate::models::ConfigChannelsItem>**](ConfigChannelsItem.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_server_config

> crate::models::ConfigServer update_server_config(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ConfigServer**](ConfigServer.md)> |  |  |

### Return type

[**crate::models::ConfigServer**](ConfigServer.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_tuners_config

> Vec<crate::models::ConfigTunersItem> update_tuners_config(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**Vec<crate::models::ConfigTunersItem>**](ConfigTunersItem.md)> |  |  |

### Return type

[**Vec<crate::models::ConfigTunersItem>**](ConfigTunersItem.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

