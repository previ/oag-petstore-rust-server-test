# pet_api

All URIs are relative to *https://petstore3.swagger.io/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
**addPet**](pet_api.md#addPet) | **POST** /pet | Add a new pet to the store
**deletePet**](pet_api.md#deletePet) | **DELETE** /pet/{petId} | Deletes a pet
**findPetsByStatus**](pet_api.md#findPetsByStatus) | **GET** /pet/findByStatus | Finds Pets by status
**findPetsByTags**](pet_api.md#findPetsByTags) | **GET** /pet/findByTags | Finds Pets by tags
**getPetById**](pet_api.md#getPetById) | **GET** /pet/{petId} | Find pet by ID
**updatePet**](pet_api.md#updatePet) | **PUT** /pet | Update an existing pet
**updatePetWithForm**](pet_api.md#updatePetWithForm) | **POST** /pet/{petId} | Updates a pet in the store with form data


# **addPet**
> models::Pet addPet(ctx, pet)
Add a new pet to the store

Add a new pet to the store

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **pet** | [**Pet**](Pet.md)| Create a new pet in the store | 

### Return type

[**models::Pet**](Pet.md)

### Authorization

[petstore_auth](../README.md#petstore_auth)

### HTTP request headers

 - **Content-Type**: application/json, application/xml, application/x-www-form-urlencoded
 - **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deletePet**
> deletePet(ctx, pet_id, optional)
Deletes a pet

delete a pet

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **pet_id** | **i64**| Pet id to delete | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **pet_id** | **i64**| Pet id to delete | 
 **api_key** | **String**|  | 

### Return type

 (empty response body)

### Authorization

[petstore_auth](../README.md#petstore_auth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **findPetsByStatus**
> Vec<models::Pet> findPetsByStatus(ctx, optional)
Finds Pets by status

Multiple status values can be provided with comma separated strings

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **status** | **String**| Status values that need to be considered for filter | [default to "available".to_string()]

### Return type

[**Vec<models::Pet>**](Pet.md)

### Authorization

[petstore_auth](../README.md#petstore_auth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **findPetsByTags**
> Vec<models::Pet> findPetsByTags(ctx, optional)
Finds Pets by tags

Multiple tags can be provided with comma separated strings. Use tag1, tag2, tag3 for testing.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **tags** | [**String**](String.md)| Tags to filter by | 

### Return type

[**Vec<models::Pet>**](Pet.md)

### Authorization

[petstore_auth](../README.md#petstore_auth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getPetById**
> models::Pet getPetById(ctx, ctx, pet_id)
Find pet by ID

Returns a single pet

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **pet_id** | **i64**| ID of pet to return | 

### Return type

[**models::Pet**](Pet.md)

### Authorization

[api_key](../README.md#api_key), [petstore_auth](../README.md#petstore_auth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **updatePet**
> models::Pet updatePet(ctx, pet)
Update an existing pet

Update an existing pet by Id

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **pet** | [**Pet**](Pet.md)| Update an existent pet in the store | 

### Return type

[**models::Pet**](Pet.md)

### Authorization

[petstore_auth](../README.md#petstore_auth)

### HTTP request headers

 - **Content-Type**: application/json, application/xml, application/x-www-form-urlencoded
 - **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **updatePetWithForm**
> updatePetWithForm(ctx, pet_id, optional)
Updates a pet in the store with form data



### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **pet_id** | **i64**| ID of pet that needs to be updated | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **pet_id** | **i64**| ID of pet that needs to be updated | 
 **name** | **String**| Name of pet that needs to be updated | 
 **status** | **String**| Status of pet that needs to be updated | 

### Return type

 (empty response body)

### Authorization

[petstore_auth](../README.md#petstore_auth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

