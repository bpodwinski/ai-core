# General Information

This section provides an overview of the API, including versioning, authentication and error handling.

---

## Versioning guideline

The HTTP API is versioned with 'URI Versioning'. That means the version info is part of each endpoint.

For versioning we use only a MAJOR version. The version schema follows the this scheme:

The schema is: `../v{apiVersion}/..`

Example:
`induflow.boschrexroth.com/api/v{apiVersion}/product`

### No version update

A new release has no breaking changes in the api.

- New endpoints are added  
- Previous endpoint have no changes  
- External users has no need to change their running system  

### Version update

A new release has breaking changes in the api.

Breaking changes could be:
- A change to the data objects  
- The removal of a previous endpoint  

External users must change their running system, if they want to use the new functionality.

---

## How to use the API

The API request must include both api and the api version.

### Valid API request

The following is a example of a request using `https://induflow.boschrexroth.com` as the base path.

```bash
curl -X 'GET' \
  'https://induflow.boschrexroth.com/api/v1/product' \
  -H 'accept: application/json'
```

---

## Authentication

All API request requires authentication.

You must authenticate with a bearer access token as a header prefix. The access token can be requested from our open id connect authority.

Therefore you need:
- a client id  
- a client secret  

The client information can be requested from the developer team.

The token can be requested from the open id connect authority. For further information on how to retrieve a token, please refer to the send your first request section.

### OpenID Configuration

- OpenId Configuration: https://sandbox.auth.boschrexroth.com/auth/realms/dc5/.well-known/openid-configuration  
- Flow: clientCredentials  

With this information you can lookup the token_endpoint of our authority to request a bearer token.

### Example request with token

```bash
curl -X 'GET' \
  'https://induflow.boschrexroth.com/api/v1/product' \
  -H 'accept: application/json' \
  -H 'Authorization: Bearer <your bearer Token>'
```

### Authentication errors

- `401 Unauthorized`: authentication missing or invalid  
- `403 Forbidden`: authentication valid but insufficient permissions  

---

## Authentication example for Postman

Postman is a 3rd party program for using APIs.

### Configuration

- Type: OAuth 2.0  
- Add auth data to: Request Headers  
- Use Token Type: Access token  
- Header Prefix: Bearer  
- Token name: `<choose a client name>`  
- Grant Type: Client Credentials  
- Access Token URL: https://sandbox.auth.boschrexroth.com/auth/realms/dc5/protocol/openid-connect/token  
- Client ID: `<your client ID>`  
- Client Secret: `<your client secret>`  
- Scope: openid  
- Client Authentication: Send as Basic Auth header  

---

## Authentication example for Rest Client

Rest Client is a visual studio code extension for using APIs.

```http
###############################################
# Get access token request
###############################################

@clientId = <insert a valid client id>
@clientSecret = <insert a valid client secret>
@scope = openid
@accessTokenUrl = https://sandbox.auth.boschrexroth.com/auth/realms/dc5/protocol/openid-connect/token

# @name tokenrequest
POST {{accessTokenUrl}}
Content-Type: application/x-www-form-urlencoded

grant_type = client_credentials
& client_id={{clientId}}
& client_secret={{clientSecret}}
& scope={{scope}}

###
@accessToken = {{tokenrequest.response.body.access_token}}
###

###############################################
# Example requests
###############################################

### get product
GET {{https://induflow.boschrexroth.com}}/api/v1/product
Content-Type: application/json
Authorization: Bearer {{accessToken}}
```

---

## Status codes

The API is designed to return different status codes according to context and action.

### Request type

- `GET`: Access one or more resources and return the result as JSON  

### Return codes

- `200 OK`: request successful  
- `204 No Content`: request successful, no content  
- `400 Bad Request`: missing required attribute  
- `401 Unauthorized`: authentication required  
- `403 Forbidden`: request not allowed  
- `404 Not Found`: resource not found  
- `422 Unprocessable`: validation failed  
- `429 Too Many Requests`: rate limit exceeded  
- `500 Server Error`: server error  
- `503 Service Unavailable`: server overloaded  

---

## Data validation and error reporting

When working with the API you may encounter validation errors, in which case the API returns an HTTP 422 error.

Such errors appear in the following cases:

- A required attribute of the API request is missing  
- An attribute did not pass the validation  

### Example response

```json
HTTP/1.1 422 Unprocessable
Content-Type: application/json

{
  "title": "One or more validation errors occurred.",
  "status": 422,
  "errors": {
    "Id": [
      "The specified condition was not met for 'Id'."
    ]
  }
}
```

---

## Unknown server error

If something unexpected happens the API returns an HTTP 500 error.

```json
HTTP/1.1 500 Server Error
Content-Type: application/json

{
  "Status": "Internal Server Error",
  "Message": "An unexpected error occurred. Please contact the development team with the provided errorId if you are in need of further assistance.",
  "ErrorId": "579b30f7-b507-4ff1-a531-663ea691730c"
}
```
