# Integrate Search

Welcome to the InduFlow PartHub API Tour. In this section, you will learn how to integrate the InduFlow PartHub API to enable search functionality for Product IamNumbers and OemNumbers.  
You can also view an example of how search results might be displayed.

To implement the search and display the results, please follow these steps:

1. Query a Product Number  
2. Download Product Image  

---

## 1. Query a Product Number

To retrieve the data of a product you have to query for the specific product number. This can be achieved with the InduFlow PartHub API endpoint called `/api/v1/product/query`.

This endpoint allows you to search for products using either an IamNumber or an OemNumber, which can be found on the product's nameplate.

The product number has to be provided in the http request body. This http request body also has the option to add a machine type which the product number is correlating to.

The response contains the requested product data.

Now its your turn!

Down below is the swagger definition of the query endpoint. When you are logged in, you can use this endpoint to request the data of a specific product.

When you enable the example view, you can see an empty search bar. This could be the implementation in your application. Any product number that a user may type into the search bar will be used as a request parameter to query the product data.

Next, please query a product data, by inserting a product number into the request body in the swagger definition below. For an example you can try the product number `R909607140`.

For this example, you can leave the `MachineType` blank. After you have successfully entered a product number, you can execute the request by clicking the corresponding button.

After the execution, you can see the product data as a json in the response of the request. In the example view, you can see how this product data json could be displayed in a application.

### POST `/api/v1/product/query`

Queries a product by a number. The number could be a OemNumber or a IamNumber.

#### Parameters

No parameters

#### Request body

**application/json**

```json
{
  "Number": "0510545001",
  "MachineType": "<Designation of the machine for which the requested product is intended>"
}
```

#### Responses

- `200` OK
- `401` Unauthorized
- `403` Forbidden
- `404` Not Found
- `422` Unprocessable Content
- `429` Too Many Requests
- `500` Internal Server Error

---

## 2. Download Product Image

For optimal performance, the initial product data does not include the content of resource files such as images or documents but rather a resource id.

With the resource id those resources can be retrieved separately using the endpoint `/api/v1/resource/download/{id}`.

This endpoint returns the requested file along with its MIME type.

The resource id is provided in the query of the http request.

Please start with step one to search for a product. Once you have found a product, you can proceed to download its image.

### GET `/api/v1/resource/download/{id}`

Get the resource content

#### Parameters

- `id` *(string, path, required)*: the resource id

#### Responses

- `200` OK
- `401` Unauthorized
- `403` Forbidden
- `404` Not Found
- `422` Unprocessable Content
- `429` Too Many Requests
- `500` Internal Server Error
