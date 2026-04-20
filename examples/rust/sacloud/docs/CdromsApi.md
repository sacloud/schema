# \CdromsApi

All URIs are relative to *https://secure.sakura.ad.jp/cloud/zone/is1a/api/cloud/1.1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**close_ftp**](CdromsApi.md#close_ftp) | **delete** /cdrom/{cdromId}/ftp | Close FTP connection
[**create_cdrom**](CdromsApi.md#create_cdrom) | **post** /cdrom | Create a CD-ROM
[**delete_cdromby_id**](CdromsApi.md#delete_cdromby_id) | **delete** /cdrom/{cdromId} | Delete CD-ROM
[**list_cdroms**](CdromsApi.md#list_cdroms) | **get** /cdrom?{params} | List all CD-ROM
[**open_ftp**](CdromsApi.md#open_ftp) | **put** /cdrom/{cdromId}/ftp | Open FTP
[**show_cdromby_id**](CdromsApi.md#show_cdromby_id) | **get** /cdrom/{cdromId} | Info for a specific CD-ROM
[**update_cdromby_id**](CdromsApi.md#update_cdromby_id) | **put** /cdrom/{cdromId} | Update CD-ROM



## close_ftp

> crate::models::ActionResultEnvelope close_ftp(cdrom_id)
Close FTP connection

Close FTP connection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cdrom_id** | [**crate::models::Id**](.md) | The id of the cdrom to delete | [required] |

### Return type

[**crate::models::ActionResultEnvelope**](ActionResultEnvelope.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_cdrom

> crate::models::CdromCreateResponse create_cdrom(cdrom_create_request)
Create a CD-ROM

Create a CD-ROM

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cdrom_create_request** | [**CdromCreateRequest**](CdromCreateRequest.md) |  | [required] |

### Return type

[**crate::models::CdromCreateResponse**](CDROMCreateResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_cdromby_id

> crate::models::CdromSingleResponse delete_cdromby_id(cdrom_id)
Delete CD-ROM

DeleteCDROM

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cdrom_id** | [**crate::models::Id**](.md) | The id of the cdrom to delete | [required] |

### Return type

[**crate::models::CdromSingleResponse**](CDROMSingleResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_cdroms

> crate::models::CdromMultiResponse list_cdroms(params)
List all CD-ROM

List all CD-ROM

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**params** | [**crate::models::CdromFindRequest**](.md) |  | [required] |

### Return type

[**crate::models::CdromMultiResponse**](CDROMMultiResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## open_ftp

> crate::models::ActionResultEnvelope open_ftp(cdrom_id)
Open FTP

Open FTP connection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cdrom_id** | [**crate::models::Id**](.md) | The id of the cdrom to retrieve | [required] |

### Return type

[**crate::models::ActionResultEnvelope**](ActionResultEnvelope.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_cdromby_id

> crate::models::CdromSingleResponse show_cdromby_id(cdrom_id)
Info for a specific CD-ROM

Info for a specific CD-ROM

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cdrom_id** | [**crate::models::Id**](.md) | The id of the cdrom to retrieve | [required] |

### Return type

[**crate::models::CdromSingleResponse**](CDROMSingleResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_cdromby_id

> crate::models::CdromSingleResponse update_cdromby_id(cdrom_id, cdrom_update_request)
Update CD-ROM

UpdateCDROM

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cdrom_id** | [**crate::models::Id**](.md) | The id of the cdrom to retrieve | [required] |
**cdrom_update_request** | [**CdromUpdateRequest**](CdromUpdateRequest.md) |  | [required] |

### Return type

[**crate::models::CdromSingleResponse**](CDROMSingleResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

