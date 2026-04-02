# InduFlow PartHub API Reference

Version: 1.6.1
Base URLs:
- Production: `https://induflow.boschrexroth.com`
- Demo: `https://induflow-demo.boschrexroth.com`

Authentication: OAuth 2.0 Client Credentials flow
Token URL: `https://sandbox.auth.boschrexroth.com/auth/realms/dc5/protocol/openid-connect/token`

---

## Catalog

### GET /api/v1/catalog
Get all available product catalogs with pagination.
- Query: `PageNumber` (int), `PageSize` (int)
- Returns: `ProductCatalog[]` + `x-pagination` header
- Scope: `induflow-catalog-read`

### GET /api/v1/catalog/{id}
Get a product catalog by id.
- Path: `id` (string, required)
- Returns: `ProductCatalog`
- Scope: `induflow-catalog-read`

### GET /api/v1/catalog/{catalogId}/{treeNodeId}
Get the (sub)tree structure for a given catalog and tree node.
- Path: `catalogId` (string), `treeNodeId` (string)
- Query: `level` (int, default: 0 = full tree)
- Returns: `ProductCatalogTreeNode`
- Scope: `induflow-catalog-read`

### GET /api/v1/catalog/{catalogId}/{treeNodeId}/products
Get related products by a given catalog and tree node with pagination.
- Path: `catalogId` (string), `treeNodeId` (string)
- Query: `PageNumber` (int), `PageSize` (int)
- Returns: `Product[]`
- Scope: `induflow-catalog-read`

---

## Product

### GET /api/v1/product
Get all products with pagination.
- Query: `PageNumber` (int), `PageSize` (int)
- Returns: `Product[]` + `x-pagination` header
- Scope: `induflow-products-reader` or `induflow-products-contributor`

### GET /api/v1/product/{id}
Get a product by id.
- Path: `id` (string, required)
- Returns: `Product`
- Scope: `induflow-products-reader` or `induflow-products-contributor`

### POST /api/v1/product/query
Query a product by number (OemNumber or IamNumber).
- Body: `QueryByNumber`
- Returns: `Product`
- Scope: `induflow-products-reader` or `induflow-products-contributor`

### GET /api/v1/product/information
Get lightweight product information (deprecated, for caching/indexing).
- Query: `PageNumber` (int), `PageSize` (int)
- Returns: `ShortProductInformation[]` + `x-pagination` header
- Scope: `induflow-products-reader` or `induflow-products-contributor`

### GET /api/v1/product/{id}/deliverytime
Get delivery time for a specific product.
- Path: `id` (string, required)
- Returns: `ProductDeliveryTime[]`
- Scope: `induflow-products-reader` or `induflow-products-contributor`

---

## Resource

### GET /api/v1/resource/download/{id}
Download a product resource (image, document).
- Path: `id` (string, required — the ResourceId from a product)
- Returns: binary file with `Content-Type` and `Content-Disposition` headers
- Scope: `induflow-products-reader` or `induflow-products-contributor`

---

## Price Simulation

### POST /api/v1/pricesimulation
Simulate price for a list of products.
- Body: `AddPriceSimulation`
- Returns: `PriceSimulation` (201 Created)
- Scope: `induflow-pricing-reader`

### GET /api/v1/pricesimulation/{id}
Get a price simulation by ID (poll until state is `Finished`).
- Path: `id` (string, required)
- Returns: `PriceSimulation`
- Scope: `induflow-pricing-reader`

---

## Metrics

### POST /api/v1/metric/purchaseintention
Submit a purchase intention metric.
- Body: `AddPurchaseIntentionMetric`
- Returns: `Metric` (200 OK)
- Scope: `induflow-products-contributor`

---

## Common Response Codes

| Code | Description |
|------|-------------|
| 200  | OK |
| 201  | Created |
| 400  | Bad Request |
| 401  | Unauthorized (invalid/missing token) |
| 403  | Forbidden (insufficient scope) |
| 404  | Not Found |
| 422  | Unprocessable Content (validation error) |
| 429  | Too Many Requests (rate limited) |
| 500  | Internal Server Error |
| 503  | Service Unavailable |

---

## Pagination

Paginated endpoints accept `PageNumber` (1-based) and `PageSize` query parameters.
Response includes `x-pagination` header:

```json
{
  "TotalCount": 150,
  "PageSize": 25,
  "CurrentPage": 1,
  "TotalPages": 6,
  "HasNext": true,
  "HasPrevious": false
}
```

---

## Data Models

### Product
The main product entity with full details.

| Field | Type | Description |
|-------|------|-------------|
| Id | string | Product id |
| IamNumber | string | IAM number |
| BnrShortText | MultiLocaleText[] | BNR short text (multi-language) |
| MaterialShortText | MultiLocaleText[] | Material short text (multi-language) |
| ShortDescription | MultiLocaleText[] | Short description (multi-language) |
| Description | MultiLocaleText[] | Full description (multi-language) |
| Features | MultiLocaleTextArray[] | Features list (multi-language) |
| Image | DownloadableFileResource? | Main product image |
| AdditionalImages | DownloadableFileResource[] | Additional images |
| Properties | Property[] | Product properties (deprecated) |
| MultiLocaleProperties | MultiLocalePropertyArray[] | Properties per language |
| Resources | FileResource[] | Documents and files |
| ServiceData | MultiLocalePropertyArray[] | Repair service info |
| SparePartInformation | SparePartInformation? | Spare parts data |
| SubItems | SubItem[] | Firmware/sub-item info |
| Accessories | Accessory[] | Related accessories |
| CatalogReferences | CatalogReferences? | Catalog tree references |

