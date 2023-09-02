# \RegionsApi

All URIs are relative to *https://secure.sakura.ad.jp/cloud/zone/is1a/api/cloud/1.1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_regions**](RegionsApi.md#list_regions) | **get** /region?{params} | List all Regions
[**show_region_by_id**](RegionsApi.md#show_region_by_id) | **get** /region/{regionId} | Info for a specific Region



## list_regions

> crate::models::RegionMultiResponse list_regions(params)
List all Regions

List all Regions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**params** | [**crate::models::RegionFindRequest**](.md) |  | [required] |

### Return type

[**crate::models::RegionMultiResponse**](RegionMultiResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_region_by_id

> crate::models::RegionSingleResponse show_region_by_id(region_id)
Info for a specific Region

Info for a specific Region

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region_id** | [**crate::models::Id**](.md) | The id of the region to retrieve | [required] |

### Return type

[**crate::models::RegionSingleResponse**](RegionSingleResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

