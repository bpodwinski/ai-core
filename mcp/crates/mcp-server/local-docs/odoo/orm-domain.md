# Odoo ORM – Domain Syntax (v18)

A **domain** is a list of **criteria** (leaves) combined with **logical operators**.

## Leaf (criterion) format

```python
(field_name, operator, value)
```

Example: `('name', 'ilike', 'acme')`, `('age', '>=', 18)`, `('active', '=', True)`

---

## Comparison operators

| Operator | Meaning |
|----------|---------|
| `=`      | equal |
| `!=`     | not equal |
| `>`      | greater than |
| `>=`     | greater or equal |
| `<`      | less than |
| `<=`     | less or equal |
| `like`   | SQL `LIKE '%value%'` (case-sensitive) |
| `not like` | SQL `NOT LIKE '%value%'` |
| `ilike`  | case-insensitive `LIKE '%value%'` |
| `not ilike` | case-insensitive `NOT LIKE '%value%'` |
| `=like`  | exact pattern match, `%` and `_` are wildcards (case-sensitive) |
| `=ilike` | same but case-insensitive |
| `in`     | value is in the list: `('state', 'in', ['draft', 'confirm'])` |
| `not in` | value not in the list |
| `child_of` | record is a child (descendant) of the given id (requires `_parent_name`) |
| `parent_of` | record is a parent (ancestor) of the given id |
| `any`    | at least one related record satisfies a sub-domain |
| `not any` | no related record satisfies a sub-domain |

---

## Logical operators (prefix notation)

The default combination of multiple leaves is **AND**.

```python
# Implicit AND — all three conditions must match
[('is_company', '=', True), ('country_id.code', '=', 'FR'), ('active', '=', True)]

# Explicit AND operator '&' (same as above)
['&', ('is_company', '=', True), ('country_id.code', '=', 'FR')]

# OR operator '|'
['|', ('name', 'ilike', 'acme'), ('name', 'ilike', 'corp')]

# NOT operator '!'
['!', ('active', '=', True)]   # equivalent to ('active', '=', False) here

# Nesting: (A AND B) OR C
['|', '&', ('is_company', '=', True), ('country_id.code', '=', 'FR'), ('email', '!=', False)]
```

> **NOTE:** `&`, `|`, and `!` are **prefix** operators in Polish notation — they apply to the next N operands, not surrounding ones.

---

## Empty domain

```python
[]          # matches all records (no filter)
[('id', '=', False)]   # matches nothing (empty result)
```

---

## Relational field traversal

Use dot notation to traverse Many2one relationships:

```python
('partner_id.country_id.code', '=', 'DE')
('order_id.state', 'in', ['sale', 'done'])
```

---

## `any` / `not any` (v17+)

Filter by sub-domain on One2many / Many2many fields:

```python
# Partners that have at least one sale order in draft
('sale_order_ids', 'any', [('state', '=', 'draft')])

# Partners with no unpaid invoices
('invoice_ids', 'not any', [('payment_state', '!=', 'paid')])
```

---

## Common examples

```python
# All confirmed sale orders today
[('state', '=', 'sale'), ('date_order', '>=', fields.Date.today())]

# Active companies in France or Germany
['&', ('is_company', '=', True), '|', ('country_id.code', '=', 'FR'), ('country_id.code', '=', 'DE')]

# Products with stock < 10
[('type', '=', 'product'), ('qty_available', '<', 10)]

# Invoices overdue (not paid, date passed)
['&', ('move_type', '=', 'out_invoice'), ('payment_state', '!=', 'paid'), ('invoice_date_due', '<', fields.Date.today())]

# Users in a specific group
[('groups_id', 'in', [self.env.ref('base.group_user').id])]
```

---

## Domain helpers (odoo.osv.expression)

```python
from odoo.osv import expression

# Combine with AND
domain = expression.AND([domain_a, domain_b])

# Combine with OR
domain = expression.OR([domain_a, domain_b])

# Negate
domain = expression.NOT(domain_a)

# Normalize (add implicit & operators)
domain = expression.normalize_domain(raw_domain)
```