### ShortProductInformation (deprecated)
Lightweight version for caching.

| Field | Type | Description |
|-------|------|-------------|
| IamNumber | string | IAM number |
| MaterialShortText | string | Short text |

### ProductCatalog

| Field | Type | Description |
|-------|------|-------------|
| Id | string | Catalog id |
| RootTreeNodeId | string | Root tree node id |
| MaxDepthLevel | int | Max tree depth |
| Name | MultiLocaleText[] | Catalog name (multi-language) |

### ProductCatalogTreeNode

| Field | Type | Description |
|-------|------|-------------|
| Id | string | Tree node id |
| CatalogId | string | Parent catalog id |
| Data | CatalogTreeNodeData? | Node data (Name, Description, ImageResourceId, ImageThumbnailResourceId) |
| TreeStructure | TreeStructure | Children, AncestorId[], MaxDepthLevel, HasChildren |

### ProductDeliveryTime

| Field | Type | Description |
|-------|------|-------------|
| Current | int? | Current delivery time in working days |
| MaxQuantity | double? | Max order quantity for guaranteed delivery |
| Fallback | int? | Fallback delivery time if qty > MaxQuantity |
| MaxLength | double? | Maximum length |
| Region | string? | Region of plant |

### QueryByNumber

```json
{
  "Number": "0510545001",
  "MachineType": "<optional machine designation>"
}
```

### AddPriceSimulation

```json
{
  "Currency": "EUR",
  "ProductIds": ["<24-char hex id>", "..."]
}
```
- Currency: ISO 4217 (3 letters)
- ProductIds: 1 to 25 items, each matching `^[0-9a-fA-F]{24}$`

### PriceSimulation

| Field | Type | Description |
|-------|------|-------------|
| Id | string | Simulation id |
| Request | AddPriceSimulation | Original request |
| Metadata | PriceSimulationMetaData | CreatedAt, QueuedAt, FinishedAt, DataAvailableUntil (dates) |
| State | enum | `Queued`, `InProgress`, `Finished`, `Error` |
| RecommendedPollingInterval | int | Polling interval in milliseconds |
| PriceInformation | PriceInformation[] | Results per product |

### PriceInformation

| Field | Type | Description |
|-------|------|-------------|
| ProductId | string | Product id (24-char hex) |
| NetPrice | Price | Net price (Amount, Currency, ResultCode) |
| GrossPrice | Price | Gross price (Amount, Currency, ResultCode) |

### Price

| Field | Type | Description |
|-------|------|-------------|
| Amount | double? | Price amount |
| Currency | string? | ISO 4217 currency |
| ResultCode | enum | `Success`, `Unavailable`, `Forbidden` |

### AddPurchaseIntentionMetric

```json
{
  "IamNumber": "0510545001",
  "CustomerInformation": {
    "Identifier": "<anonymized guid>",
    "Origin": "de",
    "Language": "DE"
  }
}
```

### Metric

| Field | Type | Description |
|-------|------|-------------|
| Id | string? | Metric id |
| IamNumber | string | IAM number |
| Data | PurchaseIntentionMetric | Contains CustomerInformation |
| CreatedAt | datetime | Creation time (UTC) |

---

## Shared Types

### MultiLocaleText
```json
{ "Language": "en", "Text": "Hydraulic pump" }
```

### MultiLocaleTextArray
```json
{ "Language": "en", "Texts": ["Feature 1", "Feature 2"] }
```

### DownloadableFileResource
```json
{ "ResourceId": "uuid", "MimeType": "image/jpeg", "FileName": "product.jpg" }
```

### Property
```json
{
  "Name": "Weight",
  "PropertyId": "unique-within-product",
  "Type": "physical",
  "Value": { "kind": "physical", "Value": 5.0, "Unit": "kilogram", "Quantity": "Mass" }
}
```

Property value types (discriminated by `kind`):
- `text`: `{ "kind": "text", "Value": "some text" }`
- `number`: `{ "kind": "number", "Value": 5.0 }`
- `bool`: `{ "kind": "bool", "Value": true }`
- `physical`: `{ "kind": "physical", "Value": 5.0, "Unit": "centimeter", "Quantity": "Length" }`

### FileResource
```json
{
  "ResourceType": "DataSheet",
  "LocalizedResources": [
    { "Language": "en", "Title": "Data Sheet", "FileResource": { "ResourceId": "uuid", "MimeType": "application/pdf", "FileName": "ds.pdf" } }
  ]
}
```

### SparePartInformation
```json
{
  "SpareParts": [
    { "IamNumber": "...", "Pos": "1", "Amount": "2", "BnrShortText": [...], "MaterialShortText": [...], "SparePartInformation": null }
  ],
  "InstallationDrawing": { "ResourceId": "uuid", "MimeType": "image/png", "FileName": "drawing.png" }
}
```

### SubItem
```json
{ "ProductId": "...", "Description": [MultiLocaleText], "Type": [MultiLocaleText] }
```

### Accessory
```json
{ "IamNumber": "..." }
```

### CatalogReferences
```json
{
  "ProductHierarchy": { "Value": "full hierarchy string", "ReferenceUrl": "https://..." },
  "ProductCatalogInfo": [{ "CatalogId": "...", "TreeNodeId": "..." }]
}
```

---

## Authentication

OAuth 2.0 Client Credentials flow:

```
POST https://sandbox.auth.boschrexroth.com/auth/realms/dc5/protocol/openid-connect/token
Content-Type: application/x-www-form-urlencoded

grant_type=client_credentials&client_id=YOUR_CLIENT_ID&client_secret=YOUR_CLIENT_SECRET&scope=openid
```

Use the returned `access_token` as Bearer token in all API requests.
