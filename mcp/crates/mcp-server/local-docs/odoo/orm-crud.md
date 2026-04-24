# Odoo ORM – Common Methods (v18)

## Environment

```python
self.env          # current Environment
self.env.user     # res.users record of current user
self.env.uid      # integer uid
self.env.company  # current company (res.company)
self.env.lang     # language code string
self.env.cr       # database cursor

self.env['res.partner']          # access model
self.env.ref('base.res_partner') # resolve XML id → record
self.with_context(key=value)     # return recordset with updated context
self.sudo()                      # return recordset without access checks
self.sudo(user)                  # return recordset as given user
```

---

## CRUD methods

### `search(domain, offset=0, limit=None, order=None)`

Search for records matching `domain`.

- `domain` – list of search criteria (see domain syntax)
- `offset` – number of records to skip
- `limit` – max number of records to return (`None` = no limit)
- `order` – string like `"name asc, date desc"`
- Returns: **recordset**

```python
partners = self.env['res.partner'].search([('is_company', '=', True)], limit=10)
```

### `search_count(domain)`

Return the number of records matching `domain`.

- Returns: **int**

```python
n = self.env['sale.order'].search_count([('state', '=', 'draft')])
```

### `search_read(domain=[], fields=None, offset=0, limit=None, order=None)`

Combined search + read in a single call (safe against concurrent deletes).

- `fields` – list of field names; `None` = all stored fields
- Returns: **list of dicts** `[{'id': 1, 'name': 'ACME', ...}, ...]`

```python
rows = self.env['res.partner'].search_read(
    [('country_id.code', '=', 'FR')],
    fields=['name', 'email', 'phone'],
    limit=50,
    order='name asc',
)
```

### `read(fields=None)`

Read field values of the current recordset.

- `fields` – list of field names
- Returns: **list of dicts**

```python
records = self.env['res.partner'].browse([1, 2, 3])
data = records.read(['name', 'email'])
```

### `browse(ids)`

Return a recordset for the given ids without hitting the database.

- `ids` – int, list of ints, or empty list
- Returns: **recordset**

```python
partner = self.env['res.partner'].browse(42)
partners = self.env['res.partner'].browse([1, 2, 3])
```

### `create(vals_list)`

Create one or multiple records.

- `vals_list` – dict OR list of dicts of field values
- Returns: **recordset** of created records

```python
partner = self.env['res.partner'].create({
    'name': 'ACME Corp',
    'is_company': True,
    'email': 'info@acme.com',
})

# Batch create (more efficient)
partners = self.env['res.partner'].create([
    {'name': 'Alice', 'email': 'alice@example.com'},
    {'name': 'Bob',   'email': 'bob@example.com'},
])
```

### `write(vals)`

Update all records in the recordset with the given field values.

- `vals` – dict of field values
- Returns: **True**

```python
orders.write({'state': 'cancel'})
partner.write({'name': 'New Name', 'phone': '+33 1 23 45 67 89'})
```

### `unlink()`

Delete all records in the recordset.

- Returns: **True**

```python
old_logs = self.env['mail.message'].search([('date', '<', cutoff)])
old_logs.unlink()
```

### `copy(default=None)`

Duplicate one record (single-record recordset only).

- `default` – dict of field overrides for the copy
- Returns: **recordset** (new record)

```python
new = product.copy({'name': product.name + ' (copy)', 'default_code': False})
```

### `exists()`

Filter the recordset to only records that still exist in the database.

- Returns: **recordset**

```python
still_valid = some_records.exists()
```

### `fields_get(attributes=None)`

Return metadata about the model's fields.

- `attributes` – list like `['string', 'type', 'required']`
- Returns: dict `{field_name: {attr: value, ...}}`

---

## Recordset operations

```python
# Iteration
for rec in records:
    print(rec.name)

# Length
len(records)

# Boolean (True if non-empty)
if records:
    ...

# Membership
rec in records

# Concatenation (keeps duplicates)
combined = records_a + records_b

# Difference
remaining = records_a - records_b

# Intersection
common = records_a & records_b

# Union (no duplicates)
union = records_a | records_b

# Filtering
adults = partners.filtered(lambda p: p.age >= 18)
adults = partners.filtered('is_company')          # by truthy field

# Mapping
names = partners.mapped('name')                   # list of values
countries = partners.mapped('country_id')         # recordset

# Sorting
sorted_p = partners.sorted('name')
sorted_p = partners.sorted(lambda p: p.name, reverse=True)

# Grouping
groups = partners.grouped('country_id')           # dict {country: partners}
```

---

## Many2one / relational field values in write/create

```python
# Many2one: pass int id
{'partner_id': 42}

# Many2many / One2many: use command tuples (Command enum or legacy integers)
from odoo.fields import Command

# Replace all (6, _, [ids])
{'tag_ids': [Command.set([1, 2, 3])]}

# Add link (4, id, _)
{'tag_ids': [Command.link(5)]}

# Remove link (3, id, _)
{'tag_ids': [Command.unlink(5)]}

# Clear all (5, _, _)
{'tag_ids': [Command.clear()]}

# Create and link (0, _, vals)
{'order_line': [Command.create({'product_id': 1, 'qty': 2})]}

# Update linked record (1, id, vals)
{'order_line': [Command.update(line.id, {'qty': 5})]}

# Delete and unlink (2, id, _)
{'order_line': [Command.delete(line.id)]}
```
