# Odoo ORM – Field Types Reference (v18)

All fields are defined as class attributes on a model. Common parameters shared by all field types:

## Common field parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `string` | str | field name | User-visible label |
| `required` | bool | `False` | Field is mandatory |
| `readonly` | bool | `False` | Field is read-only in the UI |
| `index` | bool/str | `False` | Create a DB index; `'btree'`, `'trigram'` |
| `default` | value/callable | — | Default value or `lambda self: ...` |
| `help` | str | `''` | Tooltip text |
| `groups` | str | — | XML ids of groups that can access this field |
| `copy` | bool | `True` | Whether field is copied on `record.copy()` |
| `store` | bool | `True` | Whether field is stored in DB (computed fields: `False` by default) |
| `compute` | str/callable | — | Method name or lambda for computed fields |
| `depends` | tuple/list | — | Field names this field depends on (invalidation) |
| `inverse` | str | — | Method name to write back a computed field |
| `search` | str | — | Method name for custom domain search on computed field |
| `related` | str | — | Dot-path like `'partner_id.country_id.name'` |
| `company_dependent` | bool | `False` | Store a different value per company |
| `translate` | bool | `False` | Field is translatable (Char, Text, Html) |
| `sanitize` | bool | `True` | Sanitize HTML (Html field) |
| `prefetch` | bool/str | `True` | Whether to prefetch with other records |
| `column_type` | tuple | — | Override PostgreSQL column type |

---

## Basic field types

### `fields.Boolean`
```python
active = fields.Boolean(default=True)
```

### `fields.Integer`
```python
sequence = fields.Integer(default=10)
qty      = fields.Integer(string="Quantity")
```

### `fields.Float`
```python
price = fields.Float(digits=(16, 2))
ratio = fields.Float(digits='Product Price')  # precision reference
```
- `digits` – `(total_digits, decimal_digits)` or a string referencing `decimal.precision`

### `fields.Monetary`
```python
amount_total  = fields.Monetary(currency_field='currency_id')
currency_id   = fields.Many2one('res.currency')
```
- `currency_field` – name of the Many2one currency field on the same model

### `fields.Char`
```python
name  = fields.Char(required=True, size=64)
email = fields.Char(string="Email Address")
```
- `size` – max length (no DB constraint, UI only)
- `trim` – strip whitespace (default `True`)

### `fields.Text`
```python
description = fields.Text(translate=True)
```

### `fields.Html`
```python
body_html = fields.Html(sanitize=True, sanitize_style=True)
```
- `sanitize` – clean dangerous HTML tags/attrs (default `True`)
- `sanitize_style` – also sanitize CSS (default `False`)

### `fields.Date`
```python
from odoo import fields

date_order = fields.Date(default=fields.Date.today)
date_start = fields.Date()

# Utilities
fields.Date.today()                          # current date as date object
fields.Date.context_today(record)            # today in user's timezone
fields.Date.to_string(date_obj)              # date → '2024-01-15'
fields.Date.from_string('2024-01-15')        # string → date
```

### `fields.Datetime`
```python
write_date = fields.Datetime(readonly=True)
scheduled  = fields.Datetime(default=fields.Datetime.now)

# Utilities
fields.Datetime.now()                        # current UTC datetime
fields.Datetime.to_string(dt_obj)
fields.Datetime.from_string('2024-01-15 10:00:00')
fields.Datetime.context_timestamp(record, dt_utc)  # convert UTC → user tz
```

### `fields.Binary`
```python
document = fields.Binary(attachment=True)
```
- `attachment` – store as `ir.attachment` instead of inline in the DB column

### `fields.Image`
```python
image_1920 = fields.Image(max_width=1920, max_height=1920)
image_128  = fields.Image(related='image_1920', max_width=128, max_height=128, store=True)
```

### `fields.Selection`
```python
state = fields.Selection([
    ('draft',   'Draft'),
    ('confirm', 'Confirmed'),
    ('done',    'Done'),
    ('cancel',  'Cancelled'),
], default='draft', required=True)

# Dynamic selection (method or list reference)
type = fields.Selection(selection='_get_types')
```
- `selection_add` – extend selection in inherited models: `[('new_val', 'New')]`
- `ondelete` – dict of `{value: action}` when record is deleted; action = `'set null'`, `'restrict'`, `'cascade'`

