# Crawling Products with InduFlow PartHub

This section explains how to efficiently retrieve product data from the InduFlow PartHub API for use in your own catalog or applications.  
You will learn how to access basic product information as well as complete product datasets, and understand best practices for integrating this data into your systems.

The InduFlow PartHub API provides flexible endpoints to support both lightweight and comprehensive data synchronization scenarios.

---

## Overview

Depending on your use case, you may want to:

- Cache essential product details (such as name and material number) for quick lookups and minimal storage requirements.  
- Synchronize full product data for advanced features, such as detailed product pages or integration in legacy systems.  

The following examples demonstrate how to use the API endpoints for both approaches.

---

## Retrieve Basic Product Information

To collect only the product name and material number, use the `/api/v1/product/information` endpoint.

This is recommended if you want to maintain a lightweight local cache without storing large amounts of data.

The response will include a list of products with their basic identifiers, which you can use for quick reference or to trigger more detailed queries as needed.

The InduFlow PartHub API is designed for high performance and can efficiently handle large volumes of requests. In many cases, maintaining a local cache is not required unless your application has specific offline or latency requirements.

### GET `/api/v1/product/information`

Get all product information with pagination.

#### Parameters

- `PageNumber` *(integer, query)*: The page number  
- `PageSize` *(integer, query)*: The page size  

#### Responses

- `200` OK
- `401` Unauthorized
- `403` Forbidden
- `429` Too Many Requests
- `500` Internal Server Error

---

## Retrieve Complete Product Data

If your application requires full product details (including descriptions, attributes, and related resources), use the `/api/v1/product` endpoint.

Please note that retrieving all available product data can result in large payloads and significant storage requirements.

For most scenarios, we recommend fetching only the data you need, and updating your local cache periodically to stay in sync with the latest product information.

Please not that too many requests can result in an `429 too many requests` status code.

### GET `/api/v1/product`

Get all products with pagination.

#### Parameters

- `PageNumber` *(integer, query)*: The page number (1-based)  
- `PageSize` *(integer, query)*: The page size  

#### Responses

- `200` OK  
  - Headers:
    - `x-pagination` *(object)*  
- `401` Unauthorized
- `403` Forbidden
- `429` Too Many Requests
- `500` Internal Server Error
