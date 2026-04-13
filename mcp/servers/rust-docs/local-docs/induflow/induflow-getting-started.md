# Getting Started

## Getting Access

The only thing you really need to begin with exploring the InduFlow PartHub API is the access to the API.  
This is because, all API request requires authentication. Please contact us to obtain your client id and a client secret.

---

## Exploring InduFlow PartHub API

After you received your client credentials (client id and client secret) you can explore the InduFlow PartHub API via this documentation. For this, please login using the header. Then you can navigate to the OpenAPI Definition, where you can tryout the different API Endpoints. You can also proceed with the API Tour, which shows you how to use the API for different common use cases.

---

## Send your first requests

In this section you will learn how to send a valid request to the InduFlow PartHub API. The code sippets in the following sections will use cURL, which is preinstalled on the most operating systems, to send http requests. These can be copied directly to your console.

If you send a request without a valid token the InduFlow PartHub API you will receive an 401 Error. This is because you always need to be authenticated to access the InduFlow PartHub API. So before we start to send our requests to the API we need a valid token.

---

## Request a authorization token

To request a authorization token, you need to send a request with your client credentials to the keycloak authorization server from Bosch Rexroth. The Keycloak authorization server will then respond with your access token.

This is referred as the Client Credentials Flow, further information can be found in RFC 6749.

---

### Sample Request

Below you can find an example for requesting a token. This request sends the Client ID and Client Secret Base64 encoded and TLS secured to the Bosch Rexroth Keycloak Issuer.

With submitting the grant_type as client_credentials, the Keycloak Authority recognices the OAuth Flow and responds accordingly.

If you are executing the following commands behind a company proxy, you need to specify the correct proxy URL in the cURL command with the --proxy option. Otherwise, an error message like "could not resolve host" could appear.

Please don't forget to replace the clientId and clientSecret with your client credentials!

```bash
curl "https://sandbox.auth.boschrexroth.com/auth/realms/dc5/protocol/openid-connect/token" ^
  -u "clientId:clientSecret" ^
  -H "Content-Type: application/x-www-form-urlencoded;charset=UTF-8" ^
  --data-raw "grant_type=client_credentials"
```
