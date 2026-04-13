# Data Schema

The InduFlow PartHub API provides information on Bosch Rexroth products. How this information is structured is explained in this section.

---

## Product Data

The InduFlow PartHub API provides several endpoints to access the data of Bosch Rexroth Products. This product data includes, material identifier, properties, resources, descriptions etc. How the product data scheme is structured can be seen in the image below.

---

## File download

If the response is a byte array.

The file metadata will be returned in the response header.

```json
{
  "content-disposition": {
    "filename": "file.png"
  },
  "content-length": 10000,
  "content-type": "image/png"
}
```

### Headers

- `content-disposition` *(string)*: Contains information about the filename.  
- `content-length` *(long)*: The current page size, the page size could be changed by the api if the requested page size greater than maximum page size.  
- `content-type` *(string)*: The mime type of the content.  

---

## Pagination

If the response type contains a list of entries the api use pagination to divides the list into discrete pages.

The viewer can determine the page size and page number. He can add the values for this as query parameters to the request.

The maximum page size is specified by the server. If this is exceeded by the viewer, the maximum page size is automatically taken so that it cannot be exceeded.

The query parameters are optional. If there are no query parameters the default values are used.

### Query Parameters

- `PageNumber` *(int)*: 1 - n (default: 1)  
  The page number the viewer will be request.  

- `PageSize` *(int)*: 1 - maximum page size (default: maximum page size)  
  The page size determines the maximum number of entries that can be listed on the page. If there are fewer entries, fewer entries will be displayed.  

---

## Pagination Response Header

```json
{
  "x-pagination": {
    "TotalCount": 34,
    "PageSize": 5,
    "CurrentPage": 1,
    "TotalPages": 7,
    "HasNext": true,
    "HasPrevious": false
  }
}
```

### Fields

- `TotalCount` *(long)*: The total count of entries.  
- `PageSize` *(long)*: The current page size, the page size could be changed by the api if the requested page size greater than maximum page size.  
- `CurrentPage` *(long)*: The current page info.  
- `TotalPages` *(long)*: The total pages count with the given page size.  
- `HasNext` *(bool)*: Is 'true' if there are next pages otherwise 'false'.  
- `HasPrevious` *(bool)*: Is 'true' if there are prevoius pages otherwise 'false'.  
