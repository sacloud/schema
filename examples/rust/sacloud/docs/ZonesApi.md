# \ZonesApi

All URIs are relative to *https://secure.sakura.ad.jp/cloud/zone/is1a/api/cloud/1.1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_zones**](ZonesApi.md#list_zones) | **get** /zone?{params} | List all Zones
[**show_zone_by_id**](ZonesApi.md#show_zone_by_id) | **get** /zone/{zoneId} | Info for a specific Zone



## list_zones

> crate::models::ZoneMultiResponse list_zones(params)
List all Zones

List all Zones

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**params** | [**crate::models::ZoneFindRequest**](.md) |  | [required] |

### Return type

[**crate::models::ZoneMultiResponse**](ZoneMultiResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_zone_by_id

> crate::models::ZoneSingleResponse show_zone_by_id(zone_id)
Info for a specific Zone

Info for a specific Zone

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone_id** | [**crate::models::Id**](.md) | The id of the zone to retrieve | [required] |

### Return type

[**crate::models::ZoneSingleResponse**](ZoneSingleResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

