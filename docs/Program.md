# Program

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | 
**event_id** | **i32** |  | 
**service_id** | **i32** |  | 
**network_id** | **i32** |  | 
**start_at** | **i32** |  | 
**duration** | **i32** |  | 
**is_free** | **bool** |  | 
**name** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**genres** | Option<[**Vec<crate::models::ProgramGenre>**](ProgramGenre.md)> |  | [optional]
**video** | Option<[**crate::models::ProgramVideo**](Program_video.md)> |  | [optional]
**audios** | Option<[**Vec<crate::models::ProgramAudiosInner>**](Program_audios_inner.md)> |  | [optional]
**extended** | Option<[**serde_json::Value**](.md)> |  | [optional]
**related_items** | Option<[**Vec<crate::models::RelatedItem>**](RelatedItem.md)> |  | [optional]
**series** | Option<[**crate::models::ProgramSeries**](Program_series.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


