# \IconsApi

All URIs are relative to *https://secure.sakura.ad.jp/cloud/zone/is1a/api/cloud/1.1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_icon**](IconsApi.md#create_icon) | **post** /icon | Create a icon
[**delete_icon_by_id**](IconsApi.md#delete_icon_by_id) | **delete** /icon/{iconId} | Delete Icon
[**list_icons**](IconsApi.md#list_icons) | **get** /icon?{params} | List all Icons
[**show_icon_by_id**](IconsApi.md#show_icon_by_id) | **get** /icon/{iconId} | Info for a specific Icon
[**update_icon_by_id**](IconsApi.md#update_icon_by_id) | **put** /icon/{iconId} | Update Icon



## create_icon

> crate::models::IconSingleResponse create_icon(icon_create_request)
Create a icon

Create a icon

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**icon_create_request** | [**IconCreateRequest**](IconCreateRequest.md) |  | [required] |

### Return type

[**crate::models::IconSingleResponse**](IconSingleResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_icon_by_id

> crate::models::IconSingleResponse delete_icon_by_id(icon_id)
Delete Icon

DeleteIcon

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**icon_id** | [**crate::models::Id**](.md) | The id of the icon to delete | [required] |

### Return type

[**crate::models::IconSingleResponse**](IconSingleResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_icons

> crate::models::IconMultiResponse list_icons(params)
List all Icons

List all Icons

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**params** | [**crate::models::IconFindRequest**](.md) |  | [required] |

### Return type

[**crate::models::IconMultiResponse**](IconMultiResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_icon_by_id

> crate::models::IconSingleResponse show_icon_by_id(icon_id)
Info for a specific Icon

Info for a specific Icon

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**icon_id** | [**crate::models::Id**](.md) | The id of the icon to retrieve | [required] |

### Return type

[**crate::models::IconSingleResponse**](IconSingleResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_icon_by_id

> crate::models::IconSingleResponse update_icon_by_id(icon_id, icon_update_request)
Update Icon

UpdateIcon

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**icon_id** | [**crate::models::Id**](.md) | The id of the icon to retrieve | [required] |
**icon_update_request** | [**IconUpdateRequest**](IconUpdateRequest.md) |  | [required] |

### Return type

[**crate::models::IconSingleResponse**](IconSingleResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