### `fields.Json`
```python
metadata = fields.Json()
```
Stores a dict/list as JSONB. Not searchable with standard domains; use SQL for complex queries.

---

## Relational field types

### `fields.Many2one`
```python
partner_id  = fields.Many2one('res.partner', ondelete='restrict', required=True)
company_id  = fields.Many2one('res.company', default=lambda self: self.env.company)
```

Parameters:
- `comodel_name` – technical name of the target model (first positional arg)
- `ondelete` – `'set null'` (default), `'restrict'`, `'cascade'`
- `domain` – static or dynamic domain to restrict selectable records
- `context` – dict passed to form view when opening the linked record
- `delegate` – if `True`, field acts as an inherited model (_inherits)

### `fields.One2many`
```python
order_line = fields.One2many('sale.order.line', 'order_id', string="Order Lines")
```

Parameters:
- `comodel_name` – related model
- `inverse_name` – field name on the related model pointing back (Many2one)
- `domain` / `context` / `limit` / `order`

### `fields.Many2many`
```python
tag_ids = fields.Many2many('res.partner.category', string="Tags")

# With explicit relation table and column names
group_ids = fields.Many2many(
    'res.groups',
    'res_partner_res_groups_rel',
    'partner_id',
    'group_id',
    string="Groups",
)
```

Parameters:
- `comodel_name`
- `relation` – join table name (auto-generated if omitted)
- `column1` – column for this model in join table
- `column2` – column for `comodel_name` in join table

---

## Computed fields

```python
class SaleOrder(models.Model):
    _name = 'sale.order'

    amount_untaxed = fields.Float(compute='_compute_amount', store=True)
    amount_total   = fields.Float(compute='_compute_amount', store=True)

    @api.depends('order_line.price_subtotal', 'order_line.price_tax')
    def _compute_amount(self):
        for order in self:
            order.amount_untaxed = sum(order.order_line.mapped('price_subtotal'))
            order.amount_total   = order.amount_untaxed + sum(order.order_line.mapped('price_tax'))
```

- `store=True` – persisted in DB, triggers recomputation on dependency change
- `store=False` (default) – computed on the fly, not searchable without a `search=` method
- `@api.depends(...)` – list of field paths; use `.` for traversal

### Inverse (writeable computed field)
```python
    full_name = fields.Char(compute='_compute_full_name', inverse='_inverse_full_name')

    @api.depends('first_name', 'last_name')
    def _compute_full_name(self):
        for rec in self:
            rec.full_name = f"{rec.first_name} {rec.last_name}"

    def _inverse_full_name(self):
        for rec in self:
            parts = rec.full_name.split(' ', 1)
            rec.first_name = parts[0]
            rec.last_name  = parts[1] if len(parts) > 1 else ''
```

---

## Related fields

Shortcut to a field accessible through a chain of Many2one:

```python
country_name = fields.Char(related='partner_id.country_id.name', store=True, readonly=True)
```

- `store=True` to persist and allow searching; automatically invalidated when the chain changes
- Inherits `string`, `readonly`, `required` from the terminal field by default

---

## Automatic fields (always present)

| Field | Type | Description |
|-------|------|-------------|
| `id` | Integer | Record primary key |
| `create_date` | Datetime | Creation timestamp |
| `create_uid` | Many2one(res.users) | Creator |
| `write_date` | Datetime | Last modification timestamp |
| `write_uid` | Many2one(res.users) | Last editor |
| `display_name` | Char (computed) | `_rec_name` field value |
| `active` | Boolean | If defined on model, False = archived record |

---

## Reference field

```python
ref = fields.Reference(selection=[('res.partner', 'Partner'), ('product.product', 'Product')])
# Stored as 'res.partner,42' string
```

---

## `@api.depends_context`

Recompute when context keys change (without being stored):

```python
price_with_tax = fields.Float(compute='_compute_price')

@api.depends_context('company')
@api.depends('price')
def _compute_price(self):
    ...
```
