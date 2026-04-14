# Tailwind CSS v4 — Référence complète des classes

Index de 4811 classes Tailwind CSS (base, sans variants) avec leur CSS équivalent.

> **Tailwind v4** : les valeurs d'espacement utilisent `calc(var(--spacing) * n)` où `--spacing = 0.25rem` par défaut.
> Les couleurs utilisent des variables CSS : `var(--color-red-500)` etc. (valeurs oklch définies dans la base).
> Utiliser `search_docs` avec un nom de classe (`p-4`), une propriété CSS (`padding`), ou une valeur (`1rem`).

## Layout — Display

| Classe | CSS |
|--------|-----|
| `block` | `display: block` |
| `inline-block` | `display: inline-block` |
| `inline` | `display: inline` |
| `flex` | `display: flex` |
| `inline-flex` | `display: inline-flex` |
| `grid` | `display: grid` |
| `inline-grid` | `display: inline-grid` |
| `table` | `display: table` |
| `inline-table` | `display: inline-table` |
| `table-caption` | `display: table-caption` |
| `table-cell` | `display: table-cell` |
| `table-column` | `display: table-column` |
| `table-column-group` | `display: table-column-group` |
| `table-footer-group` | `display: table-footer-group` |
| `table-header-group` | `display: table-header-group` |
| `table-row-group` | `display: table-row-group` |
| `table-row` | `display: table-row` |
| `flow-root` | `display: flow-root` |
| `contents` | `display: contents` |
| `list-item` | `display: list-item` |
| `hidden` | `display: none` |

## Layout — Position

| Classe | CSS |
|--------|-----|
| `static` | `position: static` |
| `fixed` | `position: fixed` |
| `absolute` | `position: absolute` |
| `relative` | `position: relative` |
| `sticky` | `position: sticky` |
| `inset-0` | `inset: 0px` |
| `inset-px` | `inset: 1px` |
| `inset-0.5` | `inset: calc(var(--spacing) * 0.5)` |
| `inset-1` | `inset: calc(var(--spacing) * 1)` |
| `inset-1.5` | `inset: calc(var(--spacing) * 1.5)` |
| `inset-2` | `inset: calc(var(--spacing) * 2)` |
| `inset-2.5` | `inset: calc(var(--spacing) * 2.5)` |
| `inset-3` | `inset: calc(var(--spacing) * 3)` |
| `inset-3.5` | `inset: calc(var(--spacing) * 3.5)` |
| `inset-4` | `inset: calc(var(--spacing) * 4)` |
| `inset-5` | `inset: calc(var(--spacing) * 5)` |
| `inset-6` | `inset: calc(var(--spacing) * 6)` |
| `inset-7` | `inset: calc(var(--spacing) * 7)` |
| `inset-8` | `inset: calc(var(--spacing) * 8)` |
| `inset-9` | `inset: calc(var(--spacing) * 9)` |
| `inset-10` | `inset: calc(var(--spacing) * 10)` |
| `inset-11` | `inset: calc(var(--spacing) * 11)` |
| `inset-12` | `inset: calc(var(--spacing) * 12)` |
| `inset-14` | `inset: calc(var(--spacing) * 14)` |
| `inset-16` | `inset: calc(var(--spacing) * 16)` |
| `inset-20` | `inset: calc(var(--spacing) * 20)` |
| `inset-24` | `inset: calc(var(--spacing) * 24)` |
| `inset-28` | `inset: calc(var(--spacing) * 28)` |
| `inset-32` | `inset: calc(var(--spacing) * 32)` |
| `inset-36` | `inset: calc(var(--spacing) * 36)` |
| `inset-40` | `inset: calc(var(--spacing) * 40)` |
| `inset-44` | `inset: calc(var(--spacing) * 44)` |
| `inset-48` | `inset: calc(var(--spacing) * 48)` |
| `inset-52` | `inset: calc(var(--spacing) * 52)` |
| `inset-56` | `inset: calc(var(--spacing) * 56)` |
| `inset-60` | `inset: calc(var(--spacing) * 60)` |
| `inset-64` | `inset: calc(var(--spacing) * 64)` |
| `inset-72` | `inset: calc(var(--spacing) * 72)` |
| `inset-80` | `inset: calc(var(--spacing) * 80)` |
| `inset-96` | `inset: calc(var(--spacing) * 96)` |
| `inset-x-0` | `left: 0px; right: 0px` |
| `inset-x-px` | `left: 1px; right: 1px` |
| `inset-x-0.5` | `left: calc(var(--spacing) * 0.5); right: calc(var(--spacing) * 0.5)` |
| `inset-x-1` | `left: calc(var(--spacing) * 1); right: calc(var(--spacing) * 1)` |
| `inset-x-1.5` | `left: calc(var(--spacing) * 1.5); right: calc(var(--spacing) * 1.5)` |
| `inset-x-2` | `left: calc(var(--spacing) * 2); right: calc(var(--spacing) * 2)` |
| `inset-x-2.5` | `left: calc(var(--spacing) * 2.5); right: calc(var(--spacing) * 2.5)` |
| `inset-x-3` | `left: calc(var(--spacing) * 3); right: calc(var(--spacing) * 3)` |
| `inset-x-3.5` | `left: calc(var(--spacing) * 3.5); right: calc(var(--spacing) * 3.5)` |
| `inset-x-4` | `left: calc(var(--spacing) * 4); right: calc(var(--spacing) * 4)` |
| `inset-x-5` | `left: calc(var(--spacing) * 5); right: calc(var(--spacing) * 5)` |
| `inset-x-6` | `left: calc(var(--spacing) * 6); right: calc(var(--spacing) * 6)` |
| `inset-x-7` | `left: calc(var(--spacing) * 7); right: calc(var(--spacing) * 7)` |
| `inset-x-8` | `left: calc(var(--spacing) * 8); right: calc(var(--spacing) * 8)` |
| `inset-x-9` | `left: calc(var(--spacing) * 9); right: calc(var(--spacing) * 9)` |
| `inset-x-10` | `left: calc(var(--spacing) * 10); right: calc(var(--spacing) * 10)` |
| `inset-x-11` | `left: calc(var(--spacing) * 11); right: calc(var(--spacing) * 11)` |
| `inset-x-12` | `left: calc(var(--spacing) * 12); right: calc(var(--spacing) * 12)` |
| `inset-x-14` | `left: calc(var(--spacing) * 14); right: calc(var(--spacing) * 14)` |
| `inset-x-16` | `left: calc(var(--spacing) * 16); right: calc(var(--spacing) * 16)` |
| `inset-x-20` | `left: calc(var(--spacing) * 20); right: calc(var(--spacing) * 20)` |
| `inset-x-24` | `left: calc(var(--spacing) * 24); right: calc(var(--spacing) * 24)` |
| `inset-x-28` | `left: calc(var(--spacing) * 28); right: calc(var(--spacing) * 28)` |
| `inset-x-32` | `left: calc(var(--spacing) * 32); right: calc(var(--spacing) * 32)` |
| `inset-x-36` | `left: calc(var(--spacing) * 36); right: calc(var(--spacing) * 36)` |
| `inset-x-40` | `left: calc(var(--spacing) * 40); right: calc(var(--spacing) * 40)` |
| `inset-x-44` | `left: calc(var(--spacing) * 44); right: calc(var(--spacing) * 44)` |
| `inset-x-48` | `left: calc(var(--spacing) * 48); right: calc(var(--spacing) * 48)` |
| `inset-x-52` | `left: calc(var(--spacing) * 52); right: calc(var(--spacing) * 52)` |
| `inset-x-56` | `left: calc(var(--spacing) * 56); right: calc(var(--spacing) * 56)` |
| `inset-x-60` | `left: calc(var(--spacing) * 60); right: calc(var(--spacing) * 60)` |
| `inset-x-64` | `left: calc(var(--spacing) * 64); right: calc(var(--spacing) * 64)` |
| `inset-x-72` | `left: calc(var(--spacing) * 72); right: calc(var(--spacing) * 72)` |
| `inset-x-80` | `left: calc(var(--spacing) * 80); right: calc(var(--spacing) * 80)` |
| `inset-x-96` | `left: calc(var(--spacing) * 96); right: calc(var(--spacing) * 96)` |
| `inset-y-0` | `top: 0px; bottom: 0px` |
| `inset-y-px` | `top: 1px; bottom: 1px` |
| `inset-y-0.5` | `top: calc(var(--spacing) * 0.5); bottom: calc(var(--spacing) * 0.5)` |
| `inset-y-1` | `top: calc(var(--spacing) * 1); bottom: calc(var(--spacing) * 1)` |
| `inset-y-1.5` | `top: calc(var(--spacing) * 1.5); bottom: calc(var(--spacing) * 1.5)` |
| `inset-y-2` | `top: calc(var(--spacing) * 2); bottom: calc(var(--spacing) * 2)` |
| `inset-y-2.5` | `top: calc(var(--spacing) * 2.5); bottom: calc(var(--spacing) * 2.5)` |
| `inset-y-3` | `top: calc(var(--spacing) * 3); bottom: calc(var(--spacing) * 3)` |
| `inset-y-3.5` | `top: calc(var(--spacing) * 3.5); bottom: calc(var(--spacing) * 3.5)` |
| `inset-y-4` | `top: calc(var(--spacing) * 4); bottom: calc(var(--spacing) * 4)` |
| `inset-y-5` | `top: calc(var(--spacing) * 5); bottom: calc(var(--spacing) * 5)` |
| `inset-y-6` | `top: calc(var(--spacing) * 6); bottom: calc(var(--spacing) * 6)` |
| `inset-y-7` | `top: calc(var(--spacing) * 7); bottom: calc(var(--spacing) * 7)` |
| `inset-y-8` | `top: calc(var(--spacing) * 8); bottom: calc(var(--spacing) * 8)` |
| `inset-y-9` | `top: calc(var(--spacing) * 9); bottom: calc(var(--spacing) * 9)` |
| `inset-y-10` | `top: calc(var(--spacing) * 10); bottom: calc(var(--spacing) * 10)` |
| `inset-y-11` | `top: calc(var(--spacing) * 11); bottom: calc(var(--spacing) * 11)` |
| `inset-y-12` | `top: calc(var(--spacing) * 12); bottom: calc(var(--spacing) * 12)` |
| `inset-y-14` | `top: calc(var(--spacing) * 14); bottom: calc(var(--spacing) * 14)` |
| `inset-y-16` | `top: calc(var(--spacing) * 16); bottom: calc(var(--spacing) * 16)` |
| `inset-y-20` | `top: calc(var(--spacing) * 20); bottom: calc(var(--spacing) * 20)` |
| `inset-y-24` | `top: calc(var(--spacing) * 24); bottom: calc(var(--spacing) * 24)` |
| `inset-y-28` | `top: calc(var(--spacing) * 28); bottom: calc(var(--spacing) * 28)` |
| `inset-y-32` | `top: calc(var(--spacing) * 32); bottom: calc(var(--spacing) * 32)` |
| `inset-y-36` | `top: calc(var(--spacing) * 36); bottom: calc(var(--spacing) * 36)` |
| `inset-y-40` | `top: calc(var(--spacing) * 40); bottom: calc(var(--spacing) * 40)` |
| `inset-y-44` | `top: calc(var(--spacing) * 44); bottom: calc(var(--spacing) * 44)` |
| `inset-y-48` | `top: calc(var(--spacing) * 48); bottom: calc(var(--spacing) * 48)` |
| `inset-y-52` | `top: calc(var(--spacing) * 52); bottom: calc(var(--spacing) * 52)` |
| `inset-y-56` | `top: calc(var(--spacing) * 56); bottom: calc(var(--spacing) * 56)` |
| `inset-y-60` | `top: calc(var(--spacing) * 60); bottom: calc(var(--spacing) * 60)` |
| `inset-y-64` | `top: calc(var(--spacing) * 64); bottom: calc(var(--spacing) * 64)` |
| `inset-y-72` | `top: calc(var(--spacing) * 72); bottom: calc(var(--spacing) * 72)` |
| `inset-y-80` | `top: calc(var(--spacing) * 80); bottom: calc(var(--spacing) * 80)` |
| `inset-y-96` | `top: calc(var(--spacing) * 96); bottom: calc(var(--spacing) * 96)` |
| `top-0` | `top: 0px` |
| `top-px` | `top: 1px` |
| `top-0.5` | `top: calc(var(--spacing) * 0.5)` |
| `top-1` | `top: calc(var(--spacing) * 1)` |
| `top-1.5` | `top: calc(var(--spacing) * 1.5)` |
| `top-2` | `top: calc(var(--spacing) * 2)` |
| `top-2.5` | `top: calc(var(--spacing) * 2.5)` |
| `top-3` | `top: calc(var(--spacing) * 3)` |
| `top-3.5` | `top: calc(var(--spacing) * 3.5)` |
| `top-4` | `top: calc(var(--spacing) * 4)` |
| `top-5` | `top: calc(var(--spacing) * 5)` |
| `top-6` | `top: calc(var(--spacing) * 6)` |
| `top-7` | `top: calc(var(--spacing) * 7)` |
| `top-8` | `top: calc(var(--spacing) * 8)` |
| `top-9` | `top: calc(var(--spacing) * 9)` |
| `top-10` | `top: calc(var(--spacing) * 10)` |
| `top-11` | `top: calc(var(--spacing) * 11)` |
| `top-12` | `top: calc(var(--spacing) * 12)` |
| `top-14` | `top: calc(var(--spacing) * 14)` |
| `top-16` | `top: calc(var(--spacing) * 16)` |
| `top-20` | `top: calc(var(--spacing) * 20)` |
| `top-24` | `top: calc(var(--spacing) * 24)` |
| `top-28` | `top: calc(var(--spacing) * 28)` |
| `top-32` | `top: calc(var(--spacing) * 32)` |
| `top-36` | `top: calc(var(--spacing) * 36)` |
| `top-40` | `top: calc(var(--spacing) * 40)` |
| `top-44` | `top: calc(var(--spacing) * 44)` |
| `top-48` | `top: calc(var(--spacing) * 48)` |
| `top-52` | `top: calc(var(--spacing) * 52)` |
| `top-56` | `top: calc(var(--spacing) * 56)` |
| `top-60` | `top: calc(var(--spacing) * 60)` |
| `top-64` | `top: calc(var(--spacing) * 64)` |
| `top-72` | `top: calc(var(--spacing) * 72)` |
| `top-80` | `top: calc(var(--spacing) * 80)` |
| `top-96` | `top: calc(var(--spacing) * 96)` |
| `right-0` | `right: 0px` |
| `right-px` | `right: 1px` |
| `right-0.5` | `right: calc(var(--spacing) * 0.5)` |
| `right-1` | `right: calc(var(--spacing) * 1)` |
| `right-1.5` | `right: calc(var(--spacing) * 1.5)` |
| `right-2` | `right: calc(var(--spacing) * 2)` |
| `right-2.5` | `right: calc(var(--spacing) * 2.5)` |
| `right-3` | `right: calc(var(--spacing) * 3)` |
| `right-3.5` | `right: calc(var(--spacing) * 3.5)` |
| `right-4` | `right: calc(var(--spacing) * 4)` |
| `right-5` | `right: calc(var(--spacing) * 5)` |
| `right-6` | `right: calc(var(--spacing) * 6)` |
| `right-7` | `right: calc(var(--spacing) * 7)` |
| `right-8` | `right: calc(var(--spacing) * 8)` |
| `right-9` | `right: calc(var(--spacing) * 9)` |
| `right-10` | `right: calc(var(--spacing) * 10)` |
| `right-11` | `right: calc(var(--spacing) * 11)` |
| `right-12` | `right: calc(var(--spacing) * 12)` |
| `right-14` | `right: calc(var(--spacing) * 14)` |
| `right-16` | `right: calc(var(--spacing) * 16)` |
| `right-20` | `right: calc(var(--spacing) * 20)` |
| `right-24` | `right: calc(var(--spacing) * 24)` |
| `right-28` | `right: calc(var(--spacing) * 28)` |
| `right-32` | `right: calc(var(--spacing) * 32)` |
| `right-36` | `right: calc(var(--spacing) * 36)` |
| `right-40` | `right: calc(var(--spacing) * 40)` |
| `right-44` | `right: calc(var(--spacing) * 44)` |
| `right-48` | `right: calc(var(--spacing) * 48)` |
| `right-52` | `right: calc(var(--spacing) * 52)` |
| `right-56` | `right: calc(var(--spacing) * 56)` |
| `right-60` | `right: calc(var(--spacing) * 60)` |
| `right-64` | `right: calc(var(--spacing) * 64)` |
| `right-72` | `right: calc(var(--spacing) * 72)` |
| `right-80` | `right: calc(var(--spacing) * 80)` |
| `right-96` | `right: calc(var(--spacing) * 96)` |
| `bottom-0` | `bottom: 0px` |
| `bottom-px` | `bottom: 1px` |
| `bottom-0.5` | `bottom: calc(var(--spacing) * 0.5)` |
| `bottom-1` | `bottom: calc(var(--spacing) * 1)` |
| `bottom-1.5` | `bottom: calc(var(--spacing) * 1.5)` |
| `bottom-2` | `bottom: calc(var(--spacing) * 2)` |
| `bottom-2.5` | `bottom: calc(var(--spacing) * 2.5)` |
| `bottom-3` | `bottom: calc(var(--spacing) * 3)` |
| `bottom-3.5` | `bottom: calc(var(--spacing) * 3.5)` |
| `bottom-4` | `bottom: calc(var(--spacing) * 4)` |
| `bottom-5` | `bottom: calc(var(--spacing) * 5)` |
| `bottom-6` | `bottom: calc(var(--spacing) * 6)` |
| `bottom-7` | `bottom: calc(var(--spacing) * 7)` |
| `bottom-8` | `bottom: calc(var(--spacing) * 8)` |
| `bottom-9` | `bottom: calc(var(--spacing) * 9)` |
| `bottom-10` | `bottom: calc(var(--spacing) * 10)` |
| `bottom-11` | `bottom: calc(var(--spacing) * 11)` |
| `bottom-12` | `bottom: calc(var(--spacing) * 12)` |
| `bottom-14` | `bottom: calc(var(--spacing) * 14)` |
| `bottom-16` | `bottom: calc(var(--spacing) * 16)` |
| `bottom-20` | `bottom: calc(var(--spacing) * 20)` |
| `bottom-24` | `bottom: calc(var(--spacing) * 24)` |
| `bottom-28` | `bottom: calc(var(--spacing) * 28)` |
| `bottom-32` | `bottom: calc(var(--spacing) * 32)` |
| `bottom-36` | `bottom: calc(var(--spacing) * 36)` |
| `bottom-40` | `bottom: calc(var(--spacing) * 40)` |
| `bottom-44` | `bottom: calc(var(--spacing) * 44)` |
| `bottom-48` | `bottom: calc(var(--spacing) * 48)` |
| `bottom-52` | `bottom: calc(var(--spacing) * 52)` |
| `bottom-56` | `bottom: calc(var(--spacing) * 56)` |
| `bottom-60` | `bottom: calc(var(--spacing) * 60)` |
| `bottom-64` | `bottom: calc(var(--spacing) * 64)` |
| `bottom-72` | `bottom: calc(var(--spacing) * 72)` |
| `bottom-80` | `bottom: calc(var(--spacing) * 80)` |
| `bottom-96` | `bottom: calc(var(--spacing) * 96)` |
| `left-0` | `left: 0px` |
| `left-px` | `left: 1px` |
| `left-0.5` | `left: calc(var(--spacing) * 0.5)` |
| `left-1` | `left: calc(var(--spacing) * 1)` |
| `left-1.5` | `left: calc(var(--spacing) * 1.5)` |
| `left-2` | `left: calc(var(--spacing) * 2)` |
| `left-2.5` | `left: calc(var(--spacing) * 2.5)` |
| `left-3` | `left: calc(var(--spacing) * 3)` |
| `left-3.5` | `left: calc(var(--spacing) * 3.5)` |
| `left-4` | `left: calc(var(--spacing) * 4)` |
| `left-5` | `left: calc(var(--spacing) * 5)` |
| `left-6` | `left: calc(var(--spacing) * 6)` |
| `left-7` | `left: calc(var(--spacing) * 7)` |
| `left-8` | `left: calc(var(--spacing) * 8)` |
| `left-9` | `left: calc(var(--spacing) * 9)` |
| `left-10` | `left: calc(var(--spacing) * 10)` |
| `left-11` | `left: calc(var(--spacing) * 11)` |
| `left-12` | `left: calc(var(--spacing) * 12)` |
| `left-14` | `left: calc(var(--spacing) * 14)` |
| `left-16` | `left: calc(var(--spacing) * 16)` |
| `left-20` | `left: calc(var(--spacing) * 20)` |
| `left-24` | `left: calc(var(--spacing) * 24)` |
| `left-28` | `left: calc(var(--spacing) * 28)` |
| `left-32` | `left: calc(var(--spacing) * 32)` |
| `left-36` | `left: calc(var(--spacing) * 36)` |
| `left-40` | `left: calc(var(--spacing) * 40)` |
| `left-44` | `left: calc(var(--spacing) * 44)` |
| `left-48` | `left: calc(var(--spacing) * 48)` |
| `left-52` | `left: calc(var(--spacing) * 52)` |
| `left-56` | `left: calc(var(--spacing) * 56)` |
| `left-60` | `left: calc(var(--spacing) * 60)` |
| `left-64` | `left: calc(var(--spacing) * 64)` |
| `left-72` | `left: calc(var(--spacing) * 72)` |
| `left-80` | `left: calc(var(--spacing) * 80)` |
| `left-96` | `left: calc(var(--spacing) * 96)` |
| `inset-auto` | `inset: auto` |
| `top-auto` | `top: auto` |
| `right-auto` | `right: auto` |
| `bottom-auto` | `bottom: auto` |
| `left-auto` | `left: auto` |
| `inset-full` | `inset: 100%` |
| `top-full` | `top: 100%` |
| `right-full` | `right: 100%` |
| `bottom-full` | `bottom: 100%` |
| `left-full` | `left: 100%` |
| `top-1/2` | `top: 50%` |
| `right-1/2` | `right: 50%` |
| `bottom-1/2` | `bottom: 50%` |
| `left-1/2` | `left: 50%` |

## Layout — Z-Index

| Classe | CSS |
|--------|-----|
| `z-0` | `z-index: 0` |
| `z-10` | `z-index: 10` |
| `z-20` | `z-index: 20` |
| `z-30` | `z-index: 30` |
| `z-40` | `z-index: 40` |
| `z-50` | `z-index: 50` |
| `z-auto` | `z-index: auto` |

## Layout — Overflow

| Classe | CSS |
|--------|-----|
| `overflow-auto` | `overflow: auto` |
| `overflow-x-auto` | `overflow-x: auto` |
| `overflow-y-auto` | `overflow-y: auto` |
| `overflow-hidden` | `overflow: hidden` |
| `overflow-x-hidden` | `overflow-x: hidden` |
| `overflow-y-hidden` | `overflow-y: hidden` |
| `overflow-clip` | `overflow: clip` |
| `overflow-x-clip` | `overflow-x: clip` |
| `overflow-y-clip` | `overflow-y: clip` |
| `overflow-visible` | `overflow: visible` |
| `overflow-x-visible` | `overflow-x: visible` |
| `overflow-y-visible` | `overflow-y: visible` |
| `overflow-scroll` | `overflow: scroll` |
| `overflow-x-scroll` | `overflow-x: scroll` |
| `overflow-y-scroll` | `overflow-y: scroll` |
| `overscroll-auto` | `overscroll-behavior: auto` |
| `overscroll-x-auto` | `overscroll-behavior-x: auto` |
| `overscroll-y-auto` | `overscroll-behavior-y: auto` |
| `overscroll-contain` | `overscroll-behavior: contain` |
| `overscroll-x-contain` | `overscroll-behavior-x: contain` |
| `overscroll-y-contain` | `overscroll-behavior-y: contain` |
| `overscroll-none` | `overscroll-behavior: none` |
| `overscroll-x-none` | `overscroll-behavior-x: none` |
| `overscroll-y-none` | `overscroll-behavior-y: none` |

## Layout — Visibility

| Classe | CSS |
|--------|-----|
| `visible` | `visibility: visible` |
| `invisible` | `visibility: hidden` |
| `collapse` | `visibility: collapse` |
| `box-border` | `box-sizing: border-box` |
| `box-content` | `box-sizing: content-box` |
| `isolate` | `isolation: isolate` |
| `isolation-auto` | `isolation: auto` |
| `float-left` | `float: left` |
| `float-right` | `float: right` |
| `float-none` | `float: none` |
| `float-start` | `float: inline-start` |
| `float-end` | `float: inline-end` |
| `clear-left` | `clear: left` |
| `clear-right` | `clear: right` |
| `clear-both` | `clear: both` |
| `clear-none` | `clear: none` |
| `aspect-auto` | `aspect-ratio: auto` |
| `aspect-square` | `aspect-ratio: 1 / 1` |
| `aspect-video` | `aspect-ratio: 16 / 9` |

## Layout — Object Fit

| Classe | CSS |
|--------|-----|
| `object-contain` | `object-fit: contain` |
| `object-cover` | `object-fit: cover` |
| `object-fill` | `object-fit: fill` |
| `object-none` | `object-fit: none` |
| `object-scale-down` | `object-fit: scale-down` |
| `object-center` | `object-position: center` |
| `object-top` | `object-position: top` |
| `object-right` | `object-position: right` |
| `object-bottom` | `object-position: bottom` |
| `object-left` | `object-position: left` |

## Flexbox

| Classe | CSS |
|--------|-----|
| `flex-row` | `flex-direction: row` |
| `flex-row-reverse` | `flex-direction: row-reverse` |
| `flex-col` | `flex-direction: column` |
| `flex-col-reverse` | `flex-direction: column-reverse` |
| `flex-wrap` | `flex-wrap: wrap` |
| `flex-wrap-reverse` | `flex-wrap: wrap-reverse` |
| `flex-nowrap` | `flex-wrap: nowrap` |
| `flex-1` | `flex: 1 1 0%` |
| `flex-auto` | `flex: 1 1 auto` |
| `flex-initial` | `flex: 0 1 auto` |
| `flex-none` | `flex: none` |
| `grow` | `flex-grow: 1` |
| `grow-0` | `flex-grow: 0` |
| `shrink` | `flex-shrink: 1` |
| `shrink-0` | `flex-shrink: 0` |
| `basis-0` | `flex-basis: 0px` |
| `basis-px` | `flex-basis: 1px` |
| `basis-0.5` | `flex-basis: calc(var(--spacing) * 0.5)` |
| `basis-1` | `flex-basis: calc(var(--spacing) * 1)` |
| `basis-1.5` | `flex-basis: calc(var(--spacing) * 1.5)` |
| `basis-2` | `flex-basis: calc(var(--spacing) * 2)` |
| `basis-2.5` | `flex-basis: calc(var(--spacing) * 2.5)` |
| `basis-3` | `flex-basis: calc(var(--spacing) * 3)` |
| `basis-3.5` | `flex-basis: calc(var(--spacing) * 3.5)` |
| `basis-4` | `flex-basis: calc(var(--spacing) * 4)` |
| `basis-5` | `flex-basis: calc(var(--spacing) * 5)` |
| `basis-6` | `flex-basis: calc(var(--spacing) * 6)` |
| `basis-7` | `flex-basis: calc(var(--spacing) * 7)` |
| `basis-8` | `flex-basis: calc(var(--spacing) * 8)` |
| `basis-9` | `flex-basis: calc(var(--spacing) * 9)` |
| `basis-10` | `flex-basis: calc(var(--spacing) * 10)` |
| `basis-11` | `flex-basis: calc(var(--spacing) * 11)` |
| `basis-12` | `flex-basis: calc(var(--spacing) * 12)` |
| `basis-14` | `flex-basis: calc(var(--spacing) * 14)` |
| `basis-16` | `flex-basis: calc(var(--spacing) * 16)` |
| `basis-20` | `flex-basis: calc(var(--spacing) * 20)` |
| `basis-24` | `flex-basis: calc(var(--spacing) * 24)` |
| `basis-28` | `flex-basis: calc(var(--spacing) * 28)` |
| `basis-32` | `flex-basis: calc(var(--spacing) * 32)` |
| `basis-36` | `flex-basis: calc(var(--spacing) * 36)` |
| `basis-40` | `flex-basis: calc(var(--spacing) * 40)` |
| `basis-44` | `flex-basis: calc(var(--spacing) * 44)` |
| `basis-48` | `flex-basis: calc(var(--spacing) * 48)` |
| `basis-52` | `flex-basis: calc(var(--spacing) * 52)` |
| `basis-56` | `flex-basis: calc(var(--spacing) * 56)` |
| `basis-60` | `flex-basis: calc(var(--spacing) * 60)` |
| `basis-64` | `flex-basis: calc(var(--spacing) * 64)` |
| `basis-72` | `flex-basis: calc(var(--spacing) * 72)` |
| `basis-80` | `flex-basis: calc(var(--spacing) * 80)` |
| `basis-96` | `flex-basis: calc(var(--spacing) * 96)` |
| `basis-auto` | `flex-basis: auto` |
| `basis-full` | `flex-basis: 100%` |
| `basis-1/2` | `flex-basis: 50%` |
| `basis-1/3` | `flex-basis: 33.333333%` |
| `basis-2/3` | `flex-basis: 66.666667%` |
| `order-1` | `order: 1` |
| `order-2` | `order: 2` |
| `order-3` | `order: 3` |
| `order-4` | `order: 4` |
| `order-5` | `order: 5` |
| `order-6` | `order: 6` |
| `order-7` | `order: 7` |
| `order-8` | `order: 8` |
| `order-9` | `order: 9` |
| `order-10` | `order: 10` |
| `order-11` | `order: 11` |
| `order-12` | `order: 12` |
| `order-first` | `order: -9999` |
| `order-last` | `order: 9999` |
| `order-none` | `order: 0` |

## Grid

| Classe | CSS |
|--------|-----|
| `grid-cols-1` | `grid-template-columns: repeat(1, minmax(0, 1fr))` |
| `grid-cols-2` | `grid-template-columns: repeat(2, minmax(0, 1fr))` |
| `grid-cols-3` | `grid-template-columns: repeat(3, minmax(0, 1fr))` |
| `grid-cols-4` | `grid-template-columns: repeat(4, minmax(0, 1fr))` |
| `grid-cols-5` | `grid-template-columns: repeat(5, minmax(0, 1fr))` |
| `grid-cols-6` | `grid-template-columns: repeat(6, minmax(0, 1fr))` |
| `grid-cols-7` | `grid-template-columns: repeat(7, minmax(0, 1fr))` |
| `grid-cols-8` | `grid-template-columns: repeat(8, minmax(0, 1fr))` |
| `grid-cols-9` | `grid-template-columns: repeat(9, minmax(0, 1fr))` |
| `grid-cols-10` | `grid-template-columns: repeat(10, minmax(0, 1fr))` |
| `grid-cols-11` | `grid-template-columns: repeat(11, minmax(0, 1fr))` |
| `grid-cols-12` | `grid-template-columns: repeat(12, minmax(0, 1fr))` |
| `grid-cols-none` | `grid-template-columns: none` |
| `grid-cols-subgrid` | `grid-template-columns: subgrid` |
| `grid-rows-1` | `grid-template-rows: repeat(1, minmax(0, 1fr))` |
| `grid-rows-2` | `grid-template-rows: repeat(2, minmax(0, 1fr))` |
| `grid-rows-3` | `grid-template-rows: repeat(3, minmax(0, 1fr))` |
| `grid-rows-4` | `grid-template-rows: repeat(4, minmax(0, 1fr))` |
| `grid-rows-5` | `grid-template-rows: repeat(5, minmax(0, 1fr))` |
| `grid-rows-6` | `grid-template-rows: repeat(6, minmax(0, 1fr))` |
| `grid-rows-7` | `grid-template-rows: repeat(7, minmax(0, 1fr))` |
| `grid-rows-8` | `grid-template-rows: repeat(8, minmax(0, 1fr))` |
| `grid-rows-9` | `grid-template-rows: repeat(9, minmax(0, 1fr))` |
| `grid-rows-10` | `grid-template-rows: repeat(10, minmax(0, 1fr))` |
| `grid-rows-11` | `grid-template-rows: repeat(11, minmax(0, 1fr))` |
| `grid-rows-12` | `grid-template-rows: repeat(12, minmax(0, 1fr))` |
| `grid-rows-none` | `grid-template-rows: none` |
| `grid-rows-subgrid` | `grid-template-rows: subgrid` |
| `col-span-1` | `grid-column: span 1 / span 1` |
| `col-span-2` | `grid-column: span 2 / span 2` |
| `col-span-3` | `grid-column: span 3 / span 3` |
| `col-span-4` | `grid-column: span 4 / span 4` |
| `col-span-5` | `grid-column: span 5 / span 5` |
| `col-span-6` | `grid-column: span 6 / span 6` |
| `col-span-7` | `grid-column: span 7 / span 7` |
| `col-span-8` | `grid-column: span 8 / span 8` |
| `col-span-9` | `grid-column: span 9 / span 9` |
| `col-span-10` | `grid-column: span 10 / span 10` |
| `col-span-11` | `grid-column: span 11 / span 11` |
| `col-span-12` | `grid-column: span 12 / span 12` |
| `col-span-full` | `grid-column: 1 / -1` |
| `col-start-1` | `grid-column-start: 1` |
| `col-start-2` | `grid-column-start: 2` |
| `col-start-3` | `grid-column-start: 3` |
| `col-start-4` | `grid-column-start: 4` |
| `col-start-5` | `grid-column-start: 5` |
| `col-start-6` | `grid-column-start: 6` |
| `col-start-7` | `grid-column-start: 7` |
| `col-start-8` | `grid-column-start: 8` |
| `col-start-9` | `grid-column-start: 9` |
| `col-start-10` | `grid-column-start: 10` |
| `col-start-11` | `grid-column-start: 11` |
| `col-start-12` | `grid-column-start: 12` |
| `col-start-13` | `grid-column-start: 13` |
| `col-start-auto` | `grid-column-start: auto` |
| `col-end-1` | `grid-column-end: 1` |
| `col-end-2` | `grid-column-end: 2` |
| `col-end-3` | `grid-column-end: 3` |
| `col-end-4` | `grid-column-end: 4` |
| `col-end-5` | `grid-column-end: 5` |
| `col-end-6` | `grid-column-end: 6` |
| `col-end-7` | `grid-column-end: 7` |
| `col-end-8` | `grid-column-end: 8` |
| `col-end-9` | `grid-column-end: 9` |
| `col-end-10` | `grid-column-end: 10` |
| `col-end-11` | `grid-column-end: 11` |
| `col-end-12` | `grid-column-end: 12` |
| `col-end-13` | `grid-column-end: 13` |
| `col-end-auto` | `grid-column-end: auto` |
| `row-span-1` | `grid-row: span 1 / span 1` |
| `row-span-2` | `grid-row: span 2 / span 2` |
| `row-span-3` | `grid-row: span 3 / span 3` |
| `row-span-4` | `grid-row: span 4 / span 4` |
| `row-span-5` | `grid-row: span 5 / span 5` |
| `row-span-6` | `grid-row: span 6 / span 6` |
| `row-span-full` | `grid-row: 1 / -1` |
| `row-start-1` | `grid-row-start: 1` |
| `row-start-2` | `grid-row-start: 2` |
| `row-start-3` | `grid-row-start: 3` |
| `row-start-4` | `grid-row-start: 4` |
| `row-start-5` | `grid-row-start: 5` |
| `row-start-6` | `grid-row-start: 6` |
| `row-start-7` | `grid-row-start: 7` |
| `row-start-auto` | `grid-row-start: auto` |
| `row-end-1` | `grid-row-end: 1` |
| `row-end-2` | `grid-row-end: 2` |
| `row-end-3` | `grid-row-end: 3` |
| `row-end-4` | `grid-row-end: 4` |
| `row-end-5` | `grid-row-end: 5` |
| `row-end-6` | `grid-row-end: 6` |
| `row-end-7` | `grid-row-end: 7` |
| `row-end-auto` | `grid-row-end: auto` |
| `grid-flow-row` | `grid-auto-flow: row` |
| `grid-flow-col` | `grid-auto-flow: column` |
| `grid-flow-dense` | `grid-auto-flow: dense` |
| `grid-flow-row-dense` | `grid-auto-flow: row dense` |
| `grid-flow-col-dense` | `grid-auto-flow: column dense` |
| `auto-cols-auto` | `grid-auto-columns: auto` |
| `auto-cols-min` | `grid-auto-columns: min` |
| `auto-cols-max` | `grid-auto-columns: max` |
| `auto-cols-fr` | `grid-auto-columns: minmax(0, 1fr)` |
| `auto-rows-auto` | `grid-auto-rows: auto` |
| `auto-rows-min` | `grid-auto-rows: min` |
| `auto-rows-max` | `grid-auto-rows: max` |
| `auto-rows-fr` | `grid-auto-rows: minmax(0, 1fr)` |

## Flexbox & Grid — Alignment

| Classe | CSS |
|--------|-----|
| `justify-normal` | `justify-content: normal` |
| `justify-start` | `justify-content: start` |
| `justify-end` | `justify-content: end` |
| `justify-center` | `justify-content: center` |
| `justify-between` | `justify-content: space-between` |
| `justify-around` | `justify-content: space-around` |
| `justify-evenly` | `justify-content: space-evenly` |
| `justify-stretch` | `justify-content: stretch` |
| `justify-items-start` | `justify-items: start` |
| `justify-items-end` | `justify-items: end` |
| `justify-items-center` | `justify-items: center` |
| `justify-items-stretch` | `justify-items: stretch` |
| `justify-items-normal` | `justify-items: normal` |
| `justify-self-auto` | `justify-self: auto` |
| `justify-self-start` | `justify-self: start` |
| `justify-self-end` | `justify-self: end` |
| `justify-self-center` | `justify-self: center` |
| `justify-self-stretch` | `justify-self: stretch` |
| `items-start` | `align-items: start` |
| `items-end` | `align-items: end` |
| `items-center` | `align-items: center` |
| `items-baseline` | `align-items: baseline` |
| `items-stretch` | `align-items: stretch` |
| `content-normal` | `align-content: normal` |
| `content-center` | `align-content: center` |
| `content-start` | `align-content: start` |
| `content-end` | `align-content: end` |
| `content-between` | `align-content: space-between` |
| `content-around` | `align-content: space-around` |
| `content-evenly` | `align-content: space-evenly` |
| `content-baseline` | `align-content: baseline` |
| `content-stretch` | `align-content: stretch` |
| `self-auto` | `align-self: auto` |
| `self-start` | `align-self: start` |
| `self-end` | `align-self: end` |
| `self-center` | `align-self: center` |
| `self-stretch` | `align-self: stretch` |
| `self-baseline` | `align-self: baseline` |
| `place-content-center` | `place-content: center` |
| `place-content-start` | `place-content: start` |
| `place-content-end` | `place-content: end` |
| `place-content-between` | `place-content: space-between` |
| `place-content-around` | `place-content: space-around` |
| `place-content-evenly` | `place-content: evenly` |
| `place-content-baseline` | `place-content: baseline` |
| `place-content-stretch` | `place-content: stretch` |
| `place-items-center` | `place-items: center` |
| `place-items-start` | `place-items: start` |
| `place-items-end` | `place-items: end` |
| `place-items-baseline` | `place-items: baseline` |
| `place-items-stretch` | `place-items: stretch` |
| `place-self-auto` | `place-self: auto` |
| `place-self-start` | `place-self: start` |
| `place-self-end` | `place-self: end` |
| `place-self-center` | `place-self: center` |
| `place-self-stretch` | `place-self: stretch` |

## Spacing — Gap

| Classe | CSS |
|--------|-----|
| `gap-0` | `gap: 0px` |
| `gap-px` | `gap: 1px` |
| `gap-0.5` | `gap: calc(var(--spacing) * 0.5)` |
| `gap-1` | `gap: calc(var(--spacing) * 1)` |
| `gap-1.5` | `gap: calc(var(--spacing) * 1.5)` |
| `gap-2` | `gap: calc(var(--spacing) * 2)` |
| `gap-2.5` | `gap: calc(var(--spacing) * 2.5)` |
| `gap-3` | `gap: calc(var(--spacing) * 3)` |
| `gap-3.5` | `gap: calc(var(--spacing) * 3.5)` |
| `gap-4` | `gap: calc(var(--spacing) * 4)` |
| `gap-5` | `gap: calc(var(--spacing) * 5)` |
| `gap-6` | `gap: calc(var(--spacing) * 6)` |
| `gap-7` | `gap: calc(var(--spacing) * 7)` |
| `gap-8` | `gap: calc(var(--spacing) * 8)` |
| `gap-9` | `gap: calc(var(--spacing) * 9)` |
| `gap-10` | `gap: calc(var(--spacing) * 10)` |
| `gap-11` | `gap: calc(var(--spacing) * 11)` |
| `gap-12` | `gap: calc(var(--spacing) * 12)` |
| `gap-14` | `gap: calc(var(--spacing) * 14)` |
| `gap-16` | `gap: calc(var(--spacing) * 16)` |
| `gap-20` | `gap: calc(var(--spacing) * 20)` |
| `gap-24` | `gap: calc(var(--spacing) * 24)` |
| `gap-28` | `gap: calc(var(--spacing) * 28)` |
| `gap-32` | `gap: calc(var(--spacing) * 32)` |
| `gap-36` | `gap: calc(var(--spacing) * 36)` |
| `gap-40` | `gap: calc(var(--spacing) * 40)` |
| `gap-44` | `gap: calc(var(--spacing) * 44)` |
| `gap-48` | `gap: calc(var(--spacing) * 48)` |
| `gap-52` | `gap: calc(var(--spacing) * 52)` |
| `gap-56` | `gap: calc(var(--spacing) * 56)` |
| `gap-60` | `gap: calc(var(--spacing) * 60)` |
| `gap-64` | `gap: calc(var(--spacing) * 64)` |
| `gap-72` | `gap: calc(var(--spacing) * 72)` |
| `gap-80` | `gap: calc(var(--spacing) * 80)` |
| `gap-96` | `gap: calc(var(--spacing) * 96)` |
| `gap-x-0` | `column-gap: 0px` |
| `gap-x-px` | `column-gap: 1px` |
| `gap-x-0.5` | `column-gap: calc(var(--spacing) * 0.5)` |
| `gap-x-1` | `column-gap: calc(var(--spacing) * 1)` |
| `gap-x-1.5` | `column-gap: calc(var(--spacing) * 1.5)` |
| `gap-x-2` | `column-gap: calc(var(--spacing) * 2)` |
| `gap-x-2.5` | `column-gap: calc(var(--spacing) * 2.5)` |
| `gap-x-3` | `column-gap: calc(var(--spacing) * 3)` |
| `gap-x-3.5` | `column-gap: calc(var(--spacing) * 3.5)` |
| `gap-x-4` | `column-gap: calc(var(--spacing) * 4)` |
| `gap-x-5` | `column-gap: calc(var(--spacing) * 5)` |
| `gap-x-6` | `column-gap: calc(var(--spacing) * 6)` |
| `gap-x-7` | `column-gap: calc(var(--spacing) * 7)` |
| `gap-x-8` | `column-gap: calc(var(--spacing) * 8)` |
| `gap-x-9` | `column-gap: calc(var(--spacing) * 9)` |
| `gap-x-10` | `column-gap: calc(var(--spacing) * 10)` |
| `gap-x-11` | `column-gap: calc(var(--spacing) * 11)` |
| `gap-x-12` | `column-gap: calc(var(--spacing) * 12)` |
| `gap-x-14` | `column-gap: calc(var(--spacing) * 14)` |
| `gap-x-16` | `column-gap: calc(var(--spacing) * 16)` |
| `gap-x-20` | `column-gap: calc(var(--spacing) * 20)` |
| `gap-x-24` | `column-gap: calc(var(--spacing) * 24)` |
| `gap-x-28` | `column-gap: calc(var(--spacing) * 28)` |
| `gap-x-32` | `column-gap: calc(var(--spacing) * 32)` |
| `gap-x-36` | `column-gap: calc(var(--spacing) * 36)` |
| `gap-x-40` | `column-gap: calc(var(--spacing) * 40)` |
| `gap-x-44` | `column-gap: calc(var(--spacing) * 44)` |
| `gap-x-48` | `column-gap: calc(var(--spacing) * 48)` |
| `gap-x-52` | `column-gap: calc(var(--spacing) * 52)` |
| `gap-x-56` | `column-gap: calc(var(--spacing) * 56)` |
| `gap-x-60` | `column-gap: calc(var(--spacing) * 60)` |
| `gap-x-64` | `column-gap: calc(var(--spacing) * 64)` |
| `gap-x-72` | `column-gap: calc(var(--spacing) * 72)` |
| `gap-x-80` | `column-gap: calc(var(--spacing) * 80)` |
| `gap-x-96` | `column-gap: calc(var(--spacing) * 96)` |
| `gap-y-0` | `row-gap: 0px` |
| `gap-y-px` | `row-gap: 1px` |
| `gap-y-0.5` | `row-gap: calc(var(--spacing) * 0.5)` |
| `gap-y-1` | `row-gap: calc(var(--spacing) * 1)` |
| `gap-y-1.5` | `row-gap: calc(var(--spacing) * 1.5)` |
| `gap-y-2` | `row-gap: calc(var(--spacing) * 2)` |
| `gap-y-2.5` | `row-gap: calc(var(--spacing) * 2.5)` |
| `gap-y-3` | `row-gap: calc(var(--spacing) * 3)` |
| `gap-y-3.5` | `row-gap: calc(var(--spacing) * 3.5)` |
| `gap-y-4` | `row-gap: calc(var(--spacing) * 4)` |
| `gap-y-5` | `row-gap: calc(var(--spacing) * 5)` |
| `gap-y-6` | `row-gap: calc(var(--spacing) * 6)` |
| `gap-y-7` | `row-gap: calc(var(--spacing) * 7)` |
| `gap-y-8` | `row-gap: calc(var(--spacing) * 8)` |
| `gap-y-9` | `row-gap: calc(var(--spacing) * 9)` |
| `gap-y-10` | `row-gap: calc(var(--spacing) * 10)` |
| `gap-y-11` | `row-gap: calc(var(--spacing) * 11)` |
| `gap-y-12` | `row-gap: calc(var(--spacing) * 12)` |
| `gap-y-14` | `row-gap: calc(var(--spacing) * 14)` |
| `gap-y-16` | `row-gap: calc(var(--spacing) * 16)` |
| `gap-y-20` | `row-gap: calc(var(--spacing) * 20)` |
| `gap-y-24` | `row-gap: calc(var(--spacing) * 24)` |
| `gap-y-28` | `row-gap: calc(var(--spacing) * 28)` |
| `gap-y-32` | `row-gap: calc(var(--spacing) * 32)` |
| `gap-y-36` | `row-gap: calc(var(--spacing) * 36)` |
| `gap-y-40` | `row-gap: calc(var(--spacing) * 40)` |
| `gap-y-44` | `row-gap: calc(var(--spacing) * 44)` |
| `gap-y-48` | `row-gap: calc(var(--spacing) * 48)` |
| `gap-y-52` | `row-gap: calc(var(--spacing) * 52)` |
| `gap-y-56` | `row-gap: calc(var(--spacing) * 56)` |
| `gap-y-60` | `row-gap: calc(var(--spacing) * 60)` |
| `gap-y-64` | `row-gap: calc(var(--spacing) * 64)` |
| `gap-y-72` | `row-gap: calc(var(--spacing) * 72)` |
| `gap-y-80` | `row-gap: calc(var(--spacing) * 80)` |
| `gap-y-96` | `row-gap: calc(var(--spacing) * 96)` |

## Spacing — Padding

| Classe | CSS |
|--------|-----|
| `p-0` | `padding: 0px` |
| `p-px` | `padding: 1px` |
| `p-0.5` | `padding: calc(var(--spacing) * 0.5)` |
| `p-1` | `padding: calc(var(--spacing) * 1)` |
| `p-1.5` | `padding: calc(var(--spacing) * 1.5)` |
| `p-2` | `padding: calc(var(--spacing) * 2)` |
| `p-2.5` | `padding: calc(var(--spacing) * 2.5)` |
| `p-3` | `padding: calc(var(--spacing) * 3)` |
| `p-3.5` | `padding: calc(var(--spacing) * 3.5)` |
| `p-4` | `padding: calc(var(--spacing) * 4)` |
| `p-5` | `padding: calc(var(--spacing) * 5)` |
| `p-6` | `padding: calc(var(--spacing) * 6)` |
| `p-7` | `padding: calc(var(--spacing) * 7)` |
| `p-8` | `padding: calc(var(--spacing) * 8)` |
| `p-9` | `padding: calc(var(--spacing) * 9)` |
| `p-10` | `padding: calc(var(--spacing) * 10)` |
| `p-11` | `padding: calc(var(--spacing) * 11)` |
| `p-12` | `padding: calc(var(--spacing) * 12)` |
| `p-14` | `padding: calc(var(--spacing) * 14)` |
| `p-16` | `padding: calc(var(--spacing) * 16)` |
| `p-20` | `padding: calc(var(--spacing) * 20)` |
| `p-24` | `padding: calc(var(--spacing) * 24)` |
| `p-28` | `padding: calc(var(--spacing) * 28)` |
| `p-32` | `padding: calc(var(--spacing) * 32)` |
| `p-36` | `padding: calc(var(--spacing) * 36)` |
| `p-40` | `padding: calc(var(--spacing) * 40)` |
| `p-44` | `padding: calc(var(--spacing) * 44)` |
| `p-48` | `padding: calc(var(--spacing) * 48)` |
| `p-52` | `padding: calc(var(--spacing) * 52)` |
| `p-56` | `padding: calc(var(--spacing) * 56)` |
| `p-60` | `padding: calc(var(--spacing) * 60)` |
| `p-64` | `padding: calc(var(--spacing) * 64)` |
| `p-72` | `padding: calc(var(--spacing) * 72)` |
| `p-80` | `padding: calc(var(--spacing) * 80)` |
| `p-96` | `padding: calc(var(--spacing) * 96)` |
| `px-0` | `padding-left: 0px; padding-right: 0px` |
| `px-px` | `padding-left: 1px; padding-right: 1px` |
| `px-0.5` | `padding-left: calc(var(--spacing) * 0.5); padding-right: calc(var(--spacing) * 0.5)` |
| `px-1` | `padding-left: calc(var(--spacing) * 1); padding-right: calc(var(--spacing) * 1)` |
| `px-1.5` | `padding-left: calc(var(--spacing) * 1.5); padding-right: calc(var(--spacing) * 1.5)` |
| `px-2` | `padding-left: calc(var(--spacing) * 2); padding-right: calc(var(--spacing) * 2)` |
| `px-2.5` | `padding-left: calc(var(--spacing) * 2.5); padding-right: calc(var(--spacing) * 2.5)` |
| `px-3` | `padding-left: calc(var(--spacing) * 3); padding-right: calc(var(--spacing) * 3)` |
| `px-3.5` | `padding-left: calc(var(--spacing) * 3.5); padding-right: calc(var(--spacing) * 3.5)` |
| `px-4` | `padding-left: calc(var(--spacing) * 4); padding-right: calc(var(--spacing) * 4)` |
| `px-5` | `padding-left: calc(var(--spacing) * 5); padding-right: calc(var(--spacing) * 5)` |
| `px-6` | `padding-left: calc(var(--spacing) * 6); padding-right: calc(var(--spacing) * 6)` |
| `px-7` | `padding-left: calc(var(--spacing) * 7); padding-right: calc(var(--spacing) * 7)` |
| `px-8` | `padding-left: calc(var(--spacing) * 8); padding-right: calc(var(--spacing) * 8)` |
| `px-9` | `padding-left: calc(var(--spacing) * 9); padding-right: calc(var(--spacing) * 9)` |
| `px-10` | `padding-left: calc(var(--spacing) * 10); padding-right: calc(var(--spacing) * 10)` |
| `px-11` | `padding-left: calc(var(--spacing) * 11); padding-right: calc(var(--spacing) * 11)` |
| `px-12` | `padding-left: calc(var(--spacing) * 12); padding-right: calc(var(--spacing) * 12)` |
| `px-14` | `padding-left: calc(var(--spacing) * 14); padding-right: calc(var(--spacing) * 14)` |
| `px-16` | `padding-left: calc(var(--spacing) * 16); padding-right: calc(var(--spacing) * 16)` |
| `px-20` | `padding-left: calc(var(--spacing) * 20); padding-right: calc(var(--spacing) * 20)` |
| `px-24` | `padding-left: calc(var(--spacing) * 24); padding-right: calc(var(--spacing) * 24)` |
| `px-28` | `padding-left: calc(var(--spacing) * 28); padding-right: calc(var(--spacing) * 28)` |
| `px-32` | `padding-left: calc(var(--spacing) * 32); padding-right: calc(var(--spacing) * 32)` |
| `px-36` | `padding-left: calc(var(--spacing) * 36); padding-right: calc(var(--spacing) * 36)` |
| `px-40` | `padding-left: calc(var(--spacing) * 40); padding-right: calc(var(--spacing) * 40)` |
| `px-44` | `padding-left: calc(var(--spacing) * 44); padding-right: calc(var(--spacing) * 44)` |
| `px-48` | `padding-left: calc(var(--spacing) * 48); padding-right: calc(var(--spacing) * 48)` |
| `px-52` | `padding-left: calc(var(--spacing) * 52); padding-right: calc(var(--spacing) * 52)` |
| `px-56` | `padding-left: calc(var(--spacing) * 56); padding-right: calc(var(--spacing) * 56)` |
| `px-60` | `padding-left: calc(var(--spacing) * 60); padding-right: calc(var(--spacing) * 60)` |
| `px-64` | `padding-left: calc(var(--spacing) * 64); padding-right: calc(var(--spacing) * 64)` |
| `px-72` | `padding-left: calc(var(--spacing) * 72); padding-right: calc(var(--spacing) * 72)` |
| `px-80` | `padding-left: calc(var(--spacing) * 80); padding-right: calc(var(--spacing) * 80)` |
| `px-96` | `padding-left: calc(var(--spacing) * 96); padding-right: calc(var(--spacing) * 96)` |
| `py-0` | `padding-top: 0px; padding-bottom: 0px` |
| `py-px` | `padding-top: 1px; padding-bottom: 1px` |
| `py-0.5` | `padding-top: calc(var(--spacing) * 0.5); padding-bottom: calc(var(--spacing) * 0.5)` |
| `py-1` | `padding-top: calc(var(--spacing) * 1); padding-bottom: calc(var(--spacing) * 1)` |
| `py-1.5` | `padding-top: calc(var(--spacing) * 1.5); padding-bottom: calc(var(--spacing) * 1.5)` |
| `py-2` | `padding-top: calc(var(--spacing) * 2); padding-bottom: calc(var(--spacing) * 2)` |
| `py-2.5` | `padding-top: calc(var(--spacing) * 2.5); padding-bottom: calc(var(--spacing) * 2.5)` |
| `py-3` | `padding-top: calc(var(--spacing) * 3); padding-bottom: calc(var(--spacing) * 3)` |
| `py-3.5` | `padding-top: calc(var(--spacing) * 3.5); padding-bottom: calc(var(--spacing) * 3.5)` |
| `py-4` | `padding-top: calc(var(--spacing) * 4); padding-bottom: calc(var(--spacing) * 4)` |
| `py-5` | `padding-top: calc(var(--spacing) * 5); padding-bottom: calc(var(--spacing) * 5)` |
| `py-6` | `padding-top: calc(var(--spacing) * 6); padding-bottom: calc(var(--spacing) * 6)` |
| `py-7` | `padding-top: calc(var(--spacing) * 7); padding-bottom: calc(var(--spacing) * 7)` |
| `py-8` | `padding-top: calc(var(--spacing) * 8); padding-bottom: calc(var(--spacing) * 8)` |
| `py-9` | `padding-top: calc(var(--spacing) * 9); padding-bottom: calc(var(--spacing) * 9)` |
| `py-10` | `padding-top: calc(var(--spacing) * 10); padding-bottom: calc(var(--spacing) * 10)` |
| `py-11` | `padding-top: calc(var(--spacing) * 11); padding-bottom: calc(var(--spacing) * 11)` |
| `py-12` | `padding-top: calc(var(--spacing) * 12); padding-bottom: calc(var(--spacing) * 12)` |
| `py-14` | `padding-top: calc(var(--spacing) * 14); padding-bottom: calc(var(--spacing) * 14)` |
| `py-16` | `padding-top: calc(var(--spacing) * 16); padding-bottom: calc(var(--spacing) * 16)` |
| `py-20` | `padding-top: calc(var(--spacing) * 20); padding-bottom: calc(var(--spacing) * 20)` |
| `py-24` | `padding-top: calc(var(--spacing) * 24); padding-bottom: calc(var(--spacing) * 24)` |
| `py-28` | `padding-top: calc(var(--spacing) * 28); padding-bottom: calc(var(--spacing) * 28)` |
| `py-32` | `padding-top: calc(var(--spacing) * 32); padding-bottom: calc(var(--spacing) * 32)` |
| `py-36` | `padding-top: calc(var(--spacing) * 36); padding-bottom: calc(var(--spacing) * 36)` |
| `py-40` | `padding-top: calc(var(--spacing) * 40); padding-bottom: calc(var(--spacing) * 40)` |
| `py-44` | `padding-top: calc(var(--spacing) * 44); padding-bottom: calc(var(--spacing) * 44)` |
| `py-48` | `padding-top: calc(var(--spacing) * 48); padding-bottom: calc(var(--spacing) * 48)` |
| `py-52` | `padding-top: calc(var(--spacing) * 52); padding-bottom: calc(var(--spacing) * 52)` |
| `py-56` | `padding-top: calc(var(--spacing) * 56); padding-bottom: calc(var(--spacing) * 56)` |
| `py-60` | `padding-top: calc(var(--spacing) * 60); padding-bottom: calc(var(--spacing) * 60)` |
| `py-64` | `padding-top: calc(var(--spacing) * 64); padding-bottom: calc(var(--spacing) * 64)` |
| `py-72` | `padding-top: calc(var(--spacing) * 72); padding-bottom: calc(var(--spacing) * 72)` |
| `py-80` | `padding-top: calc(var(--spacing) * 80); padding-bottom: calc(var(--spacing) * 80)` |
| `py-96` | `padding-top: calc(var(--spacing) * 96); padding-bottom: calc(var(--spacing) * 96)` |
| `pt-0` | `padding-top: 0px` |
| `pt-px` | `padding-top: 1px` |
| `pt-0.5` | `padding-top: calc(var(--spacing) * 0.5)` |
| `pt-1` | `padding-top: calc(var(--spacing) * 1)` |
| `pt-1.5` | `padding-top: calc(var(--spacing) * 1.5)` |
| `pt-2` | `padding-top: calc(var(--spacing) * 2)` |
| `pt-2.5` | `padding-top: calc(var(--spacing) * 2.5)` |
| `pt-3` | `padding-top: calc(var(--spacing) * 3)` |
| `pt-3.5` | `padding-top: calc(var(--spacing) * 3.5)` |
| `pt-4` | `padding-top: calc(var(--spacing) * 4)` |
| `pt-5` | `padding-top: calc(var(--spacing) * 5)` |
| `pt-6` | `padding-top: calc(var(--spacing) * 6)` |
| `pt-7` | `padding-top: calc(var(--spacing) * 7)` |
| `pt-8` | `padding-top: calc(var(--spacing) * 8)` |
| `pt-9` | `padding-top: calc(var(--spacing) * 9)` |
| `pt-10` | `padding-top: calc(var(--spacing) * 10)` |
| `pt-11` | `padding-top: calc(var(--spacing) * 11)` |
| `pt-12` | `padding-top: calc(var(--spacing) * 12)` |
| `pt-14` | `padding-top: calc(var(--spacing) * 14)` |
| `pt-16` | `padding-top: calc(var(--spacing) * 16)` |
| `pt-20` | `padding-top: calc(var(--spacing) * 20)` |
| `pt-24` | `padding-top: calc(var(--spacing) * 24)` |
| `pt-28` | `padding-top: calc(var(--spacing) * 28)` |
| `pt-32` | `padding-top: calc(var(--spacing) * 32)` |
| `pt-36` | `padding-top: calc(var(--spacing) * 36)` |
| `pt-40` | `padding-top: calc(var(--spacing) * 40)` |
| `pt-44` | `padding-top: calc(var(--spacing) * 44)` |
| `pt-48` | `padding-top: calc(var(--spacing) * 48)` |
| `pt-52` | `padding-top: calc(var(--spacing) * 52)` |
| `pt-56` | `padding-top: calc(var(--spacing) * 56)` |
| `pt-60` | `padding-top: calc(var(--spacing) * 60)` |
| `pt-64` | `padding-top: calc(var(--spacing) * 64)` |
| `pt-72` | `padding-top: calc(var(--spacing) * 72)` |
| `pt-80` | `padding-top: calc(var(--spacing) * 80)` |
| `pt-96` | `padding-top: calc(var(--spacing) * 96)` |
| `pr-0` | `padding-right: 0px` |
| `pr-px` | `padding-right: 1px` |
| `pr-0.5` | `padding-right: calc(var(--spacing) * 0.5)` |
| `pr-1` | `padding-right: calc(var(--spacing) * 1)` |
| `pr-1.5` | `padding-right: calc(var(--spacing) * 1.5)` |
| `pr-2` | `padding-right: calc(var(--spacing) * 2)` |
| `pr-2.5` | `padding-right: calc(var(--spacing) * 2.5)` |
| `pr-3` | `padding-right: calc(var(--spacing) * 3)` |
| `pr-3.5` | `padding-right: calc(var(--spacing) * 3.5)` |
| `pr-4` | `padding-right: calc(var(--spacing) * 4)` |
| `pr-5` | `padding-right: calc(var(--spacing) * 5)` |
| `pr-6` | `padding-right: calc(var(--spacing) * 6)` |
| `pr-7` | `padding-right: calc(var(--spacing) * 7)` |
| `pr-8` | `padding-right: calc(var(--spacing) * 8)` |
| `pr-9` | `padding-right: calc(var(--spacing) * 9)` |
| `pr-10` | `padding-right: calc(var(--spacing) * 10)` |
| `pr-11` | `padding-right: calc(var(--spacing) * 11)` |
| `pr-12` | `padding-right: calc(var(--spacing) * 12)` |
| `pr-14` | `padding-right: calc(var(--spacing) * 14)` |
| `pr-16` | `padding-right: calc(var(--spacing) * 16)` |
| `pr-20` | `padding-right: calc(var(--spacing) * 20)` |
| `pr-24` | `padding-right: calc(var(--spacing) * 24)` |
| `pr-28` | `padding-right: calc(var(--spacing) * 28)` |
| `pr-32` | `padding-right: calc(var(--spacing) * 32)` |
| `pr-36` | `padding-right: calc(var(--spacing) * 36)` |
| `pr-40` | `padding-right: calc(var(--spacing) * 40)` |
| `pr-44` | `padding-right: calc(var(--spacing) * 44)` |
| `pr-48` | `padding-right: calc(var(--spacing) * 48)` |
| `pr-52` | `padding-right: calc(var(--spacing) * 52)` |
| `pr-56` | `padding-right: calc(var(--spacing) * 56)` |
| `pr-60` | `padding-right: calc(var(--spacing) * 60)` |
| `pr-64` | `padding-right: calc(var(--spacing) * 64)` |
| `pr-72` | `padding-right: calc(var(--spacing) * 72)` |
| `pr-80` | `padding-right: calc(var(--spacing) * 80)` |
| `pr-96` | `padding-right: calc(var(--spacing) * 96)` |
| `pb-0` | `padding-bottom: 0px` |
| `pb-px` | `padding-bottom: 1px` |
| `pb-0.5` | `padding-bottom: calc(var(--spacing) * 0.5)` |
| `pb-1` | `padding-bottom: calc(var(--spacing) * 1)` |
| `pb-1.5` | `padding-bottom: calc(var(--spacing) * 1.5)` |
| `pb-2` | `padding-bottom: calc(var(--spacing) * 2)` |
| `pb-2.5` | `padding-bottom: calc(var(--spacing) * 2.5)` |
| `pb-3` | `padding-bottom: calc(var(--spacing) * 3)` |
| `pb-3.5` | `padding-bottom: calc(var(--spacing) * 3.5)` |
| `pb-4` | `padding-bottom: calc(var(--spacing) * 4)` |
| `pb-5` | `padding-bottom: calc(var(--spacing) * 5)` |
| `pb-6` | `padding-bottom: calc(var(--spacing) * 6)` |
| `pb-7` | `padding-bottom: calc(var(--spacing) * 7)` |
| `pb-8` | `padding-bottom: calc(var(--spacing) * 8)` |
| `pb-9` | `padding-bottom: calc(var(--spacing) * 9)` |
| `pb-10` | `padding-bottom: calc(var(--spacing) * 10)` |
| `pb-11` | `padding-bottom: calc(var(--spacing) * 11)` |
| `pb-12` | `padding-bottom: calc(var(--spacing) * 12)` |
| `pb-14` | `padding-bottom: calc(var(--spacing) * 14)` |
| `pb-16` | `padding-bottom: calc(var(--spacing) * 16)` |
| `pb-20` | `padding-bottom: calc(var(--spacing) * 20)` |
| `pb-24` | `padding-bottom: calc(var(--spacing) * 24)` |
| `pb-28` | `padding-bottom: calc(var(--spacing) * 28)` |
| `pb-32` | `padding-bottom: calc(var(--spacing) * 32)` |
| `pb-36` | `padding-bottom: calc(var(--spacing) * 36)` |
| `pb-40` | `padding-bottom: calc(var(--spacing) * 40)` |
| `pb-44` | `padding-bottom: calc(var(--spacing) * 44)` |
| `pb-48` | `padding-bottom: calc(var(--spacing) * 48)` |
| `pb-52` | `padding-bottom: calc(var(--spacing) * 52)` |
| `pb-56` | `padding-bottom: calc(var(--spacing) * 56)` |
| `pb-60` | `padding-bottom: calc(var(--spacing) * 60)` |
| `pb-64` | `padding-bottom: calc(var(--spacing) * 64)` |
| `pb-72` | `padding-bottom: calc(var(--spacing) * 72)` |
| `pb-80` | `padding-bottom: calc(var(--spacing) * 80)` |
| `pb-96` | `padding-bottom: calc(var(--spacing) * 96)` |
| `pl-0` | `padding-left: 0px` |
| `pl-px` | `padding-left: 1px` |
| `pl-0.5` | `padding-left: calc(var(--spacing) * 0.5)` |
| `pl-1` | `padding-left: calc(var(--spacing) * 1)` |
| `pl-1.5` | `padding-left: calc(var(--spacing) * 1.5)` |
| `pl-2` | `padding-left: calc(var(--spacing) * 2)` |
| `pl-2.5` | `padding-left: calc(var(--spacing) * 2.5)` |
| `pl-3` | `padding-left: calc(var(--spacing) * 3)` |
| `pl-3.5` | `padding-left: calc(var(--spacing) * 3.5)` |
| `pl-4` | `padding-left: calc(var(--spacing) * 4)` |
| `pl-5` | `padding-left: calc(var(--spacing) * 5)` |
| `pl-6` | `padding-left: calc(var(--spacing) * 6)` |
| `pl-7` | `padding-left: calc(var(--spacing) * 7)` |
| `pl-8` | `padding-left: calc(var(--spacing) * 8)` |
| `pl-9` | `padding-left: calc(var(--spacing) * 9)` |
| `pl-10` | `padding-left: calc(var(--spacing) * 10)` |
| `pl-11` | `padding-left: calc(var(--spacing) * 11)` |
| `pl-12` | `padding-left: calc(var(--spacing) * 12)` |
| `pl-14` | `padding-left: calc(var(--spacing) * 14)` |
| `pl-16` | `padding-left: calc(var(--spacing) * 16)` |
| `pl-20` | `padding-left: calc(var(--spacing) * 20)` |
| `pl-24` | `padding-left: calc(var(--spacing) * 24)` |
| `pl-28` | `padding-left: calc(var(--spacing) * 28)` |
| `pl-32` | `padding-left: calc(var(--spacing) * 32)` |
| `pl-36` | `padding-left: calc(var(--spacing) * 36)` |
| `pl-40` | `padding-left: calc(var(--spacing) * 40)` |
| `pl-44` | `padding-left: calc(var(--spacing) * 44)` |
| `pl-48` | `padding-left: calc(var(--spacing) * 48)` |
| `pl-52` | `padding-left: calc(var(--spacing) * 52)` |
| `pl-56` | `padding-left: calc(var(--spacing) * 56)` |
| `pl-60` | `padding-left: calc(var(--spacing) * 60)` |
| `pl-64` | `padding-left: calc(var(--spacing) * 64)` |
| `pl-72` | `padding-left: calc(var(--spacing) * 72)` |
| `pl-80` | `padding-left: calc(var(--spacing) * 80)` |
| `pl-96` | `padding-left: calc(var(--spacing) * 96)` |
| `ps-0` | `padding-inline-start: 0px` |
| `ps-px` | `padding-inline-start: 1px` |
| `ps-0.5` | `padding-inline-start: calc(var(--spacing) * 0.5)` |
| `ps-1` | `padding-inline-start: calc(var(--spacing) * 1)` |
| `ps-1.5` | `padding-inline-start: calc(var(--spacing) * 1.5)` |
| `ps-2` | `padding-inline-start: calc(var(--spacing) * 2)` |
| `ps-2.5` | `padding-inline-start: calc(var(--spacing) * 2.5)` |
| `ps-3` | `padding-inline-start: calc(var(--spacing) * 3)` |
| `ps-3.5` | `padding-inline-start: calc(var(--spacing) * 3.5)` |
| `ps-4` | `padding-inline-start: calc(var(--spacing) * 4)` |
| `ps-5` | `padding-inline-start: calc(var(--spacing) * 5)` |
| `ps-6` | `padding-inline-start: calc(var(--spacing) * 6)` |
| `ps-7` | `padding-inline-start: calc(var(--spacing) * 7)` |
| `ps-8` | `padding-inline-start: calc(var(--spacing) * 8)` |
| `ps-9` | `padding-inline-start: calc(var(--spacing) * 9)` |
| `ps-10` | `padding-inline-start: calc(var(--spacing) * 10)` |
| `ps-11` | `padding-inline-start: calc(var(--spacing) * 11)` |
| `ps-12` | `padding-inline-start: calc(var(--spacing) * 12)` |
| `ps-14` | `padding-inline-start: calc(var(--spacing) * 14)` |
| `ps-16` | `padding-inline-start: calc(var(--spacing) * 16)` |
| `ps-20` | `padding-inline-start: calc(var(--spacing) * 20)` |
| `ps-24` | `padding-inline-start: calc(var(--spacing) * 24)` |
| `ps-28` | `padding-inline-start: calc(var(--spacing) * 28)` |
| `ps-32` | `padding-inline-start: calc(var(--spacing) * 32)` |
| `ps-36` | `padding-inline-start: calc(var(--spacing) * 36)` |
| `ps-40` | `padding-inline-start: calc(var(--spacing) * 40)` |
| `ps-44` | `padding-inline-start: calc(var(--spacing) * 44)` |
| `ps-48` | `padding-inline-start: calc(var(--spacing) * 48)` |
| `ps-52` | `padding-inline-start: calc(var(--spacing) * 52)` |
| `ps-56` | `padding-inline-start: calc(var(--spacing) * 56)` |
| `ps-60` | `padding-inline-start: calc(var(--spacing) * 60)` |
| `ps-64` | `padding-inline-start: calc(var(--spacing) * 64)` |
| `ps-72` | `padding-inline-start: calc(var(--spacing) * 72)` |
| `ps-80` | `padding-inline-start: calc(var(--spacing) * 80)` |
| `ps-96` | `padding-inline-start: calc(var(--spacing) * 96)` |
| `pe-0` | `padding-inline-end: 0px` |
| `pe-px` | `padding-inline-end: 1px` |
| `pe-0.5` | `padding-inline-end: calc(var(--spacing) * 0.5)` |
| `pe-1` | `padding-inline-end: calc(var(--spacing) * 1)` |
| `pe-1.5` | `padding-inline-end: calc(var(--spacing) * 1.5)` |
| `pe-2` | `padding-inline-end: calc(var(--spacing) * 2)` |
| `pe-2.5` | `padding-inline-end: calc(var(--spacing) * 2.5)` |
| `pe-3` | `padding-inline-end: calc(var(--spacing) * 3)` |
| `pe-3.5` | `padding-inline-end: calc(var(--spacing) * 3.5)` |
| `pe-4` | `padding-inline-end: calc(var(--spacing) * 4)` |
| `pe-5` | `padding-inline-end: calc(var(--spacing) * 5)` |
| `pe-6` | `padding-inline-end: calc(var(--spacing) * 6)` |
| `pe-7` | `padding-inline-end: calc(var(--spacing) * 7)` |
| `pe-8` | `padding-inline-end: calc(var(--spacing) * 8)` |
| `pe-9` | `padding-inline-end: calc(var(--spacing) * 9)` |
| `pe-10` | `padding-inline-end: calc(var(--spacing) * 10)` |
| `pe-11` | `padding-inline-end: calc(var(--spacing) * 11)` |
| `pe-12` | `padding-inline-end: calc(var(--spacing) * 12)` |
| `pe-14` | `padding-inline-end: calc(var(--spacing) * 14)` |
| `pe-16` | `padding-inline-end: calc(var(--spacing) * 16)` |
| `pe-20` | `padding-inline-end: calc(var(--spacing) * 20)` |
| `pe-24` | `padding-inline-end: calc(var(--spacing) * 24)` |
| `pe-28` | `padding-inline-end: calc(var(--spacing) * 28)` |
| `pe-32` | `padding-inline-end: calc(var(--spacing) * 32)` |
| `pe-36` | `padding-inline-end: calc(var(--spacing) * 36)` |
| `pe-40` | `padding-inline-end: calc(var(--spacing) * 40)` |
| `pe-44` | `padding-inline-end: calc(var(--spacing) * 44)` |
| `pe-48` | `padding-inline-end: calc(var(--spacing) * 48)` |
| `pe-52` | `padding-inline-end: calc(var(--spacing) * 52)` |
| `pe-56` | `padding-inline-end: calc(var(--spacing) * 56)` |
| `pe-60` | `padding-inline-end: calc(var(--spacing) * 60)` |
| `pe-64` | `padding-inline-end: calc(var(--spacing) * 64)` |
| `pe-72` | `padding-inline-end: calc(var(--spacing) * 72)` |
| `pe-80` | `padding-inline-end: calc(var(--spacing) * 80)` |
| `pe-96` | `padding-inline-end: calc(var(--spacing) * 96)` |

## Spacing — Margin

| Classe | CSS |
|--------|-----|
| `m-0` | `margin: 0px` |
| `m-px` | `margin: 1px` |
| `m-0.5` | `margin: calc(var(--spacing) * 0.5)` |
| `m-1` | `margin: calc(var(--spacing) * 1)` |
| `m-1.5` | `margin: calc(var(--spacing) * 1.5)` |
| `m-2` | `margin: calc(var(--spacing) * 2)` |
| `m-2.5` | `margin: calc(var(--spacing) * 2.5)` |
| `m-3` | `margin: calc(var(--spacing) * 3)` |
| `m-3.5` | `margin: calc(var(--spacing) * 3.5)` |
| `m-4` | `margin: calc(var(--spacing) * 4)` |
| `m-5` | `margin: calc(var(--spacing) * 5)` |
| `m-6` | `margin: calc(var(--spacing) * 6)` |
| `m-7` | `margin: calc(var(--spacing) * 7)` |
| `m-8` | `margin: calc(var(--spacing) * 8)` |
| `m-9` | `margin: calc(var(--spacing) * 9)` |
| `m-10` | `margin: calc(var(--spacing) * 10)` |
| `m-11` | `margin: calc(var(--spacing) * 11)` |
| `m-12` | `margin: calc(var(--spacing) * 12)` |
| `m-14` | `margin: calc(var(--spacing) * 14)` |
| `m-16` | `margin: calc(var(--spacing) * 16)` |
| `m-20` | `margin: calc(var(--spacing) * 20)` |
| `m-24` | `margin: calc(var(--spacing) * 24)` |
| `m-28` | `margin: calc(var(--spacing) * 28)` |
| `m-32` | `margin: calc(var(--spacing) * 32)` |
| `m-36` | `margin: calc(var(--spacing) * 36)` |
| `m-40` | `margin: calc(var(--spacing) * 40)` |
| `m-44` | `margin: calc(var(--spacing) * 44)` |
| `m-48` | `margin: calc(var(--spacing) * 48)` |
| `m-52` | `margin: calc(var(--spacing) * 52)` |
| `m-56` | `margin: calc(var(--spacing) * 56)` |
| `m-60` | `margin: calc(var(--spacing) * 60)` |
| `m-64` | `margin: calc(var(--spacing) * 64)` |
| `m-72` | `margin: calc(var(--spacing) * 72)` |
| `m-80` | `margin: calc(var(--spacing) * 80)` |
| `m-96` | `margin: calc(var(--spacing) * 96)` |
| `mx-0` | `margin-left: 0px; margin-right: 0px` |
| `mx-px` | `margin-left: 1px; margin-right: 1px` |
| `mx-0.5` | `margin-left: calc(var(--spacing) * 0.5); margin-right: calc(var(--spacing) * 0.5)` |
| `mx-1` | `margin-left: calc(var(--spacing) * 1); margin-right: calc(var(--spacing) * 1)` |
| `mx-1.5` | `margin-left: calc(var(--spacing) * 1.5); margin-right: calc(var(--spacing) * 1.5)` |
| `mx-2` | `margin-left: calc(var(--spacing) * 2); margin-right: calc(var(--spacing) * 2)` |
| `mx-2.5` | `margin-left: calc(var(--spacing) * 2.5); margin-right: calc(var(--spacing) * 2.5)` |
| `mx-3` | `margin-left: calc(var(--spacing) * 3); margin-right: calc(var(--spacing) * 3)` |
| `mx-3.5` | `margin-left: calc(var(--spacing) * 3.5); margin-right: calc(var(--spacing) * 3.5)` |
| `mx-4` | `margin-left: calc(var(--spacing) * 4); margin-right: calc(var(--spacing) * 4)` |
| `mx-5` | `margin-left: calc(var(--spacing) * 5); margin-right: calc(var(--spacing) * 5)` |
| `mx-6` | `margin-left: calc(var(--spacing) * 6); margin-right: calc(var(--spacing) * 6)` |
| `mx-7` | `margin-left: calc(var(--spacing) * 7); margin-right: calc(var(--spacing) * 7)` |
| `mx-8` | `margin-left: calc(var(--spacing) * 8); margin-right: calc(var(--spacing) * 8)` |
| `mx-9` | `margin-left: calc(var(--spacing) * 9); margin-right: calc(var(--spacing) * 9)` |
| `mx-10` | `margin-left: calc(var(--spacing) * 10); margin-right: calc(var(--spacing) * 10)` |
| `mx-11` | `margin-left: calc(var(--spacing) * 11); margin-right: calc(var(--spacing) * 11)` |
| `mx-12` | `margin-left: calc(var(--spacing) * 12); margin-right: calc(var(--spacing) * 12)` |
| `mx-14` | `margin-left: calc(var(--spacing) * 14); margin-right: calc(var(--spacing) * 14)` |
| `mx-16` | `margin-left: calc(var(--spacing) * 16); margin-right: calc(var(--spacing) * 16)` |
| `mx-20` | `margin-left: calc(var(--spacing) * 20); margin-right: calc(var(--spacing) * 20)` |
| `mx-24` | `margin-left: calc(var(--spacing) * 24); margin-right: calc(var(--spacing) * 24)` |
| `mx-28` | `margin-left: calc(var(--spacing) * 28); margin-right: calc(var(--spacing) * 28)` |
| `mx-32` | `margin-left: calc(var(--spacing) * 32); margin-right: calc(var(--spacing) * 32)` |
| `mx-36` | `margin-left: calc(var(--spacing) * 36); margin-right: calc(var(--spacing) * 36)` |
| `mx-40` | `margin-left: calc(var(--spacing) * 40); margin-right: calc(var(--spacing) * 40)` |
| `mx-44` | `margin-left: calc(var(--spacing) * 44); margin-right: calc(var(--spacing) * 44)` |
| `mx-48` | `margin-left: calc(var(--spacing) * 48); margin-right: calc(var(--spacing) * 48)` |
| `mx-52` | `margin-left: calc(var(--spacing) * 52); margin-right: calc(var(--spacing) * 52)` |
| `mx-56` | `margin-left: calc(var(--spacing) * 56); margin-right: calc(var(--spacing) * 56)` |
| `mx-60` | `margin-left: calc(var(--spacing) * 60); margin-right: calc(var(--spacing) * 60)` |
| `mx-64` | `margin-left: calc(var(--spacing) * 64); margin-right: calc(var(--spacing) * 64)` |
| `mx-72` | `margin-left: calc(var(--spacing) * 72); margin-right: calc(var(--spacing) * 72)` |
| `mx-80` | `margin-left: calc(var(--spacing) * 80); margin-right: calc(var(--spacing) * 80)` |
| `mx-96` | `margin-left: calc(var(--spacing) * 96); margin-right: calc(var(--spacing) * 96)` |
| `my-0` | `margin-top: 0px; margin-bottom: 0px` |
| `my-px` | `margin-top: 1px; margin-bottom: 1px` |
| `my-0.5` | `margin-top: calc(var(--spacing) * 0.5); margin-bottom: calc(var(--spacing) * 0.5)` |
| `my-1` | `margin-top: calc(var(--spacing) * 1); margin-bottom: calc(var(--spacing) * 1)` |
| `my-1.5` | `margin-top: calc(var(--spacing) * 1.5); margin-bottom: calc(var(--spacing) * 1.5)` |
| `my-2` | `margin-top: calc(var(--spacing) * 2); margin-bottom: calc(var(--spacing) * 2)` |
| `my-2.5` | `margin-top: calc(var(--spacing) * 2.5); margin-bottom: calc(var(--spacing) * 2.5)` |
| `my-3` | `margin-top: calc(var(--spacing) * 3); margin-bottom: calc(var(--spacing) * 3)` |
| `my-3.5` | `margin-top: calc(var(--spacing) * 3.5); margin-bottom: calc(var(--spacing) * 3.5)` |
| `my-4` | `margin-top: calc(var(--spacing) * 4); margin-bottom: calc(var(--spacing) * 4)` |
| `my-5` | `margin-top: calc(var(--spacing) * 5); margin-bottom: calc(var(--spacing) * 5)` |
| `my-6` | `margin-top: calc(var(--spacing) * 6); margin-bottom: calc(var(--spacing) * 6)` |
| `my-7` | `margin-top: calc(var(--spacing) * 7); margin-bottom: calc(var(--spacing) * 7)` |
| `my-8` | `margin-top: calc(var(--spacing) * 8); margin-bottom: calc(var(--spacing) * 8)` |
| `my-9` | `margin-top: calc(var(--spacing) * 9); margin-bottom: calc(var(--spacing) * 9)` |
| `my-10` | `margin-top: calc(var(--spacing) * 10); margin-bottom: calc(var(--spacing) * 10)` |
| `my-11` | `margin-top: calc(var(--spacing) * 11); margin-bottom: calc(var(--spacing) * 11)` |
| `my-12` | `margin-top: calc(var(--spacing) * 12); margin-bottom: calc(var(--spacing) * 12)` |
| `my-14` | `margin-top: calc(var(--spacing) * 14); margin-bottom: calc(var(--spacing) * 14)` |
| `my-16` | `margin-top: calc(var(--spacing) * 16); margin-bottom: calc(var(--spacing) * 16)` |
| `my-20` | `margin-top: calc(var(--spacing) * 20); margin-bottom: calc(var(--spacing) * 20)` |
| `my-24` | `margin-top: calc(var(--spacing) * 24); margin-bottom: calc(var(--spacing) * 24)` |
| `my-28` | `margin-top: calc(var(--spacing) * 28); margin-bottom: calc(var(--spacing) * 28)` |
| `my-32` | `margin-top: calc(var(--spacing) * 32); margin-bottom: calc(var(--spacing) * 32)` |
| `my-36` | `margin-top: calc(var(--spacing) * 36); margin-bottom: calc(var(--spacing) * 36)` |
| `my-40` | `margin-top: calc(var(--spacing) * 40); margin-bottom: calc(var(--spacing) * 40)` |
| `my-44` | `margin-top: calc(var(--spacing) * 44); margin-bottom: calc(var(--spacing) * 44)` |
| `my-48` | `margin-top: calc(var(--spacing) * 48); margin-bottom: calc(var(--spacing) * 48)` |
| `my-52` | `margin-top: calc(var(--spacing) * 52); margin-bottom: calc(var(--spacing) * 52)` |
| `my-56` | `margin-top: calc(var(--spacing) * 56); margin-bottom: calc(var(--spacing) * 56)` |
| `my-60` | `margin-top: calc(var(--spacing) * 60); margin-bottom: calc(var(--spacing) * 60)` |
| `my-64` | `margin-top: calc(var(--spacing) * 64); margin-bottom: calc(var(--spacing) * 64)` |
| `my-72` | `margin-top: calc(var(--spacing) * 72); margin-bottom: calc(var(--spacing) * 72)` |
| `my-80` | `margin-top: calc(var(--spacing) * 80); margin-bottom: calc(var(--spacing) * 80)` |
| `my-96` | `margin-top: calc(var(--spacing) * 96); margin-bottom: calc(var(--spacing) * 96)` |
| `mt-0` | `margin-top: 0px` |
| `mt-px` | `margin-top: 1px` |
| `mt-0.5` | `margin-top: calc(var(--spacing) * 0.5)` |
| `mt-1` | `margin-top: calc(var(--spacing) * 1)` |
| `mt-1.5` | `margin-top: calc(var(--spacing) * 1.5)` |
| `mt-2` | `margin-top: calc(var(--spacing) * 2)` |
| `mt-2.5` | `margin-top: calc(var(--spacing) * 2.5)` |
| `mt-3` | `margin-top: calc(var(--spacing) * 3)` |
| `mt-3.5` | `margin-top: calc(var(--spacing) * 3.5)` |
| `mt-4` | `margin-top: calc(var(--spacing) * 4)` |
| `mt-5` | `margin-top: calc(var(--spacing) * 5)` |
| `mt-6` | `margin-top: calc(var(--spacing) * 6)` |
| `mt-7` | `margin-top: calc(var(--spacing) * 7)` |
| `mt-8` | `margin-top: calc(var(--spacing) * 8)` |
| `mt-9` | `margin-top: calc(var(--spacing) * 9)` |
| `mt-10` | `margin-top: calc(var(--spacing) * 10)` |
| `mt-11` | `margin-top: calc(var(--spacing) * 11)` |
| `mt-12` | `margin-top: calc(var(--spacing) * 12)` |
| `mt-14` | `margin-top: calc(var(--spacing) * 14)` |
| `mt-16` | `margin-top: calc(var(--spacing) * 16)` |
| `mt-20` | `margin-top: calc(var(--spacing) * 20)` |
| `mt-24` | `margin-top: calc(var(--spacing) * 24)` |
| `mt-28` | `margin-top: calc(var(--spacing) * 28)` |
| `mt-32` | `margin-top: calc(var(--spacing) * 32)` |
| `mt-36` | `margin-top: calc(var(--spacing) * 36)` |
| `mt-40` | `margin-top: calc(var(--spacing) * 40)` |
| `mt-44` | `margin-top: calc(var(--spacing) * 44)` |
| `mt-48` | `margin-top: calc(var(--spacing) * 48)` |
| `mt-52` | `margin-top: calc(var(--spacing) * 52)` |
| `mt-56` | `margin-top: calc(var(--spacing) * 56)` |
| `mt-60` | `margin-top: calc(var(--spacing) * 60)` |
| `mt-64` | `margin-top: calc(var(--spacing) * 64)` |
| `mt-72` | `margin-top: calc(var(--spacing) * 72)` |
| `mt-80` | `margin-top: calc(var(--spacing) * 80)` |
| `mt-96` | `margin-top: calc(var(--spacing) * 96)` |
| `mr-0` | `margin-right: 0px` |
| `mr-px` | `margin-right: 1px` |
| `mr-0.5` | `margin-right: calc(var(--spacing) * 0.5)` |
| `mr-1` | `margin-right: calc(var(--spacing) * 1)` |
| `mr-1.5` | `margin-right: calc(var(--spacing) * 1.5)` |
| `mr-2` | `margin-right: calc(var(--spacing) * 2)` |
| `mr-2.5` | `margin-right: calc(var(--spacing) * 2.5)` |
| `mr-3` | `margin-right: calc(var(--spacing) * 3)` |
| `mr-3.5` | `margin-right: calc(var(--spacing) * 3.5)` |
| `mr-4` | `margin-right: calc(var(--spacing) * 4)` |
| `mr-5` | `margin-right: calc(var(--spacing) * 5)` |
| `mr-6` | `margin-right: calc(var(--spacing) * 6)` |
| `mr-7` | `margin-right: calc(var(--spacing) * 7)` |
| `mr-8` | `margin-right: calc(var(--spacing) * 8)` |
| `mr-9` | `margin-right: calc(var(--spacing) * 9)` |
| `mr-10` | `margin-right: calc(var(--spacing) * 10)` |
| `mr-11` | `margin-right: calc(var(--spacing) * 11)` |
| `mr-12` | `margin-right: calc(var(--spacing) * 12)` |
| `mr-14` | `margin-right: calc(var(--spacing) * 14)` |
| `mr-16` | `margin-right: calc(var(--spacing) * 16)` |
| `mr-20` | `margin-right: calc(var(--spacing) * 20)` |
| `mr-24` | `margin-right: calc(var(--spacing) * 24)` |
| `mr-28` | `margin-right: calc(var(--spacing) * 28)` |
| `mr-32` | `margin-right: calc(var(--spacing) * 32)` |
| `mr-36` | `margin-right: calc(var(--spacing) * 36)` |
| `mr-40` | `margin-right: calc(var(--spacing) * 40)` |
| `mr-44` | `margin-right: calc(var(--spacing) * 44)` |
| `mr-48` | `margin-right: calc(var(--spacing) * 48)` |
| `mr-52` | `margin-right: calc(var(--spacing) * 52)` |
| `mr-56` | `margin-right: calc(var(--spacing) * 56)` |
| `mr-60` | `margin-right: calc(var(--spacing) * 60)` |
| `mr-64` | `margin-right: calc(var(--spacing) * 64)` |
| `mr-72` | `margin-right: calc(var(--spacing) * 72)` |
| `mr-80` | `margin-right: calc(var(--spacing) * 80)` |
| `mr-96` | `margin-right: calc(var(--spacing) * 96)` |
| `mb-0` | `margin-bottom: 0px` |
| `mb-px` | `margin-bottom: 1px` |
| `mb-0.5` | `margin-bottom: calc(var(--spacing) * 0.5)` |
| `mb-1` | `margin-bottom: calc(var(--spacing) * 1)` |
| `mb-1.5` | `margin-bottom: calc(var(--spacing) * 1.5)` |
| `mb-2` | `margin-bottom: calc(var(--spacing) * 2)` |
| `mb-2.5` | `margin-bottom: calc(var(--spacing) * 2.5)` |
| `mb-3` | `margin-bottom: calc(var(--spacing) * 3)` |
| `mb-3.5` | `margin-bottom: calc(var(--spacing) * 3.5)` |
| `mb-4` | `margin-bottom: calc(var(--spacing) * 4)` |
| `mb-5` | `margin-bottom: calc(var(--spacing) * 5)` |
| `mb-6` | `margin-bottom: calc(var(--spacing) * 6)` |
| `mb-7` | `margin-bottom: calc(var(--spacing) * 7)` |
| `mb-8` | `margin-bottom: calc(var(--spacing) * 8)` |
| `mb-9` | `margin-bottom: calc(var(--spacing) * 9)` |
| `mb-10` | `margin-bottom: calc(var(--spacing) * 10)` |
| `mb-11` | `margin-bottom: calc(var(--spacing) * 11)` |
| `mb-12` | `margin-bottom: calc(var(--spacing) * 12)` |
| `mb-14` | `margin-bottom: calc(var(--spacing) * 14)` |
| `mb-16` | `margin-bottom: calc(var(--spacing) * 16)` |
| `mb-20` | `margin-bottom: calc(var(--spacing) * 20)` |
| `mb-24` | `margin-bottom: calc(var(--spacing) * 24)` |
| `mb-28` | `margin-bottom: calc(var(--spacing) * 28)` |
| `mb-32` | `margin-bottom: calc(var(--spacing) * 32)` |
| `mb-36` | `margin-bottom: calc(var(--spacing) * 36)` |
| `mb-40` | `margin-bottom: calc(var(--spacing) * 40)` |
| `mb-44` | `margin-bottom: calc(var(--spacing) * 44)` |
| `mb-48` | `margin-bottom: calc(var(--spacing) * 48)` |
| `mb-52` | `margin-bottom: calc(var(--spacing) * 52)` |
| `mb-56` | `margin-bottom: calc(var(--spacing) * 56)` |
| `mb-60` | `margin-bottom: calc(var(--spacing) * 60)` |
| `mb-64` | `margin-bottom: calc(var(--spacing) * 64)` |
| `mb-72` | `margin-bottom: calc(var(--spacing) * 72)` |
| `mb-80` | `margin-bottom: calc(var(--spacing) * 80)` |
| `mb-96` | `margin-bottom: calc(var(--spacing) * 96)` |
| `ml-0` | `margin-left: 0px` |
| `ml-px` | `margin-left: 1px` |
| `ml-0.5` | `margin-left: calc(var(--spacing) * 0.5)` |
| `ml-1` | `margin-left: calc(var(--spacing) * 1)` |
| `ml-1.5` | `margin-left: calc(var(--spacing) * 1.5)` |
| `ml-2` | `margin-left: calc(var(--spacing) * 2)` |
| `ml-2.5` | `margin-left: calc(var(--spacing) * 2.5)` |
| `ml-3` | `margin-left: calc(var(--spacing) * 3)` |
| `ml-3.5` | `margin-left: calc(var(--spacing) * 3.5)` |
| `ml-4` | `margin-left: calc(var(--spacing) * 4)` |
| `ml-5` | `margin-left: calc(var(--spacing) * 5)` |
| `ml-6` | `margin-left: calc(var(--spacing) * 6)` |
| `ml-7` | `margin-left: calc(var(--spacing) * 7)` |
| `ml-8` | `margin-left: calc(var(--spacing) * 8)` |
| `ml-9` | `margin-left: calc(var(--spacing) * 9)` |
| `ml-10` | `margin-left: calc(var(--spacing) * 10)` |
| `ml-11` | `margin-left: calc(var(--spacing) * 11)` |
| `ml-12` | `margin-left: calc(var(--spacing) * 12)` |
| `ml-14` | `margin-left: calc(var(--spacing) * 14)` |
| `ml-16` | `margin-left: calc(var(--spacing) * 16)` |
| `ml-20` | `margin-left: calc(var(--spacing) * 20)` |
| `ml-24` | `margin-left: calc(var(--spacing) * 24)` |
| `ml-28` | `margin-left: calc(var(--spacing) * 28)` |
| `ml-32` | `margin-left: calc(var(--spacing) * 32)` |
| `ml-36` | `margin-left: calc(var(--spacing) * 36)` |
| `ml-40` | `margin-left: calc(var(--spacing) * 40)` |
| `ml-44` | `margin-left: calc(var(--spacing) * 44)` |
| `ml-48` | `margin-left: calc(var(--spacing) * 48)` |
| `ml-52` | `margin-left: calc(var(--spacing) * 52)` |
| `ml-56` | `margin-left: calc(var(--spacing) * 56)` |
| `ml-60` | `margin-left: calc(var(--spacing) * 60)` |
| `ml-64` | `margin-left: calc(var(--spacing) * 64)` |
| `ml-72` | `margin-left: calc(var(--spacing) * 72)` |
| `ml-80` | `margin-left: calc(var(--spacing) * 80)` |
| `ml-96` | `margin-left: calc(var(--spacing) * 96)` |
| `ms-0` | `margin-inline-start: 0px` |
| `ms-px` | `margin-inline-start: 1px` |
| `ms-0.5` | `margin-inline-start: calc(var(--spacing) * 0.5)` |
| `ms-1` | `margin-inline-start: calc(var(--spacing) * 1)` |
| `ms-1.5` | `margin-inline-start: calc(var(--spacing) * 1.5)` |
| `ms-2` | `margin-inline-start: calc(var(--spacing) * 2)` |
| `ms-2.5` | `margin-inline-start: calc(var(--spacing) * 2.5)` |
| `ms-3` | `margin-inline-start: calc(var(--spacing) * 3)` |
| `ms-3.5` | `margin-inline-start: calc(var(--spacing) * 3.5)` |
| `ms-4` | `margin-inline-start: calc(var(--spacing) * 4)` |
| `ms-5` | `margin-inline-start: calc(var(--spacing) * 5)` |
| `ms-6` | `margin-inline-start: calc(var(--spacing) * 6)` |
| `ms-7` | `margin-inline-start: calc(var(--spacing) * 7)` |
| `ms-8` | `margin-inline-start: calc(var(--spacing) * 8)` |
| `ms-9` | `margin-inline-start: calc(var(--spacing) * 9)` |
| `ms-10` | `margin-inline-start: calc(var(--spacing) * 10)` |
| `ms-11` | `margin-inline-start: calc(var(--spacing) * 11)` |
| `ms-12` | `margin-inline-start: calc(var(--spacing) * 12)` |
| `ms-14` | `margin-inline-start: calc(var(--spacing) * 14)` |
| `ms-16` | `margin-inline-start: calc(var(--spacing) * 16)` |
| `ms-20` | `margin-inline-start: calc(var(--spacing) * 20)` |
| `ms-24` | `margin-inline-start: calc(var(--spacing) * 24)` |
| `ms-28` | `margin-inline-start: calc(var(--spacing) * 28)` |
| `ms-32` | `margin-inline-start: calc(var(--spacing) * 32)` |
| `ms-36` | `margin-inline-start: calc(var(--spacing) * 36)` |
| `ms-40` | `margin-inline-start: calc(var(--spacing) * 40)` |
| `ms-44` | `margin-inline-start: calc(var(--spacing) * 44)` |
| `ms-48` | `margin-inline-start: calc(var(--spacing) * 48)` |
| `ms-52` | `margin-inline-start: calc(var(--spacing) * 52)` |
| `ms-56` | `margin-inline-start: calc(var(--spacing) * 56)` |
| `ms-60` | `margin-inline-start: calc(var(--spacing) * 60)` |
| `ms-64` | `margin-inline-start: calc(var(--spacing) * 64)` |
| `ms-72` | `margin-inline-start: calc(var(--spacing) * 72)` |
| `ms-80` | `margin-inline-start: calc(var(--spacing) * 80)` |
| `ms-96` | `margin-inline-start: calc(var(--spacing) * 96)` |
| `me-0` | `margin-inline-end: 0px` |
| `me-px` | `margin-inline-end: 1px` |
| `me-0.5` | `margin-inline-end: calc(var(--spacing) * 0.5)` |
| `me-1` | `margin-inline-end: calc(var(--spacing) * 1)` |
| `me-1.5` | `margin-inline-end: calc(var(--spacing) * 1.5)` |
| `me-2` | `margin-inline-end: calc(var(--spacing) * 2)` |
| `me-2.5` | `margin-inline-end: calc(var(--spacing) * 2.5)` |
| `me-3` | `margin-inline-end: calc(var(--spacing) * 3)` |
| `me-3.5` | `margin-inline-end: calc(var(--spacing) * 3.5)` |
| `me-4` | `margin-inline-end: calc(var(--spacing) * 4)` |
| `me-5` | `margin-inline-end: calc(var(--spacing) * 5)` |
| `me-6` | `margin-inline-end: calc(var(--spacing) * 6)` |
| `me-7` | `margin-inline-end: calc(var(--spacing) * 7)` |
| `me-8` | `margin-inline-end: calc(var(--spacing) * 8)` |
| `me-9` | `margin-inline-end: calc(var(--spacing) * 9)` |
| `me-10` | `margin-inline-end: calc(var(--spacing) * 10)` |
| `me-11` | `margin-inline-end: calc(var(--spacing) * 11)` |
| `me-12` | `margin-inline-end: calc(var(--spacing) * 12)` |
| `me-14` | `margin-inline-end: calc(var(--spacing) * 14)` |
| `me-16` | `margin-inline-end: calc(var(--spacing) * 16)` |
| `me-20` | `margin-inline-end: calc(var(--spacing) * 20)` |
| `me-24` | `margin-inline-end: calc(var(--spacing) * 24)` |
| `me-28` | `margin-inline-end: calc(var(--spacing) * 28)` |
| `me-32` | `margin-inline-end: calc(var(--spacing) * 32)` |
| `me-36` | `margin-inline-end: calc(var(--spacing) * 36)` |
| `me-40` | `margin-inline-end: calc(var(--spacing) * 40)` |
| `me-44` | `margin-inline-end: calc(var(--spacing) * 44)` |
| `me-48` | `margin-inline-end: calc(var(--spacing) * 48)` |
| `me-52` | `margin-inline-end: calc(var(--spacing) * 52)` |
| `me-56` | `margin-inline-end: calc(var(--spacing) * 56)` |
| `me-60` | `margin-inline-end: calc(var(--spacing) * 60)` |
| `me-64` | `margin-inline-end: calc(var(--spacing) * 64)` |
| `me-72` | `margin-inline-end: calc(var(--spacing) * 72)` |
| `me-80` | `margin-inline-end: calc(var(--spacing) * 80)` |
| `me-96` | `margin-inline-end: calc(var(--spacing) * 96)` |
| `m-auto` | `margin: auto` |
| `mx-auto` | `margin-left: auto; margin-right: auto` |
| `my-auto` | `margin-top: auto; margin-bottom: auto` |
| `mt-auto` | `margin-top: auto` |
| `mr-auto` | `margin-right: auto` |
| `mb-auto` | `margin-bottom: auto` |
| `ml-auto` | `margin-left: auto` |
| `space-x-0` | `--tw-space-x-reverse: 0; margin-right: calc(0px * var(--tw-space-x-reverse)); margin-left: calc(0px * calc(1 - var(--tw-space-x-reverse)))` |
| `space-x-px` | `--tw-space-x-reverse: 0; margin-right: calc(1px * var(--tw-space-x-reverse)); margin-left: calc(1px * calc(1 - var(--tw-space-x-reverse)))` |
| `space-x-0.5` | `--tw-space-x-reverse: 0; margin-right: calc(calc(var(--spacing) * 0.5) * var(--tw-space-x-reverse)); margin-left: calc(calc(var(--spacing) * 0.5) * calc(1 - var(--tw-space-x-reverse)))` |
| `space-x-1` | `--tw-space-x-reverse: 0; margin-right: calc(calc(var(--spacing) * 1) * var(--tw-space-x-reverse)); margin-left: calc(calc(var(--spacing) * 1) * calc(1 - var(--tw-space-x-reverse)))` |
| `space-x-1.5` | `--tw-space-x-reverse: 0; margin-right: calc(calc(var(--spacing) * 1.5) * var(--tw-space-x-reverse)); margin-left: calc(calc(var(--spacing) * 1.5) * calc(1 - var(--tw-space-x-reverse)))` |
| `space-x-2` | `--tw-space-x-reverse: 0; margin-right: calc(calc(var(--spacing) * 2) * var(--tw-space-x-reverse)); margin-left: calc(calc(var(--spacing) * 2) * calc(1 - var(--tw-space-x-reverse)))` |
| `space-x-2.5` | `--tw-space-x-reverse: 0; margin-right: calc(calc(var(--spacing) * 2.5) * var(--tw-space-x-reverse)); margin-left: calc(calc(var(--spacing) * 2.5) * calc(1 - var(--tw-space-x-reverse)))` |
| `space-x-3` | `--tw-space-x-reverse: 0; margin-right: calc(calc(var(--spacing) * 3) * var(--tw-space-x-reverse)); margin-left: calc(calc(var(--spacing) * 3) * calc(1 - var(--tw-space-x-reverse)))` |
| `space-x-3.5` | `--tw-space-x-reverse: 0; margin-right: calc(calc(var(--spacing) * 3.5) * var(--tw-space-x-reverse)); margin-left: calc(calc(var(--spacing) * 3.5) * calc(1 - var(--tw-space-x-reverse)))` |
| `space-x-4` | `--tw-space-x-reverse: 0; margin-right: calc(calc(var(--spacing) * 4) * var(--tw-space-x-reverse)); margin-left: calc(calc(var(--spacing) * 4) * calc(1 - var(--tw-space-x-reverse)))` |
| `space-x-5` | `--tw-space-x-reverse: 0; margin-right: calc(calc(var(--spacing) * 5) * var(--tw-space-x-reverse)); margin-left: calc(calc(var(--spacing) * 5) * calc(1 - var(--tw-space-x-reverse)))` |
| `space-x-6` | `--tw-space-x-reverse: 0; margin-right: calc(calc(var(--spacing) * 6) * var(--tw-space-x-reverse)); margin-left: calc(calc(var(--spacing) * 6) * calc(1 - var(--tw-space-x-reverse)))` |
| `space-x-7` | `--tw-space-x-reverse: 0; margin-right: calc(calc(var(--spacing) * 7) * var(--tw-space-x-reverse)); margin-left: calc(calc(var(--spacing) * 7) * calc(1 - var(--tw-space-x-reverse)))` |
| `space-x-8` | `--tw-space-x-reverse: 0; margin-right: calc(calc(var(--spacing) * 8) * var(--tw-space-x-reverse)); margin-left: calc(calc(var(--spacing) * 8) * calc(1 - var(--tw-space-x-reverse)))` |
| `space-x-9` | `--tw-space-x-reverse: 0; margin-right: calc(calc(var(--spacing) * 9) * var(--tw-space-x-reverse)); margin-left: calc(calc(var(--spacing) * 9) * calc(1 - var(--tw-space-x-reverse)))` |
| `space-x-10` | `--tw-space-x-reverse: 0; margin-right: calc(calc(var(--spacing) * 10) * var(--tw-space-x-reverse)); margin-left: calc(calc(var(--spacing) * 10) * calc(1 - var(--tw-space-x-reverse)))` |
| `space-x-11` | `--tw-space-x-reverse: 0; margin-right: calc(calc(var(--spacing) * 11) * var(--tw-space-x-reverse)); margin-left: calc(calc(var(--spacing) * 11) * calc(1 - var(--tw-space-x-reverse)))` |
| `space-x-12` | `--tw-space-x-reverse: 0; margin-right: calc(calc(var(--spacing) * 12) * var(--tw-space-x-reverse)); margin-left: calc(calc(var(--spacing) * 12) * calc(1 - var(--tw-space-x-reverse)))` |
| `space-x-14` | `--tw-space-x-reverse: 0; margin-right: calc(calc(var(--spacing) * 14) * var(--tw-space-x-reverse)); margin-left: calc(calc(var(--spacing) * 14) * calc(1 - var(--tw-space-x-reverse)))` |
| `space-x-16` | `--tw-space-x-reverse: 0; margin-right: calc(calc(var(--spacing) * 16) * var(--tw-space-x-reverse)); margin-left: calc(calc(var(--spacing) * 16) * calc(1 - var(--tw-space-x-reverse)))` |
| `space-x-20` | `--tw-space-x-reverse: 0; margin-right: calc(calc(var(--spacing) * 20) * var(--tw-space-x-reverse)); margin-left: calc(calc(var(--spacing) * 20) * calc(1 - var(--tw-space-x-reverse)))` |
| `space-x-24` | `--tw-space-x-reverse: 0; margin-right: calc(calc(var(--spacing) * 24) * var(--tw-space-x-reverse)); margin-left: calc(calc(var(--spacing) * 24) * calc(1 - var(--tw-space-x-reverse)))` |
| `space-x-28` | `--tw-space-x-reverse: 0; margin-right: calc(calc(var(--spacing) * 28) * var(--tw-space-x-reverse)); margin-left: calc(calc(var(--spacing) * 28) * calc(1 - var(--tw-space-x-reverse)))` |
| `space-x-32` | `--tw-space-x-reverse: 0; margin-right: calc(calc(var(--spacing) * 32) * var(--tw-space-x-reverse)); margin-left: calc(calc(var(--spacing) * 32) * calc(1 - var(--tw-space-x-reverse)))` |
| `space-x-36` | `--tw-space-x-reverse: 0; margin-right: calc(calc(var(--spacing) * 36) * var(--tw-space-x-reverse)); margin-left: calc(calc(var(--spacing) * 36) * calc(1 - var(--tw-space-x-reverse)))` |
| `space-x-40` | `--tw-space-x-reverse: 0; margin-right: calc(calc(var(--spacing) * 40) * var(--tw-space-x-reverse)); margin-left: calc(calc(var(--spacing) * 40) * calc(1 - var(--tw-space-x-reverse)))` |
| `space-x-44` | `--tw-space-x-reverse: 0; margin-right: calc(calc(var(--spacing) * 44) * var(--tw-space-x-reverse)); margin-left: calc(calc(var(--spacing) * 44) * calc(1 - var(--tw-space-x-reverse)))` |
| `space-x-48` | `--tw-space-x-reverse: 0; margin-right: calc(calc(var(--spacing) * 48) * var(--tw-space-x-reverse)); margin-left: calc(calc(var(--spacing) * 48) * calc(1 - var(--tw-space-x-reverse)))` |
| `space-x-52` | `--tw-space-x-reverse: 0; margin-right: calc(calc(var(--spacing) * 52) * var(--tw-space-x-reverse)); margin-left: calc(calc(var(--spacing) * 52) * calc(1 - var(--tw-space-x-reverse)))` |
| `space-x-56` | `--tw-space-x-reverse: 0; margin-right: calc(calc(var(--spacing) * 56) * var(--tw-space-x-reverse)); margin-left: calc(calc(var(--spacing) * 56) * calc(1 - var(--tw-space-x-reverse)))` |
| `space-x-60` | `--tw-space-x-reverse: 0; margin-right: calc(calc(var(--spacing) * 60) * var(--tw-space-x-reverse)); margin-left: calc(calc(var(--spacing) * 60) * calc(1 - var(--tw-space-x-reverse)))` |
| `space-x-64` | `--tw-space-x-reverse: 0; margin-right: calc(calc(var(--spacing) * 64) * var(--tw-space-x-reverse)); margin-left: calc(calc(var(--spacing) * 64) * calc(1 - var(--tw-space-x-reverse)))` |
| `space-x-72` | `--tw-space-x-reverse: 0; margin-right: calc(calc(var(--spacing) * 72) * var(--tw-space-x-reverse)); margin-left: calc(calc(var(--spacing) * 72) * calc(1 - var(--tw-space-x-reverse)))` |
| `space-x-80` | `--tw-space-x-reverse: 0; margin-right: calc(calc(var(--spacing) * 80) * var(--tw-space-x-reverse)); margin-left: calc(calc(var(--spacing) * 80) * calc(1 - var(--tw-space-x-reverse)))` |
| `space-x-96` | `--tw-space-x-reverse: 0; margin-right: calc(calc(var(--spacing) * 96) * var(--tw-space-x-reverse)); margin-left: calc(calc(var(--spacing) * 96) * calc(1 - var(--tw-space-x-reverse)))` |
| `space-y-0` | `--tw-space-y-reverse: 0; margin-bottom: calc(0px * var(--tw-space-y-reverse)); margin-top: calc(0px * calc(1 - var(--tw-space-y-reverse)))` |
| `space-y-px` | `--tw-space-y-reverse: 0; margin-bottom: calc(1px * var(--tw-space-y-reverse)); margin-top: calc(1px * calc(1 - var(--tw-space-y-reverse)))` |
| `space-y-0.5` | `--tw-space-y-reverse: 0; margin-bottom: calc(calc(var(--spacing) * 0.5) * var(--tw-space-y-reverse)); margin-top: calc(calc(var(--spacing) * 0.5) * calc(1 - var(--tw-space-y-reverse)))` |
| `space-y-1` | `--tw-space-y-reverse: 0; margin-bottom: calc(calc(var(--spacing) * 1) * var(--tw-space-y-reverse)); margin-top: calc(calc(var(--spacing) * 1) * calc(1 - var(--tw-space-y-reverse)))` |
| `space-y-1.5` | `--tw-space-y-reverse: 0; margin-bottom: calc(calc(var(--spacing) * 1.5) * var(--tw-space-y-reverse)); margin-top: calc(calc(var(--spacing) * 1.5) * calc(1 - var(--tw-space-y-reverse)))` |
| `space-y-2` | `--tw-space-y-reverse: 0; margin-bottom: calc(calc(var(--spacing) * 2) * var(--tw-space-y-reverse)); margin-top: calc(calc(var(--spacing) * 2) * calc(1 - var(--tw-space-y-reverse)))` |
| `space-y-2.5` | `--tw-space-y-reverse: 0; margin-bottom: calc(calc(var(--spacing) * 2.5) * var(--tw-space-y-reverse)); margin-top: calc(calc(var(--spacing) * 2.5) * calc(1 - var(--tw-space-y-reverse)))` |
| `space-y-3` | `--tw-space-y-reverse: 0; margin-bottom: calc(calc(var(--spacing) * 3) * var(--tw-space-y-reverse)); margin-top: calc(calc(var(--spacing) * 3) * calc(1 - var(--tw-space-y-reverse)))` |
| `space-y-3.5` | `--tw-space-y-reverse: 0; margin-bottom: calc(calc(var(--spacing) * 3.5) * var(--tw-space-y-reverse)); margin-top: calc(calc(var(--spacing) * 3.5) * calc(1 - var(--tw-space-y-reverse)))` |
| `space-y-4` | `--tw-space-y-reverse: 0; margin-bottom: calc(calc(var(--spacing) * 4) * var(--tw-space-y-reverse)); margin-top: calc(calc(var(--spacing) * 4) * calc(1 - var(--tw-space-y-reverse)))` |
| `space-y-5` | `--tw-space-y-reverse: 0; margin-bottom: calc(calc(var(--spacing) * 5) * var(--tw-space-y-reverse)); margin-top: calc(calc(var(--spacing) * 5) * calc(1 - var(--tw-space-y-reverse)))` |
| `space-y-6` | `--tw-space-y-reverse: 0; margin-bottom: calc(calc(var(--spacing) * 6) * var(--tw-space-y-reverse)); margin-top: calc(calc(var(--spacing) * 6) * calc(1 - var(--tw-space-y-reverse)))` |
| `space-y-7` | `--tw-space-y-reverse: 0; margin-bottom: calc(calc(var(--spacing) * 7) * var(--tw-space-y-reverse)); margin-top: calc(calc(var(--spacing) * 7) * calc(1 - var(--tw-space-y-reverse)))` |
| `space-y-8` | `--tw-space-y-reverse: 0; margin-bottom: calc(calc(var(--spacing) * 8) * var(--tw-space-y-reverse)); margin-top: calc(calc(var(--spacing) * 8) * calc(1 - var(--tw-space-y-reverse)))` |
| `space-y-9` | `--tw-space-y-reverse: 0; margin-bottom: calc(calc(var(--spacing) * 9) * var(--tw-space-y-reverse)); margin-top: calc(calc(var(--spacing) * 9) * calc(1 - var(--tw-space-y-reverse)))` |
| `space-y-10` | `--tw-space-y-reverse: 0; margin-bottom: calc(calc(var(--spacing) * 10) * var(--tw-space-y-reverse)); margin-top: calc(calc(var(--spacing) * 10) * calc(1 - var(--tw-space-y-reverse)))` |
| `space-y-11` | `--tw-space-y-reverse: 0; margin-bottom: calc(calc(var(--spacing) * 11) * var(--tw-space-y-reverse)); margin-top: calc(calc(var(--spacing) * 11) * calc(1 - var(--tw-space-y-reverse)))` |
| `space-y-12` | `--tw-space-y-reverse: 0; margin-bottom: calc(calc(var(--spacing) * 12) * var(--tw-space-y-reverse)); margin-top: calc(calc(var(--spacing) * 12) * calc(1 - var(--tw-space-y-reverse)))` |
| `space-y-14` | `--tw-space-y-reverse: 0; margin-bottom: calc(calc(var(--spacing) * 14) * var(--tw-space-y-reverse)); margin-top: calc(calc(var(--spacing) * 14) * calc(1 - var(--tw-space-y-reverse)))` |
| `space-y-16` | `--tw-space-y-reverse: 0; margin-bottom: calc(calc(var(--spacing) * 16) * var(--tw-space-y-reverse)); margin-top: calc(calc(var(--spacing) * 16) * calc(1 - var(--tw-space-y-reverse)))` |
| `space-y-20` | `--tw-space-y-reverse: 0; margin-bottom: calc(calc(var(--spacing) * 20) * var(--tw-space-y-reverse)); margin-top: calc(calc(var(--spacing) * 20) * calc(1 - var(--tw-space-y-reverse)))` |
| `space-y-24` | `--tw-space-y-reverse: 0; margin-bottom: calc(calc(var(--spacing) * 24) * var(--tw-space-y-reverse)); margin-top: calc(calc(var(--spacing) * 24) * calc(1 - var(--tw-space-y-reverse)))` |
| `space-y-28` | `--tw-space-y-reverse: 0; margin-bottom: calc(calc(var(--spacing) * 28) * var(--tw-space-y-reverse)); margin-top: calc(calc(var(--spacing) * 28) * calc(1 - var(--tw-space-y-reverse)))` |
| `space-y-32` | `--tw-space-y-reverse: 0; margin-bottom: calc(calc(var(--spacing) * 32) * var(--tw-space-y-reverse)); margin-top: calc(calc(var(--spacing) * 32) * calc(1 - var(--tw-space-y-reverse)))` |
| `space-y-36` | `--tw-space-y-reverse: 0; margin-bottom: calc(calc(var(--spacing) * 36) * var(--tw-space-y-reverse)); margin-top: calc(calc(var(--spacing) * 36) * calc(1 - var(--tw-space-y-reverse)))` |
| `space-y-40` | `--tw-space-y-reverse: 0; margin-bottom: calc(calc(var(--spacing) * 40) * var(--tw-space-y-reverse)); margin-top: calc(calc(var(--spacing) * 40) * calc(1 - var(--tw-space-y-reverse)))` |
| `space-y-44` | `--tw-space-y-reverse: 0; margin-bottom: calc(calc(var(--spacing) * 44) * var(--tw-space-y-reverse)); margin-top: calc(calc(var(--spacing) * 44) * calc(1 - var(--tw-space-y-reverse)))` |
| `space-y-48` | `--tw-space-y-reverse: 0; margin-bottom: calc(calc(var(--spacing) * 48) * var(--tw-space-y-reverse)); margin-top: calc(calc(var(--spacing) * 48) * calc(1 - var(--tw-space-y-reverse)))` |
| `space-y-52` | `--tw-space-y-reverse: 0; margin-bottom: calc(calc(var(--spacing) * 52) * var(--tw-space-y-reverse)); margin-top: calc(calc(var(--spacing) * 52) * calc(1 - var(--tw-space-y-reverse)))` |
| `space-y-56` | `--tw-space-y-reverse: 0; margin-bottom: calc(calc(var(--spacing) * 56) * var(--tw-space-y-reverse)); margin-top: calc(calc(var(--spacing) * 56) * calc(1 - var(--tw-space-y-reverse)))` |
| `space-y-60` | `--tw-space-y-reverse: 0; margin-bottom: calc(calc(var(--spacing) * 60) * var(--tw-space-y-reverse)); margin-top: calc(calc(var(--spacing) * 60) * calc(1 - var(--tw-space-y-reverse)))` |
| `space-y-64` | `--tw-space-y-reverse: 0; margin-bottom: calc(calc(var(--spacing) * 64) * var(--tw-space-y-reverse)); margin-top: calc(calc(var(--spacing) * 64) * calc(1 - var(--tw-space-y-reverse)))` |
| `space-y-72` | `--tw-space-y-reverse: 0; margin-bottom: calc(calc(var(--spacing) * 72) * var(--tw-space-y-reverse)); margin-top: calc(calc(var(--spacing) * 72) * calc(1 - var(--tw-space-y-reverse)))` |
| `space-y-80` | `--tw-space-y-reverse: 0; margin-bottom: calc(calc(var(--spacing) * 80) * var(--tw-space-y-reverse)); margin-top: calc(calc(var(--spacing) * 80) * calc(1 - var(--tw-space-y-reverse)))` |
| `space-y-96` | `--tw-space-y-reverse: 0; margin-bottom: calc(calc(var(--spacing) * 96) * var(--tw-space-y-reverse)); margin-top: calc(calc(var(--spacing) * 96) * calc(1 - var(--tw-space-y-reverse)))` |

## Sizing — Width

| Classe | CSS |
|--------|-----|
| `w-0` | `width: 0px` |
| `w-px` | `width: 1px` |
| `w-0.5` | `width: calc(var(--spacing) * 0.5)` |
| `w-1` | `width: calc(var(--spacing) * 1)` |
| `w-1.5` | `width: calc(var(--spacing) * 1.5)` |
| `w-2` | `width: calc(var(--spacing) * 2)` |
| `w-2.5` | `width: calc(var(--spacing) * 2.5)` |
| `w-3` | `width: calc(var(--spacing) * 3)` |
| `w-3.5` | `width: calc(var(--spacing) * 3.5)` |
| `w-4` | `width: calc(var(--spacing) * 4)` |
| `w-5` | `width: calc(var(--spacing) * 5)` |
| `w-6` | `width: calc(var(--spacing) * 6)` |
| `w-7` | `width: calc(var(--spacing) * 7)` |
| `w-8` | `width: calc(var(--spacing) * 8)` |
| `w-9` | `width: calc(var(--spacing) * 9)` |
| `w-10` | `width: calc(var(--spacing) * 10)` |
| `w-11` | `width: calc(var(--spacing) * 11)` |
| `w-12` | `width: calc(var(--spacing) * 12)` |
| `w-14` | `width: calc(var(--spacing) * 14)` |
| `w-16` | `width: calc(var(--spacing) * 16)` |
| `w-20` | `width: calc(var(--spacing) * 20)` |
| `w-24` | `width: calc(var(--spacing) * 24)` |
| `w-28` | `width: calc(var(--spacing) * 28)` |
| `w-32` | `width: calc(var(--spacing) * 32)` |
| `w-36` | `width: calc(var(--spacing) * 36)` |
| `w-40` | `width: calc(var(--spacing) * 40)` |
| `w-44` | `width: calc(var(--spacing) * 44)` |
| `w-48` | `width: calc(var(--spacing) * 48)` |
| `w-52` | `width: calc(var(--spacing) * 52)` |
| `w-56` | `width: calc(var(--spacing) * 56)` |
| `w-60` | `width: calc(var(--spacing) * 60)` |
| `w-64` | `width: calc(var(--spacing) * 64)` |
| `w-72` | `width: calc(var(--spacing) * 72)` |
| `w-80` | `width: calc(var(--spacing) * 80)` |
| `w-96` | `width: calc(var(--spacing) * 96)` |
| `w-auto` | `width: auto` |
| `w-full` | `width: 100%` |
| `w-screen` | `width: 100vw` |
| `w-svw` | `width: 100svw` |
| `w-dvw` | `width: 100dvw` |
| `w-min` | `width: min-content` |
| `w-max` | `width: max-content` |
| `w-fit` | `width: fit-content` |
| `w-1/2` | `width: 50%` |
| `w-1/3` | `width: 33.333333%` |
| `w-2/3` | `width: 66.666667%` |
| `w-1/4` | `width: 25%` |
| `w-3/4` | `width: 75%` |
| `w-1/5` | `width: 20%` |
| `w-4/5` | `width: 80%` |
| `w-1/6` | `width: 16.666667%` |
| `w-5/6` | `width: 83.333333%` |
| `min-w-0` | `min-width: 0px` |
| `min-w-px` | `min-width: 1px` |
| `min-w-0.5` | `min-width: calc(var(--spacing) * 0.5)` |
| `min-w-1` | `min-width: calc(var(--spacing) * 1)` |
| `min-w-1.5` | `min-width: calc(var(--spacing) * 1.5)` |
| `min-w-2` | `min-width: calc(var(--spacing) * 2)` |
| `min-w-2.5` | `min-width: calc(var(--spacing) * 2.5)` |
| `min-w-3` | `min-width: calc(var(--spacing) * 3)` |
| `min-w-3.5` | `min-width: calc(var(--spacing) * 3.5)` |
| `min-w-4` | `min-width: calc(var(--spacing) * 4)` |
| `min-w-5` | `min-width: calc(var(--spacing) * 5)` |
| `min-w-6` | `min-width: calc(var(--spacing) * 6)` |
| `min-w-7` | `min-width: calc(var(--spacing) * 7)` |
| `min-w-8` | `min-width: calc(var(--spacing) * 8)` |
| `min-w-9` | `min-width: calc(var(--spacing) * 9)` |
| `min-w-10` | `min-width: calc(var(--spacing) * 10)` |
| `min-w-11` | `min-width: calc(var(--spacing) * 11)` |
| `min-w-12` | `min-width: calc(var(--spacing) * 12)` |
| `min-w-14` | `min-width: calc(var(--spacing) * 14)` |
| `min-w-16` | `min-width: calc(var(--spacing) * 16)` |
| `min-w-20` | `min-width: calc(var(--spacing) * 20)` |
| `min-w-24` | `min-width: calc(var(--spacing) * 24)` |
| `min-w-28` | `min-width: calc(var(--spacing) * 28)` |
| `min-w-32` | `min-width: calc(var(--spacing) * 32)` |
| `min-w-36` | `min-width: calc(var(--spacing) * 36)` |
| `min-w-40` | `min-width: calc(var(--spacing) * 40)` |
| `min-w-44` | `min-width: calc(var(--spacing) * 44)` |
| `min-w-48` | `min-width: calc(var(--spacing) * 48)` |
| `min-w-52` | `min-width: calc(var(--spacing) * 52)` |
| `min-w-56` | `min-width: calc(var(--spacing) * 56)` |
| `min-w-60` | `min-width: calc(var(--spacing) * 60)` |
| `min-w-64` | `min-width: calc(var(--spacing) * 64)` |
| `min-w-72` | `min-width: calc(var(--spacing) * 72)` |
| `min-w-80` | `min-width: calc(var(--spacing) * 80)` |
| `min-w-96` | `min-width: calc(var(--spacing) * 96)` |
| `min-w-full` | `min-width: 100%` |
| `min-w-min` | `min-width: min-content` |
| `min-w-max` | `min-width: max-content` |
| `min-w-fit` | `min-width: fit-content` |
| `max-w-0` | `max-width: 0px` |
| `max-w-px` | `max-width: 1px` |
| `max-w-0.5` | `max-width: calc(var(--spacing) * 0.5)` |
| `max-w-1` | `max-width: calc(var(--spacing) * 1)` |
| `max-w-1.5` | `max-width: calc(var(--spacing) * 1.5)` |
| `max-w-2` | `max-width: calc(var(--spacing) * 2)` |
| `max-w-2.5` | `max-width: calc(var(--spacing) * 2.5)` |
| `max-w-3` | `max-width: calc(var(--spacing) * 3)` |
| `max-w-3.5` | `max-width: calc(var(--spacing) * 3.5)` |
| `max-w-4` | `max-width: calc(var(--spacing) * 4)` |
| `max-w-5` | `max-width: calc(var(--spacing) * 5)` |
| `max-w-6` | `max-width: calc(var(--spacing) * 6)` |
| `max-w-7` | `max-width: calc(var(--spacing) * 7)` |
| `max-w-8` | `max-width: calc(var(--spacing) * 8)` |
| `max-w-9` | `max-width: calc(var(--spacing) * 9)` |
| `max-w-10` | `max-width: calc(var(--spacing) * 10)` |
| `max-w-11` | `max-width: calc(var(--spacing) * 11)` |
| `max-w-12` | `max-width: calc(var(--spacing) * 12)` |
| `max-w-14` | `max-width: calc(var(--spacing) * 14)` |
| `max-w-16` | `max-width: calc(var(--spacing) * 16)` |
| `max-w-20` | `max-width: calc(var(--spacing) * 20)` |
| `max-w-24` | `max-width: calc(var(--spacing) * 24)` |
| `max-w-28` | `max-width: calc(var(--spacing) * 28)` |
| `max-w-32` | `max-width: calc(var(--spacing) * 32)` |
| `max-w-36` | `max-width: calc(var(--spacing) * 36)` |
| `max-w-40` | `max-width: calc(var(--spacing) * 40)` |
| `max-w-44` | `max-width: calc(var(--spacing) * 44)` |
| `max-w-48` | `max-width: calc(var(--spacing) * 48)` |
| `max-w-52` | `max-width: calc(var(--spacing) * 52)` |
| `max-w-56` | `max-width: calc(var(--spacing) * 56)` |
| `max-w-60` | `max-width: calc(var(--spacing) * 60)` |
| `max-w-64` | `max-width: calc(var(--spacing) * 64)` |
| `max-w-72` | `max-width: calc(var(--spacing) * 72)` |
| `max-w-80` | `max-width: calc(var(--spacing) * 80)` |
| `max-w-96` | `max-width: calc(var(--spacing) * 96)` |
| `max-w-none` | `max-width: none` |
| `max-w-full` | `max-width: 100%` |
| `max-w-xs` | `max-width: 20rem` |
| `max-w-sm` | `max-width: 24rem` |
| `max-w-md` | `max-width: 28rem` |
| `max-w-lg` | `max-width: 32rem` |
| `max-w-xl` | `max-width: 36rem` |
| `max-w-2xl` | `max-width: 42rem` |
| `max-w-3xl` | `max-width: 48rem` |
| `max-w-4xl` | `max-width: 56rem` |
| `max-w-5xl` | `max-width: 64rem` |
| `max-w-6xl` | `max-width: 72rem` |
| `max-w-7xl` | `max-width: 80rem` |
| `max-w-prose` | `max-width: 65ch` |
| `max-w-screen-sm` | `max-width: 640px` |
| `max-w-screen-md` | `max-width: 768px` |
| `max-w-screen-lg` | `max-width: 1024px` |
| `max-w-screen-xl` | `max-width: 1280px` |
| `max-w-screen-2xl` | `max-width: 1536px` |

## Sizing — Height

| Classe | CSS |
|--------|-----|
| `h-0` | `height: 0px` |
| `h-px` | `height: 1px` |
| `h-0.5` | `height: calc(var(--spacing) * 0.5)` |
| `h-1` | `height: calc(var(--spacing) * 1)` |
| `h-1.5` | `height: calc(var(--spacing) * 1.5)` |
| `h-2` | `height: calc(var(--spacing) * 2)` |
| `h-2.5` | `height: calc(var(--spacing) * 2.5)` |
| `h-3` | `height: calc(var(--spacing) * 3)` |
| `h-3.5` | `height: calc(var(--spacing) * 3.5)` |
| `h-4` | `height: calc(var(--spacing) * 4)` |
| `h-5` | `height: calc(var(--spacing) * 5)` |
| `h-6` | `height: calc(var(--spacing) * 6)` |
| `h-7` | `height: calc(var(--spacing) * 7)` |
| `h-8` | `height: calc(var(--spacing) * 8)` |
| `h-9` | `height: calc(var(--spacing) * 9)` |
| `h-10` | `height: calc(var(--spacing) * 10)` |
| `h-11` | `height: calc(var(--spacing) * 11)` |
| `h-12` | `height: calc(var(--spacing) * 12)` |
| `h-14` | `height: calc(var(--spacing) * 14)` |
| `h-16` | `height: calc(var(--spacing) * 16)` |
| `h-20` | `height: calc(var(--spacing) * 20)` |
| `h-24` | `height: calc(var(--spacing) * 24)` |
| `h-28` | `height: calc(var(--spacing) * 28)` |
| `h-32` | `height: calc(var(--spacing) * 32)` |
| `h-36` | `height: calc(var(--spacing) * 36)` |
| `h-40` | `height: calc(var(--spacing) * 40)` |
| `h-44` | `height: calc(var(--spacing) * 44)` |
| `h-48` | `height: calc(var(--spacing) * 48)` |
| `h-52` | `height: calc(var(--spacing) * 52)` |
| `h-56` | `height: calc(var(--spacing) * 56)` |
| `h-60` | `height: calc(var(--spacing) * 60)` |
| `h-64` | `height: calc(var(--spacing) * 64)` |
| `h-72` | `height: calc(var(--spacing) * 72)` |
| `h-80` | `height: calc(var(--spacing) * 80)` |
| `h-96` | `height: calc(var(--spacing) * 96)` |
| `h-auto` | `height: auto` |
| `h-full` | `height: 100%` |
| `h-screen` | `height: 100vh` |
| `h-svh` | `height: 100svh` |
| `h-dvh` | `height: 100dvh` |
| `h-min` | `height: min-content` |
| `h-max` | `height: max-content` |
| `h-fit` | `height: fit-content` |
| `min-h-0` | `min-height: 0px` |
| `min-h-px` | `min-height: 1px` |
| `min-h-0.5` | `min-height: calc(var(--spacing) * 0.5)` |
| `min-h-1` | `min-height: calc(var(--spacing) * 1)` |
| `min-h-1.5` | `min-height: calc(var(--spacing) * 1.5)` |
| `min-h-2` | `min-height: calc(var(--spacing) * 2)` |
| `min-h-2.5` | `min-height: calc(var(--spacing) * 2.5)` |
| `min-h-3` | `min-height: calc(var(--spacing) * 3)` |
| `min-h-3.5` | `min-height: calc(var(--spacing) * 3.5)` |
| `min-h-4` | `min-height: calc(var(--spacing) * 4)` |
| `min-h-5` | `min-height: calc(var(--spacing) * 5)` |
| `min-h-6` | `min-height: calc(var(--spacing) * 6)` |
| `min-h-7` | `min-height: calc(var(--spacing) * 7)` |
| `min-h-8` | `min-height: calc(var(--spacing) * 8)` |
| `min-h-9` | `min-height: calc(var(--spacing) * 9)` |
| `min-h-10` | `min-height: calc(var(--spacing) * 10)` |
| `min-h-11` | `min-height: calc(var(--spacing) * 11)` |
| `min-h-12` | `min-height: calc(var(--spacing) * 12)` |
| `min-h-14` | `min-height: calc(var(--spacing) * 14)` |
| `min-h-16` | `min-height: calc(var(--spacing) * 16)` |
| `min-h-20` | `min-height: calc(var(--spacing) * 20)` |
| `min-h-24` | `min-height: calc(var(--spacing) * 24)` |
| `min-h-28` | `min-height: calc(var(--spacing) * 28)` |
| `min-h-32` | `min-height: calc(var(--spacing) * 32)` |
| `min-h-36` | `min-height: calc(var(--spacing) * 36)` |
| `min-h-40` | `min-height: calc(var(--spacing) * 40)` |
| `min-h-44` | `min-height: calc(var(--spacing) * 44)` |
| `min-h-48` | `min-height: calc(var(--spacing) * 48)` |
| `min-h-52` | `min-height: calc(var(--spacing) * 52)` |
| `min-h-56` | `min-height: calc(var(--spacing) * 56)` |
| `min-h-60` | `min-height: calc(var(--spacing) * 60)` |
| `min-h-64` | `min-height: calc(var(--spacing) * 64)` |
| `min-h-72` | `min-height: calc(var(--spacing) * 72)` |
| `min-h-80` | `min-height: calc(var(--spacing) * 80)` |
| `min-h-96` | `min-height: calc(var(--spacing) * 96)` |
| `min-h-full` | `min-height: 100%` |
| `min-h-screen` | `min-height: 100vh` |
| `min-h-svh` | `min-height: 100svh` |
| `min-h-dvh` | `min-height: 100dvh` |
| `min-h-min` | `min-height: min-content` |
| `min-h-max` | `min-height: max-content` |
| `min-h-fit` | `min-height: fit-content` |
| `max-h-0` | `max-height: 0px` |
| `max-h-px` | `max-height: 1px` |
| `max-h-0.5` | `max-height: calc(var(--spacing) * 0.5)` |
| `max-h-1` | `max-height: calc(var(--spacing) * 1)` |
| `max-h-1.5` | `max-height: calc(var(--spacing) * 1.5)` |
| `max-h-2` | `max-height: calc(var(--spacing) * 2)` |
| `max-h-2.5` | `max-height: calc(var(--spacing) * 2.5)` |
| `max-h-3` | `max-height: calc(var(--spacing) * 3)` |
| `max-h-3.5` | `max-height: calc(var(--spacing) * 3.5)` |
| `max-h-4` | `max-height: calc(var(--spacing) * 4)` |
| `max-h-5` | `max-height: calc(var(--spacing) * 5)` |
| `max-h-6` | `max-height: calc(var(--spacing) * 6)` |
| `max-h-7` | `max-height: calc(var(--spacing) * 7)` |
| `max-h-8` | `max-height: calc(var(--spacing) * 8)` |
| `max-h-9` | `max-height: calc(var(--spacing) * 9)` |
| `max-h-10` | `max-height: calc(var(--spacing) * 10)` |
| `max-h-11` | `max-height: calc(var(--spacing) * 11)` |
| `max-h-12` | `max-height: calc(var(--spacing) * 12)` |
| `max-h-14` | `max-height: calc(var(--spacing) * 14)` |
| `max-h-16` | `max-height: calc(var(--spacing) * 16)` |
| `max-h-20` | `max-height: calc(var(--spacing) * 20)` |
| `max-h-24` | `max-height: calc(var(--spacing) * 24)` |
| `max-h-28` | `max-height: calc(var(--spacing) * 28)` |
| `max-h-32` | `max-height: calc(var(--spacing) * 32)` |
| `max-h-36` | `max-height: calc(var(--spacing) * 36)` |
| `max-h-40` | `max-height: calc(var(--spacing) * 40)` |
| `max-h-44` | `max-height: calc(var(--spacing) * 44)` |
| `max-h-48` | `max-height: calc(var(--spacing) * 48)` |
| `max-h-52` | `max-height: calc(var(--spacing) * 52)` |
| `max-h-56` | `max-height: calc(var(--spacing) * 56)` |
| `max-h-60` | `max-height: calc(var(--spacing) * 60)` |
| `max-h-64` | `max-height: calc(var(--spacing) * 64)` |
| `max-h-72` | `max-height: calc(var(--spacing) * 72)` |
| `max-h-80` | `max-height: calc(var(--spacing) * 80)` |
| `max-h-96` | `max-height: calc(var(--spacing) * 96)` |
| `max-h-none` | `max-height: none` |
| `max-h-full` | `max-height: 100%` |
| `max-h-screen` | `max-height: 100vh` |
| `max-h-svh` | `max-height: 100svh` |
| `max-h-dvh` | `max-height: 100dvh` |
| `size-0` | `width: 0px; height: 0px` |
| `size-px` | `width: 1px; height: 1px` |
| `size-0.5` | `width: calc(var(--spacing) * 0.5); height: calc(var(--spacing) * 0.5)` |
| `size-1` | `width: calc(var(--spacing) * 1); height: calc(var(--spacing) * 1)` |
| `size-1.5` | `width: calc(var(--spacing) * 1.5); height: calc(var(--spacing) * 1.5)` |
| `size-2` | `width: calc(var(--spacing) * 2); height: calc(var(--spacing) * 2)` |
| `size-2.5` | `width: calc(var(--spacing) * 2.5); height: calc(var(--spacing) * 2.5)` |
| `size-3` | `width: calc(var(--spacing) * 3); height: calc(var(--spacing) * 3)` |
| `size-3.5` | `width: calc(var(--spacing) * 3.5); height: calc(var(--spacing) * 3.5)` |
| `size-4` | `width: calc(var(--spacing) * 4); height: calc(var(--spacing) * 4)` |
| `size-5` | `width: calc(var(--spacing) * 5); height: calc(var(--spacing) * 5)` |
| `size-6` | `width: calc(var(--spacing) * 6); height: calc(var(--spacing) * 6)` |
| `size-7` | `width: calc(var(--spacing) * 7); height: calc(var(--spacing) * 7)` |
| `size-8` | `width: calc(var(--spacing) * 8); height: calc(var(--spacing) * 8)` |
| `size-9` | `width: calc(var(--spacing) * 9); height: calc(var(--spacing) * 9)` |
| `size-10` | `width: calc(var(--spacing) * 10); height: calc(var(--spacing) * 10)` |
| `size-11` | `width: calc(var(--spacing) * 11); height: calc(var(--spacing) * 11)` |
| `size-12` | `width: calc(var(--spacing) * 12); height: calc(var(--spacing) * 12)` |
| `size-14` | `width: calc(var(--spacing) * 14); height: calc(var(--spacing) * 14)` |
| `size-16` | `width: calc(var(--spacing) * 16); height: calc(var(--spacing) * 16)` |
| `size-20` | `width: calc(var(--spacing) * 20); height: calc(var(--spacing) * 20)` |
| `size-24` | `width: calc(var(--spacing) * 24); height: calc(var(--spacing) * 24)` |
| `size-28` | `width: calc(var(--spacing) * 28); height: calc(var(--spacing) * 28)` |
| `size-32` | `width: calc(var(--spacing) * 32); height: calc(var(--spacing) * 32)` |
| `size-36` | `width: calc(var(--spacing) * 36); height: calc(var(--spacing) * 36)` |
| `size-40` | `width: calc(var(--spacing) * 40); height: calc(var(--spacing) * 40)` |
| `size-44` | `width: calc(var(--spacing) * 44); height: calc(var(--spacing) * 44)` |
| `size-48` | `width: calc(var(--spacing) * 48); height: calc(var(--spacing) * 48)` |
| `size-52` | `width: calc(var(--spacing) * 52); height: calc(var(--spacing) * 52)` |
| `size-56` | `width: calc(var(--spacing) * 56); height: calc(var(--spacing) * 56)` |
| `size-60` | `width: calc(var(--spacing) * 60); height: calc(var(--spacing) * 60)` |
| `size-64` | `width: calc(var(--spacing) * 64); height: calc(var(--spacing) * 64)` |
| `size-72` | `width: calc(var(--spacing) * 72); height: calc(var(--spacing) * 72)` |
| `size-80` | `width: calc(var(--spacing) * 80); height: calc(var(--spacing) * 80)` |
| `size-96` | `width: calc(var(--spacing) * 96); height: calc(var(--spacing) * 96)` |
| `size-auto` | `width: auto; height: auto` |
| `size-full` | `width: 100%; height: 100%` |
| `size-min` | `width: min-content; height: min-content` |
| `size-max` | `width: max-content; height: max-content` |

## Typography — Font

| Classe | CSS |
|--------|-----|
| `font-sans` | `font-family: var(--font-sans)` |
| `font-serif` | `font-family: var(--font-serif)` |
| `font-mono` | `font-family: var(--font-mono)` |
| `text-xs` | `font-size: 0.75rem; line-height: 1rem` |
| `text-sm` | `font-size: 0.875rem; line-height: 1.25rem` |
| `text-base` | `font-size: 1rem; line-height: 1.5rem` |
| `text-lg` | `font-size: 1.125rem; line-height: 1.75rem` |
| `text-xl` | `font-size: 1.25rem; line-height: 1.75rem` |
| `text-2xl` | `font-size: 1.5rem; line-height: 2rem` |
| `text-3xl` | `font-size: 1.875rem; line-height: 2.25rem` |
| `text-4xl` | `font-size: 2.25rem; line-height: 2.5rem` |
| `text-5xl` | `font-size: 3rem; line-height: 1` |
| `text-6xl` | `font-size: 3.75rem; line-height: 1` |
| `text-7xl` | `font-size: 4.5rem; line-height: 1` |
| `text-8xl` | `font-size: 6rem; line-height: 1` |
| `text-9xl` | `font-size: 8rem; line-height: 1` |
| `font-thin` | `font-weight: 100` |
| `font-extralight` | `font-weight: 200` |
| `font-light` | `font-weight: 300` |
| `font-normal` | `font-weight: 400` |
| `font-medium` | `font-weight: 500` |
| `font-semibold` | `font-weight: 600` |
| `font-bold` | `font-weight: 700` |
| `font-extrabold` | `font-weight: 800` |
| `font-black` | `font-weight: 900` |
| `italic` | `font-style: italic` |
| `not-italic` | `font-style: normal` |
| `antialiased` | `-webkit-font-smoothing: antialiased; -moz-osx-font-smoothing: grayscale` |
| `subpixel-antialiased` | `-webkit-font-smoothing: auto; -moz-osx-font-smoothing: auto` |

## Typography — Spacing

| Classe | CSS |
|--------|-----|
| `leading-none` | `line-height: 1` |
| `leading-tight` | `line-height: 1.25` |
| `leading-snug` | `line-height: 1.375` |
| `leading-normal` | `line-height: 1.5` |
| `leading-relaxed` | `line-height: 1.625` |
| `leading-loose` | `line-height: 2` |
| `leading-3` | `line-height: 0.75rem` |
| `leading-4` | `line-height: 1rem` |
| `leading-5` | `line-height: 1.25rem` |
| `leading-6` | `line-height: 1.5rem` |
| `leading-7` | `line-height: 1.75rem` |
| `leading-8` | `line-height: 2rem` |
| `leading-9` | `line-height: 2.25rem` |
| `leading-10` | `line-height: 2.5rem` |
| `tracking-tighter` | `letter-spacing: -0.05em` |
| `tracking-tight` | `letter-spacing: -0.025em` |
| `tracking-normal` | `letter-spacing: 0em` |
| `tracking-wide` | `letter-spacing: 0.025em` |
| `tracking-wider` | `letter-spacing: 0.05em` |
| `tracking-widest` | `letter-spacing: 0.1em` |

## Typography — Text

| Classe | CSS |
|--------|-----|
| `text-left` | `text-align: left` |
| `text-center` | `text-align: center` |
| `text-right` | `text-align: right` |
| `text-justify` | `text-align: justify` |
| `text-start` | `text-align: start` |
| `text-end` | `text-align: end` |
| `underline` | `text-decoration-line: underline` |
| `overline` | `text-decoration-line: overline` |
| `line-through` | `text-decoration-line: line-through` |
| `no-underline` | `text-decoration-line: none` |
| `decoration-solid` | `text-decoration-style: solid` |
| `decoration-double` | `text-decoration-style: double` |
| `decoration-dotted` | `text-decoration-style: dotted` |
| `decoration-dashed` | `text-decoration-style: dashed` |
| `decoration-wavy` | `text-decoration-style: wavy` |
| `decoration-0` | `text-decoration-thickness: 0px` |
| `decoration-1` | `text-decoration-thickness: 1px` |
| `decoration-2` | `text-decoration-thickness: 2px` |
| `decoration-4` | `text-decoration-thickness: 4px` |
| `decoration-8` | `text-decoration-thickness: 8px` |
| `underline-offset-auto` | `text-underline-offset: auto` |
| `underline-offset-0` | `text-underline-offset: 0px` |
| `underline-offset-1` | `text-underline-offset: 1px` |
| `underline-offset-2` | `text-underline-offset: 2px` |
| `underline-offset-4` | `text-underline-offset: 4px` |
| `underline-offset-8` | `text-underline-offset: 8px` |
| `uppercase` | `text-transform: uppercase` |
| `lowercase` | `text-transform: lowercase` |
| `capitalize` | `text-transform: capitalize` |
| `normal-case` | `text-transform: none` |
| `truncate` | `overflow: hidden; text-overflow: ellipsis; white-space: nowrap` |
| `text-ellipsis` | `text-overflow: ellipsis` |
| `text-clip` | `text-overflow: clip` |
| `indent-0` | `text-indent: 0px` |
| `indent-px` | `text-indent: 1px` |
| `indent-0.5` | `text-indent: calc(var(--spacing) * 0.5)` |
| `indent-1` | `text-indent: calc(var(--spacing) * 1)` |
| `indent-1.5` | `text-indent: calc(var(--spacing) * 1.5)` |
| `indent-2` | `text-indent: calc(var(--spacing) * 2)` |
| `indent-2.5` | `text-indent: calc(var(--spacing) * 2.5)` |
| `indent-3` | `text-indent: calc(var(--spacing) * 3)` |
| `indent-3.5` | `text-indent: calc(var(--spacing) * 3.5)` |
| `indent-4` | `text-indent: calc(var(--spacing) * 4)` |
| `indent-5` | `text-indent: calc(var(--spacing) * 5)` |
| `indent-6` | `text-indent: calc(var(--spacing) * 6)` |
| `indent-7` | `text-indent: calc(var(--spacing) * 7)` |
| `indent-8` | `text-indent: calc(var(--spacing) * 8)` |
| `indent-9` | `text-indent: calc(var(--spacing) * 9)` |
| `indent-10` | `text-indent: calc(var(--spacing) * 10)` |
| `indent-11` | `text-indent: calc(var(--spacing) * 11)` |
| `indent-12` | `text-indent: calc(var(--spacing) * 12)` |
| `indent-14` | `text-indent: calc(var(--spacing) * 14)` |
| `indent-16` | `text-indent: calc(var(--spacing) * 16)` |
| `indent-20` | `text-indent: calc(var(--spacing) * 20)` |
| `indent-24` | `text-indent: calc(var(--spacing) * 24)` |
| `indent-28` | `text-indent: calc(var(--spacing) * 28)` |
| `indent-32` | `text-indent: calc(var(--spacing) * 32)` |
| `indent-36` | `text-indent: calc(var(--spacing) * 36)` |
| `indent-40` | `text-indent: calc(var(--spacing) * 40)` |
| `indent-44` | `text-indent: calc(var(--spacing) * 44)` |
| `indent-48` | `text-indent: calc(var(--spacing) * 48)` |
| `indent-52` | `text-indent: calc(var(--spacing) * 52)` |
| `indent-56` | `text-indent: calc(var(--spacing) * 56)` |
| `indent-60` | `text-indent: calc(var(--spacing) * 60)` |
| `indent-64` | `text-indent: calc(var(--spacing) * 64)` |
| `indent-72` | `text-indent: calc(var(--spacing) * 72)` |
| `indent-80` | `text-indent: calc(var(--spacing) * 80)` |
| `indent-96` | `text-indent: calc(var(--spacing) * 96)` |
| `align-baseline` | `vertical-align: baseline` |
| `align-top` | `vertical-align: top` |
| `align-middle` | `vertical-align: middle` |
| `align-bottom` | `vertical-align: bottom` |
| `align-text-top` | `vertical-align: text-top` |
| `align-text-bottom` | `vertical-align: text-bottom` |
| `whitespace-normal` | `white-space: normal` |
| `whitespace-nowrap` | `white-space: nowrap` |
| `whitespace-pre` | `white-space: pre` |
| `whitespace-pre-line` | `white-space: pre-line` |
| `whitespace-pre-wrap` | `white-space: pre-wrap` |
| `whitespace-break-spaces` | `white-space: break-spaces` |
| `break-normal` | `overflow-wrap: normal; word-break: normal` |
| `break-words` | `overflow-wrap: break-word` |
| `break-all` | `word-break: break-all` |
| `break-keep` | `word-break: keep-all` |
| `list-none` | `list-style-type: none` |
| `list-disc` | `list-style-type: disc` |
| `list-decimal` | `list-style-type: decimal` |
| `list-inside` | `list-style-position: inside` |
| `list-outside` | `list-style-position: outside` |

## Typography — Text Color

| Classe | CSS |
|--------|-----|
| `text-slate-50` | `color: var(--color-slate-50)` |
| `text-slate-100` | `color: var(--color-slate-100)` |
| `text-slate-200` | `color: var(--color-slate-200)` |
| `text-slate-300` | `color: var(--color-slate-300)` |
| `text-slate-400` | `color: var(--color-slate-400)` |
| `text-slate-500` | `color: var(--color-slate-500)` |
| `text-slate-600` | `color: var(--color-slate-600)` |
| `text-slate-700` | `color: var(--color-slate-700)` |
| `text-slate-800` | `color: var(--color-slate-800)` |
| `text-slate-900` | `color: var(--color-slate-900)` |
| `text-slate-950` | `color: var(--color-slate-950)` |
| `text-gray-50` | `color: var(--color-gray-50)` |
| `text-gray-100` | `color: var(--color-gray-100)` |
| `text-gray-200` | `color: var(--color-gray-200)` |
| `text-gray-300` | `color: var(--color-gray-300)` |
| `text-gray-400` | `color: var(--color-gray-400)` |
| `text-gray-500` | `color: var(--color-gray-500)` |
| `text-gray-600` | `color: var(--color-gray-600)` |
| `text-gray-700` | `color: var(--color-gray-700)` |
| `text-gray-800` | `color: var(--color-gray-800)` |
| `text-gray-900` | `color: var(--color-gray-900)` |
| `text-gray-950` | `color: var(--color-gray-950)` |
| `text-zinc-50` | `color: var(--color-zinc-50)` |
| `text-zinc-100` | `color: var(--color-zinc-100)` |
| `text-zinc-200` | `color: var(--color-zinc-200)` |
| `text-zinc-300` | `color: var(--color-zinc-300)` |
| `text-zinc-400` | `color: var(--color-zinc-400)` |
| `text-zinc-500` | `color: var(--color-zinc-500)` |
| `text-zinc-600` | `color: var(--color-zinc-600)` |
| `text-zinc-700` | `color: var(--color-zinc-700)` |
| `text-zinc-800` | `color: var(--color-zinc-800)` |
| `text-zinc-900` | `color: var(--color-zinc-900)` |
| `text-zinc-950` | `color: var(--color-zinc-950)` |
| `text-neutral-50` | `color: var(--color-neutral-50)` |
| `text-neutral-100` | `color: var(--color-neutral-100)` |
| `text-neutral-200` | `color: var(--color-neutral-200)` |
| `text-neutral-300` | `color: var(--color-neutral-300)` |
| `text-neutral-400` | `color: var(--color-neutral-400)` |
| `text-neutral-500` | `color: var(--color-neutral-500)` |
| `text-neutral-600` | `color: var(--color-neutral-600)` |
| `text-neutral-700` | `color: var(--color-neutral-700)` |
| `text-neutral-800` | `color: var(--color-neutral-800)` |
| `text-neutral-900` | `color: var(--color-neutral-900)` |
| `text-neutral-950` | `color: var(--color-neutral-950)` |
| `text-stone-50` | `color: var(--color-stone-50)` |
| `text-stone-100` | `color: var(--color-stone-100)` |
| `text-stone-200` | `color: var(--color-stone-200)` |
| `text-stone-300` | `color: var(--color-stone-300)` |
| `text-stone-400` | `color: var(--color-stone-400)` |
| `text-stone-500` | `color: var(--color-stone-500)` |
| `text-stone-600` | `color: var(--color-stone-600)` |
| `text-stone-700` | `color: var(--color-stone-700)` |
| `text-stone-800` | `color: var(--color-stone-800)` |
| `text-stone-900` | `color: var(--color-stone-900)` |
| `text-stone-950` | `color: var(--color-stone-950)` |
| `text-red-50` | `color: var(--color-red-50)` |
| `text-red-100` | `color: var(--color-red-100)` |
| `text-red-200` | `color: var(--color-red-200)` |
| `text-red-300` | `color: var(--color-red-300)` |
| `text-red-400` | `color: var(--color-red-400)` |
| `text-red-500` | `color: var(--color-red-500)` |
| `text-red-600` | `color: var(--color-red-600)` |
| `text-red-700` | `color: var(--color-red-700)` |
| `text-red-800` | `color: var(--color-red-800)` |
| `text-red-900` | `color: var(--color-red-900)` |
| `text-red-950` | `color: var(--color-red-950)` |
| `text-orange-50` | `color: var(--color-orange-50)` |
| `text-orange-100` | `color: var(--color-orange-100)` |
| `text-orange-200` | `color: var(--color-orange-200)` |
| `text-orange-300` | `color: var(--color-orange-300)` |
| `text-orange-400` | `color: var(--color-orange-400)` |
| `text-orange-500` | `color: var(--color-orange-500)` |
| `text-orange-600` | `color: var(--color-orange-600)` |
| `text-orange-700` | `color: var(--color-orange-700)` |
| `text-orange-800` | `color: var(--color-orange-800)` |
| `text-orange-900` | `color: var(--color-orange-900)` |
| `text-orange-950` | `color: var(--color-orange-950)` |
| `text-amber-50` | `color: var(--color-amber-50)` |
| `text-amber-100` | `color: var(--color-amber-100)` |
| `text-amber-200` | `color: var(--color-amber-200)` |
| `text-amber-300` | `color: var(--color-amber-300)` |
| `text-amber-400` | `color: var(--color-amber-400)` |
| `text-amber-500` | `color: var(--color-amber-500)` |
| `text-amber-600` | `color: var(--color-amber-600)` |
| `text-amber-700` | `color: var(--color-amber-700)` |
| `text-amber-800` | `color: var(--color-amber-800)` |
| `text-amber-900` | `color: var(--color-amber-900)` |
| `text-amber-950` | `color: var(--color-amber-950)` |
| `text-yellow-50` | `color: var(--color-yellow-50)` |
| `text-yellow-100` | `color: var(--color-yellow-100)` |
| `text-yellow-200` | `color: var(--color-yellow-200)` |
| `text-yellow-300` | `color: var(--color-yellow-300)` |
| `text-yellow-400` | `color: var(--color-yellow-400)` |
| `text-yellow-500` | `color: var(--color-yellow-500)` |
| `text-yellow-600` | `color: var(--color-yellow-600)` |
| `text-yellow-700` | `color: var(--color-yellow-700)` |
| `text-yellow-800` | `color: var(--color-yellow-800)` |
| `text-yellow-900` | `color: var(--color-yellow-900)` |
| `text-yellow-950` | `color: var(--color-yellow-950)` |
| `text-lime-50` | `color: var(--color-lime-50)` |
| `text-lime-100` | `color: var(--color-lime-100)` |
| `text-lime-200` | `color: var(--color-lime-200)` |
| `text-lime-300` | `color: var(--color-lime-300)` |
| `text-lime-400` | `color: var(--color-lime-400)` |
| `text-lime-500` | `color: var(--color-lime-500)` |
| `text-lime-600` | `color: var(--color-lime-600)` |
| `text-lime-700` | `color: var(--color-lime-700)` |
| `text-lime-800` | `color: var(--color-lime-800)` |
| `text-lime-900` | `color: var(--color-lime-900)` |
| `text-lime-950` | `color: var(--color-lime-950)` |
| `text-green-50` | `color: var(--color-green-50)` |
| `text-green-100` | `color: var(--color-green-100)` |
| `text-green-200` | `color: var(--color-green-200)` |
| `text-green-300` | `color: var(--color-green-300)` |
| `text-green-400` | `color: var(--color-green-400)` |
| `text-green-500` | `color: var(--color-green-500)` |
| `text-green-600` | `color: var(--color-green-600)` |
| `text-green-700` | `color: var(--color-green-700)` |
| `text-green-800` | `color: var(--color-green-800)` |
| `text-green-900` | `color: var(--color-green-900)` |
| `text-green-950` | `color: var(--color-green-950)` |
| `text-emerald-50` | `color: var(--color-emerald-50)` |
| `text-emerald-100` | `color: var(--color-emerald-100)` |
| `text-emerald-200` | `color: var(--color-emerald-200)` |
| `text-emerald-300` | `color: var(--color-emerald-300)` |
| `text-emerald-400` | `color: var(--color-emerald-400)` |
| `text-emerald-500` | `color: var(--color-emerald-500)` |
| `text-emerald-600` | `color: var(--color-emerald-600)` |
| `text-emerald-700` | `color: var(--color-emerald-700)` |
| `text-emerald-800` | `color: var(--color-emerald-800)` |
| `text-emerald-900` | `color: var(--color-emerald-900)` |
| `text-emerald-950` | `color: var(--color-emerald-950)` |
| `text-teal-50` | `color: var(--color-teal-50)` |
| `text-teal-100` | `color: var(--color-teal-100)` |
| `text-teal-200` | `color: var(--color-teal-200)` |
| `text-teal-300` | `color: var(--color-teal-300)` |
| `text-teal-400` | `color: var(--color-teal-400)` |
| `text-teal-500` | `color: var(--color-teal-500)` |
| `text-teal-600` | `color: var(--color-teal-600)` |
| `text-teal-700` | `color: var(--color-teal-700)` |
| `text-teal-800` | `color: var(--color-teal-800)` |
| `text-teal-900` | `color: var(--color-teal-900)` |
| `text-teal-950` | `color: var(--color-teal-950)` |
| `text-cyan-50` | `color: var(--color-cyan-50)` |
| `text-cyan-100` | `color: var(--color-cyan-100)` |
| `text-cyan-200` | `color: var(--color-cyan-200)` |
| `text-cyan-300` | `color: var(--color-cyan-300)` |
| `text-cyan-400` | `color: var(--color-cyan-400)` |
| `text-cyan-500` | `color: var(--color-cyan-500)` |
| `text-cyan-600` | `color: var(--color-cyan-600)` |
| `text-cyan-700` | `color: var(--color-cyan-700)` |
| `text-cyan-800` | `color: var(--color-cyan-800)` |
| `text-cyan-900` | `color: var(--color-cyan-900)` |
| `text-cyan-950` | `color: var(--color-cyan-950)` |
| `text-sky-50` | `color: var(--color-sky-50)` |
| `text-sky-100` | `color: var(--color-sky-100)` |
| `text-sky-200` | `color: var(--color-sky-200)` |
| `text-sky-300` | `color: var(--color-sky-300)` |
| `text-sky-400` | `color: var(--color-sky-400)` |
| `text-sky-500` | `color: var(--color-sky-500)` |
| `text-sky-600` | `color: var(--color-sky-600)` |
| `text-sky-700` | `color: var(--color-sky-700)` |
| `text-sky-800` | `color: var(--color-sky-800)` |
| `text-sky-900` | `color: var(--color-sky-900)` |
| `text-sky-950` | `color: var(--color-sky-950)` |
| `text-blue-50` | `color: var(--color-blue-50)` |
| `text-blue-100` | `color: var(--color-blue-100)` |
| `text-blue-200` | `color: var(--color-blue-200)` |
| `text-blue-300` | `color: var(--color-blue-300)` |
| `text-blue-400` | `color: var(--color-blue-400)` |
| `text-blue-500` | `color: var(--color-blue-500)` |
| `text-blue-600` | `color: var(--color-blue-600)` |
| `text-blue-700` | `color: var(--color-blue-700)` |
| `text-blue-800` | `color: var(--color-blue-800)` |
| `text-blue-900` | `color: var(--color-blue-900)` |
| `text-blue-950` | `color: var(--color-blue-950)` |
| `text-indigo-50` | `color: var(--color-indigo-50)` |
| `text-indigo-100` | `color: var(--color-indigo-100)` |
| `text-indigo-200` | `color: var(--color-indigo-200)` |
| `text-indigo-300` | `color: var(--color-indigo-300)` |
| `text-indigo-400` | `color: var(--color-indigo-400)` |
| `text-indigo-500` | `color: var(--color-indigo-500)` |
| `text-indigo-600` | `color: var(--color-indigo-600)` |
| `text-indigo-700` | `color: var(--color-indigo-700)` |
| `text-indigo-800` | `color: var(--color-indigo-800)` |
| `text-indigo-900` | `color: var(--color-indigo-900)` |
| `text-indigo-950` | `color: var(--color-indigo-950)` |
| `text-violet-50` | `color: var(--color-violet-50)` |
| `text-violet-100` | `color: var(--color-violet-100)` |
| `text-violet-200` | `color: var(--color-violet-200)` |
| `text-violet-300` | `color: var(--color-violet-300)` |
| `text-violet-400` | `color: var(--color-violet-400)` |
| `text-violet-500` | `color: var(--color-violet-500)` |
| `text-violet-600` | `color: var(--color-violet-600)` |
| `text-violet-700` | `color: var(--color-violet-700)` |
| `text-violet-800` | `color: var(--color-violet-800)` |
| `text-violet-900` | `color: var(--color-violet-900)` |
| `text-violet-950` | `color: var(--color-violet-950)` |
| `text-purple-50` | `color: var(--color-purple-50)` |
| `text-purple-100` | `color: var(--color-purple-100)` |
| `text-purple-200` | `color: var(--color-purple-200)` |
| `text-purple-300` | `color: var(--color-purple-300)` |
| `text-purple-400` | `color: var(--color-purple-400)` |
| `text-purple-500` | `color: var(--color-purple-500)` |
| `text-purple-600` | `color: var(--color-purple-600)` |
| `text-purple-700` | `color: var(--color-purple-700)` |
| `text-purple-800` | `color: var(--color-purple-800)` |
| `text-purple-900` | `color: var(--color-purple-900)` |
| `text-purple-950` | `color: var(--color-purple-950)` |
| `text-fuchsia-50` | `color: var(--color-fuchsia-50)` |
| `text-fuchsia-100` | `color: var(--color-fuchsia-100)` |
| `text-fuchsia-200` | `color: var(--color-fuchsia-200)` |
| `text-fuchsia-300` | `color: var(--color-fuchsia-300)` |
| `text-fuchsia-400` | `color: var(--color-fuchsia-400)` |
| `text-fuchsia-500` | `color: var(--color-fuchsia-500)` |
| `text-fuchsia-600` | `color: var(--color-fuchsia-600)` |
| `text-fuchsia-700` | `color: var(--color-fuchsia-700)` |
| `text-fuchsia-800` | `color: var(--color-fuchsia-800)` |
| `text-fuchsia-900` | `color: var(--color-fuchsia-900)` |
| `text-fuchsia-950` | `color: var(--color-fuchsia-950)` |
| `text-pink-50` | `color: var(--color-pink-50)` |
| `text-pink-100` | `color: var(--color-pink-100)` |
| `text-pink-200` | `color: var(--color-pink-200)` |
| `text-pink-300` | `color: var(--color-pink-300)` |
| `text-pink-400` | `color: var(--color-pink-400)` |
| `text-pink-500` | `color: var(--color-pink-500)` |
| `text-pink-600` | `color: var(--color-pink-600)` |
| `text-pink-700` | `color: var(--color-pink-700)` |
| `text-pink-800` | `color: var(--color-pink-800)` |
| `text-pink-900` | `color: var(--color-pink-900)` |
| `text-pink-950` | `color: var(--color-pink-950)` |
| `text-rose-50` | `color: var(--color-rose-50)` |
| `text-rose-100` | `color: var(--color-rose-100)` |
| `text-rose-200` | `color: var(--color-rose-200)` |
| `text-rose-300` | `color: var(--color-rose-300)` |
| `text-rose-400` | `color: var(--color-rose-400)` |
| `text-rose-500` | `color: var(--color-rose-500)` |
| `text-rose-600` | `color: var(--color-rose-600)` |
| `text-rose-700` | `color: var(--color-rose-700)` |
| `text-rose-800` | `color: var(--color-rose-800)` |
| `text-rose-900` | `color: var(--color-rose-900)` |
| `text-rose-950` | `color: var(--color-rose-950)` |
| `text-white` | `color: white` |
| `text-black` | `color: black` |
| `text-transparent` | `color: transparent` |
| `text-inherit` | `color: inherit` |
| `text-current` | `color: current` |

## Backgrounds — Color

| Classe | CSS |
|--------|-----|
| `bg-slate-50` | `background-color: var(--color-slate-50)` |
| `bg-slate-100` | `background-color: var(--color-slate-100)` |
| `bg-slate-200` | `background-color: var(--color-slate-200)` |
| `bg-slate-300` | `background-color: var(--color-slate-300)` |
| `bg-slate-400` | `background-color: var(--color-slate-400)` |
| `bg-slate-500` | `background-color: var(--color-slate-500)` |
| `bg-slate-600` | `background-color: var(--color-slate-600)` |
| `bg-slate-700` | `background-color: var(--color-slate-700)` |
| `bg-slate-800` | `background-color: var(--color-slate-800)` |
| `bg-slate-900` | `background-color: var(--color-slate-900)` |
| `bg-slate-950` | `background-color: var(--color-slate-950)` |
| `bg-gray-50` | `background-color: var(--color-gray-50)` |
| `bg-gray-100` | `background-color: var(--color-gray-100)` |
| `bg-gray-200` | `background-color: var(--color-gray-200)` |
| `bg-gray-300` | `background-color: var(--color-gray-300)` |
| `bg-gray-400` | `background-color: var(--color-gray-400)` |
| `bg-gray-500` | `background-color: var(--color-gray-500)` |
| `bg-gray-600` | `background-color: var(--color-gray-600)` |
| `bg-gray-700` | `background-color: var(--color-gray-700)` |
| `bg-gray-800` | `background-color: var(--color-gray-800)` |
| `bg-gray-900` | `background-color: var(--color-gray-900)` |
| `bg-gray-950` | `background-color: var(--color-gray-950)` |
| `bg-zinc-50` | `background-color: var(--color-zinc-50)` |
| `bg-zinc-100` | `background-color: var(--color-zinc-100)` |
| `bg-zinc-200` | `background-color: var(--color-zinc-200)` |
| `bg-zinc-300` | `background-color: var(--color-zinc-300)` |
| `bg-zinc-400` | `background-color: var(--color-zinc-400)` |
| `bg-zinc-500` | `background-color: var(--color-zinc-500)` |
| `bg-zinc-600` | `background-color: var(--color-zinc-600)` |
| `bg-zinc-700` | `background-color: var(--color-zinc-700)` |
| `bg-zinc-800` | `background-color: var(--color-zinc-800)` |
| `bg-zinc-900` | `background-color: var(--color-zinc-900)` |
| `bg-zinc-950` | `background-color: var(--color-zinc-950)` |
| `bg-neutral-50` | `background-color: var(--color-neutral-50)` |
| `bg-neutral-100` | `background-color: var(--color-neutral-100)` |
| `bg-neutral-200` | `background-color: var(--color-neutral-200)` |
| `bg-neutral-300` | `background-color: var(--color-neutral-300)` |
| `bg-neutral-400` | `background-color: var(--color-neutral-400)` |
| `bg-neutral-500` | `background-color: var(--color-neutral-500)` |
| `bg-neutral-600` | `background-color: var(--color-neutral-600)` |
| `bg-neutral-700` | `background-color: var(--color-neutral-700)` |
| `bg-neutral-800` | `background-color: var(--color-neutral-800)` |
| `bg-neutral-900` | `background-color: var(--color-neutral-900)` |
| `bg-neutral-950` | `background-color: var(--color-neutral-950)` |
| `bg-stone-50` | `background-color: var(--color-stone-50)` |
| `bg-stone-100` | `background-color: var(--color-stone-100)` |
| `bg-stone-200` | `background-color: var(--color-stone-200)` |
| `bg-stone-300` | `background-color: var(--color-stone-300)` |
| `bg-stone-400` | `background-color: var(--color-stone-400)` |
| `bg-stone-500` | `background-color: var(--color-stone-500)` |
| `bg-stone-600` | `background-color: var(--color-stone-600)` |
| `bg-stone-700` | `background-color: var(--color-stone-700)` |
| `bg-stone-800` | `background-color: var(--color-stone-800)` |
| `bg-stone-900` | `background-color: var(--color-stone-900)` |
| `bg-stone-950` | `background-color: var(--color-stone-950)` |
| `bg-red-50` | `background-color: var(--color-red-50)` |
| `bg-red-100` | `background-color: var(--color-red-100)` |
| `bg-red-200` | `background-color: var(--color-red-200)` |
| `bg-red-300` | `background-color: var(--color-red-300)` |
| `bg-red-400` | `background-color: var(--color-red-400)` |
| `bg-red-500` | `background-color: var(--color-red-500)` |
| `bg-red-600` | `background-color: var(--color-red-600)` |
| `bg-red-700` | `background-color: var(--color-red-700)` |
| `bg-red-800` | `background-color: var(--color-red-800)` |
| `bg-red-900` | `background-color: var(--color-red-900)` |
| `bg-red-950` | `background-color: var(--color-red-950)` |
| `bg-orange-50` | `background-color: var(--color-orange-50)` |
| `bg-orange-100` | `background-color: var(--color-orange-100)` |
| `bg-orange-200` | `background-color: var(--color-orange-200)` |
| `bg-orange-300` | `background-color: var(--color-orange-300)` |
| `bg-orange-400` | `background-color: var(--color-orange-400)` |
| `bg-orange-500` | `background-color: var(--color-orange-500)` |
| `bg-orange-600` | `background-color: var(--color-orange-600)` |
| `bg-orange-700` | `background-color: var(--color-orange-700)` |
| `bg-orange-800` | `background-color: var(--color-orange-800)` |
| `bg-orange-900` | `background-color: var(--color-orange-900)` |
| `bg-orange-950` | `background-color: var(--color-orange-950)` |
| `bg-amber-50` | `background-color: var(--color-amber-50)` |
| `bg-amber-100` | `background-color: var(--color-amber-100)` |
| `bg-amber-200` | `background-color: var(--color-amber-200)` |
| `bg-amber-300` | `background-color: var(--color-amber-300)` |
| `bg-amber-400` | `background-color: var(--color-amber-400)` |
| `bg-amber-500` | `background-color: var(--color-amber-500)` |
| `bg-amber-600` | `background-color: var(--color-amber-600)` |
| `bg-amber-700` | `background-color: var(--color-amber-700)` |
| `bg-amber-800` | `background-color: var(--color-amber-800)` |
| `bg-amber-900` | `background-color: var(--color-amber-900)` |
| `bg-amber-950` | `background-color: var(--color-amber-950)` |
| `bg-yellow-50` | `background-color: var(--color-yellow-50)` |
| `bg-yellow-100` | `background-color: var(--color-yellow-100)` |
| `bg-yellow-200` | `background-color: var(--color-yellow-200)` |
| `bg-yellow-300` | `background-color: var(--color-yellow-300)` |
| `bg-yellow-400` | `background-color: var(--color-yellow-400)` |
| `bg-yellow-500` | `background-color: var(--color-yellow-500)` |
| `bg-yellow-600` | `background-color: var(--color-yellow-600)` |
| `bg-yellow-700` | `background-color: var(--color-yellow-700)` |
| `bg-yellow-800` | `background-color: var(--color-yellow-800)` |
| `bg-yellow-900` | `background-color: var(--color-yellow-900)` |
| `bg-yellow-950` | `background-color: var(--color-yellow-950)` |
| `bg-lime-50` | `background-color: var(--color-lime-50)` |
| `bg-lime-100` | `background-color: var(--color-lime-100)` |
| `bg-lime-200` | `background-color: var(--color-lime-200)` |
| `bg-lime-300` | `background-color: var(--color-lime-300)` |
| `bg-lime-400` | `background-color: var(--color-lime-400)` |
| `bg-lime-500` | `background-color: var(--color-lime-500)` |
| `bg-lime-600` | `background-color: var(--color-lime-600)` |
| `bg-lime-700` | `background-color: var(--color-lime-700)` |
| `bg-lime-800` | `background-color: var(--color-lime-800)` |
| `bg-lime-900` | `background-color: var(--color-lime-900)` |
| `bg-lime-950` | `background-color: var(--color-lime-950)` |
| `bg-green-50` | `background-color: var(--color-green-50)` |
| `bg-green-100` | `background-color: var(--color-green-100)` |
| `bg-green-200` | `background-color: var(--color-green-200)` |
| `bg-green-300` | `background-color: var(--color-green-300)` |
| `bg-green-400` | `background-color: var(--color-green-400)` |
| `bg-green-500` | `background-color: var(--color-green-500)` |
| `bg-green-600` | `background-color: var(--color-green-600)` |
| `bg-green-700` | `background-color: var(--color-green-700)` |
| `bg-green-800` | `background-color: var(--color-green-800)` |
| `bg-green-900` | `background-color: var(--color-green-900)` |
| `bg-green-950` | `background-color: var(--color-green-950)` |
| `bg-emerald-50` | `background-color: var(--color-emerald-50)` |
| `bg-emerald-100` | `background-color: var(--color-emerald-100)` |
| `bg-emerald-200` | `background-color: var(--color-emerald-200)` |
| `bg-emerald-300` | `background-color: var(--color-emerald-300)` |
| `bg-emerald-400` | `background-color: var(--color-emerald-400)` |
| `bg-emerald-500` | `background-color: var(--color-emerald-500)` |
| `bg-emerald-600` | `background-color: var(--color-emerald-600)` |
| `bg-emerald-700` | `background-color: var(--color-emerald-700)` |
| `bg-emerald-800` | `background-color: var(--color-emerald-800)` |
| `bg-emerald-900` | `background-color: var(--color-emerald-900)` |
| `bg-emerald-950` | `background-color: var(--color-emerald-950)` |
| `bg-teal-50` | `background-color: var(--color-teal-50)` |
| `bg-teal-100` | `background-color: var(--color-teal-100)` |
| `bg-teal-200` | `background-color: var(--color-teal-200)` |
| `bg-teal-300` | `background-color: var(--color-teal-300)` |
| `bg-teal-400` | `background-color: var(--color-teal-400)` |
| `bg-teal-500` | `background-color: var(--color-teal-500)` |
| `bg-teal-600` | `background-color: var(--color-teal-600)` |
| `bg-teal-700` | `background-color: var(--color-teal-700)` |
| `bg-teal-800` | `background-color: var(--color-teal-800)` |
| `bg-teal-900` | `background-color: var(--color-teal-900)` |
| `bg-teal-950` | `background-color: var(--color-teal-950)` |
| `bg-cyan-50` | `background-color: var(--color-cyan-50)` |
| `bg-cyan-100` | `background-color: var(--color-cyan-100)` |
| `bg-cyan-200` | `background-color: var(--color-cyan-200)` |
| `bg-cyan-300` | `background-color: var(--color-cyan-300)` |
| `bg-cyan-400` | `background-color: var(--color-cyan-400)` |
| `bg-cyan-500` | `background-color: var(--color-cyan-500)` |
| `bg-cyan-600` | `background-color: var(--color-cyan-600)` |
| `bg-cyan-700` | `background-color: var(--color-cyan-700)` |
| `bg-cyan-800` | `background-color: var(--color-cyan-800)` |
| `bg-cyan-900` | `background-color: var(--color-cyan-900)` |
| `bg-cyan-950` | `background-color: var(--color-cyan-950)` |
| `bg-sky-50` | `background-color: var(--color-sky-50)` |
| `bg-sky-100` | `background-color: var(--color-sky-100)` |
| `bg-sky-200` | `background-color: var(--color-sky-200)` |
| `bg-sky-300` | `background-color: var(--color-sky-300)` |
| `bg-sky-400` | `background-color: var(--color-sky-400)` |
| `bg-sky-500` | `background-color: var(--color-sky-500)` |
| `bg-sky-600` | `background-color: var(--color-sky-600)` |
| `bg-sky-700` | `background-color: var(--color-sky-700)` |
| `bg-sky-800` | `background-color: var(--color-sky-800)` |
| `bg-sky-900` | `background-color: var(--color-sky-900)` |
| `bg-sky-950` | `background-color: var(--color-sky-950)` |
| `bg-blue-50` | `background-color: var(--color-blue-50)` |
| `bg-blue-100` | `background-color: var(--color-blue-100)` |
| `bg-blue-200` | `background-color: var(--color-blue-200)` |
| `bg-blue-300` | `background-color: var(--color-blue-300)` |
| `bg-blue-400` | `background-color: var(--color-blue-400)` |
| `bg-blue-500` | `background-color: var(--color-blue-500)` |
| `bg-blue-600` | `background-color: var(--color-blue-600)` |
| `bg-blue-700` | `background-color: var(--color-blue-700)` |
| `bg-blue-800` | `background-color: var(--color-blue-800)` |
| `bg-blue-900` | `background-color: var(--color-blue-900)` |
| `bg-blue-950` | `background-color: var(--color-blue-950)` |
| `bg-indigo-50` | `background-color: var(--color-indigo-50)` |
| `bg-indigo-100` | `background-color: var(--color-indigo-100)` |
| `bg-indigo-200` | `background-color: var(--color-indigo-200)` |
| `bg-indigo-300` | `background-color: var(--color-indigo-300)` |
| `bg-indigo-400` | `background-color: var(--color-indigo-400)` |
| `bg-indigo-500` | `background-color: var(--color-indigo-500)` |
| `bg-indigo-600` | `background-color: var(--color-indigo-600)` |
| `bg-indigo-700` | `background-color: var(--color-indigo-700)` |
| `bg-indigo-800` | `background-color: var(--color-indigo-800)` |
| `bg-indigo-900` | `background-color: var(--color-indigo-900)` |
| `bg-indigo-950` | `background-color: var(--color-indigo-950)` |
| `bg-violet-50` | `background-color: var(--color-violet-50)` |
| `bg-violet-100` | `background-color: var(--color-violet-100)` |
| `bg-violet-200` | `background-color: var(--color-violet-200)` |
| `bg-violet-300` | `background-color: var(--color-violet-300)` |
| `bg-violet-400` | `background-color: var(--color-violet-400)` |
| `bg-violet-500` | `background-color: var(--color-violet-500)` |
| `bg-violet-600` | `background-color: var(--color-violet-600)` |
| `bg-violet-700` | `background-color: var(--color-violet-700)` |
| `bg-violet-800` | `background-color: var(--color-violet-800)` |
| `bg-violet-900` | `background-color: var(--color-violet-900)` |
| `bg-violet-950` | `background-color: var(--color-violet-950)` |
| `bg-purple-50` | `background-color: var(--color-purple-50)` |
| `bg-purple-100` | `background-color: var(--color-purple-100)` |
| `bg-purple-200` | `background-color: var(--color-purple-200)` |
| `bg-purple-300` | `background-color: var(--color-purple-300)` |
| `bg-purple-400` | `background-color: var(--color-purple-400)` |
| `bg-purple-500` | `background-color: var(--color-purple-500)` |
| `bg-purple-600` | `background-color: var(--color-purple-600)` |
| `bg-purple-700` | `background-color: var(--color-purple-700)` |
| `bg-purple-800` | `background-color: var(--color-purple-800)` |
| `bg-purple-900` | `background-color: var(--color-purple-900)` |
| `bg-purple-950` | `background-color: var(--color-purple-950)` |
| `bg-fuchsia-50` | `background-color: var(--color-fuchsia-50)` |
| `bg-fuchsia-100` | `background-color: var(--color-fuchsia-100)` |
| `bg-fuchsia-200` | `background-color: var(--color-fuchsia-200)` |
| `bg-fuchsia-300` | `background-color: var(--color-fuchsia-300)` |
| `bg-fuchsia-400` | `background-color: var(--color-fuchsia-400)` |
| `bg-fuchsia-500` | `background-color: var(--color-fuchsia-500)` |
| `bg-fuchsia-600` | `background-color: var(--color-fuchsia-600)` |
| `bg-fuchsia-700` | `background-color: var(--color-fuchsia-700)` |
| `bg-fuchsia-800` | `background-color: var(--color-fuchsia-800)` |
| `bg-fuchsia-900` | `background-color: var(--color-fuchsia-900)` |
| `bg-fuchsia-950` | `background-color: var(--color-fuchsia-950)` |
| `bg-pink-50` | `background-color: var(--color-pink-50)` |
| `bg-pink-100` | `background-color: var(--color-pink-100)` |
| `bg-pink-200` | `background-color: var(--color-pink-200)` |
| `bg-pink-300` | `background-color: var(--color-pink-300)` |
| `bg-pink-400` | `background-color: var(--color-pink-400)` |
| `bg-pink-500` | `background-color: var(--color-pink-500)` |
| `bg-pink-600` | `background-color: var(--color-pink-600)` |
| `bg-pink-700` | `background-color: var(--color-pink-700)` |
| `bg-pink-800` | `background-color: var(--color-pink-800)` |
| `bg-pink-900` | `background-color: var(--color-pink-900)` |
| `bg-pink-950` | `background-color: var(--color-pink-950)` |
| `bg-rose-50` | `background-color: var(--color-rose-50)` |
| `bg-rose-100` | `background-color: var(--color-rose-100)` |
| `bg-rose-200` | `background-color: var(--color-rose-200)` |
| `bg-rose-300` | `background-color: var(--color-rose-300)` |
| `bg-rose-400` | `background-color: var(--color-rose-400)` |
| `bg-rose-500` | `background-color: var(--color-rose-500)` |
| `bg-rose-600` | `background-color: var(--color-rose-600)` |
| `bg-rose-700` | `background-color: var(--color-rose-700)` |
| `bg-rose-800` | `background-color: var(--color-rose-800)` |
| `bg-rose-900` | `background-color: var(--color-rose-900)` |
| `bg-rose-950` | `background-color: var(--color-rose-950)` |
| `bg-white` | `background-color: white` |
| `bg-black` | `background-color: black` |
| `bg-transparent` | `background-color: transparent` |
| `bg-inherit` | `background-color: inherit` |
| `bg-current` | `background-color: current` |

## Backgrounds — Other

| Classe | CSS |
|--------|-----|
| `bg-none` | `background-image: none` |
| `bg-gradient-to-t` | `background-image: linear-gradient(to top, var(--tw-gradient-stops))` |
| `bg-gradient-to-tr` | `background-image: linear-gradient(to top right, var(--tw-gradient-stops))` |
| `bg-gradient-to-r` | `background-image: linear-gradient(to right, var(--tw-gradient-stops))` |
| `bg-gradient-to-br` | `background-image: linear-gradient(to bottom right, var(--tw-gradient-stops))` |
| `bg-gradient-to-b` | `background-image: linear-gradient(to bottom, var(--tw-gradient-stops))` |
| `bg-gradient-to-bl` | `background-image: linear-gradient(to bottom left, var(--tw-gradient-stops))` |
| `bg-gradient-to-l` | `background-image: linear-gradient(to left, var(--tw-gradient-stops))` |
| `bg-gradient-to-tl` | `background-image: linear-gradient(to top left, var(--tw-gradient-stops))` |
| `bg-auto` | `background-size: auto` |
| `bg-cover` | `background-size: cover` |
| `bg-contain` | `background-size: contain` |
| `bg-center` | `background-position: center` |
| `bg-top` | `background-position: top` |
| `bg-right` | `background-position: right` |
| `bg-bottom` | `background-position: bottom` |
| `bg-left` | `background-position: left` |
| `bg-repeat` | `background-repeat: repeat` |
| `bg-no-repeat` | `background-repeat: no-repeat` |
| `bg-repeat-x` | `background-repeat: repeat-x` |
| `bg-repeat-y` | `background-repeat: repeat-y` |
| `bg-fixed` | `background-attachment: fixed` |
| `bg-local` | `background-attachment: local` |
| `bg-scroll` | `background-attachment: scroll` |
| `bg-clip-border` | `background-clip: border-box` |
| `bg-clip-padding` | `background-clip: padding-box` |
| `bg-clip-content` | `background-clip: content-box` |
| `bg-clip-text` | `background-clip: text; -webkit-background-clip: text` |

## Backgrounds — Gradient From

| Classe | CSS |
|--------|-----|
| `from-slate-50` | `--tw-gradient-from: var(--color-slate-50)` |
| `from-slate-100` | `--tw-gradient-from: var(--color-slate-100)` |
| `from-slate-200` | `--tw-gradient-from: var(--color-slate-200)` |
| `from-slate-300` | `--tw-gradient-from: var(--color-slate-300)` |
| `from-slate-400` | `--tw-gradient-from: var(--color-slate-400)` |
| `from-slate-500` | `--tw-gradient-from: var(--color-slate-500)` |
| `from-slate-600` | `--tw-gradient-from: var(--color-slate-600)` |
| `from-slate-700` | `--tw-gradient-from: var(--color-slate-700)` |
| `from-slate-800` | `--tw-gradient-from: var(--color-slate-800)` |
| `from-slate-900` | `--tw-gradient-from: var(--color-slate-900)` |
| `from-slate-950` | `--tw-gradient-from: var(--color-slate-950)` |
| `from-gray-50` | `--tw-gradient-from: var(--color-gray-50)` |
| `from-gray-100` | `--tw-gradient-from: var(--color-gray-100)` |
| `from-gray-200` | `--tw-gradient-from: var(--color-gray-200)` |
| `from-gray-300` | `--tw-gradient-from: var(--color-gray-300)` |
| `from-gray-400` | `--tw-gradient-from: var(--color-gray-400)` |
| `from-gray-500` | `--tw-gradient-from: var(--color-gray-500)` |
| `from-gray-600` | `--tw-gradient-from: var(--color-gray-600)` |
| `from-gray-700` | `--tw-gradient-from: var(--color-gray-700)` |
| `from-gray-800` | `--tw-gradient-from: var(--color-gray-800)` |
| `from-gray-900` | `--tw-gradient-from: var(--color-gray-900)` |
| `from-gray-950` | `--tw-gradient-from: var(--color-gray-950)` |
| `from-zinc-50` | `--tw-gradient-from: var(--color-zinc-50)` |
| `from-zinc-100` | `--tw-gradient-from: var(--color-zinc-100)` |
| `from-zinc-200` | `--tw-gradient-from: var(--color-zinc-200)` |
| `from-zinc-300` | `--tw-gradient-from: var(--color-zinc-300)` |
| `from-zinc-400` | `--tw-gradient-from: var(--color-zinc-400)` |
| `from-zinc-500` | `--tw-gradient-from: var(--color-zinc-500)` |
| `from-zinc-600` | `--tw-gradient-from: var(--color-zinc-600)` |
| `from-zinc-700` | `--tw-gradient-from: var(--color-zinc-700)` |
| `from-zinc-800` | `--tw-gradient-from: var(--color-zinc-800)` |
| `from-zinc-900` | `--tw-gradient-from: var(--color-zinc-900)` |
| `from-zinc-950` | `--tw-gradient-from: var(--color-zinc-950)` |
| `from-neutral-50` | `--tw-gradient-from: var(--color-neutral-50)` |
| `from-neutral-100` | `--tw-gradient-from: var(--color-neutral-100)` |
| `from-neutral-200` | `--tw-gradient-from: var(--color-neutral-200)` |
| `from-neutral-300` | `--tw-gradient-from: var(--color-neutral-300)` |
| `from-neutral-400` | `--tw-gradient-from: var(--color-neutral-400)` |
| `from-neutral-500` | `--tw-gradient-from: var(--color-neutral-500)` |
| `from-neutral-600` | `--tw-gradient-from: var(--color-neutral-600)` |
| `from-neutral-700` | `--tw-gradient-from: var(--color-neutral-700)` |
| `from-neutral-800` | `--tw-gradient-from: var(--color-neutral-800)` |
| `from-neutral-900` | `--tw-gradient-from: var(--color-neutral-900)` |
| `from-neutral-950` | `--tw-gradient-from: var(--color-neutral-950)` |
| `from-stone-50` | `--tw-gradient-from: var(--color-stone-50)` |
| `from-stone-100` | `--tw-gradient-from: var(--color-stone-100)` |
| `from-stone-200` | `--tw-gradient-from: var(--color-stone-200)` |
| `from-stone-300` | `--tw-gradient-from: var(--color-stone-300)` |
| `from-stone-400` | `--tw-gradient-from: var(--color-stone-400)` |
| `from-stone-500` | `--tw-gradient-from: var(--color-stone-500)` |
| `from-stone-600` | `--tw-gradient-from: var(--color-stone-600)` |
| `from-stone-700` | `--tw-gradient-from: var(--color-stone-700)` |
| `from-stone-800` | `--tw-gradient-from: var(--color-stone-800)` |
| `from-stone-900` | `--tw-gradient-from: var(--color-stone-900)` |
| `from-stone-950` | `--tw-gradient-from: var(--color-stone-950)` |
| `from-red-50` | `--tw-gradient-from: var(--color-red-50)` |
| `from-red-100` | `--tw-gradient-from: var(--color-red-100)` |
| `from-red-200` | `--tw-gradient-from: var(--color-red-200)` |
| `from-red-300` | `--tw-gradient-from: var(--color-red-300)` |
| `from-red-400` | `--tw-gradient-from: var(--color-red-400)` |
| `from-red-500` | `--tw-gradient-from: var(--color-red-500)` |
| `from-red-600` | `--tw-gradient-from: var(--color-red-600)` |
| `from-red-700` | `--tw-gradient-from: var(--color-red-700)` |
| `from-red-800` | `--tw-gradient-from: var(--color-red-800)` |
| `from-red-900` | `--tw-gradient-from: var(--color-red-900)` |
| `from-red-950` | `--tw-gradient-from: var(--color-red-950)` |
| `from-orange-50` | `--tw-gradient-from: var(--color-orange-50)` |
| `from-orange-100` | `--tw-gradient-from: var(--color-orange-100)` |
| `from-orange-200` | `--tw-gradient-from: var(--color-orange-200)` |
| `from-orange-300` | `--tw-gradient-from: var(--color-orange-300)` |
| `from-orange-400` | `--tw-gradient-from: var(--color-orange-400)` |
| `from-orange-500` | `--tw-gradient-from: var(--color-orange-500)` |
| `from-orange-600` | `--tw-gradient-from: var(--color-orange-600)` |
| `from-orange-700` | `--tw-gradient-from: var(--color-orange-700)` |
| `from-orange-800` | `--tw-gradient-from: var(--color-orange-800)` |
| `from-orange-900` | `--tw-gradient-from: var(--color-orange-900)` |
| `from-orange-950` | `--tw-gradient-from: var(--color-orange-950)` |
| `from-amber-50` | `--tw-gradient-from: var(--color-amber-50)` |
| `from-amber-100` | `--tw-gradient-from: var(--color-amber-100)` |
| `from-amber-200` | `--tw-gradient-from: var(--color-amber-200)` |
| `from-amber-300` | `--tw-gradient-from: var(--color-amber-300)` |
| `from-amber-400` | `--tw-gradient-from: var(--color-amber-400)` |
| `from-amber-500` | `--tw-gradient-from: var(--color-amber-500)` |
| `from-amber-600` | `--tw-gradient-from: var(--color-amber-600)` |
| `from-amber-700` | `--tw-gradient-from: var(--color-amber-700)` |
| `from-amber-800` | `--tw-gradient-from: var(--color-amber-800)` |
| `from-amber-900` | `--tw-gradient-from: var(--color-amber-900)` |
| `from-amber-950` | `--tw-gradient-from: var(--color-amber-950)` |
| `from-yellow-50` | `--tw-gradient-from: var(--color-yellow-50)` |
| `from-yellow-100` | `--tw-gradient-from: var(--color-yellow-100)` |
| `from-yellow-200` | `--tw-gradient-from: var(--color-yellow-200)` |
| `from-yellow-300` | `--tw-gradient-from: var(--color-yellow-300)` |
| `from-yellow-400` | `--tw-gradient-from: var(--color-yellow-400)` |
| `from-yellow-500` | `--tw-gradient-from: var(--color-yellow-500)` |
| `from-yellow-600` | `--tw-gradient-from: var(--color-yellow-600)` |
| `from-yellow-700` | `--tw-gradient-from: var(--color-yellow-700)` |
| `from-yellow-800` | `--tw-gradient-from: var(--color-yellow-800)` |
| `from-yellow-900` | `--tw-gradient-from: var(--color-yellow-900)` |
| `from-yellow-950` | `--tw-gradient-from: var(--color-yellow-950)` |
| `from-lime-50` | `--tw-gradient-from: var(--color-lime-50)` |
| `from-lime-100` | `--tw-gradient-from: var(--color-lime-100)` |
| `from-lime-200` | `--tw-gradient-from: var(--color-lime-200)` |
| `from-lime-300` | `--tw-gradient-from: var(--color-lime-300)` |
| `from-lime-400` | `--tw-gradient-from: var(--color-lime-400)` |
| `from-lime-500` | `--tw-gradient-from: var(--color-lime-500)` |
| `from-lime-600` | `--tw-gradient-from: var(--color-lime-600)` |
| `from-lime-700` | `--tw-gradient-from: var(--color-lime-700)` |
| `from-lime-800` | `--tw-gradient-from: var(--color-lime-800)` |
| `from-lime-900` | `--tw-gradient-from: var(--color-lime-900)` |
| `from-lime-950` | `--tw-gradient-from: var(--color-lime-950)` |
| `from-green-50` | `--tw-gradient-from: var(--color-green-50)` |
| `from-green-100` | `--tw-gradient-from: var(--color-green-100)` |
| `from-green-200` | `--tw-gradient-from: var(--color-green-200)` |
| `from-green-300` | `--tw-gradient-from: var(--color-green-300)` |
| `from-green-400` | `--tw-gradient-from: var(--color-green-400)` |
| `from-green-500` | `--tw-gradient-from: var(--color-green-500)` |
| `from-green-600` | `--tw-gradient-from: var(--color-green-600)` |
| `from-green-700` | `--tw-gradient-from: var(--color-green-700)` |
| `from-green-800` | `--tw-gradient-from: var(--color-green-800)` |
| `from-green-900` | `--tw-gradient-from: var(--color-green-900)` |
| `from-green-950` | `--tw-gradient-from: var(--color-green-950)` |
| `from-emerald-50` | `--tw-gradient-from: var(--color-emerald-50)` |
| `from-emerald-100` | `--tw-gradient-from: var(--color-emerald-100)` |
| `from-emerald-200` | `--tw-gradient-from: var(--color-emerald-200)` |
| `from-emerald-300` | `--tw-gradient-from: var(--color-emerald-300)` |
| `from-emerald-400` | `--tw-gradient-from: var(--color-emerald-400)` |
| `from-emerald-500` | `--tw-gradient-from: var(--color-emerald-500)` |
| `from-emerald-600` | `--tw-gradient-from: var(--color-emerald-600)` |
| `from-emerald-700` | `--tw-gradient-from: var(--color-emerald-700)` |
| `from-emerald-800` | `--tw-gradient-from: var(--color-emerald-800)` |
| `from-emerald-900` | `--tw-gradient-from: var(--color-emerald-900)` |
| `from-emerald-950` | `--tw-gradient-from: var(--color-emerald-950)` |
| `from-teal-50` | `--tw-gradient-from: var(--color-teal-50)` |
| `from-teal-100` | `--tw-gradient-from: var(--color-teal-100)` |
| `from-teal-200` | `--tw-gradient-from: var(--color-teal-200)` |
| `from-teal-300` | `--tw-gradient-from: var(--color-teal-300)` |
| `from-teal-400` | `--tw-gradient-from: var(--color-teal-400)` |
| `from-teal-500` | `--tw-gradient-from: var(--color-teal-500)` |
| `from-teal-600` | `--tw-gradient-from: var(--color-teal-600)` |
| `from-teal-700` | `--tw-gradient-from: var(--color-teal-700)` |
| `from-teal-800` | `--tw-gradient-from: var(--color-teal-800)` |
| `from-teal-900` | `--tw-gradient-from: var(--color-teal-900)` |
| `from-teal-950` | `--tw-gradient-from: var(--color-teal-950)` |
| `from-cyan-50` | `--tw-gradient-from: var(--color-cyan-50)` |
| `from-cyan-100` | `--tw-gradient-from: var(--color-cyan-100)` |
| `from-cyan-200` | `--tw-gradient-from: var(--color-cyan-200)` |
| `from-cyan-300` | `--tw-gradient-from: var(--color-cyan-300)` |
| `from-cyan-400` | `--tw-gradient-from: var(--color-cyan-400)` |
| `from-cyan-500` | `--tw-gradient-from: var(--color-cyan-500)` |
| `from-cyan-600` | `--tw-gradient-from: var(--color-cyan-600)` |
| `from-cyan-700` | `--tw-gradient-from: var(--color-cyan-700)` |
| `from-cyan-800` | `--tw-gradient-from: var(--color-cyan-800)` |
| `from-cyan-900` | `--tw-gradient-from: var(--color-cyan-900)` |
| `from-cyan-950` | `--tw-gradient-from: var(--color-cyan-950)` |
| `from-sky-50` | `--tw-gradient-from: var(--color-sky-50)` |
| `from-sky-100` | `--tw-gradient-from: var(--color-sky-100)` |
| `from-sky-200` | `--tw-gradient-from: var(--color-sky-200)` |
| `from-sky-300` | `--tw-gradient-from: var(--color-sky-300)` |
| `from-sky-400` | `--tw-gradient-from: var(--color-sky-400)` |
| `from-sky-500` | `--tw-gradient-from: var(--color-sky-500)` |
| `from-sky-600` | `--tw-gradient-from: var(--color-sky-600)` |
| `from-sky-700` | `--tw-gradient-from: var(--color-sky-700)` |
| `from-sky-800` | `--tw-gradient-from: var(--color-sky-800)` |
| `from-sky-900` | `--tw-gradient-from: var(--color-sky-900)` |
| `from-sky-950` | `--tw-gradient-from: var(--color-sky-950)` |
| `from-blue-50` | `--tw-gradient-from: var(--color-blue-50)` |
| `from-blue-100` | `--tw-gradient-from: var(--color-blue-100)` |
| `from-blue-200` | `--tw-gradient-from: var(--color-blue-200)` |
| `from-blue-300` | `--tw-gradient-from: var(--color-blue-300)` |
| `from-blue-400` | `--tw-gradient-from: var(--color-blue-400)` |
| `from-blue-500` | `--tw-gradient-from: var(--color-blue-500)` |
| `from-blue-600` | `--tw-gradient-from: var(--color-blue-600)` |
| `from-blue-700` | `--tw-gradient-from: var(--color-blue-700)` |
| `from-blue-800` | `--tw-gradient-from: var(--color-blue-800)` |
| `from-blue-900` | `--tw-gradient-from: var(--color-blue-900)` |
| `from-blue-950` | `--tw-gradient-from: var(--color-blue-950)` |
| `from-indigo-50` | `--tw-gradient-from: var(--color-indigo-50)` |
| `from-indigo-100` | `--tw-gradient-from: var(--color-indigo-100)` |
| `from-indigo-200` | `--tw-gradient-from: var(--color-indigo-200)` |
| `from-indigo-300` | `--tw-gradient-from: var(--color-indigo-300)` |
| `from-indigo-400` | `--tw-gradient-from: var(--color-indigo-400)` |
| `from-indigo-500` | `--tw-gradient-from: var(--color-indigo-500)` |
| `from-indigo-600` | `--tw-gradient-from: var(--color-indigo-600)` |
| `from-indigo-700` | `--tw-gradient-from: var(--color-indigo-700)` |
| `from-indigo-800` | `--tw-gradient-from: var(--color-indigo-800)` |
| `from-indigo-900` | `--tw-gradient-from: var(--color-indigo-900)` |
| `from-indigo-950` | `--tw-gradient-from: var(--color-indigo-950)` |
| `from-violet-50` | `--tw-gradient-from: var(--color-violet-50)` |
| `from-violet-100` | `--tw-gradient-from: var(--color-violet-100)` |
| `from-violet-200` | `--tw-gradient-from: var(--color-violet-200)` |
| `from-violet-300` | `--tw-gradient-from: var(--color-violet-300)` |
| `from-violet-400` | `--tw-gradient-from: var(--color-violet-400)` |
| `from-violet-500` | `--tw-gradient-from: var(--color-violet-500)` |
| `from-violet-600` | `--tw-gradient-from: var(--color-violet-600)` |
| `from-violet-700` | `--tw-gradient-from: var(--color-violet-700)` |
| `from-violet-800` | `--tw-gradient-from: var(--color-violet-800)` |
| `from-violet-900` | `--tw-gradient-from: var(--color-violet-900)` |
| `from-violet-950` | `--tw-gradient-from: var(--color-violet-950)` |
| `from-purple-50` | `--tw-gradient-from: var(--color-purple-50)` |
| `from-purple-100` | `--tw-gradient-from: var(--color-purple-100)` |
| `from-purple-200` | `--tw-gradient-from: var(--color-purple-200)` |
| `from-purple-300` | `--tw-gradient-from: var(--color-purple-300)` |
| `from-purple-400` | `--tw-gradient-from: var(--color-purple-400)` |
| `from-purple-500` | `--tw-gradient-from: var(--color-purple-500)` |
| `from-purple-600` | `--tw-gradient-from: var(--color-purple-600)` |
| `from-purple-700` | `--tw-gradient-from: var(--color-purple-700)` |
| `from-purple-800` | `--tw-gradient-from: var(--color-purple-800)` |
| `from-purple-900` | `--tw-gradient-from: var(--color-purple-900)` |
| `from-purple-950` | `--tw-gradient-from: var(--color-purple-950)` |
| `from-fuchsia-50` | `--tw-gradient-from: var(--color-fuchsia-50)` |
| `from-fuchsia-100` | `--tw-gradient-from: var(--color-fuchsia-100)` |
| `from-fuchsia-200` | `--tw-gradient-from: var(--color-fuchsia-200)` |
| `from-fuchsia-300` | `--tw-gradient-from: var(--color-fuchsia-300)` |
| `from-fuchsia-400` | `--tw-gradient-from: var(--color-fuchsia-400)` |
| `from-fuchsia-500` | `--tw-gradient-from: var(--color-fuchsia-500)` |
| `from-fuchsia-600` | `--tw-gradient-from: var(--color-fuchsia-600)` |
| `from-fuchsia-700` | `--tw-gradient-from: var(--color-fuchsia-700)` |
| `from-fuchsia-800` | `--tw-gradient-from: var(--color-fuchsia-800)` |
| `from-fuchsia-900` | `--tw-gradient-from: var(--color-fuchsia-900)` |
| `from-fuchsia-950` | `--tw-gradient-from: var(--color-fuchsia-950)` |
| `from-pink-50` | `--tw-gradient-from: var(--color-pink-50)` |
| `from-pink-100` | `--tw-gradient-from: var(--color-pink-100)` |
| `from-pink-200` | `--tw-gradient-from: var(--color-pink-200)` |
| `from-pink-300` | `--tw-gradient-from: var(--color-pink-300)` |
| `from-pink-400` | `--tw-gradient-from: var(--color-pink-400)` |
| `from-pink-500` | `--tw-gradient-from: var(--color-pink-500)` |
| `from-pink-600` | `--tw-gradient-from: var(--color-pink-600)` |
| `from-pink-700` | `--tw-gradient-from: var(--color-pink-700)` |
| `from-pink-800` | `--tw-gradient-from: var(--color-pink-800)` |
| `from-pink-900` | `--tw-gradient-from: var(--color-pink-900)` |
| `from-pink-950` | `--tw-gradient-from: var(--color-pink-950)` |
| `from-rose-50` | `--tw-gradient-from: var(--color-rose-50)` |
| `from-rose-100` | `--tw-gradient-from: var(--color-rose-100)` |
| `from-rose-200` | `--tw-gradient-from: var(--color-rose-200)` |
| `from-rose-300` | `--tw-gradient-from: var(--color-rose-300)` |
| `from-rose-400` | `--tw-gradient-from: var(--color-rose-400)` |
| `from-rose-500` | `--tw-gradient-from: var(--color-rose-500)` |
| `from-rose-600` | `--tw-gradient-from: var(--color-rose-600)` |
| `from-rose-700` | `--tw-gradient-from: var(--color-rose-700)` |
| `from-rose-800` | `--tw-gradient-from: var(--color-rose-800)` |
| `from-rose-900` | `--tw-gradient-from: var(--color-rose-900)` |
| `from-rose-950` | `--tw-gradient-from: var(--color-rose-950)` |
| `from-white` | `--tw-gradient-from: white` |
| `from-black` | `--tw-gradient-from: black` |
| `from-transparent` | `--tw-gradient-from: transparent` |
| `from-inherit` | `--tw-gradient-from: inherit` |
| `from-current` | `--tw-gradient-from: current` |

## Backgrounds — Gradient Via

| Classe | CSS |
|--------|-----|
| `via-slate-50` | `--tw-gradient-via: var(--color-slate-50)` |
| `via-slate-100` | `--tw-gradient-via: var(--color-slate-100)` |
| `via-slate-200` | `--tw-gradient-via: var(--color-slate-200)` |
| `via-slate-300` | `--tw-gradient-via: var(--color-slate-300)` |
| `via-slate-400` | `--tw-gradient-via: var(--color-slate-400)` |
| `via-slate-500` | `--tw-gradient-via: var(--color-slate-500)` |
| `via-slate-600` | `--tw-gradient-via: var(--color-slate-600)` |
| `via-slate-700` | `--tw-gradient-via: var(--color-slate-700)` |
| `via-slate-800` | `--tw-gradient-via: var(--color-slate-800)` |
| `via-slate-900` | `--tw-gradient-via: var(--color-slate-900)` |
| `via-slate-950` | `--tw-gradient-via: var(--color-slate-950)` |
| `via-gray-50` | `--tw-gradient-via: var(--color-gray-50)` |
| `via-gray-100` | `--tw-gradient-via: var(--color-gray-100)` |
| `via-gray-200` | `--tw-gradient-via: var(--color-gray-200)` |
| `via-gray-300` | `--tw-gradient-via: var(--color-gray-300)` |
| `via-gray-400` | `--tw-gradient-via: var(--color-gray-400)` |
| `via-gray-500` | `--tw-gradient-via: var(--color-gray-500)` |
| `via-gray-600` | `--tw-gradient-via: var(--color-gray-600)` |
| `via-gray-700` | `--tw-gradient-via: var(--color-gray-700)` |
| `via-gray-800` | `--tw-gradient-via: var(--color-gray-800)` |
| `via-gray-900` | `--tw-gradient-via: var(--color-gray-900)` |
| `via-gray-950` | `--tw-gradient-via: var(--color-gray-950)` |
| `via-zinc-50` | `--tw-gradient-via: var(--color-zinc-50)` |
| `via-zinc-100` | `--tw-gradient-via: var(--color-zinc-100)` |
| `via-zinc-200` | `--tw-gradient-via: var(--color-zinc-200)` |
| `via-zinc-300` | `--tw-gradient-via: var(--color-zinc-300)` |
| `via-zinc-400` | `--tw-gradient-via: var(--color-zinc-400)` |
| `via-zinc-500` | `--tw-gradient-via: var(--color-zinc-500)` |
| `via-zinc-600` | `--tw-gradient-via: var(--color-zinc-600)` |
| `via-zinc-700` | `--tw-gradient-via: var(--color-zinc-700)` |
| `via-zinc-800` | `--tw-gradient-via: var(--color-zinc-800)` |
| `via-zinc-900` | `--tw-gradient-via: var(--color-zinc-900)` |
| `via-zinc-950` | `--tw-gradient-via: var(--color-zinc-950)` |
| `via-neutral-50` | `--tw-gradient-via: var(--color-neutral-50)` |
| `via-neutral-100` | `--tw-gradient-via: var(--color-neutral-100)` |
| `via-neutral-200` | `--tw-gradient-via: var(--color-neutral-200)` |
| `via-neutral-300` | `--tw-gradient-via: var(--color-neutral-300)` |
| `via-neutral-400` | `--tw-gradient-via: var(--color-neutral-400)` |
| `via-neutral-500` | `--tw-gradient-via: var(--color-neutral-500)` |
| `via-neutral-600` | `--tw-gradient-via: var(--color-neutral-600)` |
| `via-neutral-700` | `--tw-gradient-via: var(--color-neutral-700)` |
| `via-neutral-800` | `--tw-gradient-via: var(--color-neutral-800)` |
| `via-neutral-900` | `--tw-gradient-via: var(--color-neutral-900)` |
| `via-neutral-950` | `--tw-gradient-via: var(--color-neutral-950)` |
| `via-stone-50` | `--tw-gradient-via: var(--color-stone-50)` |
| `via-stone-100` | `--tw-gradient-via: var(--color-stone-100)` |
| `via-stone-200` | `--tw-gradient-via: var(--color-stone-200)` |
| `via-stone-300` | `--tw-gradient-via: var(--color-stone-300)` |
| `via-stone-400` | `--tw-gradient-via: var(--color-stone-400)` |
| `via-stone-500` | `--tw-gradient-via: var(--color-stone-500)` |
| `via-stone-600` | `--tw-gradient-via: var(--color-stone-600)` |
| `via-stone-700` | `--tw-gradient-via: var(--color-stone-700)` |
| `via-stone-800` | `--tw-gradient-via: var(--color-stone-800)` |
| `via-stone-900` | `--tw-gradient-via: var(--color-stone-900)` |
| `via-stone-950` | `--tw-gradient-via: var(--color-stone-950)` |
| `via-red-50` | `--tw-gradient-via: var(--color-red-50)` |
| `via-red-100` | `--tw-gradient-via: var(--color-red-100)` |
| `via-red-200` | `--tw-gradient-via: var(--color-red-200)` |
| `via-red-300` | `--tw-gradient-via: var(--color-red-300)` |
| `via-red-400` | `--tw-gradient-via: var(--color-red-400)` |
| `via-red-500` | `--tw-gradient-via: var(--color-red-500)` |
| `via-red-600` | `--tw-gradient-via: var(--color-red-600)` |
| `via-red-700` | `--tw-gradient-via: var(--color-red-700)` |
| `via-red-800` | `--tw-gradient-via: var(--color-red-800)` |
| `via-red-900` | `--tw-gradient-via: var(--color-red-900)` |
| `via-red-950` | `--tw-gradient-via: var(--color-red-950)` |
| `via-orange-50` | `--tw-gradient-via: var(--color-orange-50)` |
| `via-orange-100` | `--tw-gradient-via: var(--color-orange-100)` |
| `via-orange-200` | `--tw-gradient-via: var(--color-orange-200)` |
| `via-orange-300` | `--tw-gradient-via: var(--color-orange-300)` |
| `via-orange-400` | `--tw-gradient-via: var(--color-orange-400)` |
| `via-orange-500` | `--tw-gradient-via: var(--color-orange-500)` |
| `via-orange-600` | `--tw-gradient-via: var(--color-orange-600)` |
| `via-orange-700` | `--tw-gradient-via: var(--color-orange-700)` |
| `via-orange-800` | `--tw-gradient-via: var(--color-orange-800)` |
| `via-orange-900` | `--tw-gradient-via: var(--color-orange-900)` |
| `via-orange-950` | `--tw-gradient-via: var(--color-orange-950)` |
| `via-amber-50` | `--tw-gradient-via: var(--color-amber-50)` |
| `via-amber-100` | `--tw-gradient-via: var(--color-amber-100)` |
| `via-amber-200` | `--tw-gradient-via: var(--color-amber-200)` |
| `via-amber-300` | `--tw-gradient-via: var(--color-amber-300)` |
| `via-amber-400` | `--tw-gradient-via: var(--color-amber-400)` |
| `via-amber-500` | `--tw-gradient-via: var(--color-amber-500)` |
| `via-amber-600` | `--tw-gradient-via: var(--color-amber-600)` |
| `via-amber-700` | `--tw-gradient-via: var(--color-amber-700)` |
| `via-amber-800` | `--tw-gradient-via: var(--color-amber-800)` |
| `via-amber-900` | `--tw-gradient-via: var(--color-amber-900)` |
| `via-amber-950` | `--tw-gradient-via: var(--color-amber-950)` |
| `via-yellow-50` | `--tw-gradient-via: var(--color-yellow-50)` |
| `via-yellow-100` | `--tw-gradient-via: var(--color-yellow-100)` |
| `via-yellow-200` | `--tw-gradient-via: var(--color-yellow-200)` |
| `via-yellow-300` | `--tw-gradient-via: var(--color-yellow-300)` |
| `via-yellow-400` | `--tw-gradient-via: var(--color-yellow-400)` |
| `via-yellow-500` | `--tw-gradient-via: var(--color-yellow-500)` |
| `via-yellow-600` | `--tw-gradient-via: var(--color-yellow-600)` |
| `via-yellow-700` | `--tw-gradient-via: var(--color-yellow-700)` |
| `via-yellow-800` | `--tw-gradient-via: var(--color-yellow-800)` |
| `via-yellow-900` | `--tw-gradient-via: var(--color-yellow-900)` |
| `via-yellow-950` | `--tw-gradient-via: var(--color-yellow-950)` |
| `via-lime-50` | `--tw-gradient-via: var(--color-lime-50)` |
| `via-lime-100` | `--tw-gradient-via: var(--color-lime-100)` |
| `via-lime-200` | `--tw-gradient-via: var(--color-lime-200)` |
| `via-lime-300` | `--tw-gradient-via: var(--color-lime-300)` |
| `via-lime-400` | `--tw-gradient-via: var(--color-lime-400)` |
| `via-lime-500` | `--tw-gradient-via: var(--color-lime-500)` |
| `via-lime-600` | `--tw-gradient-via: var(--color-lime-600)` |
| `via-lime-700` | `--tw-gradient-via: var(--color-lime-700)` |
| `via-lime-800` | `--tw-gradient-via: var(--color-lime-800)` |
| `via-lime-900` | `--tw-gradient-via: var(--color-lime-900)` |
| `via-lime-950` | `--tw-gradient-via: var(--color-lime-950)` |
| `via-green-50` | `--tw-gradient-via: var(--color-green-50)` |
| `via-green-100` | `--tw-gradient-via: var(--color-green-100)` |
| `via-green-200` | `--tw-gradient-via: var(--color-green-200)` |
| `via-green-300` | `--tw-gradient-via: var(--color-green-300)` |
| `via-green-400` | `--tw-gradient-via: var(--color-green-400)` |
| `via-green-500` | `--tw-gradient-via: var(--color-green-500)` |
| `via-green-600` | `--tw-gradient-via: var(--color-green-600)` |
| `via-green-700` | `--tw-gradient-via: var(--color-green-700)` |
| `via-green-800` | `--tw-gradient-via: var(--color-green-800)` |
| `via-green-900` | `--tw-gradient-via: var(--color-green-900)` |
| `via-green-950` | `--tw-gradient-via: var(--color-green-950)` |
| `via-emerald-50` | `--tw-gradient-via: var(--color-emerald-50)` |
| `via-emerald-100` | `--tw-gradient-via: var(--color-emerald-100)` |
| `via-emerald-200` | `--tw-gradient-via: var(--color-emerald-200)` |
| `via-emerald-300` | `--tw-gradient-via: var(--color-emerald-300)` |
| `via-emerald-400` | `--tw-gradient-via: var(--color-emerald-400)` |
| `via-emerald-500` | `--tw-gradient-via: var(--color-emerald-500)` |
| `via-emerald-600` | `--tw-gradient-via: var(--color-emerald-600)` |
| `via-emerald-700` | `--tw-gradient-via: var(--color-emerald-700)` |
| `via-emerald-800` | `--tw-gradient-via: var(--color-emerald-800)` |
| `via-emerald-900` | `--tw-gradient-via: var(--color-emerald-900)` |
| `via-emerald-950` | `--tw-gradient-via: var(--color-emerald-950)` |
| `via-teal-50` | `--tw-gradient-via: var(--color-teal-50)` |
| `via-teal-100` | `--tw-gradient-via: var(--color-teal-100)` |
| `via-teal-200` | `--tw-gradient-via: var(--color-teal-200)` |
| `via-teal-300` | `--tw-gradient-via: var(--color-teal-300)` |
| `via-teal-400` | `--tw-gradient-via: var(--color-teal-400)` |
| `via-teal-500` | `--tw-gradient-via: var(--color-teal-500)` |
| `via-teal-600` | `--tw-gradient-via: var(--color-teal-600)` |
| `via-teal-700` | `--tw-gradient-via: var(--color-teal-700)` |
| `via-teal-800` | `--tw-gradient-via: var(--color-teal-800)` |
| `via-teal-900` | `--tw-gradient-via: var(--color-teal-900)` |
| `via-teal-950` | `--tw-gradient-via: var(--color-teal-950)` |
| `via-cyan-50` | `--tw-gradient-via: var(--color-cyan-50)` |
| `via-cyan-100` | `--tw-gradient-via: var(--color-cyan-100)` |
| `via-cyan-200` | `--tw-gradient-via: var(--color-cyan-200)` |
| `via-cyan-300` | `--tw-gradient-via: var(--color-cyan-300)` |
| `via-cyan-400` | `--tw-gradient-via: var(--color-cyan-400)` |
| `via-cyan-500` | `--tw-gradient-via: var(--color-cyan-500)` |
| `via-cyan-600` | `--tw-gradient-via: var(--color-cyan-600)` |
| `via-cyan-700` | `--tw-gradient-via: var(--color-cyan-700)` |
| `via-cyan-800` | `--tw-gradient-via: var(--color-cyan-800)` |
| `via-cyan-900` | `--tw-gradient-via: var(--color-cyan-900)` |
| `via-cyan-950` | `--tw-gradient-via: var(--color-cyan-950)` |
| `via-sky-50` | `--tw-gradient-via: var(--color-sky-50)` |
| `via-sky-100` | `--tw-gradient-via: var(--color-sky-100)` |
| `via-sky-200` | `--tw-gradient-via: var(--color-sky-200)` |
| `via-sky-300` | `--tw-gradient-via: var(--color-sky-300)` |
| `via-sky-400` | `--tw-gradient-via: var(--color-sky-400)` |
| `via-sky-500` | `--tw-gradient-via: var(--color-sky-500)` |
| `via-sky-600` | `--tw-gradient-via: var(--color-sky-600)` |
| `via-sky-700` | `--tw-gradient-via: var(--color-sky-700)` |
| `via-sky-800` | `--tw-gradient-via: var(--color-sky-800)` |
| `via-sky-900` | `--tw-gradient-via: var(--color-sky-900)` |
| `via-sky-950` | `--tw-gradient-via: var(--color-sky-950)` |
| `via-blue-50` | `--tw-gradient-via: var(--color-blue-50)` |
| `via-blue-100` | `--tw-gradient-via: var(--color-blue-100)` |
| `via-blue-200` | `--tw-gradient-via: var(--color-blue-200)` |
| `via-blue-300` | `--tw-gradient-via: var(--color-blue-300)` |
| `via-blue-400` | `--tw-gradient-via: var(--color-blue-400)` |
| `via-blue-500` | `--tw-gradient-via: var(--color-blue-500)` |
| `via-blue-600` | `--tw-gradient-via: var(--color-blue-600)` |
| `via-blue-700` | `--tw-gradient-via: var(--color-blue-700)` |
| `via-blue-800` | `--tw-gradient-via: var(--color-blue-800)` |
| `via-blue-900` | `--tw-gradient-via: var(--color-blue-900)` |
| `via-blue-950` | `--tw-gradient-via: var(--color-blue-950)` |
| `via-indigo-50` | `--tw-gradient-via: var(--color-indigo-50)` |
| `via-indigo-100` | `--tw-gradient-via: var(--color-indigo-100)` |
| `via-indigo-200` | `--tw-gradient-via: var(--color-indigo-200)` |
| `via-indigo-300` | `--tw-gradient-via: var(--color-indigo-300)` |
| `via-indigo-400` | `--tw-gradient-via: var(--color-indigo-400)` |
| `via-indigo-500` | `--tw-gradient-via: var(--color-indigo-500)` |
| `via-indigo-600` | `--tw-gradient-via: var(--color-indigo-600)` |
| `via-indigo-700` | `--tw-gradient-via: var(--color-indigo-700)` |
| `via-indigo-800` | `--tw-gradient-via: var(--color-indigo-800)` |
| `via-indigo-900` | `--tw-gradient-via: var(--color-indigo-900)` |
| `via-indigo-950` | `--tw-gradient-via: var(--color-indigo-950)` |
| `via-violet-50` | `--tw-gradient-via: var(--color-violet-50)` |
| `via-violet-100` | `--tw-gradient-via: var(--color-violet-100)` |
| `via-violet-200` | `--tw-gradient-via: var(--color-violet-200)` |
| `via-violet-300` | `--tw-gradient-via: var(--color-violet-300)` |
| `via-violet-400` | `--tw-gradient-via: var(--color-violet-400)` |
| `via-violet-500` | `--tw-gradient-via: var(--color-violet-500)` |
| `via-violet-600` | `--tw-gradient-via: var(--color-violet-600)` |
| `via-violet-700` | `--tw-gradient-via: var(--color-violet-700)` |
| `via-violet-800` | `--tw-gradient-via: var(--color-violet-800)` |
| `via-violet-900` | `--tw-gradient-via: var(--color-violet-900)` |
| `via-violet-950` | `--tw-gradient-via: var(--color-violet-950)` |
| `via-purple-50` | `--tw-gradient-via: var(--color-purple-50)` |
| `via-purple-100` | `--tw-gradient-via: var(--color-purple-100)` |
| `via-purple-200` | `--tw-gradient-via: var(--color-purple-200)` |
| `via-purple-300` | `--tw-gradient-via: var(--color-purple-300)` |
| `via-purple-400` | `--tw-gradient-via: var(--color-purple-400)` |
| `via-purple-500` | `--tw-gradient-via: var(--color-purple-500)` |
| `via-purple-600` | `--tw-gradient-via: var(--color-purple-600)` |
| `via-purple-700` | `--tw-gradient-via: var(--color-purple-700)` |
| `via-purple-800` | `--tw-gradient-via: var(--color-purple-800)` |
| `via-purple-900` | `--tw-gradient-via: var(--color-purple-900)` |
| `via-purple-950` | `--tw-gradient-via: var(--color-purple-950)` |
| `via-fuchsia-50` | `--tw-gradient-via: var(--color-fuchsia-50)` |
| `via-fuchsia-100` | `--tw-gradient-via: var(--color-fuchsia-100)` |
| `via-fuchsia-200` | `--tw-gradient-via: var(--color-fuchsia-200)` |
| `via-fuchsia-300` | `--tw-gradient-via: var(--color-fuchsia-300)` |
| `via-fuchsia-400` | `--tw-gradient-via: var(--color-fuchsia-400)` |
| `via-fuchsia-500` | `--tw-gradient-via: var(--color-fuchsia-500)` |
| `via-fuchsia-600` | `--tw-gradient-via: var(--color-fuchsia-600)` |
| `via-fuchsia-700` | `--tw-gradient-via: var(--color-fuchsia-700)` |
| `via-fuchsia-800` | `--tw-gradient-via: var(--color-fuchsia-800)` |
| `via-fuchsia-900` | `--tw-gradient-via: var(--color-fuchsia-900)` |
| `via-fuchsia-950` | `--tw-gradient-via: var(--color-fuchsia-950)` |
| `via-pink-50` | `--tw-gradient-via: var(--color-pink-50)` |
| `via-pink-100` | `--tw-gradient-via: var(--color-pink-100)` |
| `via-pink-200` | `--tw-gradient-via: var(--color-pink-200)` |
| `via-pink-300` | `--tw-gradient-via: var(--color-pink-300)` |
| `via-pink-400` | `--tw-gradient-via: var(--color-pink-400)` |
| `via-pink-500` | `--tw-gradient-via: var(--color-pink-500)` |
| `via-pink-600` | `--tw-gradient-via: var(--color-pink-600)` |
| `via-pink-700` | `--tw-gradient-via: var(--color-pink-700)` |
| `via-pink-800` | `--tw-gradient-via: var(--color-pink-800)` |
| `via-pink-900` | `--tw-gradient-via: var(--color-pink-900)` |
| `via-pink-950` | `--tw-gradient-via: var(--color-pink-950)` |
| `via-rose-50` | `--tw-gradient-via: var(--color-rose-50)` |
| `via-rose-100` | `--tw-gradient-via: var(--color-rose-100)` |
| `via-rose-200` | `--tw-gradient-via: var(--color-rose-200)` |
| `via-rose-300` | `--tw-gradient-via: var(--color-rose-300)` |
| `via-rose-400` | `--tw-gradient-via: var(--color-rose-400)` |
| `via-rose-500` | `--tw-gradient-via: var(--color-rose-500)` |
| `via-rose-600` | `--tw-gradient-via: var(--color-rose-600)` |
| `via-rose-700` | `--tw-gradient-via: var(--color-rose-700)` |
| `via-rose-800` | `--tw-gradient-via: var(--color-rose-800)` |
| `via-rose-900` | `--tw-gradient-via: var(--color-rose-900)` |
| `via-rose-950` | `--tw-gradient-via: var(--color-rose-950)` |
| `via-white` | `--tw-gradient-via: white` |
| `via-black` | `--tw-gradient-via: black` |
| `via-transparent` | `--tw-gradient-via: transparent` |
| `via-inherit` | `--tw-gradient-via: inherit` |
| `via-current` | `--tw-gradient-via: current` |

## Backgrounds — Gradient To

| Classe | CSS |
|--------|-----|
| `to-slate-50` | `--tw-gradient-to: var(--color-slate-50)` |
| `to-slate-100` | `--tw-gradient-to: var(--color-slate-100)` |
| `to-slate-200` | `--tw-gradient-to: var(--color-slate-200)` |
| `to-slate-300` | `--tw-gradient-to: var(--color-slate-300)` |
| `to-slate-400` | `--tw-gradient-to: var(--color-slate-400)` |
| `to-slate-500` | `--tw-gradient-to: var(--color-slate-500)` |
| `to-slate-600` | `--tw-gradient-to: var(--color-slate-600)` |
| `to-slate-700` | `--tw-gradient-to: var(--color-slate-700)` |
| `to-slate-800` | `--tw-gradient-to: var(--color-slate-800)` |
| `to-slate-900` | `--tw-gradient-to: var(--color-slate-900)` |
| `to-slate-950` | `--tw-gradient-to: var(--color-slate-950)` |
| `to-gray-50` | `--tw-gradient-to: var(--color-gray-50)` |
| `to-gray-100` | `--tw-gradient-to: var(--color-gray-100)` |
| `to-gray-200` | `--tw-gradient-to: var(--color-gray-200)` |
| `to-gray-300` | `--tw-gradient-to: var(--color-gray-300)` |
| `to-gray-400` | `--tw-gradient-to: var(--color-gray-400)` |
| `to-gray-500` | `--tw-gradient-to: var(--color-gray-500)` |
| `to-gray-600` | `--tw-gradient-to: var(--color-gray-600)` |
| `to-gray-700` | `--tw-gradient-to: var(--color-gray-700)` |
| `to-gray-800` | `--tw-gradient-to: var(--color-gray-800)` |
| `to-gray-900` | `--tw-gradient-to: var(--color-gray-900)` |
| `to-gray-950` | `--tw-gradient-to: var(--color-gray-950)` |
| `to-zinc-50` | `--tw-gradient-to: var(--color-zinc-50)` |
| `to-zinc-100` | `--tw-gradient-to: var(--color-zinc-100)` |
| `to-zinc-200` | `--tw-gradient-to: var(--color-zinc-200)` |
| `to-zinc-300` | `--tw-gradient-to: var(--color-zinc-300)` |
| `to-zinc-400` | `--tw-gradient-to: var(--color-zinc-400)` |
| `to-zinc-500` | `--tw-gradient-to: var(--color-zinc-500)` |
| `to-zinc-600` | `--tw-gradient-to: var(--color-zinc-600)` |
| `to-zinc-700` | `--tw-gradient-to: var(--color-zinc-700)` |
| `to-zinc-800` | `--tw-gradient-to: var(--color-zinc-800)` |
| `to-zinc-900` | `--tw-gradient-to: var(--color-zinc-900)` |
| `to-zinc-950` | `--tw-gradient-to: var(--color-zinc-950)` |
| `to-neutral-50` | `--tw-gradient-to: var(--color-neutral-50)` |
| `to-neutral-100` | `--tw-gradient-to: var(--color-neutral-100)` |
| `to-neutral-200` | `--tw-gradient-to: var(--color-neutral-200)` |
| `to-neutral-300` | `--tw-gradient-to: var(--color-neutral-300)` |
| `to-neutral-400` | `--tw-gradient-to: var(--color-neutral-400)` |
| `to-neutral-500` | `--tw-gradient-to: var(--color-neutral-500)` |
| `to-neutral-600` | `--tw-gradient-to: var(--color-neutral-600)` |
| `to-neutral-700` | `--tw-gradient-to: var(--color-neutral-700)` |
| `to-neutral-800` | `--tw-gradient-to: var(--color-neutral-800)` |
| `to-neutral-900` | `--tw-gradient-to: var(--color-neutral-900)` |
| `to-neutral-950` | `--tw-gradient-to: var(--color-neutral-950)` |
| `to-stone-50` | `--tw-gradient-to: var(--color-stone-50)` |
| `to-stone-100` | `--tw-gradient-to: var(--color-stone-100)` |
| `to-stone-200` | `--tw-gradient-to: var(--color-stone-200)` |
| `to-stone-300` | `--tw-gradient-to: var(--color-stone-300)` |
| `to-stone-400` | `--tw-gradient-to: var(--color-stone-400)` |
| `to-stone-500` | `--tw-gradient-to: var(--color-stone-500)` |
| `to-stone-600` | `--tw-gradient-to: var(--color-stone-600)` |
| `to-stone-700` | `--tw-gradient-to: var(--color-stone-700)` |
| `to-stone-800` | `--tw-gradient-to: var(--color-stone-800)` |
| `to-stone-900` | `--tw-gradient-to: var(--color-stone-900)` |
| `to-stone-950` | `--tw-gradient-to: var(--color-stone-950)` |
| `to-red-50` | `--tw-gradient-to: var(--color-red-50)` |
| `to-red-100` | `--tw-gradient-to: var(--color-red-100)` |
| `to-red-200` | `--tw-gradient-to: var(--color-red-200)` |
| `to-red-300` | `--tw-gradient-to: var(--color-red-300)` |
| `to-red-400` | `--tw-gradient-to: var(--color-red-400)` |
| `to-red-500` | `--tw-gradient-to: var(--color-red-500)` |
| `to-red-600` | `--tw-gradient-to: var(--color-red-600)` |
| `to-red-700` | `--tw-gradient-to: var(--color-red-700)` |
| `to-red-800` | `--tw-gradient-to: var(--color-red-800)` |
| `to-red-900` | `--tw-gradient-to: var(--color-red-900)` |
| `to-red-950` | `--tw-gradient-to: var(--color-red-950)` |
| `to-orange-50` | `--tw-gradient-to: var(--color-orange-50)` |
| `to-orange-100` | `--tw-gradient-to: var(--color-orange-100)` |
| `to-orange-200` | `--tw-gradient-to: var(--color-orange-200)` |
| `to-orange-300` | `--tw-gradient-to: var(--color-orange-300)` |
| `to-orange-400` | `--tw-gradient-to: var(--color-orange-400)` |
| `to-orange-500` | `--tw-gradient-to: var(--color-orange-500)` |
| `to-orange-600` | `--tw-gradient-to: var(--color-orange-600)` |
| `to-orange-700` | `--tw-gradient-to: var(--color-orange-700)` |
| `to-orange-800` | `--tw-gradient-to: var(--color-orange-800)` |
| `to-orange-900` | `--tw-gradient-to: var(--color-orange-900)` |
| `to-orange-950` | `--tw-gradient-to: var(--color-orange-950)` |
| `to-amber-50` | `--tw-gradient-to: var(--color-amber-50)` |
| `to-amber-100` | `--tw-gradient-to: var(--color-amber-100)` |
| `to-amber-200` | `--tw-gradient-to: var(--color-amber-200)` |
| `to-amber-300` | `--tw-gradient-to: var(--color-amber-300)` |
| `to-amber-400` | `--tw-gradient-to: var(--color-amber-400)` |
| `to-amber-500` | `--tw-gradient-to: var(--color-amber-500)` |
| `to-amber-600` | `--tw-gradient-to: var(--color-amber-600)` |
| `to-amber-700` | `--tw-gradient-to: var(--color-amber-700)` |
| `to-amber-800` | `--tw-gradient-to: var(--color-amber-800)` |
| `to-amber-900` | `--tw-gradient-to: var(--color-amber-900)` |
| `to-amber-950` | `--tw-gradient-to: var(--color-amber-950)` |
| `to-yellow-50` | `--tw-gradient-to: var(--color-yellow-50)` |
| `to-yellow-100` | `--tw-gradient-to: var(--color-yellow-100)` |
| `to-yellow-200` | `--tw-gradient-to: var(--color-yellow-200)` |
| `to-yellow-300` | `--tw-gradient-to: var(--color-yellow-300)` |
| `to-yellow-400` | `--tw-gradient-to: var(--color-yellow-400)` |
| `to-yellow-500` | `--tw-gradient-to: var(--color-yellow-500)` |
| `to-yellow-600` | `--tw-gradient-to: var(--color-yellow-600)` |
| `to-yellow-700` | `--tw-gradient-to: var(--color-yellow-700)` |
| `to-yellow-800` | `--tw-gradient-to: var(--color-yellow-800)` |
| `to-yellow-900` | `--tw-gradient-to: var(--color-yellow-900)` |
| `to-yellow-950` | `--tw-gradient-to: var(--color-yellow-950)` |
| `to-lime-50` | `--tw-gradient-to: var(--color-lime-50)` |
| `to-lime-100` | `--tw-gradient-to: var(--color-lime-100)` |
| `to-lime-200` | `--tw-gradient-to: var(--color-lime-200)` |
| `to-lime-300` | `--tw-gradient-to: var(--color-lime-300)` |
| `to-lime-400` | `--tw-gradient-to: var(--color-lime-400)` |
| `to-lime-500` | `--tw-gradient-to: var(--color-lime-500)` |
| `to-lime-600` | `--tw-gradient-to: var(--color-lime-600)` |
| `to-lime-700` | `--tw-gradient-to: var(--color-lime-700)` |
| `to-lime-800` | `--tw-gradient-to: var(--color-lime-800)` |
| `to-lime-900` | `--tw-gradient-to: var(--color-lime-900)` |
| `to-lime-950` | `--tw-gradient-to: var(--color-lime-950)` |
| `to-green-50` | `--tw-gradient-to: var(--color-green-50)` |
| `to-green-100` | `--tw-gradient-to: var(--color-green-100)` |
| `to-green-200` | `--tw-gradient-to: var(--color-green-200)` |
| `to-green-300` | `--tw-gradient-to: var(--color-green-300)` |
| `to-green-400` | `--tw-gradient-to: var(--color-green-400)` |
| `to-green-500` | `--tw-gradient-to: var(--color-green-500)` |
| `to-green-600` | `--tw-gradient-to: var(--color-green-600)` |
| `to-green-700` | `--tw-gradient-to: var(--color-green-700)` |
| `to-green-800` | `--tw-gradient-to: var(--color-green-800)` |
| `to-green-900` | `--tw-gradient-to: var(--color-green-900)` |
| `to-green-950` | `--tw-gradient-to: var(--color-green-950)` |
| `to-emerald-50` | `--tw-gradient-to: var(--color-emerald-50)` |
| `to-emerald-100` | `--tw-gradient-to: var(--color-emerald-100)` |
| `to-emerald-200` | `--tw-gradient-to: var(--color-emerald-200)` |
| `to-emerald-300` | `--tw-gradient-to: var(--color-emerald-300)` |
| `to-emerald-400` | `--tw-gradient-to: var(--color-emerald-400)` |
| `to-emerald-500` | `--tw-gradient-to: var(--color-emerald-500)` |
| `to-emerald-600` | `--tw-gradient-to: var(--color-emerald-600)` |
| `to-emerald-700` | `--tw-gradient-to: var(--color-emerald-700)` |
| `to-emerald-800` | `--tw-gradient-to: var(--color-emerald-800)` |
| `to-emerald-900` | `--tw-gradient-to: var(--color-emerald-900)` |
| `to-emerald-950` | `--tw-gradient-to: var(--color-emerald-950)` |
| `to-teal-50` | `--tw-gradient-to: var(--color-teal-50)` |
| `to-teal-100` | `--tw-gradient-to: var(--color-teal-100)` |
| `to-teal-200` | `--tw-gradient-to: var(--color-teal-200)` |
| `to-teal-300` | `--tw-gradient-to: var(--color-teal-300)` |
| `to-teal-400` | `--tw-gradient-to: var(--color-teal-400)` |
| `to-teal-500` | `--tw-gradient-to: var(--color-teal-500)` |
| `to-teal-600` | `--tw-gradient-to: var(--color-teal-600)` |
| `to-teal-700` | `--tw-gradient-to: var(--color-teal-700)` |
| `to-teal-800` | `--tw-gradient-to: var(--color-teal-800)` |
| `to-teal-900` | `--tw-gradient-to: var(--color-teal-900)` |
| `to-teal-950` | `--tw-gradient-to: var(--color-teal-950)` |
| `to-cyan-50` | `--tw-gradient-to: var(--color-cyan-50)` |
| `to-cyan-100` | `--tw-gradient-to: var(--color-cyan-100)` |
| `to-cyan-200` | `--tw-gradient-to: var(--color-cyan-200)` |
| `to-cyan-300` | `--tw-gradient-to: var(--color-cyan-300)` |
| `to-cyan-400` | `--tw-gradient-to: var(--color-cyan-400)` |
| `to-cyan-500` | `--tw-gradient-to: var(--color-cyan-500)` |
| `to-cyan-600` | `--tw-gradient-to: var(--color-cyan-600)` |
| `to-cyan-700` | `--tw-gradient-to: var(--color-cyan-700)` |
| `to-cyan-800` | `--tw-gradient-to: var(--color-cyan-800)` |
| `to-cyan-900` | `--tw-gradient-to: var(--color-cyan-900)` |
| `to-cyan-950` | `--tw-gradient-to: var(--color-cyan-950)` |
| `to-sky-50` | `--tw-gradient-to: var(--color-sky-50)` |
| `to-sky-100` | `--tw-gradient-to: var(--color-sky-100)` |
| `to-sky-200` | `--tw-gradient-to: var(--color-sky-200)` |
| `to-sky-300` | `--tw-gradient-to: var(--color-sky-300)` |
| `to-sky-400` | `--tw-gradient-to: var(--color-sky-400)` |
| `to-sky-500` | `--tw-gradient-to: var(--color-sky-500)` |
| `to-sky-600` | `--tw-gradient-to: var(--color-sky-600)` |
| `to-sky-700` | `--tw-gradient-to: var(--color-sky-700)` |
| `to-sky-800` | `--tw-gradient-to: var(--color-sky-800)` |
| `to-sky-900` | `--tw-gradient-to: var(--color-sky-900)` |
| `to-sky-950` | `--tw-gradient-to: var(--color-sky-950)` |
| `to-blue-50` | `--tw-gradient-to: var(--color-blue-50)` |
| `to-blue-100` | `--tw-gradient-to: var(--color-blue-100)` |
| `to-blue-200` | `--tw-gradient-to: var(--color-blue-200)` |
| `to-blue-300` | `--tw-gradient-to: var(--color-blue-300)` |
| `to-blue-400` | `--tw-gradient-to: var(--color-blue-400)` |
| `to-blue-500` | `--tw-gradient-to: var(--color-blue-500)` |
| `to-blue-600` | `--tw-gradient-to: var(--color-blue-600)` |
| `to-blue-700` | `--tw-gradient-to: var(--color-blue-700)` |
| `to-blue-800` | `--tw-gradient-to: var(--color-blue-800)` |
| `to-blue-900` | `--tw-gradient-to: var(--color-blue-900)` |
| `to-blue-950` | `--tw-gradient-to: var(--color-blue-950)` |
| `to-indigo-50` | `--tw-gradient-to: var(--color-indigo-50)` |
| `to-indigo-100` | `--tw-gradient-to: var(--color-indigo-100)` |
| `to-indigo-200` | `--tw-gradient-to: var(--color-indigo-200)` |
| `to-indigo-300` | `--tw-gradient-to: var(--color-indigo-300)` |
| `to-indigo-400` | `--tw-gradient-to: var(--color-indigo-400)` |
| `to-indigo-500` | `--tw-gradient-to: var(--color-indigo-500)` |
| `to-indigo-600` | `--tw-gradient-to: var(--color-indigo-600)` |
| `to-indigo-700` | `--tw-gradient-to: var(--color-indigo-700)` |
| `to-indigo-800` | `--tw-gradient-to: var(--color-indigo-800)` |
| `to-indigo-900` | `--tw-gradient-to: var(--color-indigo-900)` |
| `to-indigo-950` | `--tw-gradient-to: var(--color-indigo-950)` |
| `to-violet-50` | `--tw-gradient-to: var(--color-violet-50)` |
| `to-violet-100` | `--tw-gradient-to: var(--color-violet-100)` |
| `to-violet-200` | `--tw-gradient-to: var(--color-violet-200)` |
| `to-violet-300` | `--tw-gradient-to: var(--color-violet-300)` |
| `to-violet-400` | `--tw-gradient-to: var(--color-violet-400)` |
| `to-violet-500` | `--tw-gradient-to: var(--color-violet-500)` |
| `to-violet-600` | `--tw-gradient-to: var(--color-violet-600)` |
| `to-violet-700` | `--tw-gradient-to: var(--color-violet-700)` |
| `to-violet-800` | `--tw-gradient-to: var(--color-violet-800)` |
| `to-violet-900` | `--tw-gradient-to: var(--color-violet-900)` |
| `to-violet-950` | `--tw-gradient-to: var(--color-violet-950)` |
| `to-purple-50` | `--tw-gradient-to: var(--color-purple-50)` |
| `to-purple-100` | `--tw-gradient-to: var(--color-purple-100)` |
| `to-purple-200` | `--tw-gradient-to: var(--color-purple-200)` |
| `to-purple-300` | `--tw-gradient-to: var(--color-purple-300)` |
| `to-purple-400` | `--tw-gradient-to: var(--color-purple-400)` |
| `to-purple-500` | `--tw-gradient-to: var(--color-purple-500)` |
| `to-purple-600` | `--tw-gradient-to: var(--color-purple-600)` |
| `to-purple-700` | `--tw-gradient-to: var(--color-purple-700)` |
| `to-purple-800` | `--tw-gradient-to: var(--color-purple-800)` |
| `to-purple-900` | `--tw-gradient-to: var(--color-purple-900)` |
| `to-purple-950` | `--tw-gradient-to: var(--color-purple-950)` |
| `to-fuchsia-50` | `--tw-gradient-to: var(--color-fuchsia-50)` |
| `to-fuchsia-100` | `--tw-gradient-to: var(--color-fuchsia-100)` |
| `to-fuchsia-200` | `--tw-gradient-to: var(--color-fuchsia-200)` |
| `to-fuchsia-300` | `--tw-gradient-to: var(--color-fuchsia-300)` |
| `to-fuchsia-400` | `--tw-gradient-to: var(--color-fuchsia-400)` |
| `to-fuchsia-500` | `--tw-gradient-to: var(--color-fuchsia-500)` |
| `to-fuchsia-600` | `--tw-gradient-to: var(--color-fuchsia-600)` |
| `to-fuchsia-700` | `--tw-gradient-to: var(--color-fuchsia-700)` |
| `to-fuchsia-800` | `--tw-gradient-to: var(--color-fuchsia-800)` |
| `to-fuchsia-900` | `--tw-gradient-to: var(--color-fuchsia-900)` |
| `to-fuchsia-950` | `--tw-gradient-to: var(--color-fuchsia-950)` |
| `to-pink-50` | `--tw-gradient-to: var(--color-pink-50)` |
| `to-pink-100` | `--tw-gradient-to: var(--color-pink-100)` |
| `to-pink-200` | `--tw-gradient-to: var(--color-pink-200)` |
| `to-pink-300` | `--tw-gradient-to: var(--color-pink-300)` |
| `to-pink-400` | `--tw-gradient-to: var(--color-pink-400)` |
| `to-pink-500` | `--tw-gradient-to: var(--color-pink-500)` |
| `to-pink-600` | `--tw-gradient-to: var(--color-pink-600)` |
| `to-pink-700` | `--tw-gradient-to: var(--color-pink-700)` |
| `to-pink-800` | `--tw-gradient-to: var(--color-pink-800)` |
| `to-pink-900` | `--tw-gradient-to: var(--color-pink-900)` |
| `to-pink-950` | `--tw-gradient-to: var(--color-pink-950)` |
| `to-rose-50` | `--tw-gradient-to: var(--color-rose-50)` |
| `to-rose-100` | `--tw-gradient-to: var(--color-rose-100)` |
| `to-rose-200` | `--tw-gradient-to: var(--color-rose-200)` |
| `to-rose-300` | `--tw-gradient-to: var(--color-rose-300)` |
| `to-rose-400` | `--tw-gradient-to: var(--color-rose-400)` |
| `to-rose-500` | `--tw-gradient-to: var(--color-rose-500)` |
| `to-rose-600` | `--tw-gradient-to: var(--color-rose-600)` |
| `to-rose-700` | `--tw-gradient-to: var(--color-rose-700)` |
| `to-rose-800` | `--tw-gradient-to: var(--color-rose-800)` |
| `to-rose-900` | `--tw-gradient-to: var(--color-rose-900)` |
| `to-rose-950` | `--tw-gradient-to: var(--color-rose-950)` |
| `to-white` | `--tw-gradient-to: white` |
| `to-black` | `--tw-gradient-to: black` |
| `to-transparent` | `--tw-gradient-to: transparent` |
| `to-inherit` | `--tw-gradient-to: inherit` |
| `to-current` | `--tw-gradient-to: current` |

## Borders — Radius

| Classe | CSS |
|--------|-----|
| `rounded-none` | `border-radius: 0px` |
| `rounded-sm` | `border-radius: 0.125rem` |
| `rounded` | `border-radius: 0.25rem` |
| `rounded-md` | `border-radius: 0.375rem` |
| `rounded-lg` | `border-radius: 0.5rem` |
| `rounded-xl` | `border-radius: 0.75rem` |
| `rounded-2xl` | `border-radius: 1rem` |
| `rounded-3xl` | `border-radius: 1.5rem` |
| `rounded-full` | `border-radius: 9999px` |
| `rounded-t-none` | `border-top-left-radius: 0px; border-top-right-radius: 0px` |
| `rounded-t-sm` | `border-top-left-radius: 0.125rem; border-top-right-radius: 0.125rem` |
| `rounded-t` | `border-top-left-radius: 0.25rem; border-top-right-radius: 0.25rem` |
| `rounded-t-md` | `border-top-left-radius: 0.375rem; border-top-right-radius: 0.375rem` |
| `rounded-t-lg` | `border-top-left-radius: 0.5rem; border-top-right-radius: 0.5rem` |
| `rounded-t-xl` | `border-top-left-radius: 0.75rem; border-top-right-radius: 0.75rem` |
| `rounded-t-2xl` | `border-top-left-radius: 1rem; border-top-right-radius: 1rem` |
| `rounded-t-full` | `border-top-left-radius: 9999px; border-top-right-radius: 9999px` |
| `rounded-r-none` | `border-top-right-radius: 0px; border-bottom-right-radius: 0px` |
| `rounded-r-sm` | `border-top-right-radius: 0.125rem; border-bottom-right-radius: 0.125rem` |
| `rounded-r` | `border-top-right-radius: 0.25rem; border-bottom-right-radius: 0.25rem` |
| `rounded-r-md` | `border-top-right-radius: 0.375rem; border-bottom-right-radius: 0.375rem` |
| `rounded-r-lg` | `border-top-right-radius: 0.5rem; border-bottom-right-radius: 0.5rem` |
| `rounded-r-xl` | `border-top-right-radius: 0.75rem; border-bottom-right-radius: 0.75rem` |
| `rounded-r-2xl` | `border-top-right-radius: 1rem; border-bottom-right-radius: 1rem` |
| `rounded-r-full` | `border-top-right-radius: 9999px; border-bottom-right-radius: 9999px` |
| `rounded-b-none` | `border-bottom-right-radius: 0px; border-bottom-left-radius: 0px` |
| `rounded-b-sm` | `border-bottom-right-radius: 0.125rem; border-bottom-left-radius: 0.125rem` |
| `rounded-b` | `border-bottom-right-radius: 0.25rem; border-bottom-left-radius: 0.25rem` |
| `rounded-b-md` | `border-bottom-right-radius: 0.375rem; border-bottom-left-radius: 0.375rem` |
| `rounded-b-lg` | `border-bottom-right-radius: 0.5rem; border-bottom-left-radius: 0.5rem` |
| `rounded-b-xl` | `border-bottom-right-radius: 0.75rem; border-bottom-left-radius: 0.75rem` |
| `rounded-b-2xl` | `border-bottom-right-radius: 1rem; border-bottom-left-radius: 1rem` |
| `rounded-b-full` | `border-bottom-right-radius: 9999px; border-bottom-left-radius: 9999px` |
| `rounded-l-none` | `border-top-left-radius: 0px; border-bottom-left-radius: 0px` |
| `rounded-l-sm` | `border-top-left-radius: 0.125rem; border-bottom-left-radius: 0.125rem` |
| `rounded-l` | `border-top-left-radius: 0.25rem; border-bottom-left-radius: 0.25rem` |
| `rounded-l-md` | `border-top-left-radius: 0.375rem; border-bottom-left-radius: 0.375rem` |
| `rounded-l-lg` | `border-top-left-radius: 0.5rem; border-bottom-left-radius: 0.5rem` |
| `rounded-l-xl` | `border-top-left-radius: 0.75rem; border-bottom-left-radius: 0.75rem` |
| `rounded-l-2xl` | `border-top-left-radius: 1rem; border-bottom-left-radius: 1rem` |
| `rounded-l-full` | `border-top-left-radius: 9999px; border-bottom-left-radius: 9999px` |
| `rounded-tl-none` | `border-top-left-radius: 0px` |
| `rounded-tl-sm` | `border-top-left-radius: 0.125rem` |
| `rounded-tl` | `border-top-left-radius: 0.25rem` |
| `rounded-tl-md` | `border-top-left-radius: 0.375rem` |
| `rounded-tl-lg` | `border-top-left-radius: 0.5rem` |
| `rounded-tl-xl` | `border-top-left-radius: 0.75rem` |
| `rounded-tl-2xl` | `border-top-left-radius: 1rem` |
| `rounded-tl-full` | `border-top-left-radius: 9999px` |
| `rounded-tr-none` | `border-top-right-radius: 0px` |
| `rounded-tr-sm` | `border-top-right-radius: 0.125rem` |
| `rounded-tr` | `border-top-right-radius: 0.25rem` |
| `rounded-tr-md` | `border-top-right-radius: 0.375rem` |
| `rounded-tr-lg` | `border-top-right-radius: 0.5rem` |
| `rounded-tr-xl` | `border-top-right-radius: 0.75rem` |
| `rounded-tr-2xl` | `border-top-right-radius: 1rem` |
| `rounded-tr-full` | `border-top-right-radius: 9999px` |
| `rounded-br-none` | `border-bottom-right-radius: 0px` |
| `rounded-br-sm` | `border-bottom-right-radius: 0.125rem` |
| `rounded-br` | `border-bottom-right-radius: 0.25rem` |
| `rounded-br-md` | `border-bottom-right-radius: 0.375rem` |
| `rounded-br-lg` | `border-bottom-right-radius: 0.5rem` |
| `rounded-br-xl` | `border-bottom-right-radius: 0.75rem` |
| `rounded-br-2xl` | `border-bottom-right-radius: 1rem` |
| `rounded-br-full` | `border-bottom-right-radius: 9999px` |
| `rounded-bl-none` | `border-bottom-left-radius: 0px` |
| `rounded-bl-sm` | `border-bottom-left-radius: 0.125rem` |
| `rounded-bl` | `border-bottom-left-radius: 0.25rem` |
| `rounded-bl-md` | `border-bottom-left-radius: 0.375rem` |
| `rounded-bl-lg` | `border-bottom-left-radius: 0.5rem` |
| `rounded-bl-xl` | `border-bottom-left-radius: 0.75rem` |
| `rounded-bl-2xl` | `border-bottom-left-radius: 1rem` |
| `rounded-bl-full` | `border-bottom-left-radius: 9999px` |

## Borders — Width

| Classe | CSS |
|--------|-----|
| `border` | `border-width: 1px` |
| `border-0` | `border-width: 0px` |
| `border-2` | `border-width: 2px` |
| `border-4` | `border-width: 4px` |
| `border-8` | `border-width: 8px` |
| `border-t` | `border-t-width: 1px` |
| `border-t-0` | `border-t-width: 0px` |
| `border-t-2` | `border-t-width: 2px` |
| `border-t-4` | `border-t-width: 4px` |
| `border-t-8` | `border-t-width: 8px` |
| `border-r` | `border-r-width: 1px` |
| `border-r-0` | `border-r-width: 0px` |
| `border-r-2` | `border-r-width: 2px` |
| `border-r-4` | `border-r-width: 4px` |
| `border-r-8` | `border-r-width: 8px` |
| `border-b` | `border-b-width: 1px` |
| `border-b-0` | `border-b-width: 0px` |
| `border-b-2` | `border-b-width: 2px` |
| `border-b-4` | `border-b-width: 4px` |
| `border-b-8` | `border-b-width: 8px` |
| `border-l` | `border-l-width: 1px` |
| `border-l-0` | `border-l-width: 0px` |
| `border-l-2` | `border-l-width: 2px` |
| `border-l-4` | `border-l-width: 4px` |
| `border-l-8` | `border-l-width: 8px` |
| `border-x` | `border-left-width: 1px; border-right-width: 1px` |
| `border-x-0` | `border-left-width: ${n}px; border-right-width: 0px` |
| `border-x-2` | `border-left-width: ${n}px; border-right-width: 2px` |
| `border-x-4` | `border-left-width: ${n}px; border-right-width: 4px` |
| `border-x-8` | `border-left-width: ${n}px; border-right-width: 8px` |
| `border-y` | `border-top-width: 1px; border-bottom-width: 1px` |
| `border-y-0` | `border-top-width: ${n}px; border-bottom-width: 0px` |
| `border-y-2` | `border-top-width: ${n}px; border-bottom-width: 2px` |
| `border-y-4` | `border-top-width: ${n}px; border-bottom-width: 4px` |
| `border-y-8` | `border-top-width: ${n}px; border-bottom-width: 8px` |

## Borders — Color

| Classe | CSS |
|--------|-----|
| `border-slate-50` | `border-color: var(--color-slate-50)` |
| `border-slate-100` | `border-color: var(--color-slate-100)` |
| `border-slate-200` | `border-color: var(--color-slate-200)` |
| `border-slate-300` | `border-color: var(--color-slate-300)` |
| `border-slate-400` | `border-color: var(--color-slate-400)` |
| `border-slate-500` | `border-color: var(--color-slate-500)` |
| `border-slate-600` | `border-color: var(--color-slate-600)` |
| `border-slate-700` | `border-color: var(--color-slate-700)` |
| `border-slate-800` | `border-color: var(--color-slate-800)` |
| `border-slate-900` | `border-color: var(--color-slate-900)` |
| `border-slate-950` | `border-color: var(--color-slate-950)` |
| `border-gray-50` | `border-color: var(--color-gray-50)` |
| `border-gray-100` | `border-color: var(--color-gray-100)` |
| `border-gray-200` | `border-color: var(--color-gray-200)` |
| `border-gray-300` | `border-color: var(--color-gray-300)` |
| `border-gray-400` | `border-color: var(--color-gray-400)` |
| `border-gray-500` | `border-color: var(--color-gray-500)` |
| `border-gray-600` | `border-color: var(--color-gray-600)` |
| `border-gray-700` | `border-color: var(--color-gray-700)` |
| `border-gray-800` | `border-color: var(--color-gray-800)` |
| `border-gray-900` | `border-color: var(--color-gray-900)` |
| `border-gray-950` | `border-color: var(--color-gray-950)` |
| `border-zinc-50` | `border-color: var(--color-zinc-50)` |
| `border-zinc-100` | `border-color: var(--color-zinc-100)` |
| `border-zinc-200` | `border-color: var(--color-zinc-200)` |
| `border-zinc-300` | `border-color: var(--color-zinc-300)` |
| `border-zinc-400` | `border-color: var(--color-zinc-400)` |
| `border-zinc-500` | `border-color: var(--color-zinc-500)` |
| `border-zinc-600` | `border-color: var(--color-zinc-600)` |
| `border-zinc-700` | `border-color: var(--color-zinc-700)` |
| `border-zinc-800` | `border-color: var(--color-zinc-800)` |
| `border-zinc-900` | `border-color: var(--color-zinc-900)` |
| `border-zinc-950` | `border-color: var(--color-zinc-950)` |
| `border-neutral-50` | `border-color: var(--color-neutral-50)` |
| `border-neutral-100` | `border-color: var(--color-neutral-100)` |
| `border-neutral-200` | `border-color: var(--color-neutral-200)` |
| `border-neutral-300` | `border-color: var(--color-neutral-300)` |
| `border-neutral-400` | `border-color: var(--color-neutral-400)` |
| `border-neutral-500` | `border-color: var(--color-neutral-500)` |
| `border-neutral-600` | `border-color: var(--color-neutral-600)` |
| `border-neutral-700` | `border-color: var(--color-neutral-700)` |
| `border-neutral-800` | `border-color: var(--color-neutral-800)` |
| `border-neutral-900` | `border-color: var(--color-neutral-900)` |
| `border-neutral-950` | `border-color: var(--color-neutral-950)` |
| `border-stone-50` | `border-color: var(--color-stone-50)` |
| `border-stone-100` | `border-color: var(--color-stone-100)` |
| `border-stone-200` | `border-color: var(--color-stone-200)` |
| `border-stone-300` | `border-color: var(--color-stone-300)` |
| `border-stone-400` | `border-color: var(--color-stone-400)` |
| `border-stone-500` | `border-color: var(--color-stone-500)` |
| `border-stone-600` | `border-color: var(--color-stone-600)` |
| `border-stone-700` | `border-color: var(--color-stone-700)` |
| `border-stone-800` | `border-color: var(--color-stone-800)` |
| `border-stone-900` | `border-color: var(--color-stone-900)` |
| `border-stone-950` | `border-color: var(--color-stone-950)` |
| `border-red-50` | `border-color: var(--color-red-50)` |
| `border-red-100` | `border-color: var(--color-red-100)` |
| `border-red-200` | `border-color: var(--color-red-200)` |
| `border-red-300` | `border-color: var(--color-red-300)` |
| `border-red-400` | `border-color: var(--color-red-400)` |
| `border-red-500` | `border-color: var(--color-red-500)` |
| `border-red-600` | `border-color: var(--color-red-600)` |
| `border-red-700` | `border-color: var(--color-red-700)` |
| `border-red-800` | `border-color: var(--color-red-800)` |
| `border-red-900` | `border-color: var(--color-red-900)` |
| `border-red-950` | `border-color: var(--color-red-950)` |
| `border-orange-50` | `border-color: var(--color-orange-50)` |
| `border-orange-100` | `border-color: var(--color-orange-100)` |
| `border-orange-200` | `border-color: var(--color-orange-200)` |
| `border-orange-300` | `border-color: var(--color-orange-300)` |
| `border-orange-400` | `border-color: var(--color-orange-400)` |
| `border-orange-500` | `border-color: var(--color-orange-500)` |
| `border-orange-600` | `border-color: var(--color-orange-600)` |
| `border-orange-700` | `border-color: var(--color-orange-700)` |
| `border-orange-800` | `border-color: var(--color-orange-800)` |
| `border-orange-900` | `border-color: var(--color-orange-900)` |
| `border-orange-950` | `border-color: var(--color-orange-950)` |
| `border-amber-50` | `border-color: var(--color-amber-50)` |
| `border-amber-100` | `border-color: var(--color-amber-100)` |
| `border-amber-200` | `border-color: var(--color-amber-200)` |
| `border-amber-300` | `border-color: var(--color-amber-300)` |
| `border-amber-400` | `border-color: var(--color-amber-400)` |
| `border-amber-500` | `border-color: var(--color-amber-500)` |
| `border-amber-600` | `border-color: var(--color-amber-600)` |
| `border-amber-700` | `border-color: var(--color-amber-700)` |
| `border-amber-800` | `border-color: var(--color-amber-800)` |
| `border-amber-900` | `border-color: var(--color-amber-900)` |
| `border-amber-950` | `border-color: var(--color-amber-950)` |
| `border-yellow-50` | `border-color: var(--color-yellow-50)` |
| `border-yellow-100` | `border-color: var(--color-yellow-100)` |
| `border-yellow-200` | `border-color: var(--color-yellow-200)` |
| `border-yellow-300` | `border-color: var(--color-yellow-300)` |
| `border-yellow-400` | `border-color: var(--color-yellow-400)` |
| `border-yellow-500` | `border-color: var(--color-yellow-500)` |
| `border-yellow-600` | `border-color: var(--color-yellow-600)` |
| `border-yellow-700` | `border-color: var(--color-yellow-700)` |
| `border-yellow-800` | `border-color: var(--color-yellow-800)` |
| `border-yellow-900` | `border-color: var(--color-yellow-900)` |
| `border-yellow-950` | `border-color: var(--color-yellow-950)` |
| `border-lime-50` | `border-color: var(--color-lime-50)` |
| `border-lime-100` | `border-color: var(--color-lime-100)` |
| `border-lime-200` | `border-color: var(--color-lime-200)` |
| `border-lime-300` | `border-color: var(--color-lime-300)` |
| `border-lime-400` | `border-color: var(--color-lime-400)` |
| `border-lime-500` | `border-color: var(--color-lime-500)` |
| `border-lime-600` | `border-color: var(--color-lime-600)` |
| `border-lime-700` | `border-color: var(--color-lime-700)` |
| `border-lime-800` | `border-color: var(--color-lime-800)` |
| `border-lime-900` | `border-color: var(--color-lime-900)` |
| `border-lime-950` | `border-color: var(--color-lime-950)` |
| `border-green-50` | `border-color: var(--color-green-50)` |
| `border-green-100` | `border-color: var(--color-green-100)` |
| `border-green-200` | `border-color: var(--color-green-200)` |
| `border-green-300` | `border-color: var(--color-green-300)` |
| `border-green-400` | `border-color: var(--color-green-400)` |
| `border-green-500` | `border-color: var(--color-green-500)` |
| `border-green-600` | `border-color: var(--color-green-600)` |
| `border-green-700` | `border-color: var(--color-green-700)` |
| `border-green-800` | `border-color: var(--color-green-800)` |
| `border-green-900` | `border-color: var(--color-green-900)` |
| `border-green-950` | `border-color: var(--color-green-950)` |
| `border-emerald-50` | `border-color: var(--color-emerald-50)` |
| `border-emerald-100` | `border-color: var(--color-emerald-100)` |
| `border-emerald-200` | `border-color: var(--color-emerald-200)` |
| `border-emerald-300` | `border-color: var(--color-emerald-300)` |
| `border-emerald-400` | `border-color: var(--color-emerald-400)` |
| `border-emerald-500` | `border-color: var(--color-emerald-500)` |
| `border-emerald-600` | `border-color: var(--color-emerald-600)` |
| `border-emerald-700` | `border-color: var(--color-emerald-700)` |
| `border-emerald-800` | `border-color: var(--color-emerald-800)` |
| `border-emerald-900` | `border-color: var(--color-emerald-900)` |
| `border-emerald-950` | `border-color: var(--color-emerald-950)` |
| `border-teal-50` | `border-color: var(--color-teal-50)` |
| `border-teal-100` | `border-color: var(--color-teal-100)` |
| `border-teal-200` | `border-color: var(--color-teal-200)` |
| `border-teal-300` | `border-color: var(--color-teal-300)` |
| `border-teal-400` | `border-color: var(--color-teal-400)` |
| `border-teal-500` | `border-color: var(--color-teal-500)` |
| `border-teal-600` | `border-color: var(--color-teal-600)` |
| `border-teal-700` | `border-color: var(--color-teal-700)` |
| `border-teal-800` | `border-color: var(--color-teal-800)` |
| `border-teal-900` | `border-color: var(--color-teal-900)` |
| `border-teal-950` | `border-color: var(--color-teal-950)` |
| `border-cyan-50` | `border-color: var(--color-cyan-50)` |
| `border-cyan-100` | `border-color: var(--color-cyan-100)` |
| `border-cyan-200` | `border-color: var(--color-cyan-200)` |
| `border-cyan-300` | `border-color: var(--color-cyan-300)` |
| `border-cyan-400` | `border-color: var(--color-cyan-400)` |
| `border-cyan-500` | `border-color: var(--color-cyan-500)` |
| `border-cyan-600` | `border-color: var(--color-cyan-600)` |
| `border-cyan-700` | `border-color: var(--color-cyan-700)` |
| `border-cyan-800` | `border-color: var(--color-cyan-800)` |
| `border-cyan-900` | `border-color: var(--color-cyan-900)` |
| `border-cyan-950` | `border-color: var(--color-cyan-950)` |
| `border-sky-50` | `border-color: var(--color-sky-50)` |
| `border-sky-100` | `border-color: var(--color-sky-100)` |
| `border-sky-200` | `border-color: var(--color-sky-200)` |
| `border-sky-300` | `border-color: var(--color-sky-300)` |
| `border-sky-400` | `border-color: var(--color-sky-400)` |
| `border-sky-500` | `border-color: var(--color-sky-500)` |
| `border-sky-600` | `border-color: var(--color-sky-600)` |
| `border-sky-700` | `border-color: var(--color-sky-700)` |
| `border-sky-800` | `border-color: var(--color-sky-800)` |
| `border-sky-900` | `border-color: var(--color-sky-900)` |
| `border-sky-950` | `border-color: var(--color-sky-950)` |
| `border-blue-50` | `border-color: var(--color-blue-50)` |
| `border-blue-100` | `border-color: var(--color-blue-100)` |
| `border-blue-200` | `border-color: var(--color-blue-200)` |
| `border-blue-300` | `border-color: var(--color-blue-300)` |
| `border-blue-400` | `border-color: var(--color-blue-400)` |
| `border-blue-500` | `border-color: var(--color-blue-500)` |
| `border-blue-600` | `border-color: var(--color-blue-600)` |
| `border-blue-700` | `border-color: var(--color-blue-700)` |
| `border-blue-800` | `border-color: var(--color-blue-800)` |
| `border-blue-900` | `border-color: var(--color-blue-900)` |
| `border-blue-950` | `border-color: var(--color-blue-950)` |
| `border-indigo-50` | `border-color: var(--color-indigo-50)` |
| `border-indigo-100` | `border-color: var(--color-indigo-100)` |
| `border-indigo-200` | `border-color: var(--color-indigo-200)` |
| `border-indigo-300` | `border-color: var(--color-indigo-300)` |
| `border-indigo-400` | `border-color: var(--color-indigo-400)` |
| `border-indigo-500` | `border-color: var(--color-indigo-500)` |
| `border-indigo-600` | `border-color: var(--color-indigo-600)` |
| `border-indigo-700` | `border-color: var(--color-indigo-700)` |
| `border-indigo-800` | `border-color: var(--color-indigo-800)` |
| `border-indigo-900` | `border-color: var(--color-indigo-900)` |
| `border-indigo-950` | `border-color: var(--color-indigo-950)` |
| `border-violet-50` | `border-color: var(--color-violet-50)` |
| `border-violet-100` | `border-color: var(--color-violet-100)` |
| `border-violet-200` | `border-color: var(--color-violet-200)` |
| `border-violet-300` | `border-color: var(--color-violet-300)` |
| `border-violet-400` | `border-color: var(--color-violet-400)` |
| `border-violet-500` | `border-color: var(--color-violet-500)` |
| `border-violet-600` | `border-color: var(--color-violet-600)` |
| `border-violet-700` | `border-color: var(--color-violet-700)` |
| `border-violet-800` | `border-color: var(--color-violet-800)` |
| `border-violet-900` | `border-color: var(--color-violet-900)` |
| `border-violet-950` | `border-color: var(--color-violet-950)` |
| `border-purple-50` | `border-color: var(--color-purple-50)` |
| `border-purple-100` | `border-color: var(--color-purple-100)` |
| `border-purple-200` | `border-color: var(--color-purple-200)` |
| `border-purple-300` | `border-color: var(--color-purple-300)` |
| `border-purple-400` | `border-color: var(--color-purple-400)` |
| `border-purple-500` | `border-color: var(--color-purple-500)` |
| `border-purple-600` | `border-color: var(--color-purple-600)` |
| `border-purple-700` | `border-color: var(--color-purple-700)` |
| `border-purple-800` | `border-color: var(--color-purple-800)` |
| `border-purple-900` | `border-color: var(--color-purple-900)` |
| `border-purple-950` | `border-color: var(--color-purple-950)` |
| `border-fuchsia-50` | `border-color: var(--color-fuchsia-50)` |
| `border-fuchsia-100` | `border-color: var(--color-fuchsia-100)` |
| `border-fuchsia-200` | `border-color: var(--color-fuchsia-200)` |
| `border-fuchsia-300` | `border-color: var(--color-fuchsia-300)` |
| `border-fuchsia-400` | `border-color: var(--color-fuchsia-400)` |
| `border-fuchsia-500` | `border-color: var(--color-fuchsia-500)` |
| `border-fuchsia-600` | `border-color: var(--color-fuchsia-600)` |
| `border-fuchsia-700` | `border-color: var(--color-fuchsia-700)` |
| `border-fuchsia-800` | `border-color: var(--color-fuchsia-800)` |
| `border-fuchsia-900` | `border-color: var(--color-fuchsia-900)` |
| `border-fuchsia-950` | `border-color: var(--color-fuchsia-950)` |
| `border-pink-50` | `border-color: var(--color-pink-50)` |
| `border-pink-100` | `border-color: var(--color-pink-100)` |
| `border-pink-200` | `border-color: var(--color-pink-200)` |
| `border-pink-300` | `border-color: var(--color-pink-300)` |
| `border-pink-400` | `border-color: var(--color-pink-400)` |
| `border-pink-500` | `border-color: var(--color-pink-500)` |
| `border-pink-600` | `border-color: var(--color-pink-600)` |
| `border-pink-700` | `border-color: var(--color-pink-700)` |
| `border-pink-800` | `border-color: var(--color-pink-800)` |
| `border-pink-900` | `border-color: var(--color-pink-900)` |
| `border-pink-950` | `border-color: var(--color-pink-950)` |
| `border-rose-50` | `border-color: var(--color-rose-50)` |
| `border-rose-100` | `border-color: var(--color-rose-100)` |
| `border-rose-200` | `border-color: var(--color-rose-200)` |
| `border-rose-300` | `border-color: var(--color-rose-300)` |
| `border-rose-400` | `border-color: var(--color-rose-400)` |
| `border-rose-500` | `border-color: var(--color-rose-500)` |
| `border-rose-600` | `border-color: var(--color-rose-600)` |
| `border-rose-700` | `border-color: var(--color-rose-700)` |
| `border-rose-800` | `border-color: var(--color-rose-800)` |
| `border-rose-900` | `border-color: var(--color-rose-900)` |
| `border-rose-950` | `border-color: var(--color-rose-950)` |
| `border-white` | `border-color: white` |
| `border-black` | `border-color: black` |
| `border-transparent` | `border-color: transparent` |
| `border-inherit` | `border-color: inherit` |
| `border-current` | `border-color: current` |

## Borders — Style

| Classe | CSS |
|--------|-----|
| `border-solid` | `border-style: solid` |
| `border-dashed` | `border-style: dashed` |
| `border-dotted` | `border-style: dotted` |
| `border-double` | `border-style: double` |
| `border-hidden` | `border-style: hidden` |
| `border-none` | `border-style: none` |

## Borders — Outline

| Classe | CSS |
|--------|-----|
| `outline-none` | `outline: 2px solid transparent; outline-offset: 2px` |
| `outline` | `outline-style: solid` |
| `outline-0` | `outline-width: 0px` |
| `outline-1` | `outline-width: 1px` |
| `outline-2` | `outline-width: 2px` |
| `outline-4` | `outline-width: 4px` |
| `outline-8` | `outline-width: 8px` |
| `outline-slate-50` | `outline-color: var(--color-slate-50)` |
| `outline-slate-100` | `outline-color: var(--color-slate-100)` |
| `outline-slate-200` | `outline-color: var(--color-slate-200)` |
| `outline-slate-300` | `outline-color: var(--color-slate-300)` |
| `outline-slate-400` | `outline-color: var(--color-slate-400)` |
| `outline-slate-500` | `outline-color: var(--color-slate-500)` |
| `outline-slate-600` | `outline-color: var(--color-slate-600)` |
| `outline-slate-700` | `outline-color: var(--color-slate-700)` |
| `outline-slate-800` | `outline-color: var(--color-slate-800)` |
| `outline-slate-900` | `outline-color: var(--color-slate-900)` |
| `outline-slate-950` | `outline-color: var(--color-slate-950)` |
| `outline-gray-50` | `outline-color: var(--color-gray-50)` |
| `outline-gray-100` | `outline-color: var(--color-gray-100)` |
| `outline-gray-200` | `outline-color: var(--color-gray-200)` |
| `outline-gray-300` | `outline-color: var(--color-gray-300)` |
| `outline-gray-400` | `outline-color: var(--color-gray-400)` |
| `outline-gray-500` | `outline-color: var(--color-gray-500)` |
| `outline-gray-600` | `outline-color: var(--color-gray-600)` |
| `outline-gray-700` | `outline-color: var(--color-gray-700)` |
| `outline-gray-800` | `outline-color: var(--color-gray-800)` |
| `outline-gray-900` | `outline-color: var(--color-gray-900)` |
| `outline-gray-950` | `outline-color: var(--color-gray-950)` |
| `outline-zinc-50` | `outline-color: var(--color-zinc-50)` |
| `outline-zinc-100` | `outline-color: var(--color-zinc-100)` |
| `outline-zinc-200` | `outline-color: var(--color-zinc-200)` |
| `outline-zinc-300` | `outline-color: var(--color-zinc-300)` |
| `outline-zinc-400` | `outline-color: var(--color-zinc-400)` |
| `outline-zinc-500` | `outline-color: var(--color-zinc-500)` |
| `outline-zinc-600` | `outline-color: var(--color-zinc-600)` |
| `outline-zinc-700` | `outline-color: var(--color-zinc-700)` |
| `outline-zinc-800` | `outline-color: var(--color-zinc-800)` |
| `outline-zinc-900` | `outline-color: var(--color-zinc-900)` |
| `outline-zinc-950` | `outline-color: var(--color-zinc-950)` |
| `outline-neutral-50` | `outline-color: var(--color-neutral-50)` |
| `outline-neutral-100` | `outline-color: var(--color-neutral-100)` |
| `outline-neutral-200` | `outline-color: var(--color-neutral-200)` |
| `outline-neutral-300` | `outline-color: var(--color-neutral-300)` |
| `outline-neutral-400` | `outline-color: var(--color-neutral-400)` |
| `outline-neutral-500` | `outline-color: var(--color-neutral-500)` |
| `outline-neutral-600` | `outline-color: var(--color-neutral-600)` |
| `outline-neutral-700` | `outline-color: var(--color-neutral-700)` |
| `outline-neutral-800` | `outline-color: var(--color-neutral-800)` |
| `outline-neutral-900` | `outline-color: var(--color-neutral-900)` |
| `outline-neutral-950` | `outline-color: var(--color-neutral-950)` |
| `outline-stone-50` | `outline-color: var(--color-stone-50)` |
| `outline-stone-100` | `outline-color: var(--color-stone-100)` |
| `outline-stone-200` | `outline-color: var(--color-stone-200)` |
| `outline-stone-300` | `outline-color: var(--color-stone-300)` |
| `outline-stone-400` | `outline-color: var(--color-stone-400)` |
| `outline-stone-500` | `outline-color: var(--color-stone-500)` |
| `outline-stone-600` | `outline-color: var(--color-stone-600)` |
| `outline-stone-700` | `outline-color: var(--color-stone-700)` |
| `outline-stone-800` | `outline-color: var(--color-stone-800)` |
| `outline-stone-900` | `outline-color: var(--color-stone-900)` |
| `outline-stone-950` | `outline-color: var(--color-stone-950)` |
| `outline-red-50` | `outline-color: var(--color-red-50)` |
| `outline-red-100` | `outline-color: var(--color-red-100)` |
| `outline-red-200` | `outline-color: var(--color-red-200)` |
| `outline-red-300` | `outline-color: var(--color-red-300)` |
| `outline-red-400` | `outline-color: var(--color-red-400)` |
| `outline-red-500` | `outline-color: var(--color-red-500)` |
| `outline-red-600` | `outline-color: var(--color-red-600)` |
| `outline-red-700` | `outline-color: var(--color-red-700)` |
| `outline-red-800` | `outline-color: var(--color-red-800)` |
| `outline-red-900` | `outline-color: var(--color-red-900)` |
| `outline-red-950` | `outline-color: var(--color-red-950)` |
| `outline-orange-50` | `outline-color: var(--color-orange-50)` |
| `outline-orange-100` | `outline-color: var(--color-orange-100)` |
| `outline-orange-200` | `outline-color: var(--color-orange-200)` |
| `outline-orange-300` | `outline-color: var(--color-orange-300)` |
| `outline-orange-400` | `outline-color: var(--color-orange-400)` |
| `outline-orange-500` | `outline-color: var(--color-orange-500)` |
| `outline-orange-600` | `outline-color: var(--color-orange-600)` |
| `outline-orange-700` | `outline-color: var(--color-orange-700)` |
| `outline-orange-800` | `outline-color: var(--color-orange-800)` |
| `outline-orange-900` | `outline-color: var(--color-orange-900)` |
| `outline-orange-950` | `outline-color: var(--color-orange-950)` |
| `outline-amber-50` | `outline-color: var(--color-amber-50)` |
| `outline-amber-100` | `outline-color: var(--color-amber-100)` |
| `outline-amber-200` | `outline-color: var(--color-amber-200)` |
| `outline-amber-300` | `outline-color: var(--color-amber-300)` |
| `outline-amber-400` | `outline-color: var(--color-amber-400)` |
| `outline-amber-500` | `outline-color: var(--color-amber-500)` |
| `outline-amber-600` | `outline-color: var(--color-amber-600)` |
| `outline-amber-700` | `outline-color: var(--color-amber-700)` |
| `outline-amber-800` | `outline-color: var(--color-amber-800)` |
| `outline-amber-900` | `outline-color: var(--color-amber-900)` |
| `outline-amber-950` | `outline-color: var(--color-amber-950)` |
| `outline-yellow-50` | `outline-color: var(--color-yellow-50)` |
| `outline-yellow-100` | `outline-color: var(--color-yellow-100)` |
| `outline-yellow-200` | `outline-color: var(--color-yellow-200)` |
| `outline-yellow-300` | `outline-color: var(--color-yellow-300)` |
| `outline-yellow-400` | `outline-color: var(--color-yellow-400)` |
| `outline-yellow-500` | `outline-color: var(--color-yellow-500)` |
| `outline-yellow-600` | `outline-color: var(--color-yellow-600)` |
| `outline-yellow-700` | `outline-color: var(--color-yellow-700)` |
| `outline-yellow-800` | `outline-color: var(--color-yellow-800)` |
| `outline-yellow-900` | `outline-color: var(--color-yellow-900)` |
| `outline-yellow-950` | `outline-color: var(--color-yellow-950)` |
| `outline-lime-50` | `outline-color: var(--color-lime-50)` |
| `outline-lime-100` | `outline-color: var(--color-lime-100)` |
| `outline-lime-200` | `outline-color: var(--color-lime-200)` |
| `outline-lime-300` | `outline-color: var(--color-lime-300)` |
| `outline-lime-400` | `outline-color: var(--color-lime-400)` |
| `outline-lime-500` | `outline-color: var(--color-lime-500)` |
| `outline-lime-600` | `outline-color: var(--color-lime-600)` |
| `outline-lime-700` | `outline-color: var(--color-lime-700)` |
| `outline-lime-800` | `outline-color: var(--color-lime-800)` |
| `outline-lime-900` | `outline-color: var(--color-lime-900)` |
| `outline-lime-950` | `outline-color: var(--color-lime-950)` |
| `outline-green-50` | `outline-color: var(--color-green-50)` |
| `outline-green-100` | `outline-color: var(--color-green-100)` |
| `outline-green-200` | `outline-color: var(--color-green-200)` |
| `outline-green-300` | `outline-color: var(--color-green-300)` |
| `outline-green-400` | `outline-color: var(--color-green-400)` |
| `outline-green-500` | `outline-color: var(--color-green-500)` |
| `outline-green-600` | `outline-color: var(--color-green-600)` |
| `outline-green-700` | `outline-color: var(--color-green-700)` |
| `outline-green-800` | `outline-color: var(--color-green-800)` |
| `outline-green-900` | `outline-color: var(--color-green-900)` |
| `outline-green-950` | `outline-color: var(--color-green-950)` |
| `outline-emerald-50` | `outline-color: var(--color-emerald-50)` |
| `outline-emerald-100` | `outline-color: var(--color-emerald-100)` |
| `outline-emerald-200` | `outline-color: var(--color-emerald-200)` |
| `outline-emerald-300` | `outline-color: var(--color-emerald-300)` |
| `outline-emerald-400` | `outline-color: var(--color-emerald-400)` |
| `outline-emerald-500` | `outline-color: var(--color-emerald-500)` |
| `outline-emerald-600` | `outline-color: var(--color-emerald-600)` |
| `outline-emerald-700` | `outline-color: var(--color-emerald-700)` |
| `outline-emerald-800` | `outline-color: var(--color-emerald-800)` |
| `outline-emerald-900` | `outline-color: var(--color-emerald-900)` |
| `outline-emerald-950` | `outline-color: var(--color-emerald-950)` |
| `outline-teal-50` | `outline-color: var(--color-teal-50)` |
| `outline-teal-100` | `outline-color: var(--color-teal-100)` |
| `outline-teal-200` | `outline-color: var(--color-teal-200)` |
| `outline-teal-300` | `outline-color: var(--color-teal-300)` |
| `outline-teal-400` | `outline-color: var(--color-teal-400)` |
| `outline-teal-500` | `outline-color: var(--color-teal-500)` |
| `outline-teal-600` | `outline-color: var(--color-teal-600)` |
| `outline-teal-700` | `outline-color: var(--color-teal-700)` |
| `outline-teal-800` | `outline-color: var(--color-teal-800)` |
| `outline-teal-900` | `outline-color: var(--color-teal-900)` |
| `outline-teal-950` | `outline-color: var(--color-teal-950)` |
| `outline-cyan-50` | `outline-color: var(--color-cyan-50)` |
| `outline-cyan-100` | `outline-color: var(--color-cyan-100)` |
| `outline-cyan-200` | `outline-color: var(--color-cyan-200)` |
| `outline-cyan-300` | `outline-color: var(--color-cyan-300)` |
| `outline-cyan-400` | `outline-color: var(--color-cyan-400)` |
| `outline-cyan-500` | `outline-color: var(--color-cyan-500)` |
| `outline-cyan-600` | `outline-color: var(--color-cyan-600)` |
| `outline-cyan-700` | `outline-color: var(--color-cyan-700)` |
| `outline-cyan-800` | `outline-color: var(--color-cyan-800)` |
| `outline-cyan-900` | `outline-color: var(--color-cyan-900)` |
| `outline-cyan-950` | `outline-color: var(--color-cyan-950)` |
| `outline-sky-50` | `outline-color: var(--color-sky-50)` |
| `outline-sky-100` | `outline-color: var(--color-sky-100)` |
| `outline-sky-200` | `outline-color: var(--color-sky-200)` |
| `outline-sky-300` | `outline-color: var(--color-sky-300)` |
| `outline-sky-400` | `outline-color: var(--color-sky-400)` |
| `outline-sky-500` | `outline-color: var(--color-sky-500)` |
| `outline-sky-600` | `outline-color: var(--color-sky-600)` |
| `outline-sky-700` | `outline-color: var(--color-sky-700)` |
| `outline-sky-800` | `outline-color: var(--color-sky-800)` |
| `outline-sky-900` | `outline-color: var(--color-sky-900)` |
| `outline-sky-950` | `outline-color: var(--color-sky-950)` |
| `outline-blue-50` | `outline-color: var(--color-blue-50)` |
| `outline-blue-100` | `outline-color: var(--color-blue-100)` |
| `outline-blue-200` | `outline-color: var(--color-blue-200)` |
| `outline-blue-300` | `outline-color: var(--color-blue-300)` |
| `outline-blue-400` | `outline-color: var(--color-blue-400)` |
| `outline-blue-500` | `outline-color: var(--color-blue-500)` |
| `outline-blue-600` | `outline-color: var(--color-blue-600)` |
| `outline-blue-700` | `outline-color: var(--color-blue-700)` |
| `outline-blue-800` | `outline-color: var(--color-blue-800)` |
| `outline-blue-900` | `outline-color: var(--color-blue-900)` |
| `outline-blue-950` | `outline-color: var(--color-blue-950)` |
| `outline-indigo-50` | `outline-color: var(--color-indigo-50)` |
| `outline-indigo-100` | `outline-color: var(--color-indigo-100)` |
| `outline-indigo-200` | `outline-color: var(--color-indigo-200)` |
| `outline-indigo-300` | `outline-color: var(--color-indigo-300)` |
| `outline-indigo-400` | `outline-color: var(--color-indigo-400)` |
| `outline-indigo-500` | `outline-color: var(--color-indigo-500)` |
| `outline-indigo-600` | `outline-color: var(--color-indigo-600)` |
| `outline-indigo-700` | `outline-color: var(--color-indigo-700)` |
| `outline-indigo-800` | `outline-color: var(--color-indigo-800)` |
| `outline-indigo-900` | `outline-color: var(--color-indigo-900)` |
| `outline-indigo-950` | `outline-color: var(--color-indigo-950)` |
| `outline-violet-50` | `outline-color: var(--color-violet-50)` |
| `outline-violet-100` | `outline-color: var(--color-violet-100)` |
| `outline-violet-200` | `outline-color: var(--color-violet-200)` |
| `outline-violet-300` | `outline-color: var(--color-violet-300)` |
| `outline-violet-400` | `outline-color: var(--color-violet-400)` |
| `outline-violet-500` | `outline-color: var(--color-violet-500)` |
| `outline-violet-600` | `outline-color: var(--color-violet-600)` |
| `outline-violet-700` | `outline-color: var(--color-violet-700)` |
| `outline-violet-800` | `outline-color: var(--color-violet-800)` |
| `outline-violet-900` | `outline-color: var(--color-violet-900)` |
| `outline-violet-950` | `outline-color: var(--color-violet-950)` |
| `outline-purple-50` | `outline-color: var(--color-purple-50)` |
| `outline-purple-100` | `outline-color: var(--color-purple-100)` |
| `outline-purple-200` | `outline-color: var(--color-purple-200)` |
| `outline-purple-300` | `outline-color: var(--color-purple-300)` |
| `outline-purple-400` | `outline-color: var(--color-purple-400)` |
| `outline-purple-500` | `outline-color: var(--color-purple-500)` |
| `outline-purple-600` | `outline-color: var(--color-purple-600)` |
| `outline-purple-700` | `outline-color: var(--color-purple-700)` |
| `outline-purple-800` | `outline-color: var(--color-purple-800)` |
| `outline-purple-900` | `outline-color: var(--color-purple-900)` |
| `outline-purple-950` | `outline-color: var(--color-purple-950)` |
| `outline-fuchsia-50` | `outline-color: var(--color-fuchsia-50)` |
| `outline-fuchsia-100` | `outline-color: var(--color-fuchsia-100)` |
| `outline-fuchsia-200` | `outline-color: var(--color-fuchsia-200)` |
| `outline-fuchsia-300` | `outline-color: var(--color-fuchsia-300)` |
| `outline-fuchsia-400` | `outline-color: var(--color-fuchsia-400)` |
| `outline-fuchsia-500` | `outline-color: var(--color-fuchsia-500)` |
| `outline-fuchsia-600` | `outline-color: var(--color-fuchsia-600)` |
| `outline-fuchsia-700` | `outline-color: var(--color-fuchsia-700)` |
| `outline-fuchsia-800` | `outline-color: var(--color-fuchsia-800)` |
| `outline-fuchsia-900` | `outline-color: var(--color-fuchsia-900)` |
| `outline-fuchsia-950` | `outline-color: var(--color-fuchsia-950)` |
| `outline-pink-50` | `outline-color: var(--color-pink-50)` |
| `outline-pink-100` | `outline-color: var(--color-pink-100)` |
| `outline-pink-200` | `outline-color: var(--color-pink-200)` |
| `outline-pink-300` | `outline-color: var(--color-pink-300)` |
| `outline-pink-400` | `outline-color: var(--color-pink-400)` |
| `outline-pink-500` | `outline-color: var(--color-pink-500)` |
| `outline-pink-600` | `outline-color: var(--color-pink-600)` |
| `outline-pink-700` | `outline-color: var(--color-pink-700)` |
| `outline-pink-800` | `outline-color: var(--color-pink-800)` |
| `outline-pink-900` | `outline-color: var(--color-pink-900)` |
| `outline-pink-950` | `outline-color: var(--color-pink-950)` |
| `outline-rose-50` | `outline-color: var(--color-rose-50)` |
| `outline-rose-100` | `outline-color: var(--color-rose-100)` |
| `outline-rose-200` | `outline-color: var(--color-rose-200)` |
| `outline-rose-300` | `outline-color: var(--color-rose-300)` |
| `outline-rose-400` | `outline-color: var(--color-rose-400)` |
| `outline-rose-500` | `outline-color: var(--color-rose-500)` |
| `outline-rose-600` | `outline-color: var(--color-rose-600)` |
| `outline-rose-700` | `outline-color: var(--color-rose-700)` |
| `outline-rose-800` | `outline-color: var(--color-rose-800)` |
| `outline-rose-900` | `outline-color: var(--color-rose-900)` |
| `outline-rose-950` | `outline-color: var(--color-rose-950)` |
| `outline-white` | `outline-color: white` |
| `outline-black` | `outline-color: black` |
| `outline-transparent` | `outline-color: transparent` |
| `outline-inherit` | `outline-color: inherit` |
| `outline-current` | `outline-color: current` |
| `outline-solid` | `outline-style: solid` |
| `outline-dashed` | `outline-style: dashed` |
| `outline-dotted` | `outline-style: dotted` |
| `outline-double` | `outline-style: double` |
| `outline-offset-0` | `outline-offset: 0px` |
| `outline-offset-1` | `outline-offset: 1px` |
| `outline-offset-2` | `outline-offset: 2px` |
| `outline-offset-4` | `outline-offset: 4px` |
| `outline-offset-8` | `outline-offset: 8px` |
| `ring` | `--tw-ring-shadow: var(--tw-ring-inset) 0 0 0 calc(3px + var(--tw-ring-offset-width)) var(--tw-ring-color)` |
| `ring-0` | `--tw-ring-shadow: var(--tw-ring-inset) 0 0 0 calc(0px + var(--tw-ring-offset-width)) var(--tw-ring-color)` |
| `ring-1` | `--tw-ring-shadow: var(--tw-ring-inset) 0 0 0 calc(1px + var(--tw-ring-offset-width)) var(--tw-ring-color)` |
| `ring-2` | `--tw-ring-shadow: var(--tw-ring-inset) 0 0 0 calc(2px + var(--tw-ring-offset-width)) var(--tw-ring-color)` |
| `ring-4` | `--tw-ring-shadow: var(--tw-ring-inset) 0 0 0 calc(4px + var(--tw-ring-offset-width)) var(--tw-ring-color)` |
| `ring-8` | `--tw-ring-shadow: var(--tw-ring-inset) 0 0 0 calc(8px + var(--tw-ring-offset-width)) var(--tw-ring-color)` |
| `ring-inset` | `--tw-ring-inset: inset` |
| `ring-slate-50` | `--tw-ring-color: var(--color-slate-50)` |
| `ring-slate-100` | `--tw-ring-color: var(--color-slate-100)` |
| `ring-slate-200` | `--tw-ring-color: var(--color-slate-200)` |
| `ring-slate-300` | `--tw-ring-color: var(--color-slate-300)` |
| `ring-slate-400` | `--tw-ring-color: var(--color-slate-400)` |
| `ring-slate-500` | `--tw-ring-color: var(--color-slate-500)` |
| `ring-slate-600` | `--tw-ring-color: var(--color-slate-600)` |
| `ring-slate-700` | `--tw-ring-color: var(--color-slate-700)` |
| `ring-slate-800` | `--tw-ring-color: var(--color-slate-800)` |
| `ring-slate-900` | `--tw-ring-color: var(--color-slate-900)` |
| `ring-slate-950` | `--tw-ring-color: var(--color-slate-950)` |
| `ring-gray-50` | `--tw-ring-color: var(--color-gray-50)` |
| `ring-gray-100` | `--tw-ring-color: var(--color-gray-100)` |
| `ring-gray-200` | `--tw-ring-color: var(--color-gray-200)` |
| `ring-gray-300` | `--tw-ring-color: var(--color-gray-300)` |
| `ring-gray-400` | `--tw-ring-color: var(--color-gray-400)` |
| `ring-gray-500` | `--tw-ring-color: var(--color-gray-500)` |
| `ring-gray-600` | `--tw-ring-color: var(--color-gray-600)` |
| `ring-gray-700` | `--tw-ring-color: var(--color-gray-700)` |
| `ring-gray-800` | `--tw-ring-color: var(--color-gray-800)` |
| `ring-gray-900` | `--tw-ring-color: var(--color-gray-900)` |
| `ring-gray-950` | `--tw-ring-color: var(--color-gray-950)` |
| `ring-zinc-50` | `--tw-ring-color: var(--color-zinc-50)` |
| `ring-zinc-100` | `--tw-ring-color: var(--color-zinc-100)` |
| `ring-zinc-200` | `--tw-ring-color: var(--color-zinc-200)` |
| `ring-zinc-300` | `--tw-ring-color: var(--color-zinc-300)` |
| `ring-zinc-400` | `--tw-ring-color: var(--color-zinc-400)` |
| `ring-zinc-500` | `--tw-ring-color: var(--color-zinc-500)` |
| `ring-zinc-600` | `--tw-ring-color: var(--color-zinc-600)` |
| `ring-zinc-700` | `--tw-ring-color: var(--color-zinc-700)` |
| `ring-zinc-800` | `--tw-ring-color: var(--color-zinc-800)` |
| `ring-zinc-900` | `--tw-ring-color: var(--color-zinc-900)` |
| `ring-zinc-950` | `--tw-ring-color: var(--color-zinc-950)` |
| `ring-neutral-50` | `--tw-ring-color: var(--color-neutral-50)` |
| `ring-neutral-100` | `--tw-ring-color: var(--color-neutral-100)` |
| `ring-neutral-200` | `--tw-ring-color: var(--color-neutral-200)` |
| `ring-neutral-300` | `--tw-ring-color: var(--color-neutral-300)` |
| `ring-neutral-400` | `--tw-ring-color: var(--color-neutral-400)` |
| `ring-neutral-500` | `--tw-ring-color: var(--color-neutral-500)` |
| `ring-neutral-600` | `--tw-ring-color: var(--color-neutral-600)` |
| `ring-neutral-700` | `--tw-ring-color: var(--color-neutral-700)` |
| `ring-neutral-800` | `--tw-ring-color: var(--color-neutral-800)` |
| `ring-neutral-900` | `--tw-ring-color: var(--color-neutral-900)` |
| `ring-neutral-950` | `--tw-ring-color: var(--color-neutral-950)` |
| `ring-stone-50` | `--tw-ring-color: var(--color-stone-50)` |
| `ring-stone-100` | `--tw-ring-color: var(--color-stone-100)` |
| `ring-stone-200` | `--tw-ring-color: var(--color-stone-200)` |
| `ring-stone-300` | `--tw-ring-color: var(--color-stone-300)` |
| `ring-stone-400` | `--tw-ring-color: var(--color-stone-400)` |
| `ring-stone-500` | `--tw-ring-color: var(--color-stone-500)` |
| `ring-stone-600` | `--tw-ring-color: var(--color-stone-600)` |
| `ring-stone-700` | `--tw-ring-color: var(--color-stone-700)` |
| `ring-stone-800` | `--tw-ring-color: var(--color-stone-800)` |
| `ring-stone-900` | `--tw-ring-color: var(--color-stone-900)` |
| `ring-stone-950` | `--tw-ring-color: var(--color-stone-950)` |
| `ring-red-50` | `--tw-ring-color: var(--color-red-50)` |
| `ring-red-100` | `--tw-ring-color: var(--color-red-100)` |
| `ring-red-200` | `--tw-ring-color: var(--color-red-200)` |
| `ring-red-300` | `--tw-ring-color: var(--color-red-300)` |
| `ring-red-400` | `--tw-ring-color: var(--color-red-400)` |
| `ring-red-500` | `--tw-ring-color: var(--color-red-500)` |
| `ring-red-600` | `--tw-ring-color: var(--color-red-600)` |
| `ring-red-700` | `--tw-ring-color: var(--color-red-700)` |
| `ring-red-800` | `--tw-ring-color: var(--color-red-800)` |
| `ring-red-900` | `--tw-ring-color: var(--color-red-900)` |
| `ring-red-950` | `--tw-ring-color: var(--color-red-950)` |
| `ring-orange-50` | `--tw-ring-color: var(--color-orange-50)` |
| `ring-orange-100` | `--tw-ring-color: var(--color-orange-100)` |
| `ring-orange-200` | `--tw-ring-color: var(--color-orange-200)` |
| `ring-orange-300` | `--tw-ring-color: var(--color-orange-300)` |
| `ring-orange-400` | `--tw-ring-color: var(--color-orange-400)` |
| `ring-orange-500` | `--tw-ring-color: var(--color-orange-500)` |
| `ring-orange-600` | `--tw-ring-color: var(--color-orange-600)` |
| `ring-orange-700` | `--tw-ring-color: var(--color-orange-700)` |
| `ring-orange-800` | `--tw-ring-color: var(--color-orange-800)` |
| `ring-orange-900` | `--tw-ring-color: var(--color-orange-900)` |
| `ring-orange-950` | `--tw-ring-color: var(--color-orange-950)` |
| `ring-amber-50` | `--tw-ring-color: var(--color-amber-50)` |
| `ring-amber-100` | `--tw-ring-color: var(--color-amber-100)` |
| `ring-amber-200` | `--tw-ring-color: var(--color-amber-200)` |
| `ring-amber-300` | `--tw-ring-color: var(--color-amber-300)` |
| `ring-amber-400` | `--tw-ring-color: var(--color-amber-400)` |
| `ring-amber-500` | `--tw-ring-color: var(--color-amber-500)` |
| `ring-amber-600` | `--tw-ring-color: var(--color-amber-600)` |
| `ring-amber-700` | `--tw-ring-color: var(--color-amber-700)` |
| `ring-amber-800` | `--tw-ring-color: var(--color-amber-800)` |
| `ring-amber-900` | `--tw-ring-color: var(--color-amber-900)` |
| `ring-amber-950` | `--tw-ring-color: var(--color-amber-950)` |
| `ring-yellow-50` | `--tw-ring-color: var(--color-yellow-50)` |
| `ring-yellow-100` | `--tw-ring-color: var(--color-yellow-100)` |
| `ring-yellow-200` | `--tw-ring-color: var(--color-yellow-200)` |
| `ring-yellow-300` | `--tw-ring-color: var(--color-yellow-300)` |
| `ring-yellow-400` | `--tw-ring-color: var(--color-yellow-400)` |
| `ring-yellow-500` | `--tw-ring-color: var(--color-yellow-500)` |
| `ring-yellow-600` | `--tw-ring-color: var(--color-yellow-600)` |
| `ring-yellow-700` | `--tw-ring-color: var(--color-yellow-700)` |
| `ring-yellow-800` | `--tw-ring-color: var(--color-yellow-800)` |
| `ring-yellow-900` | `--tw-ring-color: var(--color-yellow-900)` |
| `ring-yellow-950` | `--tw-ring-color: var(--color-yellow-950)` |
| `ring-lime-50` | `--tw-ring-color: var(--color-lime-50)` |
| `ring-lime-100` | `--tw-ring-color: var(--color-lime-100)` |
| `ring-lime-200` | `--tw-ring-color: var(--color-lime-200)` |
| `ring-lime-300` | `--tw-ring-color: var(--color-lime-300)` |
| `ring-lime-400` | `--tw-ring-color: var(--color-lime-400)` |
| `ring-lime-500` | `--tw-ring-color: var(--color-lime-500)` |
| `ring-lime-600` | `--tw-ring-color: var(--color-lime-600)` |
| `ring-lime-700` | `--tw-ring-color: var(--color-lime-700)` |
| `ring-lime-800` | `--tw-ring-color: var(--color-lime-800)` |
| `ring-lime-900` | `--tw-ring-color: var(--color-lime-900)` |
| `ring-lime-950` | `--tw-ring-color: var(--color-lime-950)` |
| `ring-green-50` | `--tw-ring-color: var(--color-green-50)` |
| `ring-green-100` | `--tw-ring-color: var(--color-green-100)` |
| `ring-green-200` | `--tw-ring-color: var(--color-green-200)` |
| `ring-green-300` | `--tw-ring-color: var(--color-green-300)` |
| `ring-green-400` | `--tw-ring-color: var(--color-green-400)` |
| `ring-green-500` | `--tw-ring-color: var(--color-green-500)` |
| `ring-green-600` | `--tw-ring-color: var(--color-green-600)` |
| `ring-green-700` | `--tw-ring-color: var(--color-green-700)` |
| `ring-green-800` | `--tw-ring-color: var(--color-green-800)` |
| `ring-green-900` | `--tw-ring-color: var(--color-green-900)` |
| `ring-green-950` | `--tw-ring-color: var(--color-green-950)` |
| `ring-emerald-50` | `--tw-ring-color: var(--color-emerald-50)` |
| `ring-emerald-100` | `--tw-ring-color: var(--color-emerald-100)` |
| `ring-emerald-200` | `--tw-ring-color: var(--color-emerald-200)` |
| `ring-emerald-300` | `--tw-ring-color: var(--color-emerald-300)` |
| `ring-emerald-400` | `--tw-ring-color: var(--color-emerald-400)` |
| `ring-emerald-500` | `--tw-ring-color: var(--color-emerald-500)` |
| `ring-emerald-600` | `--tw-ring-color: var(--color-emerald-600)` |
| `ring-emerald-700` | `--tw-ring-color: var(--color-emerald-700)` |
| `ring-emerald-800` | `--tw-ring-color: var(--color-emerald-800)` |
| `ring-emerald-900` | `--tw-ring-color: var(--color-emerald-900)` |
| `ring-emerald-950` | `--tw-ring-color: var(--color-emerald-950)` |
| `ring-teal-50` | `--tw-ring-color: var(--color-teal-50)` |
| `ring-teal-100` | `--tw-ring-color: var(--color-teal-100)` |
| `ring-teal-200` | `--tw-ring-color: var(--color-teal-200)` |
| `ring-teal-300` | `--tw-ring-color: var(--color-teal-300)` |
| `ring-teal-400` | `--tw-ring-color: var(--color-teal-400)` |
| `ring-teal-500` | `--tw-ring-color: var(--color-teal-500)` |
| `ring-teal-600` | `--tw-ring-color: var(--color-teal-600)` |
| `ring-teal-700` | `--tw-ring-color: var(--color-teal-700)` |
| `ring-teal-800` | `--tw-ring-color: var(--color-teal-800)` |
| `ring-teal-900` | `--tw-ring-color: var(--color-teal-900)` |
| `ring-teal-950` | `--tw-ring-color: var(--color-teal-950)` |
| `ring-cyan-50` | `--tw-ring-color: var(--color-cyan-50)` |
| `ring-cyan-100` | `--tw-ring-color: var(--color-cyan-100)` |
| `ring-cyan-200` | `--tw-ring-color: var(--color-cyan-200)` |
| `ring-cyan-300` | `--tw-ring-color: var(--color-cyan-300)` |
| `ring-cyan-400` | `--tw-ring-color: var(--color-cyan-400)` |
| `ring-cyan-500` | `--tw-ring-color: var(--color-cyan-500)` |
| `ring-cyan-600` | `--tw-ring-color: var(--color-cyan-600)` |
| `ring-cyan-700` | `--tw-ring-color: var(--color-cyan-700)` |
| `ring-cyan-800` | `--tw-ring-color: var(--color-cyan-800)` |
| `ring-cyan-900` | `--tw-ring-color: var(--color-cyan-900)` |
| `ring-cyan-950` | `--tw-ring-color: var(--color-cyan-950)` |
| `ring-sky-50` | `--tw-ring-color: var(--color-sky-50)` |
| `ring-sky-100` | `--tw-ring-color: var(--color-sky-100)` |
| `ring-sky-200` | `--tw-ring-color: var(--color-sky-200)` |
| `ring-sky-300` | `--tw-ring-color: var(--color-sky-300)` |
| `ring-sky-400` | `--tw-ring-color: var(--color-sky-400)` |
| `ring-sky-500` | `--tw-ring-color: var(--color-sky-500)` |
| `ring-sky-600` | `--tw-ring-color: var(--color-sky-600)` |
| `ring-sky-700` | `--tw-ring-color: var(--color-sky-700)` |
| `ring-sky-800` | `--tw-ring-color: var(--color-sky-800)` |
| `ring-sky-900` | `--tw-ring-color: var(--color-sky-900)` |
| `ring-sky-950` | `--tw-ring-color: var(--color-sky-950)` |
| `ring-blue-50` | `--tw-ring-color: var(--color-blue-50)` |
| `ring-blue-100` | `--tw-ring-color: var(--color-blue-100)` |
| `ring-blue-200` | `--tw-ring-color: var(--color-blue-200)` |
| `ring-blue-300` | `--tw-ring-color: var(--color-blue-300)` |
| `ring-blue-400` | `--tw-ring-color: var(--color-blue-400)` |
| `ring-blue-500` | `--tw-ring-color: var(--color-blue-500)` |
| `ring-blue-600` | `--tw-ring-color: var(--color-blue-600)` |
| `ring-blue-700` | `--tw-ring-color: var(--color-blue-700)` |
| `ring-blue-800` | `--tw-ring-color: var(--color-blue-800)` |
| `ring-blue-900` | `--tw-ring-color: var(--color-blue-900)` |
| `ring-blue-950` | `--tw-ring-color: var(--color-blue-950)` |
| `ring-indigo-50` | `--tw-ring-color: var(--color-indigo-50)` |
| `ring-indigo-100` | `--tw-ring-color: var(--color-indigo-100)` |
| `ring-indigo-200` | `--tw-ring-color: var(--color-indigo-200)` |
| `ring-indigo-300` | `--tw-ring-color: var(--color-indigo-300)` |
| `ring-indigo-400` | `--tw-ring-color: var(--color-indigo-400)` |
| `ring-indigo-500` | `--tw-ring-color: var(--color-indigo-500)` |
| `ring-indigo-600` | `--tw-ring-color: var(--color-indigo-600)` |
| `ring-indigo-700` | `--tw-ring-color: var(--color-indigo-700)` |
| `ring-indigo-800` | `--tw-ring-color: var(--color-indigo-800)` |
| `ring-indigo-900` | `--tw-ring-color: var(--color-indigo-900)` |
| `ring-indigo-950` | `--tw-ring-color: var(--color-indigo-950)` |
| `ring-violet-50` | `--tw-ring-color: var(--color-violet-50)` |
| `ring-violet-100` | `--tw-ring-color: var(--color-violet-100)` |
| `ring-violet-200` | `--tw-ring-color: var(--color-violet-200)` |
| `ring-violet-300` | `--tw-ring-color: var(--color-violet-300)` |
| `ring-violet-400` | `--tw-ring-color: var(--color-violet-400)` |
| `ring-violet-500` | `--tw-ring-color: var(--color-violet-500)` |
| `ring-violet-600` | `--tw-ring-color: var(--color-violet-600)` |
| `ring-violet-700` | `--tw-ring-color: var(--color-violet-700)` |
| `ring-violet-800` | `--tw-ring-color: var(--color-violet-800)` |
| `ring-violet-900` | `--tw-ring-color: var(--color-violet-900)` |
| `ring-violet-950` | `--tw-ring-color: var(--color-violet-950)` |
| `ring-purple-50` | `--tw-ring-color: var(--color-purple-50)` |
| `ring-purple-100` | `--tw-ring-color: var(--color-purple-100)` |
| `ring-purple-200` | `--tw-ring-color: var(--color-purple-200)` |
| `ring-purple-300` | `--tw-ring-color: var(--color-purple-300)` |
| `ring-purple-400` | `--tw-ring-color: var(--color-purple-400)` |
| `ring-purple-500` | `--tw-ring-color: var(--color-purple-500)` |
| `ring-purple-600` | `--tw-ring-color: var(--color-purple-600)` |
| `ring-purple-700` | `--tw-ring-color: var(--color-purple-700)` |
| `ring-purple-800` | `--tw-ring-color: var(--color-purple-800)` |
| `ring-purple-900` | `--tw-ring-color: var(--color-purple-900)` |
| `ring-purple-950` | `--tw-ring-color: var(--color-purple-950)` |
| `ring-fuchsia-50` | `--tw-ring-color: var(--color-fuchsia-50)` |
| `ring-fuchsia-100` | `--tw-ring-color: var(--color-fuchsia-100)` |
| `ring-fuchsia-200` | `--tw-ring-color: var(--color-fuchsia-200)` |
| `ring-fuchsia-300` | `--tw-ring-color: var(--color-fuchsia-300)` |
| `ring-fuchsia-400` | `--tw-ring-color: var(--color-fuchsia-400)` |
| `ring-fuchsia-500` | `--tw-ring-color: var(--color-fuchsia-500)` |
| `ring-fuchsia-600` | `--tw-ring-color: var(--color-fuchsia-600)` |
| `ring-fuchsia-700` | `--tw-ring-color: var(--color-fuchsia-700)` |
| `ring-fuchsia-800` | `--tw-ring-color: var(--color-fuchsia-800)` |
| `ring-fuchsia-900` | `--tw-ring-color: var(--color-fuchsia-900)` |
| `ring-fuchsia-950` | `--tw-ring-color: var(--color-fuchsia-950)` |
| `ring-pink-50` | `--tw-ring-color: var(--color-pink-50)` |
| `ring-pink-100` | `--tw-ring-color: var(--color-pink-100)` |
| `ring-pink-200` | `--tw-ring-color: var(--color-pink-200)` |
| `ring-pink-300` | `--tw-ring-color: var(--color-pink-300)` |
| `ring-pink-400` | `--tw-ring-color: var(--color-pink-400)` |
| `ring-pink-500` | `--tw-ring-color: var(--color-pink-500)` |
| `ring-pink-600` | `--tw-ring-color: var(--color-pink-600)` |
| `ring-pink-700` | `--tw-ring-color: var(--color-pink-700)` |
| `ring-pink-800` | `--tw-ring-color: var(--color-pink-800)` |
| `ring-pink-900` | `--tw-ring-color: var(--color-pink-900)` |
| `ring-pink-950` | `--tw-ring-color: var(--color-pink-950)` |
| `ring-rose-50` | `--tw-ring-color: var(--color-rose-50)` |
| `ring-rose-100` | `--tw-ring-color: var(--color-rose-100)` |
| `ring-rose-200` | `--tw-ring-color: var(--color-rose-200)` |
| `ring-rose-300` | `--tw-ring-color: var(--color-rose-300)` |
| `ring-rose-400` | `--tw-ring-color: var(--color-rose-400)` |
| `ring-rose-500` | `--tw-ring-color: var(--color-rose-500)` |
| `ring-rose-600` | `--tw-ring-color: var(--color-rose-600)` |
| `ring-rose-700` | `--tw-ring-color: var(--color-rose-700)` |
| `ring-rose-800` | `--tw-ring-color: var(--color-rose-800)` |
| `ring-rose-900` | `--tw-ring-color: var(--color-rose-900)` |
| `ring-rose-950` | `--tw-ring-color: var(--color-rose-950)` |
| `ring-white` | `--tw-ring-color: white` |
| `ring-black` | `--tw-ring-color: black` |
| `ring-transparent` | `--tw-ring-color: transparent` |
| `ring-inherit` | `--tw-ring-color: inherit` |
| `ring-current` | `--tw-ring-color: current` |
| `ring-offset-0` | `--tw-ring-offset-width: 0px` |
| `ring-offset-1` | `--tw-ring-offset-width: 1px` |
| `ring-offset-2` | `--tw-ring-offset-width: 2px` |
| `ring-offset-4` | `--tw-ring-offset-width: 4px` |
| `ring-offset-8` | `--tw-ring-offset-width: 8px` |

## Effects — Shadow

| Classe | CSS |
|--------|-----|
| `shadow-sm` | `box-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.05)` |
| `shadow` | `box-shadow: 0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)` |
| `shadow-md` | `box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)` |
| `shadow-lg` | `box-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)` |
| `shadow-xl` | `box-shadow: 0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)` |
| `shadow-2xl` | `box-shadow: 0 25px 50px -12px rgb(0 0 0 / 0.25)` |
| `shadow-inner` | `box-shadow: inset 0 2px 4px 0 rgb(0 0 0 / 0.05)` |
| `shadow-none` | `box-shadow: 0 0 #0000` |

## Effects — Opacity

| Classe | CSS |
|--------|-----|
| `opacity-0` | `opacity: 0` |
| `opacity-5` | `opacity: 0.05` |
| `opacity-10` | `opacity: 0.1` |
| `opacity-15` | `opacity: 0.15` |
| `opacity-20` | `opacity: 0.2` |
| `opacity-25` | `opacity: 0.25` |
| `opacity-30` | `opacity: 0.3` |
| `opacity-35` | `opacity: 0.35` |
| `opacity-40` | `opacity: 0.4` |
| `opacity-45` | `opacity: 0.45` |
| `opacity-50` | `opacity: 0.5` |
| `opacity-55` | `opacity: 0.55` |
| `opacity-60` | `opacity: 0.6` |
| `opacity-65` | `opacity: 0.65` |
| `opacity-70` | `opacity: 0.7` |
| `opacity-75` | `opacity: 0.75` |
| `opacity-80` | `opacity: 0.8` |
| `opacity-85` | `opacity: 0.85` |
| `opacity-90` | `opacity: 0.9` |
| `opacity-95` | `opacity: 0.95` |
| `opacity-100` | `opacity: 1` |

## Filters

| Classe | CSS |
|--------|-----|
| `blur-none` | `filter: blur(0)` |
| `blur-sm` | `filter: blur(4px)` |
| `blur` | `filter: blur(8px)` |
| `blur-md` | `filter: blur(12px)` |
| `blur-lg` | `filter: blur(16px)` |
| `blur-xl` | `filter: blur(24px)` |
| `blur-2xl` | `filter: blur(40px)` |
| `blur-3xl` | `filter: blur(64px)` |
| `brightness-0` | `filter: brightness(0)` |
| `brightness-50` | `filter: brightness(0.5)` |
| `brightness-75` | `filter: brightness(0.75)` |
| `brightness-90` | `filter: brightness(0.9)` |
| `brightness-95` | `filter: brightness(0.95)` |
| `brightness-100` | `filter: brightness(1)` |
| `brightness-105` | `filter: brightness(1.05)` |
| `brightness-110` | `filter: brightness(1.1)` |
| `brightness-125` | `filter: brightness(1.25)` |
| `brightness-150` | `filter: brightness(1.5)` |
| `brightness-200` | `filter: brightness(2)` |
| `contrast-0` | `filter: contrast(0)` |
| `contrast-50` | `filter: contrast(0.5)` |
| `contrast-75` | `filter: contrast(0.75)` |
| `contrast-100` | `filter: contrast(1)` |
| `contrast-125` | `filter: contrast(1.25)` |
| `contrast-150` | `filter: contrast(1.5)` |
| `contrast-200` | `filter: contrast(2)` |
| `grayscale` | `filter: grayscale(100%)` |
| `grayscale-0` | `filter: grayscale(0)` |
| `hue-rotate-0` | `filter: hue-rotate(0deg)` |
| `hue-rotate-15` | `filter: hue-rotate(15deg)` |
| `hue-rotate-30` | `filter: hue-rotate(30deg)` |
| `hue-rotate-60` | `filter: hue-rotate(60deg)` |
| `hue-rotate-90` | `filter: hue-rotate(90deg)` |
| `hue-rotate-180` | `filter: hue-rotate(180deg)` |
| `invert` | `filter: invert(100%)` |
| `invert-0` | `filter: invert(0)` |
| `saturate-0` | `filter: saturate(0)` |
| `saturate-50` | `filter: saturate(0.5)` |
| `saturate-100` | `filter: saturate(1)` |
| `saturate-150` | `filter: saturate(1.5)` |
| `saturate-200` | `filter: saturate(2)` |
| `sepia` | `filter: sepia(100%)` |
| `sepia-0` | `filter: sepia(0)` |
| `drop-shadow-sm` | `filter: drop-shadow(0 1px 1px rgb(0 0 0 / 0.05))` |
| `drop-shadow` | `filter: drop-shadow(0 1px 2px rgb(0 0 0 / 0.1)) drop-shadow(0 1px 1px rgb(0 0 0 / 0.06))` |
| `drop-shadow-md` | `filter: drop-shadow(0 4px 3px rgb(0 0 0 / 0.07)) drop-shadow(0 2px 2px rgb(0 0 0 / 0.06))` |
| `drop-shadow-lg` | `filter: drop-shadow(0 10px 8px rgb(0 0 0 / 0.04)) drop-shadow(0 4px 3px rgb(0 0 0 / 0.1))` |
| `drop-shadow-xl` | `filter: drop-shadow(0 20px 13px rgb(0 0 0 / 0.03)) drop-shadow(0 8px 5px rgb(0 0 0 / 0.08))` |
| `drop-shadow-none` | `filter: drop-shadow(0 0 #0000)` |

## Transitions

| Classe | CSS |
|--------|-----|
| `transition-none` | `transition-property: none` |
| `transition-all` | `transition-property: all; transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); transition-duration: 150ms` |
| `transition` | `transition-property: color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter; transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); transition-duration: 150ms` |
| `transition-colors` | `transition-property: color, background-color, border-color, text-decoration-color, fill, stroke; transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); transition-duration: 150ms` |
| `transition-opacity` | `transition-property: opacity; transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); transition-duration: 150ms` |
| `transition-shadow` | `transition-property: box-shadow; transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); transition-duration: 150ms` |
| `transition-transform` | `transition-property: transform; transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); transition-duration: 150ms` |
| `duration-0` | `transition-duration: 0ms` |
| `duration-75` | `transition-duration: 75ms` |
| `duration-100` | `transition-duration: 100ms` |
| `duration-150` | `transition-duration: 150ms` |
| `duration-200` | `transition-duration: 200ms` |
| `duration-300` | `transition-duration: 300ms` |
| `duration-500` | `transition-duration: 500ms` |
| `duration-700` | `transition-duration: 700ms` |
| `duration-1000` | `transition-duration: 1000ms` |
| `ease-linear` | `transition-timing-function: linear` |
| `ease-in` | `transition-timing-function: cubic-bezier(0.4, 0, 1, 1)` |
| `ease-out` | `transition-timing-function: cubic-bezier(0, 0, 0.2, 1)` |
| `ease-in-out` | `transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1)` |
| `delay-0` | `transition-delay: 0ms` |
| `delay-75` | `transition-delay: 75ms` |
| `delay-100` | `transition-delay: 100ms` |
| `delay-150` | `transition-delay: 150ms` |
| `delay-200` | `transition-delay: 200ms` |
| `delay-300` | `transition-delay: 300ms` |
| `delay-500` | `transition-delay: 500ms` |
| `delay-700` | `transition-delay: 700ms` |
| `delay-1000` | `transition-delay: 1000ms` |
| `animate-none` | `animation: none` |
| `animate-spin` | `animation: spin 1s linear infinite` |
| `animate-ping` | `animation: ping 1s cubic-bezier(0, 0, 0.2, 1) infinite` |
| `animate-pulse` | `animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite` |
| `animate-bounce` | `animation: bounce 1s infinite` |

## Transforms

| Classe | CSS |
|--------|-----|
| `scale-0` | `--tw-scale-x: 0; --tw-scale-y: 0; transform: scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))` |
| `scale-x-0` | `--tw-scale-x: 0; transform: scaleX(var(--tw-scale-x))` |
| `scale-y-0` | `--tw-scale-y: 0; transform: scaleY(var(--tw-scale-y))` |
| `scale-50` | `--tw-scale-x: 0.5; --tw-scale-y: 0.5; transform: scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))` |
| `scale-x-50` | `--tw-scale-x: 0.5; transform: scaleX(var(--tw-scale-x))` |
| `scale-y-50` | `--tw-scale-y: 0.5; transform: scaleY(var(--tw-scale-y))` |
| `scale-75` | `--tw-scale-x: 0.75; --tw-scale-y: 0.75; transform: scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))` |
| `scale-x-75` | `--tw-scale-x: 0.75; transform: scaleX(var(--tw-scale-x))` |
| `scale-y-75` | `--tw-scale-y: 0.75; transform: scaleY(var(--tw-scale-y))` |
| `scale-90` | `--tw-scale-x: 0.9; --tw-scale-y: 0.9; transform: scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))` |
| `scale-x-90` | `--tw-scale-x: 0.9; transform: scaleX(var(--tw-scale-x))` |
| `scale-y-90` | `--tw-scale-y: 0.9; transform: scaleY(var(--tw-scale-y))` |
| `scale-95` | `--tw-scale-x: 0.95; --tw-scale-y: 0.95; transform: scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))` |
| `scale-x-95` | `--tw-scale-x: 0.95; transform: scaleX(var(--tw-scale-x))` |
| `scale-y-95` | `--tw-scale-y: 0.95; transform: scaleY(var(--tw-scale-y))` |
| `scale-100` | `--tw-scale-x: 1; --tw-scale-y: 1; transform: scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))` |
| `scale-x-100` | `--tw-scale-x: 1; transform: scaleX(var(--tw-scale-x))` |
| `scale-y-100` | `--tw-scale-y: 1; transform: scaleY(var(--tw-scale-y))` |
| `scale-105` | `--tw-scale-x: 1.05; --tw-scale-y: 1.05; transform: scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))` |
| `scale-x-105` | `--tw-scale-x: 1.05; transform: scaleX(var(--tw-scale-x))` |
| `scale-y-105` | `--tw-scale-y: 1.05; transform: scaleY(var(--tw-scale-y))` |
| `scale-110` | `--tw-scale-x: 1.1; --tw-scale-y: 1.1; transform: scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))` |
| `scale-x-110` | `--tw-scale-x: 1.1; transform: scaleX(var(--tw-scale-x))` |
| `scale-y-110` | `--tw-scale-y: 1.1; transform: scaleY(var(--tw-scale-y))` |
| `scale-125` | `--tw-scale-x: 1.25; --tw-scale-y: 1.25; transform: scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))` |
| `scale-x-125` | `--tw-scale-x: 1.25; transform: scaleX(var(--tw-scale-x))` |
| `scale-y-125` | `--tw-scale-y: 1.25; transform: scaleY(var(--tw-scale-y))` |
| `scale-150` | `--tw-scale-x: 1.5; --tw-scale-y: 1.5; transform: scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))` |
| `scale-x-150` | `--tw-scale-x: 1.5; transform: scaleX(var(--tw-scale-x))` |
| `scale-y-150` | `--tw-scale-y: 1.5; transform: scaleY(var(--tw-scale-y))` |
| `rotate-0` | `--tw-rotate: 0deg; transform: rotate(var(--tw-rotate))` |
| `rotate-1` | `--tw-rotate: 1deg; transform: rotate(var(--tw-rotate))` |
| `-rotate-1` | `--tw-rotate: -1deg; transform: rotate(var(--tw-rotate))` |
| `rotate-2` | `--tw-rotate: 2deg; transform: rotate(var(--tw-rotate))` |
| `-rotate-2` | `--tw-rotate: -2deg; transform: rotate(var(--tw-rotate))` |
| `rotate-3` | `--tw-rotate: 3deg; transform: rotate(var(--tw-rotate))` |
| `-rotate-3` | `--tw-rotate: -3deg; transform: rotate(var(--tw-rotate))` |
| `rotate-6` | `--tw-rotate: 6deg; transform: rotate(var(--tw-rotate))` |
| `-rotate-6` | `--tw-rotate: -6deg; transform: rotate(var(--tw-rotate))` |
| `rotate-12` | `--tw-rotate: 12deg; transform: rotate(var(--tw-rotate))` |
| `-rotate-12` | `--tw-rotate: -12deg; transform: rotate(var(--tw-rotate))` |
| `rotate-45` | `--tw-rotate: 45deg; transform: rotate(var(--tw-rotate))` |
| `-rotate-45` | `--tw-rotate: -45deg; transform: rotate(var(--tw-rotate))` |
| `rotate-90` | `--tw-rotate: 90deg; transform: rotate(var(--tw-rotate))` |
| `-rotate-90` | `--tw-rotate: -90deg; transform: rotate(var(--tw-rotate))` |
| `rotate-180` | `--tw-rotate: 180deg; transform: rotate(var(--tw-rotate))` |
| `-rotate-180` | `--tw-rotate: -180deg; transform: rotate(var(--tw-rotate))` |
| `translate-x-0` | `--tw-translate-x: 0px; transform: translateX(var(--tw-translate-x))` |
| `translate-x-px` | `--tw-translate-x: 1px; transform: translateX(var(--tw-translate-x))` |
| `translate-x-0.5` | `--tw-translate-x: calc(var(--spacing) * 0.5); transform: translateX(var(--tw-translate-x))` |
| `translate-x-1` | `--tw-translate-x: calc(var(--spacing) * 1); transform: translateX(var(--tw-translate-x))` |
| `translate-x-1.5` | `--tw-translate-x: calc(var(--spacing) * 1.5); transform: translateX(var(--tw-translate-x))` |
| `translate-x-2` | `--tw-translate-x: calc(var(--spacing) * 2); transform: translateX(var(--tw-translate-x))` |
| `translate-x-2.5` | `--tw-translate-x: calc(var(--spacing) * 2.5); transform: translateX(var(--tw-translate-x))` |
| `translate-x-3` | `--tw-translate-x: calc(var(--spacing) * 3); transform: translateX(var(--tw-translate-x))` |
| `translate-x-3.5` | `--tw-translate-x: calc(var(--spacing) * 3.5); transform: translateX(var(--tw-translate-x))` |
| `translate-x-4` | `--tw-translate-x: calc(var(--spacing) * 4); transform: translateX(var(--tw-translate-x))` |
| `translate-x-5` | `--tw-translate-x: calc(var(--spacing) * 5); transform: translateX(var(--tw-translate-x))` |
| `translate-x-6` | `--tw-translate-x: calc(var(--spacing) * 6); transform: translateX(var(--tw-translate-x))` |
| `translate-x-7` | `--tw-translate-x: calc(var(--spacing) * 7); transform: translateX(var(--tw-translate-x))` |
| `translate-x-8` | `--tw-translate-x: calc(var(--spacing) * 8); transform: translateX(var(--tw-translate-x))` |
| `translate-x-9` | `--tw-translate-x: calc(var(--spacing) * 9); transform: translateX(var(--tw-translate-x))` |
| `translate-x-10` | `--tw-translate-x: calc(var(--spacing) * 10); transform: translateX(var(--tw-translate-x))` |
| `translate-x-11` | `--tw-translate-x: calc(var(--spacing) * 11); transform: translateX(var(--tw-translate-x))` |
| `translate-x-12` | `--tw-translate-x: calc(var(--spacing) * 12); transform: translateX(var(--tw-translate-x))` |
| `translate-x-14` | `--tw-translate-x: calc(var(--spacing) * 14); transform: translateX(var(--tw-translate-x))` |
| `translate-x-16` | `--tw-translate-x: calc(var(--spacing) * 16); transform: translateX(var(--tw-translate-x))` |
| `translate-x-20` | `--tw-translate-x: calc(var(--spacing) * 20); transform: translateX(var(--tw-translate-x))` |
| `translate-x-24` | `--tw-translate-x: calc(var(--spacing) * 24); transform: translateX(var(--tw-translate-x))` |
| `translate-x-28` | `--tw-translate-x: calc(var(--spacing) * 28); transform: translateX(var(--tw-translate-x))` |
| `translate-x-32` | `--tw-translate-x: calc(var(--spacing) * 32); transform: translateX(var(--tw-translate-x))` |
| `translate-x-36` | `--tw-translate-x: calc(var(--spacing) * 36); transform: translateX(var(--tw-translate-x))` |
| `translate-x-40` | `--tw-translate-x: calc(var(--spacing) * 40); transform: translateX(var(--tw-translate-x))` |
| `translate-x-44` | `--tw-translate-x: calc(var(--spacing) * 44); transform: translateX(var(--tw-translate-x))` |
| `translate-x-48` | `--tw-translate-x: calc(var(--spacing) * 48); transform: translateX(var(--tw-translate-x))` |
| `translate-x-52` | `--tw-translate-x: calc(var(--spacing) * 52); transform: translateX(var(--tw-translate-x))` |
| `translate-x-56` | `--tw-translate-x: calc(var(--spacing) * 56); transform: translateX(var(--tw-translate-x))` |
| `translate-x-60` | `--tw-translate-x: calc(var(--spacing) * 60); transform: translateX(var(--tw-translate-x))` |
| `translate-x-64` | `--tw-translate-x: calc(var(--spacing) * 64); transform: translateX(var(--tw-translate-x))` |
| `translate-x-72` | `--tw-translate-x: calc(var(--spacing) * 72); transform: translateX(var(--tw-translate-x))` |
| `translate-x-80` | `--tw-translate-x: calc(var(--spacing) * 80); transform: translateX(var(--tw-translate-x))` |
| `translate-x-96` | `--tw-translate-x: calc(var(--spacing) * 96); transform: translateX(var(--tw-translate-x))` |
| `translate-y-0` | `--tw-translate-y: 0px; transform: translateY(var(--tw-translate-y))` |
| `translate-y-px` | `--tw-translate-y: 1px; transform: translateY(var(--tw-translate-y))` |
| `translate-y-0.5` | `--tw-translate-y: calc(var(--spacing) * 0.5); transform: translateY(var(--tw-translate-y))` |
| `translate-y-1` | `--tw-translate-y: calc(var(--spacing) * 1); transform: translateY(var(--tw-translate-y))` |
| `translate-y-1.5` | `--tw-translate-y: calc(var(--spacing) * 1.5); transform: translateY(var(--tw-translate-y))` |
| `translate-y-2` | `--tw-translate-y: calc(var(--spacing) * 2); transform: translateY(var(--tw-translate-y))` |
| `translate-y-2.5` | `--tw-translate-y: calc(var(--spacing) * 2.5); transform: translateY(var(--tw-translate-y))` |
| `translate-y-3` | `--tw-translate-y: calc(var(--spacing) * 3); transform: translateY(var(--tw-translate-y))` |
| `translate-y-3.5` | `--tw-translate-y: calc(var(--spacing) * 3.5); transform: translateY(var(--tw-translate-y))` |
| `translate-y-4` | `--tw-translate-y: calc(var(--spacing) * 4); transform: translateY(var(--tw-translate-y))` |
| `translate-y-5` | `--tw-translate-y: calc(var(--spacing) * 5); transform: translateY(var(--tw-translate-y))` |
| `translate-y-6` | `--tw-translate-y: calc(var(--spacing) * 6); transform: translateY(var(--tw-translate-y))` |
| `translate-y-7` | `--tw-translate-y: calc(var(--spacing) * 7); transform: translateY(var(--tw-translate-y))` |
| `translate-y-8` | `--tw-translate-y: calc(var(--spacing) * 8); transform: translateY(var(--tw-translate-y))` |
| `translate-y-9` | `--tw-translate-y: calc(var(--spacing) * 9); transform: translateY(var(--tw-translate-y))` |
| `translate-y-10` | `--tw-translate-y: calc(var(--spacing) * 10); transform: translateY(var(--tw-translate-y))` |
| `translate-y-11` | `--tw-translate-y: calc(var(--spacing) * 11); transform: translateY(var(--tw-translate-y))` |
| `translate-y-12` | `--tw-translate-y: calc(var(--spacing) * 12); transform: translateY(var(--tw-translate-y))` |
| `translate-y-14` | `--tw-translate-y: calc(var(--spacing) * 14); transform: translateY(var(--tw-translate-y))` |
| `translate-y-16` | `--tw-translate-y: calc(var(--spacing) * 16); transform: translateY(var(--tw-translate-y))` |
| `translate-y-20` | `--tw-translate-y: calc(var(--spacing) * 20); transform: translateY(var(--tw-translate-y))` |
| `translate-y-24` | `--tw-translate-y: calc(var(--spacing) * 24); transform: translateY(var(--tw-translate-y))` |
| `translate-y-28` | `--tw-translate-y: calc(var(--spacing) * 28); transform: translateY(var(--tw-translate-y))` |
| `translate-y-32` | `--tw-translate-y: calc(var(--spacing) * 32); transform: translateY(var(--tw-translate-y))` |
| `translate-y-36` | `--tw-translate-y: calc(var(--spacing) * 36); transform: translateY(var(--tw-translate-y))` |
| `translate-y-40` | `--tw-translate-y: calc(var(--spacing) * 40); transform: translateY(var(--tw-translate-y))` |
| `translate-y-44` | `--tw-translate-y: calc(var(--spacing) * 44); transform: translateY(var(--tw-translate-y))` |
| `translate-y-48` | `--tw-translate-y: calc(var(--spacing) * 48); transform: translateY(var(--tw-translate-y))` |
| `translate-y-52` | `--tw-translate-y: calc(var(--spacing) * 52); transform: translateY(var(--tw-translate-y))` |
| `translate-y-56` | `--tw-translate-y: calc(var(--spacing) * 56); transform: translateY(var(--tw-translate-y))` |
| `translate-y-60` | `--tw-translate-y: calc(var(--spacing) * 60); transform: translateY(var(--tw-translate-y))` |
| `translate-y-64` | `--tw-translate-y: calc(var(--spacing) * 64); transform: translateY(var(--tw-translate-y))` |
| `translate-y-72` | `--tw-translate-y: calc(var(--spacing) * 72); transform: translateY(var(--tw-translate-y))` |
| `translate-y-80` | `--tw-translate-y: calc(var(--spacing) * 80); transform: translateY(var(--tw-translate-y))` |
| `translate-y-96` | `--tw-translate-y: calc(var(--spacing) * 96); transform: translateY(var(--tw-translate-y))` |
| `translate-x-full` | `--tw-translate-x: 100%; transform: translateX(var(--tw-translate-x))` |
| `translate-y-full` | `--tw-translate-y: 100%; transform: translateY(var(--tw-translate-y))` |
| `translate-x-1/2` | `--tw-translate-x: 50%; transform: translateX(var(--tw-translate-x))` |
| `translate-y-1/2` | `--tw-translate-y: 50%; transform: translateY(var(--tw-translate-y))` |
| `skew-x-0` | `--tw-skew-x: 0deg; transform: skewX(var(--tw-skew-x))` |
| `skew-y-0` | `--tw-skew-y: 0deg; transform: skewY(var(--tw-skew-y))` |
| `skew-x-1` | `--tw-skew-x: 1deg; transform: skewX(var(--tw-skew-x))` |
| `skew-y-1` | `--tw-skew-y: 1deg; transform: skewY(var(--tw-skew-y))` |
| `skew-x-2` | `--tw-skew-x: 2deg; transform: skewX(var(--tw-skew-x))` |
| `skew-y-2` | `--tw-skew-y: 2deg; transform: skewY(var(--tw-skew-y))` |
| `skew-x-3` | `--tw-skew-x: 3deg; transform: skewX(var(--tw-skew-x))` |
| `skew-y-3` | `--tw-skew-y: 3deg; transform: skewY(var(--tw-skew-y))` |
| `skew-x-6` | `--tw-skew-x: 6deg; transform: skewX(var(--tw-skew-x))` |
| `skew-y-6` | `--tw-skew-y: 6deg; transform: skewY(var(--tw-skew-y))` |
| `skew-x-12` | `--tw-skew-x: 12deg; transform: skewX(var(--tw-skew-x))` |
| `skew-y-12` | `--tw-skew-y: 12deg; transform: skewY(var(--tw-skew-y))` |
| `origin-center` | `transform-origin: center` |
| `origin-top` | `transform-origin: top` |
| `origin-top-right` | `transform-origin: top-right` |
| `origin-right` | `transform-origin: right` |
| `origin-bottom-right` | `transform-origin: bottom-right` |
| `origin-bottom` | `transform-origin: bottom` |
| `origin-bottom-left` | `transform-origin: bottom-left` |
| `origin-left` | `transform-origin: left` |
| `origin-top-left` | `transform-origin: top-left` |

## Interactivity

| Classe | CSS |
|--------|-----|
| `cursor-auto` | `cursor: auto` |
| `cursor-default` | `cursor: default` |
| `cursor-pointer` | `cursor: pointer` |
| `cursor-wait` | `cursor: wait` |
| `cursor-text` | `cursor: text` |
| `cursor-move` | `cursor: move` |
| `cursor-help` | `cursor: help` |
| `cursor-not-allowed` | `cursor: not-allowed` |
| `cursor-none` | `cursor: none` |
| `cursor-context-menu` | `cursor: context-menu` |
| `cursor-progress` | `cursor: progress` |
| `cursor-cell` | `cursor: cell` |
| `cursor-crosshair` | `cursor: crosshair` |
| `cursor-vertical-text` | `cursor: vertical-text` |
| `cursor-alias` | `cursor: alias` |
| `cursor-copy` | `cursor: copy` |
| `cursor-no-drop` | `cursor: no-drop` |
| `cursor-grab` | `cursor: grab` |
| `cursor-grabbing` | `cursor: grabbing` |
| `cursor-all-scroll` | `cursor: all-scroll` |
| `cursor-col-resize` | `cursor: col-resize` |
| `cursor-row-resize` | `cursor: row-resize` |
| `cursor-n-resize` | `cursor: n-resize` |
| `cursor-e-resize` | `cursor: e-resize` |
| `cursor-s-resize` | `cursor: s-resize` |
| `cursor-w-resize` | `cursor: w-resize` |
| `cursor-ne-resize` | `cursor: ne-resize` |
| `cursor-nw-resize` | `cursor: nw-resize` |
| `cursor-se-resize` | `cursor: se-resize` |
| `cursor-sw-resize` | `cursor: sw-resize` |
| `cursor-ew-resize` | `cursor: ew-resize` |
| `cursor-ns-resize` | `cursor: ns-resize` |
| `cursor-nesw-resize` | `cursor: nesw-resize` |
| `cursor-nwse-resize` | `cursor: nwse-resize` |
| `cursor-zoom-in` | `cursor: zoom-in` |
| `cursor-zoom-out` | `cursor: zoom-out` |
| `touch-auto` | `touch-action: auto` |
| `touch-none` | `touch-action: none` |
| `touch-pan-x` | `touch-action: pan-x` |
| `touch-pan-left` | `touch-action: pan-left` |
| `touch-pan-right` | `touch-action: pan-right` |
| `touch-pan-y` | `touch-action: pan-y` |
| `touch-pan-up` | `touch-action: pan-up` |
| `touch-pan-down` | `touch-action: pan-down` |
| `touch-pinch-zoom` | `touch-action: pinch-zoom` |
| `touch-manipulation` | `touch-action: manipulation` |
| `select-none` | `user-select: none` |
| `select-text` | `user-select: text` |
| `select-all` | `user-select: all` |
| `select-auto` | `user-select: auto` |
| `resize` | `resize: both` |
| `resize-none` | `resize: none` |
| `resize-y` | `resize: vertical` |
| `resize-x` | `resize: horizontal` |
| `scroll-auto` | `scroll-behavior: auto` |
| `scroll-smooth` | `scroll-behavior: smooth` |
| `snap-none` | `scroll-snap-type: none` |
| `snap-x` | `scroll-snap-type: x var(--tw-scroll-snap-strictness)` |
| `snap-y` | `scroll-snap-type: y var(--tw-scroll-snap-strictness)` |
| `snap-mandatory` | `--tw-scroll-snap-strictness: mandatory` |
| `snap-proximity` | `--tw-scroll-snap-strictness: proximity` |
| `snap-start` | `scroll-snap-align: start` |
| `snap-end` | `scroll-snap-align: end` |
| `snap-center` | `scroll-snap-align: center` |
| `pointer-events-none` | `pointer-events: none` |
| `pointer-events-auto` | `pointer-events: auto` |
| `will-change-auto` | `will-change: auto` |
| `will-change-scroll` | `will-change: scroll-position` |
| `will-change-contents` | `will-change: contents` |
| `will-change-transform` | `will-change: transform` |
| `appearance-none` | `appearance: none` |
| `appearance-auto` | `appearance: auto` |

## SVG

| Classe | CSS |
|--------|-----|
| `fill-slate-50` | `fill: var(--color-slate-50)` |
| `fill-slate-100` | `fill: var(--color-slate-100)` |
| `fill-slate-200` | `fill: var(--color-slate-200)` |
| `fill-slate-300` | `fill: var(--color-slate-300)` |
| `fill-slate-400` | `fill: var(--color-slate-400)` |
| `fill-slate-500` | `fill: var(--color-slate-500)` |
| `fill-slate-600` | `fill: var(--color-slate-600)` |
| `fill-slate-700` | `fill: var(--color-slate-700)` |
| `fill-slate-800` | `fill: var(--color-slate-800)` |
| `fill-slate-900` | `fill: var(--color-slate-900)` |
| `fill-slate-950` | `fill: var(--color-slate-950)` |
| `fill-gray-50` | `fill: var(--color-gray-50)` |
| `fill-gray-100` | `fill: var(--color-gray-100)` |
| `fill-gray-200` | `fill: var(--color-gray-200)` |
| `fill-gray-300` | `fill: var(--color-gray-300)` |
| `fill-gray-400` | `fill: var(--color-gray-400)` |
| `fill-gray-500` | `fill: var(--color-gray-500)` |
| `fill-gray-600` | `fill: var(--color-gray-600)` |
| `fill-gray-700` | `fill: var(--color-gray-700)` |
| `fill-gray-800` | `fill: var(--color-gray-800)` |
| `fill-gray-900` | `fill: var(--color-gray-900)` |
| `fill-gray-950` | `fill: var(--color-gray-950)` |
| `fill-zinc-50` | `fill: var(--color-zinc-50)` |
| `fill-zinc-100` | `fill: var(--color-zinc-100)` |
| `fill-zinc-200` | `fill: var(--color-zinc-200)` |
| `fill-zinc-300` | `fill: var(--color-zinc-300)` |
| `fill-zinc-400` | `fill: var(--color-zinc-400)` |
| `fill-zinc-500` | `fill: var(--color-zinc-500)` |
| `fill-zinc-600` | `fill: var(--color-zinc-600)` |
| `fill-zinc-700` | `fill: var(--color-zinc-700)` |
| `fill-zinc-800` | `fill: var(--color-zinc-800)` |
| `fill-zinc-900` | `fill: var(--color-zinc-900)` |
| `fill-zinc-950` | `fill: var(--color-zinc-950)` |
| `fill-neutral-50` | `fill: var(--color-neutral-50)` |
| `fill-neutral-100` | `fill: var(--color-neutral-100)` |
| `fill-neutral-200` | `fill: var(--color-neutral-200)` |
| `fill-neutral-300` | `fill: var(--color-neutral-300)` |
| `fill-neutral-400` | `fill: var(--color-neutral-400)` |
| `fill-neutral-500` | `fill: var(--color-neutral-500)` |
| `fill-neutral-600` | `fill: var(--color-neutral-600)` |
| `fill-neutral-700` | `fill: var(--color-neutral-700)` |
| `fill-neutral-800` | `fill: var(--color-neutral-800)` |
| `fill-neutral-900` | `fill: var(--color-neutral-900)` |
| `fill-neutral-950` | `fill: var(--color-neutral-950)` |
| `fill-stone-50` | `fill: var(--color-stone-50)` |
| `fill-stone-100` | `fill: var(--color-stone-100)` |
| `fill-stone-200` | `fill: var(--color-stone-200)` |
| `fill-stone-300` | `fill: var(--color-stone-300)` |
| `fill-stone-400` | `fill: var(--color-stone-400)` |
| `fill-stone-500` | `fill: var(--color-stone-500)` |
| `fill-stone-600` | `fill: var(--color-stone-600)` |
| `fill-stone-700` | `fill: var(--color-stone-700)` |
| `fill-stone-800` | `fill: var(--color-stone-800)` |
| `fill-stone-900` | `fill: var(--color-stone-900)` |
| `fill-stone-950` | `fill: var(--color-stone-950)` |
| `fill-red-50` | `fill: var(--color-red-50)` |
| `fill-red-100` | `fill: var(--color-red-100)` |
| `fill-red-200` | `fill: var(--color-red-200)` |
| `fill-red-300` | `fill: var(--color-red-300)` |
| `fill-red-400` | `fill: var(--color-red-400)` |
| `fill-red-500` | `fill: var(--color-red-500)` |
| `fill-red-600` | `fill: var(--color-red-600)` |
| `fill-red-700` | `fill: var(--color-red-700)` |
| `fill-red-800` | `fill: var(--color-red-800)` |
| `fill-red-900` | `fill: var(--color-red-900)` |
| `fill-red-950` | `fill: var(--color-red-950)` |
| `fill-orange-50` | `fill: var(--color-orange-50)` |
| `fill-orange-100` | `fill: var(--color-orange-100)` |
| `fill-orange-200` | `fill: var(--color-orange-200)` |
| `fill-orange-300` | `fill: var(--color-orange-300)` |
| `fill-orange-400` | `fill: var(--color-orange-400)` |
| `fill-orange-500` | `fill: var(--color-orange-500)` |
| `fill-orange-600` | `fill: var(--color-orange-600)` |
| `fill-orange-700` | `fill: var(--color-orange-700)` |
| `fill-orange-800` | `fill: var(--color-orange-800)` |
| `fill-orange-900` | `fill: var(--color-orange-900)` |
| `fill-orange-950` | `fill: var(--color-orange-950)` |
| `fill-amber-50` | `fill: var(--color-amber-50)` |
| `fill-amber-100` | `fill: var(--color-amber-100)` |
| `fill-amber-200` | `fill: var(--color-amber-200)` |
| `fill-amber-300` | `fill: var(--color-amber-300)` |
| `fill-amber-400` | `fill: var(--color-amber-400)` |
| `fill-amber-500` | `fill: var(--color-amber-500)` |
| `fill-amber-600` | `fill: var(--color-amber-600)` |
| `fill-amber-700` | `fill: var(--color-amber-700)` |
| `fill-amber-800` | `fill: var(--color-amber-800)` |
| `fill-amber-900` | `fill: var(--color-amber-900)` |
| `fill-amber-950` | `fill: var(--color-amber-950)` |
| `fill-yellow-50` | `fill: var(--color-yellow-50)` |
| `fill-yellow-100` | `fill: var(--color-yellow-100)` |
| `fill-yellow-200` | `fill: var(--color-yellow-200)` |
| `fill-yellow-300` | `fill: var(--color-yellow-300)` |
| `fill-yellow-400` | `fill: var(--color-yellow-400)` |
| `fill-yellow-500` | `fill: var(--color-yellow-500)` |
| `fill-yellow-600` | `fill: var(--color-yellow-600)` |
| `fill-yellow-700` | `fill: var(--color-yellow-700)` |
| `fill-yellow-800` | `fill: var(--color-yellow-800)` |
| `fill-yellow-900` | `fill: var(--color-yellow-900)` |
| `fill-yellow-950` | `fill: var(--color-yellow-950)` |
| `fill-lime-50` | `fill: var(--color-lime-50)` |
| `fill-lime-100` | `fill: var(--color-lime-100)` |
| `fill-lime-200` | `fill: var(--color-lime-200)` |
| `fill-lime-300` | `fill: var(--color-lime-300)` |
| `fill-lime-400` | `fill: var(--color-lime-400)` |
| `fill-lime-500` | `fill: var(--color-lime-500)` |
| `fill-lime-600` | `fill: var(--color-lime-600)` |
| `fill-lime-700` | `fill: var(--color-lime-700)` |
| `fill-lime-800` | `fill: var(--color-lime-800)` |
| `fill-lime-900` | `fill: var(--color-lime-900)` |
| `fill-lime-950` | `fill: var(--color-lime-950)` |
| `fill-green-50` | `fill: var(--color-green-50)` |
| `fill-green-100` | `fill: var(--color-green-100)` |
| `fill-green-200` | `fill: var(--color-green-200)` |
| `fill-green-300` | `fill: var(--color-green-300)` |
| `fill-green-400` | `fill: var(--color-green-400)` |
| `fill-green-500` | `fill: var(--color-green-500)` |
| `fill-green-600` | `fill: var(--color-green-600)` |
| `fill-green-700` | `fill: var(--color-green-700)` |
| `fill-green-800` | `fill: var(--color-green-800)` |
| `fill-green-900` | `fill: var(--color-green-900)` |
| `fill-green-950` | `fill: var(--color-green-950)` |
| `fill-emerald-50` | `fill: var(--color-emerald-50)` |
| `fill-emerald-100` | `fill: var(--color-emerald-100)` |
| `fill-emerald-200` | `fill: var(--color-emerald-200)` |
| `fill-emerald-300` | `fill: var(--color-emerald-300)` |
| `fill-emerald-400` | `fill: var(--color-emerald-400)` |
| `fill-emerald-500` | `fill: var(--color-emerald-500)` |
| `fill-emerald-600` | `fill: var(--color-emerald-600)` |
| `fill-emerald-700` | `fill: var(--color-emerald-700)` |
| `fill-emerald-800` | `fill: var(--color-emerald-800)` |
| `fill-emerald-900` | `fill: var(--color-emerald-900)` |
| `fill-emerald-950` | `fill: var(--color-emerald-950)` |
| `fill-teal-50` | `fill: var(--color-teal-50)` |
| `fill-teal-100` | `fill: var(--color-teal-100)` |
| `fill-teal-200` | `fill: var(--color-teal-200)` |
| `fill-teal-300` | `fill: var(--color-teal-300)` |
| `fill-teal-400` | `fill: var(--color-teal-400)` |
| `fill-teal-500` | `fill: var(--color-teal-500)` |
| `fill-teal-600` | `fill: var(--color-teal-600)` |
| `fill-teal-700` | `fill: var(--color-teal-700)` |
| `fill-teal-800` | `fill: var(--color-teal-800)` |
| `fill-teal-900` | `fill: var(--color-teal-900)` |
| `fill-teal-950` | `fill: var(--color-teal-950)` |
| `fill-cyan-50` | `fill: var(--color-cyan-50)` |
| `fill-cyan-100` | `fill: var(--color-cyan-100)` |
| `fill-cyan-200` | `fill: var(--color-cyan-200)` |
| `fill-cyan-300` | `fill: var(--color-cyan-300)` |
| `fill-cyan-400` | `fill: var(--color-cyan-400)` |
| `fill-cyan-500` | `fill: var(--color-cyan-500)` |
| `fill-cyan-600` | `fill: var(--color-cyan-600)` |
| `fill-cyan-700` | `fill: var(--color-cyan-700)` |
| `fill-cyan-800` | `fill: var(--color-cyan-800)` |
| `fill-cyan-900` | `fill: var(--color-cyan-900)` |
| `fill-cyan-950` | `fill: var(--color-cyan-950)` |
| `fill-sky-50` | `fill: var(--color-sky-50)` |
| `fill-sky-100` | `fill: var(--color-sky-100)` |
| `fill-sky-200` | `fill: var(--color-sky-200)` |
| `fill-sky-300` | `fill: var(--color-sky-300)` |
| `fill-sky-400` | `fill: var(--color-sky-400)` |
| `fill-sky-500` | `fill: var(--color-sky-500)` |
| `fill-sky-600` | `fill: var(--color-sky-600)` |
| `fill-sky-700` | `fill: var(--color-sky-700)` |
| `fill-sky-800` | `fill: var(--color-sky-800)` |
| `fill-sky-900` | `fill: var(--color-sky-900)` |
| `fill-sky-950` | `fill: var(--color-sky-950)` |
| `fill-blue-50` | `fill: var(--color-blue-50)` |
| `fill-blue-100` | `fill: var(--color-blue-100)` |
| `fill-blue-200` | `fill: var(--color-blue-200)` |
| `fill-blue-300` | `fill: var(--color-blue-300)` |
| `fill-blue-400` | `fill: var(--color-blue-400)` |
| `fill-blue-500` | `fill: var(--color-blue-500)` |
| `fill-blue-600` | `fill: var(--color-blue-600)` |
| `fill-blue-700` | `fill: var(--color-blue-700)` |
| `fill-blue-800` | `fill: var(--color-blue-800)` |
| `fill-blue-900` | `fill: var(--color-blue-900)` |
| `fill-blue-950` | `fill: var(--color-blue-950)` |
| `fill-indigo-50` | `fill: var(--color-indigo-50)` |
| `fill-indigo-100` | `fill: var(--color-indigo-100)` |
| `fill-indigo-200` | `fill: var(--color-indigo-200)` |
| `fill-indigo-300` | `fill: var(--color-indigo-300)` |
| `fill-indigo-400` | `fill: var(--color-indigo-400)` |
| `fill-indigo-500` | `fill: var(--color-indigo-500)` |
| `fill-indigo-600` | `fill: var(--color-indigo-600)` |
| `fill-indigo-700` | `fill: var(--color-indigo-700)` |
| `fill-indigo-800` | `fill: var(--color-indigo-800)` |
| `fill-indigo-900` | `fill: var(--color-indigo-900)` |
| `fill-indigo-950` | `fill: var(--color-indigo-950)` |
| `fill-violet-50` | `fill: var(--color-violet-50)` |
| `fill-violet-100` | `fill: var(--color-violet-100)` |
| `fill-violet-200` | `fill: var(--color-violet-200)` |
| `fill-violet-300` | `fill: var(--color-violet-300)` |
| `fill-violet-400` | `fill: var(--color-violet-400)` |
| `fill-violet-500` | `fill: var(--color-violet-500)` |
| `fill-violet-600` | `fill: var(--color-violet-600)` |
| `fill-violet-700` | `fill: var(--color-violet-700)` |
| `fill-violet-800` | `fill: var(--color-violet-800)` |
| `fill-violet-900` | `fill: var(--color-violet-900)` |
| `fill-violet-950` | `fill: var(--color-violet-950)` |
| `fill-purple-50` | `fill: var(--color-purple-50)` |
| `fill-purple-100` | `fill: var(--color-purple-100)` |
| `fill-purple-200` | `fill: var(--color-purple-200)` |
| `fill-purple-300` | `fill: var(--color-purple-300)` |
| `fill-purple-400` | `fill: var(--color-purple-400)` |
| `fill-purple-500` | `fill: var(--color-purple-500)` |
| `fill-purple-600` | `fill: var(--color-purple-600)` |
| `fill-purple-700` | `fill: var(--color-purple-700)` |
| `fill-purple-800` | `fill: var(--color-purple-800)` |
| `fill-purple-900` | `fill: var(--color-purple-900)` |
| `fill-purple-950` | `fill: var(--color-purple-950)` |
| `fill-fuchsia-50` | `fill: var(--color-fuchsia-50)` |
| `fill-fuchsia-100` | `fill: var(--color-fuchsia-100)` |
| `fill-fuchsia-200` | `fill: var(--color-fuchsia-200)` |
| `fill-fuchsia-300` | `fill: var(--color-fuchsia-300)` |
| `fill-fuchsia-400` | `fill: var(--color-fuchsia-400)` |
| `fill-fuchsia-500` | `fill: var(--color-fuchsia-500)` |
| `fill-fuchsia-600` | `fill: var(--color-fuchsia-600)` |
| `fill-fuchsia-700` | `fill: var(--color-fuchsia-700)` |
| `fill-fuchsia-800` | `fill: var(--color-fuchsia-800)` |
| `fill-fuchsia-900` | `fill: var(--color-fuchsia-900)` |
| `fill-fuchsia-950` | `fill: var(--color-fuchsia-950)` |
| `fill-pink-50` | `fill: var(--color-pink-50)` |
| `fill-pink-100` | `fill: var(--color-pink-100)` |
| `fill-pink-200` | `fill: var(--color-pink-200)` |
| `fill-pink-300` | `fill: var(--color-pink-300)` |
| `fill-pink-400` | `fill: var(--color-pink-400)` |
| `fill-pink-500` | `fill: var(--color-pink-500)` |
| `fill-pink-600` | `fill: var(--color-pink-600)` |
| `fill-pink-700` | `fill: var(--color-pink-700)` |
| `fill-pink-800` | `fill: var(--color-pink-800)` |
| `fill-pink-900` | `fill: var(--color-pink-900)` |
| `fill-pink-950` | `fill: var(--color-pink-950)` |
| `fill-rose-50` | `fill: var(--color-rose-50)` |
| `fill-rose-100` | `fill: var(--color-rose-100)` |
| `fill-rose-200` | `fill: var(--color-rose-200)` |
| `fill-rose-300` | `fill: var(--color-rose-300)` |
| `fill-rose-400` | `fill: var(--color-rose-400)` |
| `fill-rose-500` | `fill: var(--color-rose-500)` |
| `fill-rose-600` | `fill: var(--color-rose-600)` |
| `fill-rose-700` | `fill: var(--color-rose-700)` |
| `fill-rose-800` | `fill: var(--color-rose-800)` |
| `fill-rose-900` | `fill: var(--color-rose-900)` |
| `fill-rose-950` | `fill: var(--color-rose-950)` |
| `fill-white` | `fill: white` |
| `fill-black` | `fill: black` |
| `fill-transparent` | `fill: transparent` |
| `fill-inherit` | `fill: inherit` |
| `fill-current` | `fill: current` |
| `fill-none` | `fill: none` |
| `stroke-slate-50` | `stroke: var(--color-slate-50)` |
| `stroke-slate-100` | `stroke: var(--color-slate-100)` |
| `stroke-slate-200` | `stroke: var(--color-slate-200)` |
| `stroke-slate-300` | `stroke: var(--color-slate-300)` |
| `stroke-slate-400` | `stroke: var(--color-slate-400)` |
| `stroke-slate-500` | `stroke: var(--color-slate-500)` |
| `stroke-slate-600` | `stroke: var(--color-slate-600)` |
| `stroke-slate-700` | `stroke: var(--color-slate-700)` |
| `stroke-slate-800` | `stroke: var(--color-slate-800)` |
| `stroke-slate-900` | `stroke: var(--color-slate-900)` |
| `stroke-slate-950` | `stroke: var(--color-slate-950)` |
| `stroke-gray-50` | `stroke: var(--color-gray-50)` |
| `stroke-gray-100` | `stroke: var(--color-gray-100)` |
| `stroke-gray-200` | `stroke: var(--color-gray-200)` |
| `stroke-gray-300` | `stroke: var(--color-gray-300)` |
| `stroke-gray-400` | `stroke: var(--color-gray-400)` |
| `stroke-gray-500` | `stroke: var(--color-gray-500)` |
| `stroke-gray-600` | `stroke: var(--color-gray-600)` |
| `stroke-gray-700` | `stroke: var(--color-gray-700)` |
| `stroke-gray-800` | `stroke: var(--color-gray-800)` |
| `stroke-gray-900` | `stroke: var(--color-gray-900)` |
| `stroke-gray-950` | `stroke: var(--color-gray-950)` |
| `stroke-zinc-50` | `stroke: var(--color-zinc-50)` |
| `stroke-zinc-100` | `stroke: var(--color-zinc-100)` |
| `stroke-zinc-200` | `stroke: var(--color-zinc-200)` |
| `stroke-zinc-300` | `stroke: var(--color-zinc-300)` |
| `stroke-zinc-400` | `stroke: var(--color-zinc-400)` |
| `stroke-zinc-500` | `stroke: var(--color-zinc-500)` |
| `stroke-zinc-600` | `stroke: var(--color-zinc-600)` |
| `stroke-zinc-700` | `stroke: var(--color-zinc-700)` |
| `stroke-zinc-800` | `stroke: var(--color-zinc-800)` |
| `stroke-zinc-900` | `stroke: var(--color-zinc-900)` |
| `stroke-zinc-950` | `stroke: var(--color-zinc-950)` |
| `stroke-neutral-50` | `stroke: var(--color-neutral-50)` |
| `stroke-neutral-100` | `stroke: var(--color-neutral-100)` |
| `stroke-neutral-200` | `stroke: var(--color-neutral-200)` |
| `stroke-neutral-300` | `stroke: var(--color-neutral-300)` |
| `stroke-neutral-400` | `stroke: var(--color-neutral-400)` |
| `stroke-neutral-500` | `stroke: var(--color-neutral-500)` |
| `stroke-neutral-600` | `stroke: var(--color-neutral-600)` |
| `stroke-neutral-700` | `stroke: var(--color-neutral-700)` |
| `stroke-neutral-800` | `stroke: var(--color-neutral-800)` |
| `stroke-neutral-900` | `stroke: var(--color-neutral-900)` |
| `stroke-neutral-950` | `stroke: var(--color-neutral-950)` |
| `stroke-stone-50` | `stroke: var(--color-stone-50)` |
| `stroke-stone-100` | `stroke: var(--color-stone-100)` |
| `stroke-stone-200` | `stroke: var(--color-stone-200)` |
| `stroke-stone-300` | `stroke: var(--color-stone-300)` |
| `stroke-stone-400` | `stroke: var(--color-stone-400)` |
| `stroke-stone-500` | `stroke: var(--color-stone-500)` |
| `stroke-stone-600` | `stroke: var(--color-stone-600)` |
| `stroke-stone-700` | `stroke: var(--color-stone-700)` |
| `stroke-stone-800` | `stroke: var(--color-stone-800)` |
| `stroke-stone-900` | `stroke: var(--color-stone-900)` |
| `stroke-stone-950` | `stroke: var(--color-stone-950)` |
| `stroke-red-50` | `stroke: var(--color-red-50)` |
| `stroke-red-100` | `stroke: var(--color-red-100)` |
| `stroke-red-200` | `stroke: var(--color-red-200)` |
| `stroke-red-300` | `stroke: var(--color-red-300)` |
| `stroke-red-400` | `stroke: var(--color-red-400)` |
| `stroke-red-500` | `stroke: var(--color-red-500)` |
| `stroke-red-600` | `stroke: var(--color-red-600)` |
| `stroke-red-700` | `stroke: var(--color-red-700)` |
| `stroke-red-800` | `stroke: var(--color-red-800)` |
| `stroke-red-900` | `stroke: var(--color-red-900)` |
| `stroke-red-950` | `stroke: var(--color-red-950)` |
| `stroke-orange-50` | `stroke: var(--color-orange-50)` |
| `stroke-orange-100` | `stroke: var(--color-orange-100)` |
| `stroke-orange-200` | `stroke: var(--color-orange-200)` |
| `stroke-orange-300` | `stroke: var(--color-orange-300)` |
| `stroke-orange-400` | `stroke: var(--color-orange-400)` |
| `stroke-orange-500` | `stroke: var(--color-orange-500)` |
| `stroke-orange-600` | `stroke: var(--color-orange-600)` |
| `stroke-orange-700` | `stroke: var(--color-orange-700)` |
| `stroke-orange-800` | `stroke: var(--color-orange-800)` |
| `stroke-orange-900` | `stroke: var(--color-orange-900)` |
| `stroke-orange-950` | `stroke: var(--color-orange-950)` |
| `stroke-amber-50` | `stroke: var(--color-amber-50)` |
| `stroke-amber-100` | `stroke: var(--color-amber-100)` |
| `stroke-amber-200` | `stroke: var(--color-amber-200)` |
| `stroke-amber-300` | `stroke: var(--color-amber-300)` |
| `stroke-amber-400` | `stroke: var(--color-amber-400)` |
| `stroke-amber-500` | `stroke: var(--color-amber-500)` |
| `stroke-amber-600` | `stroke: var(--color-amber-600)` |
| `stroke-amber-700` | `stroke: var(--color-amber-700)` |
| `stroke-amber-800` | `stroke: var(--color-amber-800)` |
| `stroke-amber-900` | `stroke: var(--color-amber-900)` |
| `stroke-amber-950` | `stroke: var(--color-amber-950)` |
| `stroke-yellow-50` | `stroke: var(--color-yellow-50)` |
| `stroke-yellow-100` | `stroke: var(--color-yellow-100)` |
| `stroke-yellow-200` | `stroke: var(--color-yellow-200)` |
| `stroke-yellow-300` | `stroke: var(--color-yellow-300)` |
| `stroke-yellow-400` | `stroke: var(--color-yellow-400)` |
| `stroke-yellow-500` | `stroke: var(--color-yellow-500)` |
| `stroke-yellow-600` | `stroke: var(--color-yellow-600)` |
| `stroke-yellow-700` | `stroke: var(--color-yellow-700)` |
| `stroke-yellow-800` | `stroke: var(--color-yellow-800)` |
| `stroke-yellow-900` | `stroke: var(--color-yellow-900)` |
| `stroke-yellow-950` | `stroke: var(--color-yellow-950)` |
| `stroke-lime-50` | `stroke: var(--color-lime-50)` |
| `stroke-lime-100` | `stroke: var(--color-lime-100)` |
| `stroke-lime-200` | `stroke: var(--color-lime-200)` |
| `stroke-lime-300` | `stroke: var(--color-lime-300)` |
| `stroke-lime-400` | `stroke: var(--color-lime-400)` |
| `stroke-lime-500` | `stroke: var(--color-lime-500)` |
| `stroke-lime-600` | `stroke: var(--color-lime-600)` |
| `stroke-lime-700` | `stroke: var(--color-lime-700)` |
| `stroke-lime-800` | `stroke: var(--color-lime-800)` |
| `stroke-lime-900` | `stroke: var(--color-lime-900)` |
| `stroke-lime-950` | `stroke: var(--color-lime-950)` |
| `stroke-green-50` | `stroke: var(--color-green-50)` |
| `stroke-green-100` | `stroke: var(--color-green-100)` |
| `stroke-green-200` | `stroke: var(--color-green-200)` |
| `stroke-green-300` | `stroke: var(--color-green-300)` |
| `stroke-green-400` | `stroke: var(--color-green-400)` |
| `stroke-green-500` | `stroke: var(--color-green-500)` |
| `stroke-green-600` | `stroke: var(--color-green-600)` |
| `stroke-green-700` | `stroke: var(--color-green-700)` |
| `stroke-green-800` | `stroke: var(--color-green-800)` |
| `stroke-green-900` | `stroke: var(--color-green-900)` |
| `stroke-green-950` | `stroke: var(--color-green-950)` |
| `stroke-emerald-50` | `stroke: var(--color-emerald-50)` |
| `stroke-emerald-100` | `stroke: var(--color-emerald-100)` |
| `stroke-emerald-200` | `stroke: var(--color-emerald-200)` |
| `stroke-emerald-300` | `stroke: var(--color-emerald-300)` |
| `stroke-emerald-400` | `stroke: var(--color-emerald-400)` |
| `stroke-emerald-500` | `stroke: var(--color-emerald-500)` |
| `stroke-emerald-600` | `stroke: var(--color-emerald-600)` |
| `stroke-emerald-700` | `stroke: var(--color-emerald-700)` |
| `stroke-emerald-800` | `stroke: var(--color-emerald-800)` |
| `stroke-emerald-900` | `stroke: var(--color-emerald-900)` |
| `stroke-emerald-950` | `stroke: var(--color-emerald-950)` |
| `stroke-teal-50` | `stroke: var(--color-teal-50)` |
| `stroke-teal-100` | `stroke: var(--color-teal-100)` |
| `stroke-teal-200` | `stroke: var(--color-teal-200)` |
| `stroke-teal-300` | `stroke: var(--color-teal-300)` |
| `stroke-teal-400` | `stroke: var(--color-teal-400)` |
| `stroke-teal-500` | `stroke: var(--color-teal-500)` |
| `stroke-teal-600` | `stroke: var(--color-teal-600)` |
| `stroke-teal-700` | `stroke: var(--color-teal-700)` |
| `stroke-teal-800` | `stroke: var(--color-teal-800)` |
| `stroke-teal-900` | `stroke: var(--color-teal-900)` |
| `stroke-teal-950` | `stroke: var(--color-teal-950)` |
| `stroke-cyan-50` | `stroke: var(--color-cyan-50)` |
| `stroke-cyan-100` | `stroke: var(--color-cyan-100)` |
| `stroke-cyan-200` | `stroke: var(--color-cyan-200)` |
| `stroke-cyan-300` | `stroke: var(--color-cyan-300)` |
| `stroke-cyan-400` | `stroke: var(--color-cyan-400)` |
| `stroke-cyan-500` | `stroke: var(--color-cyan-500)` |
| `stroke-cyan-600` | `stroke: var(--color-cyan-600)` |
| `stroke-cyan-700` | `stroke: var(--color-cyan-700)` |
| `stroke-cyan-800` | `stroke: var(--color-cyan-800)` |
| `stroke-cyan-900` | `stroke: var(--color-cyan-900)` |
| `stroke-cyan-950` | `stroke: var(--color-cyan-950)` |
| `stroke-sky-50` | `stroke: var(--color-sky-50)` |
| `stroke-sky-100` | `stroke: var(--color-sky-100)` |
| `stroke-sky-200` | `stroke: var(--color-sky-200)` |
| `stroke-sky-300` | `stroke: var(--color-sky-300)` |
| `stroke-sky-400` | `stroke: var(--color-sky-400)` |
| `stroke-sky-500` | `stroke: var(--color-sky-500)` |
| `stroke-sky-600` | `stroke: var(--color-sky-600)` |
| `stroke-sky-700` | `stroke: var(--color-sky-700)` |
| `stroke-sky-800` | `stroke: var(--color-sky-800)` |
| `stroke-sky-900` | `stroke: var(--color-sky-900)` |
| `stroke-sky-950` | `stroke: var(--color-sky-950)` |
| `stroke-blue-50` | `stroke: var(--color-blue-50)` |
| `stroke-blue-100` | `stroke: var(--color-blue-100)` |
| `stroke-blue-200` | `stroke: var(--color-blue-200)` |
| `stroke-blue-300` | `stroke: var(--color-blue-300)` |
| `stroke-blue-400` | `stroke: var(--color-blue-400)` |
| `stroke-blue-500` | `stroke: var(--color-blue-500)` |
| `stroke-blue-600` | `stroke: var(--color-blue-600)` |
| `stroke-blue-700` | `stroke: var(--color-blue-700)` |
| `stroke-blue-800` | `stroke: var(--color-blue-800)` |
| `stroke-blue-900` | `stroke: var(--color-blue-900)` |
| `stroke-blue-950` | `stroke: var(--color-blue-950)` |
| `stroke-indigo-50` | `stroke: var(--color-indigo-50)` |
| `stroke-indigo-100` | `stroke: var(--color-indigo-100)` |
| `stroke-indigo-200` | `stroke: var(--color-indigo-200)` |
| `stroke-indigo-300` | `stroke: var(--color-indigo-300)` |
| `stroke-indigo-400` | `stroke: var(--color-indigo-400)` |
| `stroke-indigo-500` | `stroke: var(--color-indigo-500)` |
| `stroke-indigo-600` | `stroke: var(--color-indigo-600)` |
| `stroke-indigo-700` | `stroke: var(--color-indigo-700)` |
| `stroke-indigo-800` | `stroke: var(--color-indigo-800)` |
| `stroke-indigo-900` | `stroke: var(--color-indigo-900)` |
| `stroke-indigo-950` | `stroke: var(--color-indigo-950)` |
| `stroke-violet-50` | `stroke: var(--color-violet-50)` |
| `stroke-violet-100` | `stroke: var(--color-violet-100)` |
| `stroke-violet-200` | `stroke: var(--color-violet-200)` |
| `stroke-violet-300` | `stroke: var(--color-violet-300)` |
| `stroke-violet-400` | `stroke: var(--color-violet-400)` |
| `stroke-violet-500` | `stroke: var(--color-violet-500)` |
| `stroke-violet-600` | `stroke: var(--color-violet-600)` |
| `stroke-violet-700` | `stroke: var(--color-violet-700)` |
| `stroke-violet-800` | `stroke: var(--color-violet-800)` |
| `stroke-violet-900` | `stroke: var(--color-violet-900)` |
| `stroke-violet-950` | `stroke: var(--color-violet-950)` |
| `stroke-purple-50` | `stroke: var(--color-purple-50)` |
| `stroke-purple-100` | `stroke: var(--color-purple-100)` |
| `stroke-purple-200` | `stroke: var(--color-purple-200)` |
| `stroke-purple-300` | `stroke: var(--color-purple-300)` |
| `stroke-purple-400` | `stroke: var(--color-purple-400)` |
| `stroke-purple-500` | `stroke: var(--color-purple-500)` |
| `stroke-purple-600` | `stroke: var(--color-purple-600)` |
| `stroke-purple-700` | `stroke: var(--color-purple-700)` |
| `stroke-purple-800` | `stroke: var(--color-purple-800)` |
| `stroke-purple-900` | `stroke: var(--color-purple-900)` |
| `stroke-purple-950` | `stroke: var(--color-purple-950)` |
| `stroke-fuchsia-50` | `stroke: var(--color-fuchsia-50)` |
| `stroke-fuchsia-100` | `stroke: var(--color-fuchsia-100)` |
| `stroke-fuchsia-200` | `stroke: var(--color-fuchsia-200)` |
| `stroke-fuchsia-300` | `stroke: var(--color-fuchsia-300)` |
| `stroke-fuchsia-400` | `stroke: var(--color-fuchsia-400)` |
| `stroke-fuchsia-500` | `stroke: var(--color-fuchsia-500)` |
| `stroke-fuchsia-600` | `stroke: var(--color-fuchsia-600)` |
| `stroke-fuchsia-700` | `stroke: var(--color-fuchsia-700)` |
| `stroke-fuchsia-800` | `stroke: var(--color-fuchsia-800)` |
| `stroke-fuchsia-900` | `stroke: var(--color-fuchsia-900)` |
| `stroke-fuchsia-950` | `stroke: var(--color-fuchsia-950)` |
| `stroke-pink-50` | `stroke: var(--color-pink-50)` |
| `stroke-pink-100` | `stroke: var(--color-pink-100)` |
| `stroke-pink-200` | `stroke: var(--color-pink-200)` |
| `stroke-pink-300` | `stroke: var(--color-pink-300)` |
| `stroke-pink-400` | `stroke: var(--color-pink-400)` |
| `stroke-pink-500` | `stroke: var(--color-pink-500)` |
| `stroke-pink-600` | `stroke: var(--color-pink-600)` |
| `stroke-pink-700` | `stroke: var(--color-pink-700)` |
| `stroke-pink-800` | `stroke: var(--color-pink-800)` |
| `stroke-pink-900` | `stroke: var(--color-pink-900)` |
| `stroke-pink-950` | `stroke: var(--color-pink-950)` |
| `stroke-rose-50` | `stroke: var(--color-rose-50)` |
| `stroke-rose-100` | `stroke: var(--color-rose-100)` |
| `stroke-rose-200` | `stroke: var(--color-rose-200)` |
| `stroke-rose-300` | `stroke: var(--color-rose-300)` |
| `stroke-rose-400` | `stroke: var(--color-rose-400)` |
| `stroke-rose-500` | `stroke: var(--color-rose-500)` |
| `stroke-rose-600` | `stroke: var(--color-rose-600)` |
| `stroke-rose-700` | `stroke: var(--color-rose-700)` |
| `stroke-rose-800` | `stroke: var(--color-rose-800)` |
| `stroke-rose-900` | `stroke: var(--color-rose-900)` |
| `stroke-rose-950` | `stroke: var(--color-rose-950)` |
| `stroke-white` | `stroke: white` |
| `stroke-black` | `stroke: black` |
| `stroke-transparent` | `stroke: transparent` |
| `stroke-inherit` | `stroke: inherit` |
| `stroke-current` | `stroke: current` |
| `stroke-none` | `stroke: none` |
| `stroke-0` | `stroke-width: 0` |
| `stroke-1` | `stroke-width: 1` |
| `stroke-2` | `stroke-width: 2` |

## Accessibility

| Classe | CSS |
|--------|-----|
| `sr-only` | `position: absolute; width: 1px; height: 1px; padding: 0; margin: -1px; overflow: hidden; clip: rect(0, 0, 0, 0); white-space: nowrap; border-width: 0` |
| `not-sr-only` | `position: static; width: auto; height: auto; padding: 0; margin: 0; overflow: visible; clip: auto; white-space: normal` |
| `container` | `width: 100%` |
| `table-auto` | `table-layout: auto` |
| `table-fixed` | `table-layout: fixed` |
