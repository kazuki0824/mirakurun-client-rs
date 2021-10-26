![ci workflow](https://github.com/kazuki0824/mirakurun-client-rs/actions/workflows/ci.yml/badge.svg)

# Rust API client for Mirakurun

DVR Tuner Server Service for Chinachu Air.

For more information, please visit [https://github.com/kanreisa](https://github.com/kanreisa)

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version of Mirakurun: 3.9.0-beta.7
- Package version of Mirakurun: *same*
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Add the following to `Cargo.toml` under `[dependencies]`:

```
mirakurun_client = "^1.0"
```

## Documentation for API Endpoints

All URIs are relative to *http://{your_host}:{your_port(40772)}/api*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*ChannelsApi* | [**get_channel**](docs/ChannelsApi.md#get_channel) | **GET** /channels/{type}/{channel} | 
*ChannelsApi* | [**get_channel_stream**](docs/ChannelsApi.md#get_channel_stream) | **GET** /channels/{type}/{channel}/stream | 
*ChannelsApi* | [**get_channels**](docs/ChannelsApi.md#get_channels) | **GET** /channels | 
*ChannelsApi* | [**get_channels_by_type**](docs/ChannelsApi.md#get_channels_by_type) | **GET** /channels/{type} | 
*ChannelsApi* | [**get_service_by_channel**](docs/ChannelsApi.md#get_service_by_channel) | **GET** /channels/{type}/{channel}/services/{id} | 
*ChannelsApi* | [**get_service_stream_by_channel**](docs/ChannelsApi.md#get_service_stream_by_channel) | **GET** /channels/{type}/{channel}/services/{id}/stream | 
*ChannelsApi* | [**get_services_by_channel**](docs/ChannelsApi.md#get_services_by_channel) | **GET** /channels/{type}/{channel}/services | 
*ConfigApi* | [**channel_scan**](docs/ConfigApi.md#channel_scan) | **PUT** /config/channels/scan | Channel Scan
*ConfigApi* | [**get_channels_config**](docs/ConfigApi.md#get_channels_config) | **GET** /config/channels | 
*ConfigApi* | [**get_server_config**](docs/ConfigApi.md#get_server_config) | **GET** /config/server | 
*ConfigApi* | [**get_tuners_config**](docs/ConfigApi.md#get_tuners_config) | **GET** /config/tuners | 
*ConfigApi* | [**update_channels_config**](docs/ConfigApi.md#update_channels_config) | **PUT** /config/channels | 
*ConfigApi* | [**update_server_config**](docs/ConfigApi.md#update_server_config) | **PUT** /config/server | 
*ConfigApi* | [**update_tuners_config**](docs/ConfigApi.md#update_tuners_config) | **PUT** /config/tuners | 
*EventsApi* | [**get_events**](docs/EventsApi.md#get_events) | **GET** /events | 
*EventsApi* | [**get_events_stream**](docs/EventsApi.md#get_events_stream) | **GET** /events/stream | 
*IptvApi* | [**iptv_discover_json_get**](docs/IptvApi.md#iptv_discover_json_get) | **GET** /iptv/discover.json | IPTV - Media Server Support
*IptvApi* | [**iptv_lineup_json_get**](docs/IptvApi.md#iptv_lineup_json_get) | **GET** /iptv/lineup.json | IPTV - Media Server Support
*IptvApi* | [**iptv_lineup_status_json_get**](docs/IptvApi.md#iptv_lineup_status_json_get) | **GET** /iptv/lineup_status.json | IPTV - Media Server Support
*IptvApi* | [**iptv_playlist_get**](docs/IptvApi.md#iptv_playlist_get) | **GET** /iptv/playlist | IPTV - M3U Playlist
*IptvApi* | [**iptv_xmltv_get**](docs/IptvApi.md#iptv_xmltv_get) | **GET** /iptv/xmltv | IPTV - XMLTV EPG Data
*LogApi* | [**get_log**](docs/LogApi.md#get_log) | **GET** /log | 
*LogApi* | [**get_log_stream**](docs/LogApi.md#get_log_stream) | **GET** /log/stream | 
*MiscApi* | [**restart**](docs/MiscApi.md#restart) | **PUT** /restart | Restart Mirakurun
*ProgramsApi* | [**get_program**](docs/ProgramsApi.md#get_program) | **GET** /programs/{id} | 
*ProgramsApi* | [**get_program_stream**](docs/ProgramsApi.md#get_program_stream) | **GET** /programs/{id}/stream | 
*ProgramsApi* | [**get_programs**](docs/ProgramsApi.md#get_programs) | **GET** /programs | 
*ServicesApi* | [**get_logo_image**](docs/ServicesApi.md#get_logo_image) | **GET** /services/{id}/logo | 
*ServicesApi* | [**get_service**](docs/ServicesApi.md#get_service) | **GET** /services/{id} | 
*ServicesApi* | [**get_service_by_channel**](docs/ServicesApi.md#get_service_by_channel) | **GET** /channels/{type}/{channel}/services/{id} | 
*ServicesApi* | [**get_service_stream**](docs/ServicesApi.md#get_service_stream) | **GET** /services/{id}/stream | 
*ServicesApi* | [**get_service_stream_by_channel**](docs/ServicesApi.md#get_service_stream_by_channel) | **GET** /channels/{type}/{channel}/services/{id}/stream | 
*ServicesApi* | [**get_services**](docs/ServicesApi.md#get_services) | **GET** /services | 
*ServicesApi* | [**get_services_by_channel**](docs/ServicesApi.md#get_services_by_channel) | **GET** /channels/{type}/{channel}/services | 
*StatusApi* | [**get_status**](docs/StatusApi.md#get_status) | **GET** /status | Get Status
*StreamApi* | [**get_channel_stream**](docs/StreamApi.md#get_channel_stream) | **GET** /channels/{type}/{channel}/stream | 
*StreamApi* | [**get_events_stream**](docs/StreamApi.md#get_events_stream) | **GET** /events/stream | 
*StreamApi* | [**get_log_stream**](docs/StreamApi.md#get_log_stream) | **GET** /log/stream | 
*StreamApi* | [**get_program_stream**](docs/StreamApi.md#get_program_stream) | **GET** /programs/{id}/stream | 
*StreamApi* | [**get_service_stream**](docs/StreamApi.md#get_service_stream) | **GET** /services/{id}/stream | 
*StreamApi* | [**get_service_stream_by_channel**](docs/StreamApi.md#get_service_stream_by_channel) | **GET** /channels/{type}/{channel}/services/{id}/stream | 
*TunersApi* | [**get_tuner**](docs/TunersApi.md#get_tuner) | **GET** /tuners/{index} | 
*TunersApi* | [**get_tuner_process**](docs/TunersApi.md#get_tuner_process) | **GET** /tuners/{index}/process | Get Tuner Process Info
*TunersApi* | [**get_tuners**](docs/TunersApi.md#get_tuners) | **GET** /tuners | 
*TunersApi* | [**kill_tuner_process**](docs/TunersApi.md#kill_tuner_process) | **DELETE** /tuners/{index}/process | Kill Tuner Process
*VersionApi* | [**check_version**](docs/VersionApi.md#check_version) | **GET** /version | 
*VersionApi* | [**update_version**](docs/VersionApi.md#update_version) | **PUT** /version/update | 


## Documentation For Models

 - [Channel](docs/Channel.md)
 - [ChannelType](docs/ChannelType.md)
 - [ConfigChannelsItem](docs/ConfigChannelsItem.md)
 - [ConfigServer](docs/ConfigServer.md)
 - [ConfigTunersItem](docs/ConfigTunersItem.md)
 - [Error](docs/Error.md)
 - [ErrorOfOpenApi](docs/ErrorOfOpenApi.md)
 - [Event](docs/Event.md)
 - [EventResource](docs/EventResource.md)
 - [EventType](docs/EventType.md)
 - [Program](docs/Program.md)
 - [ProgramAudioSamplingRate](docs/ProgramAudioSamplingRate.md)
 - [ProgramAudios](docs/ProgramAudios.md)
 - [ProgramGenre](docs/ProgramGenre.md)
 - [ProgramSeries](docs/ProgramSeries.md)
 - [ProgramVideo](docs/ProgramVideo.md)
 - [ProgramVideoResolution](docs/ProgramVideoResolution.md)
 - [ProgramVideoType](docs/ProgramVideoType.md)
 - [RelatedItem](docs/RelatedItem.md)
 - [Service](docs/Service.md)
 - [Status](docs/Status.md)
 - [StatusEpg](docs/StatusEpg.md)
 - [StatusErrorCount](docs/StatusErrorCount.md)
 - [StatusProcess](docs/StatusProcess.md)
 - [StatusProcessMemoryUsage](docs/StatusProcessMemoryUsage.md)
 - [StatusStreamCount](docs/StatusStreamCount.md)
 - [StatusTimerAccuracy](docs/StatusTimerAccuracy.md)
 - [StatusTimerAccuracyM1](docs/StatusTimerAccuracyM1.md)
 - [TunerDevice](docs/TunerDevice.md)
 - [TunerProcess](docs/TunerProcess.md)
 - [TunerUser](docs/TunerUser.md)
 - [TunerUserStreamInfo](docs/TunerUserStreamInfo.md)
 - [TunerUserStreamSetting](docs/TunerUserStreamSetting.md)
 - [Version](docs/Version.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



