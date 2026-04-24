# Odoo External API — JSON-2 (v18+)

The JSON-2 API allows external programs to call any public Odoo model method over HTTP.

> **NOTE:** Access to the external API requires a **Custom** Odoo pricing plan (not available on One App Free or Standard).

---

## Endpoint

```
POST /json/2/<model>/<method>
```

---

## HTTP Request

### Headers

| Header | Required | Description |
|--------|----------|-------------|
| `Authorization` | **Yes** | `bearer <api_key>` |
| `Content-Type` | **Yes** | `application/json` (charset recommended) |
| `Host` | Yes (HTTP/1.1) | Hostname of the server |
| `X-Odoo-Database` | Optional | Database name — required on multi-DB servers |
| `User-Agent` | Recommended | Your application name |

### Body (JSON object)

| Field | Description |
|-------|-------------|
| `ids` | Array of record ids. Omit (or empty) for `@api.model` methods |
| `context` | Optional dict of extra values, e.g. `{"lang": "en_US"}` |
| `<param>` | Any named parameter of the called method |

### Example request

```http
POST /json/2/res.partner/search_read HTTP/1.1
Host: mycompany.example.com
X-Odoo-Database: mycompany
Authorization: bearer 6578616d706c65206a736f6e20617069206b6579
Content-Type: application/json; charset=utf-8
User-Agent: mysoftware/1.0

{
    "context": {"lang": "en_US"},
    "domain": [["name", "ilike", "%deco%"], ["is_company", "=", true]],
    "fields": ["name", "email"]
}
```

---

## HTTP Response

### Success — HTTP 200

Returns the JSON-serialized return value of the called method.

```http
HTTP/1.1 200 OK
Content-Type: application/json; charset=utf-8

[{"id": 25, "name": "Deco Addict", "email": "deco@example.com"}]
```

### Error — HTTP 4xx / 5xx

Returns a JSON error object:

```json
{
  "name": "werkzeug.exceptions.Unauthorized",
  "message": "Invalid apikey",
  "arguments": ["Invalid apikey", 401],
  "context": {},
  "debug": "Traceback (most recent call last):..."
}
```

| Field | Description |
|-------|-------------|
| `name` | Fully qualified Python exception class name |
| `message` | Exception message (usually `arguments[0]`) |
| `arguments` | All exception arguments |
| `context` | Context used by the request |
| `debug` | Full traceback |

---

## Authentication — API Keys

Create an API key: **Settings → Preferences → Account Security → New API Key**

- Requires a description and a duration (max 3 months)
- Key is shown **once only** — copy it immediately
- Use as: `Authorization: bearer <key>`
- Treat API keys with the same security as passwords

For automated integrations, create dedicated **bot users** with minimal permissions and disable password authentication.

---

## Transactions

Each JSON-2 call runs in its own SQL transaction (committed on success, rolled back on error).  
It is **not possible** to chain multiple calls in a single transaction.

Use single methods that perform all related operations atomically, e.g.:
- `search_read` instead of `search` + `read`
- Business methods like `action_confirm`, `action_post`

---

## Code Examples

### Python

```python
import requests

BASE_URL = "https://mycompany.example.com/json/2"
API_KEY = "..."  # from secure storage
headers = {
    "Authorization": f"bearer {API_KEY}",
    "X-Odoo-Database": "mycompany",
    "Content-Type": "application/json",
}

# Search partners
res = requests.post(
    f"{BASE_URL}/res.partner/search_read",
    headers=headers,
    json={
        "context": {"lang": "en_US"},
        "domain": [["name", "ilike", "%deco%"], ["is_company", "=", True]],
        "fields": ["name", "email", "phone"],
        "limit": 10,
    },
)
res.raise_for_status()
partners = res.json()

# Create a record
res = requests.post(
    f"{BASE_URL}/res.partner/create",
    headers=headers,
    json={
        "context": {"lang": "en_US"},
        "name": "New Company",
        "is_company": True,
        "email": "contact@newco.com",
    },
)
partner_id = res.json()  # returns the new record id (int)

# Write (update)
requests.post(
    f"{BASE_URL}/res.partner/write",
    headers=headers,
    json={"ids": [partner_id], "name": "Updated Name"},
)

# Unlink (delete)
requests.post(
    f"{BASE_URL}/res.partner/unlink",
    headers=headers,
    json={"ids": [partner_id]},
)
```

### JavaScript

```javascript
const BASE_URL = "https://mycompany.example.com/json/2";
const API_KEY = "...";
const headers = {
    "Content-Type": "application/json",
    "Authorization": "bearer " + API_KEY,
    "X-Odoo-Database": "mycompany",
};

const res = await fetch(BASE_URL + "/res.partner/search_read", {
    method: "POST",
    headers,
    body: JSON.stringify({
        context: { lang: "en_US" },
        domain: [["name", "ilike", "%deco%"], ["is_company", "=", true]],
        fields: ["name", "email"],
    }),
});
const partners = await res.json();
```

### cURL / Bash

```bash
API_KEY="..."
DATABASE="mycompany"
BASE_URL="https://$DATABASE.odoo.com/json/2"

curl -s "$BASE_URL/res.partner/search_read" \
    -X POST \
    --oauth2-bearer "$API_KEY" \
    -H "X-Odoo-Database: $DATABASE" \
    -H "Content-Type: application/json" \
    -d '{"domain": [["is_company","=",true]], "fields": ["name","email"], "limit": 5}'
```

---

## Common ORM methods available via JSON-2

All public ORM methods on any model are callable. The most used:

| Method | `ids` needed | Description |
|--------|-------------|-------------|
| `search` | No | Returns list of matching ids |
| `search_read` | No | Search + read in one call |
| `search_count` | No | Count matching records |
| `read` | **Yes** | Read field values for given ids |
| `create` | No | Create record(s); pass `name`, `email`, etc. as body params |
| `write` | **Yes** | Update records; pass field values as body params |
| `unlink` | **Yes** | Delete records |
| `fields_get` | No | Get field metadata for the model |

---

## Migration from XML-RPC / JSON-RPC

XML-RPC (`/xmlrpc/2`) and JSON-RPC (`/jsonrpc`) are **deprecated** and will be removed in **Odoo 22 (2028)**.

Key differences with JSON-2:

| XML-RPC / JSON-RPC | JSON-2 |
|---------------------|--------|
| `uid` + `password` per call | API key as Bearer token |
| Model + method in args | Model + method in URL path |
| Positional args | Named args in JSON body |
| `/xmlrpc/2/object` endpoint | `/json/2/<model>/<method>` |

### XML-RPC equivalent

```python
# Old XML-RPC
from xmlrpc.client import ServerProxy
models = ServerProxy(f"{url}/xmlrpc/2/object")
ids = models.execute_kw(db, uid, pwd, 'res.partner', 'search',
    [[['is_company', '=', True]]],
    {'limit': 10}
)
records = models.execute_kw(db, uid, pwd, 'res.partner', 'read',
    [ids], {'fields': ['name', 'email']}
)

# JSON-2 equivalent
requests.post(f"{base}/res.partner/search_read", headers=headers,
    json={"domain": [["is_company", "=", True]], "fields": ["name", "email"], "limit": 10}
)
```
