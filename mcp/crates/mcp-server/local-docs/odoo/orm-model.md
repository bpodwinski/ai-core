# Odoo ORM – Model Definition & Inheritance (v18)

## Model types

```python
from odoo import models, fields, api

class MyModel(models.Model):
    """Regular persistent model — creates a DB table."""
    _name = 'my.model'

class MyWizard(models.TransientModel):
    """Temporary model — auto-vacuumed, used for wizards/dialogs."""
    _name = 'my.wizard'

class MyMixin(models.AbstractModel):
    """Abstract model — no DB table, used as mixin base."""
    _name = 'my.mixin'
```

---

## Model attributes

```python
class SaleOrder(models.Model):
    _name        = 'sale.order'          # technical name (required for new models)
    _description = 'Sales Order'         # human-readable label
    _table       = 'sale_order'          # override PostgreSQL table name
    _order       = 'date_order desc, id' # default sort order for search()
    _rec_name    = 'name'                # field used as display_name
    _inherit     = 'mail.thread'         # single string or list — see inheritance
    _inherits    = {'res.partner': 'partner_id'}  # delegation inheritance
    _log_access  = True                  # create/write date+uid columns
    _auto        = True                  # create DB table
    _abstract    = False
    _transient   = False

    _sql_constraints = [
        ('unique_ref', 'UNIQUE(reference)', 'Reference must be unique'),
        ('positive_qty', 'CHECK(qty > 0)', 'Quantity must be positive'),
    ]
```

---

## Inheritance

### Classic inheritance (extend / override methods and fields)

```python
class AccountMove(models.Model):
    _inherit = 'account.move'  # extends existing model

    custom_field = fields.Char()

    def action_post(self):
        # call original
        res = super().action_post()
        # custom logic
        return res
```

### Multiple inheritance (mixins)

```python
class MyModel(models.Model):
    _name    = 'my.model'
    _inherit = ['mail.thread', 'mail.activity.mixin']
```

### Delegation inheritance (_inherits)

The model has a Many2one to the parent; parent fields are accessible directly:

```python
class Employee(models.Model):
    _name    = 'hr.employee'
    _inherits = {'res.partner': 'partner_id'}

    partner_id = fields.Many2one('res.partner', ondelete='restrict', required=True)
    # employee.name now reads/writes res.partner.name
```

### Copy inheritance (new model, separate table)

```python
class CustomInvoice(models.Model):
    _name    = 'custom.invoice'
    _inherit = 'account.move'  # copy structure only
```

---

## Decorators

```python
from odoo import api

@api.model
def create(self, vals):
    """Class-level method — self is the model class, not a recordset."""
    return super().create(vals)

@api.depends('field_a', 'related_id.field_b')
def _compute_something(self):
    """Trigger recomputation when listed fields change."""
    for rec in self:
        rec.result = rec.field_a + rec.related_id.field_b

@api.depends_context('lang', 'company')
def _compute_context_sensitive(self):
    ...

@api.onchange('partner_id')
def _onchange_partner(self):
    """Called in the UI when partner_id changes. Modifies self (the record draft)."""
    if self.partner_id:
        self.payment_term_id = self.partner_id.property_payment_term_id

@api.constrains('amount', 'currency_id')
def _check_amount(self):
    """Raised if the constraint is violated (on create/write)."""
    for rec in self:
        if rec.amount < 0:
            raise ValidationError("Amount cannot be negative")
```

---

## CRUD hooks (override pattern)

```python
from odoo.exceptions import ValidationError, UserError

@api.model_create_multi
def create(self, vals_list):
    for vals in vals_list:
        vals.setdefault('ref', self._generate_ref())
    return super().create(vals_list)

def write(self, vals):
    if 'state' in vals and vals['state'] == 'done':
        self._validate_before_done()
    return super().write(vals)

def unlink(self):
    if any(rec.state == 'done' for rec in self):
        raise UserError("Cannot delete confirmed records.")
    return super().unlink()
```

---

## Exceptions

```python
from odoo.exceptions import (
    UserError,        # business-level error shown to the user
    ValidationError,  # field validation failure
    AccessError,      # permission denied
    MissingError,     # record no longer exists
    RedirectWarning,  # error with a redirect button
)

raise UserError("Something went wrong.")
raise ValidationError("Field 'name' is required.")
```

---

## Common utilities

```python
# Formatting
self.env['ir.qweb']._render('my_module.template', values)

# Dates
from odoo import fields
today = fields.Date.today()           # date object
now   = fields.Datetime.now()         # datetime object (UTC)

# Translations
from odoo import _
msg = _("Hello %s", partner.name)

# Access rights check
self.check_access_rights('write')
self.check_access_rule('write')

# Flush / invalidate cache
self.env.flush_all()
self.env.invalidate_all()

# Execute raw SQL (use sparingly)
self.env.cr.execute("SELECT id FROM res_partner WHERE vat = %s", (vat,))
rows = self.env.cr.fetchall()
```

---

## Mail / Chatter integration

```python
class MyModel(models.Model):
    _name    = 'my.model'
    _inherit = ['mail.thread', 'mail.activity.mixin']

    state = fields.Selection([...], tracking=True)  # log changes to chatter

    def action_confirm(self):
        self.message_post(
            body="Order confirmed.",
            message_type='notification',
            subtype_xmlid='mail.mt_note',
        )
```
