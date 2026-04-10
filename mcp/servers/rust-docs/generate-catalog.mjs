/**
 * generate-catalog.mjs — Tailwind CSS v4 (static generation)
 *
 * Génère catalog.md avec toutes les classes Tailwind CSS v4 et leur CSS équivalent.
 * Approche statique : valeurs calculées selon la spec v4, sans exécuter PostCSS.
 *
 * Valeurs v4 :
 *   - Spacing : calc(var(--spacing) * {n}) où --spacing = 0.25rem par défaut
 *   - Couleurs : var(--color-{name}-{shade}) (les variables sont définies dans la base)
 *   - Autres   : valeurs CSS directes (inchangées par rapport à v3)
 *
 * Usage: node generate-catalog.mjs <output-path>
 */

import { writeFileSync, mkdirSync } from 'fs';
import { dirname } from 'path';

// ── Échelles ────────────────────────────────────────────────────────────────

const SPACING_VALUES = [
  '0', 'px', '0.5', '1', '1.5', '2', '2.5', '3', '3.5', '4',
  '5', '6', '7', '8', '9', '10', '11', '12', '14', '16',
  '20', '24', '28', '32', '36', '40', '44', '48', '52', '56',
  '60', '64', '72', '80', '96',
];

const COLORS = [
  'slate', 'gray', 'zinc', 'neutral', 'stone',
  'red', 'orange', 'amber', 'yellow', 'lime',
  'green', 'emerald', 'teal', 'cyan', 'sky',
  'blue', 'indigo', 'violet', 'purple', 'fuchsia',
  'pink', 'rose',
];
const SHADES = ['50', '100', '200', '300', '400', '500', '600', '700', '800', '900', '950'];
const NAMED = ['white', 'black', 'transparent', 'inherit', 'current'];

// ── Helpers ─────────────────────────────────────────────────────────────────

/** Valeur CSS pour une valeur d'espacement Tailwind v4 */
function sp(v) {
  if (v === '0')   return '0px';
  if (v === 'px')  return '1px';
  const n = parseFloat(v);
  const rem = (n * 0.25).toFixed(4).replace(/\.?0+$/, '');
  return `calc(var(--spacing) * ${v})`;   // v4 — réel : ${rem}rem
}

/** Entrée classe→CSS */
function entry(cls, css) { return { class: cls, css }; }

/** Entrées pour toutes les valeurs d'espacement */
function spacing(prefix, cssProp, extra = []) {
  const rows = SPACING_VALUES.map(v => entry(`${prefix}-${v}`, `${cssProp}: ${sp(v)}`));
  return [...rows, ...extra];
}

/** Entrées pour toutes les couleurs + named */
function colorEntries(prefix, cssProp) {
  const rows = [];
  for (const c of COLORS)
    for (const s of SHADES)
      rows.push(entry(`${prefix}-${c}-${s}`, `${cssProp}: var(--color-${c}-${s})`));
  for (const n of NAMED)
    rows.push(entry(`${prefix}-${n}`, `${cssProp}: ${n}`));
  return rows;
}

// ── Catalogue ────────────────────────────────────────────────────────────────

const CATALOG = [

  // ── Layout — Display ──────────────────────────────────────────────────────
  {
    category: 'Layout — Display',
    entries: [
      entry('block',              'display: block'),
      entry('inline-block',       'display: inline-block'),
      entry('inline',             'display: inline'),
      entry('flex',               'display: flex'),
      entry('inline-flex',        'display: inline-flex'),
      entry('grid',               'display: grid'),
      entry('inline-grid',        'display: inline-grid'),
      entry('table',              'display: table'),
      entry('inline-table',       'display: inline-table'),
      entry('table-caption',      'display: table-caption'),
      entry('table-cell',         'display: table-cell'),
      entry('table-column',       'display: table-column'),
      entry('table-column-group', 'display: table-column-group'),
      entry('table-footer-group', 'display: table-footer-group'),
      entry('table-header-group', 'display: table-header-group'),
      entry('table-row-group',    'display: table-row-group'),
      entry('table-row',          'display: table-row'),
      entry('flow-root',          'display: flow-root'),
      entry('contents',           'display: contents'),
      entry('list-item',          'display: list-item'),
      entry('hidden',             'display: none'),
    ],
  },

  // ── Layout — Position ─────────────────────────────────────────────────────
  {
    category: 'Layout — Position',
    entries: [
      entry('static',   'position: static'),
      entry('fixed',    'position: fixed'),
      entry('absolute', 'position: absolute'),
      entry('relative', 'position: relative'),
      entry('sticky',   'position: sticky'),
      ...SPACING_VALUES.map(v => entry(`inset-${v}`, `inset: ${sp(v)}`)),
      ...SPACING_VALUES.map(v => entry(`inset-x-${v}`, `left: ${sp(v)}; right: ${sp(v)}`)),
      ...SPACING_VALUES.map(v => entry(`inset-y-${v}`, `top: ${sp(v)}; bottom: ${sp(v)}`)),
      ...SPACING_VALUES.map(v => entry(`top-${v}`,    `top: ${sp(v)}`)),
      ...SPACING_VALUES.map(v => entry(`right-${v}`,  `right: ${sp(v)}`)),
      ...SPACING_VALUES.map(v => entry(`bottom-${v}`, `bottom: ${sp(v)}`)),
      ...SPACING_VALUES.map(v => entry(`left-${v}`,   `left: ${sp(v)}`)),
      entry('inset-auto', 'inset: auto'),
      entry('top-auto', 'top: auto'), entry('right-auto', 'right: auto'),
      entry('bottom-auto', 'bottom: auto'), entry('left-auto', 'left: auto'),
      entry('inset-full', 'inset: 100%'),
      entry('top-full', 'top: 100%'), entry('right-full', 'right: 100%'),
      entry('bottom-full', 'bottom: 100%'), entry('left-full', 'left: 100%'),
      entry('top-1/2', 'top: 50%'), entry('right-1/2', 'right: 50%'),
      entry('bottom-1/2', 'bottom: 50%'), entry('left-1/2', 'left: 50%'),
    ],
  },

  // ── Layout — Z-Index ──────────────────────────────────────────────────────
  {
    category: 'Layout — Z-Index',
    entries: [
      entry('z-0', 'z-index: 0'), entry('z-10', 'z-index: 10'),
      entry('z-20', 'z-index: 20'), entry('z-30', 'z-index: 30'),
      entry('z-40', 'z-index: 40'), entry('z-50', 'z-index: 50'),
      entry('z-auto', 'z-index: auto'),
    ],
  },

  // ── Layout — Overflow ─────────────────────────────────────────────────────
  {
    category: 'Layout — Overflow',
    entries: [
      ...['auto','hidden','clip','visible','scroll'].flatMap(v => [
        entry(`overflow-${v}`, `overflow: ${v}`),
        entry(`overflow-x-${v}`, `overflow-x: ${v}`),
        entry(`overflow-y-${v}`, `overflow-y: ${v}`),
      ]),
      ...['auto','contain','none'].flatMap(v => [
        entry(`overscroll-${v}`, `overscroll-behavior: ${v}`),
        entry(`overscroll-x-${v}`, `overscroll-behavior-x: ${v}`),
        entry(`overscroll-y-${v}`, `overscroll-behavior-y: ${v}`),
      ]),
    ],
  },

  // ── Layout — Visibility & Misc ────────────────────────────────────────────
  {
    category: 'Layout — Visibility',
    entries: [
      entry('visible', 'visibility: visible'),
      entry('invisible', 'visibility: hidden'),
      entry('collapse', 'visibility: collapse'),
      entry('box-border', 'box-sizing: border-box'),
      entry('box-content', 'box-sizing: content-box'),
      entry('isolate', 'isolation: isolate'),
      entry('isolation-auto', 'isolation: auto'),
      entry('float-left', 'float: left'), entry('float-right', 'float: right'),
      entry('float-none', 'float: none'), entry('float-start', 'float: inline-start'),
      entry('float-end', 'float: inline-end'),
      entry('clear-left', 'clear: left'), entry('clear-right', 'clear: right'),
      entry('clear-both', 'clear: both'), entry('clear-none', 'clear: none'),
      entry('aspect-auto', 'aspect-ratio: auto'),
      entry('aspect-square', 'aspect-ratio: 1 / 1'),
      entry('aspect-video', 'aspect-ratio: 16 / 9'),
    ],
  },

  // ── Layout — Object ───────────────────────────────────────────────────────
  {
    category: 'Layout — Object Fit',
    entries: [
      entry('object-contain', 'object-fit: contain'), entry('object-cover', 'object-fit: cover'),
      entry('object-fill', 'object-fit: fill'), entry('object-none', 'object-fit: none'),
      entry('object-scale-down', 'object-fit: scale-down'),
      entry('object-center', 'object-position: center'), entry('object-top', 'object-position: top'),
      entry('object-right', 'object-position: right'), entry('object-bottom', 'object-position: bottom'),
      entry('object-left', 'object-position: left'),
    ],
  },

  // ── Flexbox ───────────────────────────────────────────────────────────────
  {
    category: 'Flexbox',
    entries: [
      entry('flex-row', 'flex-direction: row'), entry('flex-row-reverse', 'flex-direction: row-reverse'),
      entry('flex-col', 'flex-direction: column'), entry('flex-col-reverse', 'flex-direction: column-reverse'),
      entry('flex-wrap', 'flex-wrap: wrap'), entry('flex-wrap-reverse', 'flex-wrap: wrap-reverse'),
      entry('flex-nowrap', 'flex-wrap: nowrap'),
      entry('flex-1', 'flex: 1 1 0%'), entry('flex-auto', 'flex: 1 1 auto'),
      entry('flex-initial', 'flex: 0 1 auto'), entry('flex-none', 'flex: none'),
      entry('grow', 'flex-grow: 1'), entry('grow-0', 'flex-grow: 0'),
      entry('shrink', 'flex-shrink: 1'), entry('shrink-0', 'flex-shrink: 0'),
      ...SPACING_VALUES.map(v => entry(`basis-${v}`, `flex-basis: ${sp(v)}`)),
      entry('basis-auto', 'flex-basis: auto'), entry('basis-full', 'flex-basis: 100%'),
      entry('basis-1/2', 'flex-basis: 50%'), entry('basis-1/3', 'flex-basis: 33.333333%'),
      entry('basis-2/3', 'flex-basis: 66.666667%'),
      ...[1,2,3,4,5,6,7,8,9,10,11,12].map(n => entry(`order-${n}`, `order: ${n}`)),
      entry('order-first', 'order: -9999'), entry('order-last', 'order: 9999'),
      entry('order-none', 'order: 0'),
    ],
  },

  // ── Grid ──────────────────────────────────────────────────────────────────
  {
    category: 'Grid',
    entries: [
      ...[1,2,3,4,5,6,7,8,9,10,11,12].map(n => entry(`grid-cols-${n}`, `grid-template-columns: repeat(${n}, minmax(0, 1fr))`)),
      entry('grid-cols-none', 'grid-template-columns: none'),
      entry('grid-cols-subgrid', 'grid-template-columns: subgrid'),
      ...[1,2,3,4,5,6,7,8,9,10,11,12].map(n => entry(`grid-rows-${n}`, `grid-template-rows: repeat(${n}, minmax(0, 1fr))`)),
      entry('grid-rows-none', 'grid-template-rows: none'),
      entry('grid-rows-subgrid', 'grid-template-rows: subgrid'),
      ...[1,2,3,4,5,6,7,8,9,10,11,12].map(n => entry(`col-span-${n}`, `grid-column: span ${n} / span ${n}`)),
      entry('col-span-full', 'grid-column: 1 / -1'),
      ...[1,2,3,4,5,6,7,8,9,10,11,12,13].map(n => entry(`col-start-${n}`, `grid-column-start: ${n}`)),
      entry('col-start-auto', 'grid-column-start: auto'),
      ...[1,2,3,4,5,6,7,8,9,10,11,12,13].map(n => entry(`col-end-${n}`, `grid-column-end: ${n}`)),
      entry('col-end-auto', 'grid-column-end: auto'),
      ...[1,2,3,4,5,6].map(n => entry(`row-span-${n}`, `grid-row: span ${n} / span ${n}`)),
      entry('row-span-full', 'grid-row: 1 / -1'),
      ...[1,2,3,4,5,6,7].map(n => entry(`row-start-${n}`, `grid-row-start: ${n}`)),
      entry('row-start-auto', 'grid-row-start: auto'),
      ...[1,2,3,4,5,6,7].map(n => entry(`row-end-${n}`, `grid-row-end: ${n}`)),
      entry('row-end-auto', 'grid-row-end: auto'),
      entry('grid-flow-row', 'grid-auto-flow: row'), entry('grid-flow-col', 'grid-auto-flow: column'),
      entry('grid-flow-dense', 'grid-auto-flow: dense'),
      entry('grid-flow-row-dense', 'grid-auto-flow: row dense'),
      entry('grid-flow-col-dense', 'grid-auto-flow: column dense'),
      ...['auto','min','max','fr'].map(v => entry(`auto-cols-${v}`, `grid-auto-columns: ${v === 'fr' ? 'minmax(0, 1fr)' : v}`)),
      ...['auto','min','max','fr'].map(v => entry(`auto-rows-${v}`, `grid-auto-rows: ${v === 'fr' ? 'minmax(0, 1fr)' : v}`)),
    ],
  },

  // ── Flexbox & Grid — Alignment ────────────────────────────────────────────
  {
    category: 'Flexbox & Grid — Alignment',
    entries: [
      ...['normal','start','end','center','between','around','evenly','stretch'].map(v =>
        entry(`justify-${v}`, `justify-content: ${v === 'between' ? 'space-between' : v === 'around' ? 'space-around' : v === 'evenly' ? 'space-evenly' : v}`)),
      ...['start','end','center','stretch','normal'].map(v => entry(`justify-items-${v}`, `justify-items: ${v}`)),
      ...['auto','start','end','center','stretch'].map(v => entry(`justify-self-${v}`, `justify-self: ${v}`)),
      ...['start','end','center','baseline','stretch'].map(v => entry(`items-${v}`, `align-items: ${v}`)),
      ...['normal','center','start','end','between','around','evenly','baseline','stretch'].map(v =>
        entry(`content-${v}`, `align-content: ${v === 'between' ? 'space-between' : v === 'around' ? 'space-around' : v === 'evenly' ? 'space-evenly' : v}`)),
      ...['auto','start','end','center','stretch','baseline'].map(v => entry(`self-${v}`, `align-self: ${v}`)),
      ...['center','start','end','between','around','evenly','baseline','stretch'].map(v =>
        entry(`place-content-${v}`, `place-content: ${v === 'between' ? 'space-between' : v === 'around' ? 'space-around' : v}`)),
      ...['center','start','end','baseline','stretch'].map(v => entry(`place-items-${v}`, `place-items: ${v}`)),
      ...['auto','start','end','center','stretch'].map(v => entry(`place-self-${v}`, `place-self: ${v}`)),
    ],
  },

  // ── Spacing — Gap ─────────────────────────────────────────────────────────
  {
    category: 'Spacing — Gap',
    entries: [
      ...spacing('gap', 'gap'),
      ...spacing('gap-x', 'column-gap'),
      ...spacing('gap-y', 'row-gap'),
    ],
  },

  // ── Spacing — Padding ─────────────────────────────────────────────────────
  {
    category: 'Spacing — Padding',
    entries: [
      ...spacing('p', 'padding'),
      ...spacing('px', 'padding-left') .map((e,i) => ({ class: e.class.replace('px','px'), css: `padding-left: ${sp(SPACING_VALUES[i])}; padding-right: ${sp(SPACING_VALUES[i])}` })),
      ...SPACING_VALUES.map(v => entry(`px-${v}`, `padding-left: ${sp(v)}; padding-right: ${sp(v)}`)),
      ...SPACING_VALUES.map(v => entry(`py-${v}`, `padding-top: ${sp(v)}; padding-bottom: ${sp(v)}`)),
      ...SPACING_VALUES.map(v => entry(`pt-${v}`, `padding-top: ${sp(v)}`)),
      ...SPACING_VALUES.map(v => entry(`pr-${v}`, `padding-right: ${sp(v)}`)),
      ...SPACING_VALUES.map(v => entry(`pb-${v}`, `padding-bottom: ${sp(v)}`)),
      ...SPACING_VALUES.map(v => entry(`pl-${v}`, `padding-left: ${sp(v)}`)),
      ...SPACING_VALUES.map(v => entry(`ps-${v}`, `padding-inline-start: ${sp(v)}`)),
      ...SPACING_VALUES.map(v => entry(`pe-${v}`, `padding-inline-end: ${sp(v)}`)),
    ].filter((e, i, a) => a.findIndex(x => x.class === e.class) === i), // dédupliquer
  },

  // ── Spacing — Margin ──────────────────────────────────────────────────────
  {
    category: 'Spacing — Margin',
    entries: [
      ...SPACING_VALUES.map(v => entry(`m-${v}`, `margin: ${sp(v)}`)),
      ...SPACING_VALUES.map(v => entry(`mx-${v}`, `margin-left: ${sp(v)}; margin-right: ${sp(v)}`)),
      ...SPACING_VALUES.map(v => entry(`my-${v}`, `margin-top: ${sp(v)}; margin-bottom: ${sp(v)}`)),
      ...SPACING_VALUES.map(v => entry(`mt-${v}`, `margin-top: ${sp(v)}`)),
      ...SPACING_VALUES.map(v => entry(`mr-${v}`, `margin-right: ${sp(v)}`)),
      ...SPACING_VALUES.map(v => entry(`mb-${v}`, `margin-bottom: ${sp(v)}`)),
      ...SPACING_VALUES.map(v => entry(`ml-${v}`, `margin-left: ${sp(v)}`)),
      ...SPACING_VALUES.map(v => entry(`ms-${v}`, `margin-inline-start: ${sp(v)}`)),
      ...SPACING_VALUES.map(v => entry(`me-${v}`, `margin-inline-end: ${sp(v)}`)),
      entry('m-auto', 'margin: auto'), entry('mx-auto', 'margin-left: auto; margin-right: auto'),
      entry('my-auto', 'margin-top: auto; margin-bottom: auto'),
      entry('mt-auto', 'margin-top: auto'), entry('mr-auto', 'margin-right: auto'),
      entry('mb-auto', 'margin-bottom: auto'), entry('ml-auto', 'margin-left: auto'),
      ...SPACING_VALUES.map(v => entry(`space-x-${v}`, `--tw-space-x-reverse: 0; margin-right: calc(${sp(v)} * var(--tw-space-x-reverse)); margin-left: calc(${sp(v)} * calc(1 - var(--tw-space-x-reverse)))`)),
      ...SPACING_VALUES.map(v => entry(`space-y-${v}`, `--tw-space-y-reverse: 0; margin-bottom: calc(${sp(v)} * var(--tw-space-y-reverse)); margin-top: calc(${sp(v)} * calc(1 - var(--tw-space-y-reverse)))`)),
    ],
  },

  // ── Sizing — Width ────────────────────────────────────────────────────────
  {
    category: 'Sizing — Width',
    entries: [
      ...SPACING_VALUES.map(v => entry(`w-${v}`, `width: ${sp(v)}`)),
      entry('w-auto', 'width: auto'), entry('w-full', 'width: 100%'),
      entry('w-screen', 'width: 100vw'), entry('w-svw', 'width: 100svw'),
      entry('w-dvw', 'width: 100dvw'), entry('w-min', 'width: min-content'),
      entry('w-max', 'width: max-content'), entry('w-fit', 'width: fit-content'),
      entry('w-1/2', 'width: 50%'), entry('w-1/3', 'width: 33.333333%'),
      entry('w-2/3', 'width: 66.666667%'), entry('w-1/4', 'width: 25%'),
      entry('w-3/4', 'width: 75%'), entry('w-1/5', 'width: 20%'),
      entry('w-4/5', 'width: 80%'), entry('w-1/6', 'width: 16.666667%'),
      entry('w-5/6', 'width: 83.333333%'),
      ...SPACING_VALUES.map(v => entry(`min-w-${v}`, `min-width: ${sp(v)}`)),
      entry('min-w-full', 'min-width: 100%'), entry('min-w-min', 'min-width: min-content'),
      entry('min-w-max', 'min-width: max-content'), entry('min-w-fit', 'min-width: fit-content'),
      ...SPACING_VALUES.map(v => entry(`max-w-${v}`, `max-width: ${sp(v)}`)),
      entry('max-w-none', 'max-width: none'), entry('max-w-full', 'max-width: 100%'),
      entry('max-w-xs', 'max-width: 20rem'), entry('max-w-sm', 'max-width: 24rem'),
      entry('max-w-md', 'max-width: 28rem'), entry('max-w-lg', 'max-width: 32rem'),
      entry('max-w-xl', 'max-width: 36rem'), entry('max-w-2xl', 'max-width: 42rem'),
      entry('max-w-3xl', 'max-width: 48rem'), entry('max-w-4xl', 'max-width: 56rem'),
      entry('max-w-5xl', 'max-width: 64rem'), entry('max-w-6xl', 'max-width: 72rem'),
      entry('max-w-7xl', 'max-width: 80rem'), entry('max-w-prose', 'max-width: 65ch'),
      entry('max-w-screen-sm', 'max-width: 640px'), entry('max-w-screen-md', 'max-width: 768px'),
      entry('max-w-screen-lg', 'max-width: 1024px'), entry('max-w-screen-xl', 'max-width: 1280px'),
      entry('max-w-screen-2xl', 'max-width: 1536px'),
    ],
  },

  // ── Sizing — Height ───────────────────────────────────────────────────────
  {
    category: 'Sizing — Height',
    entries: [
      ...SPACING_VALUES.map(v => entry(`h-${v}`, `height: ${sp(v)}`)),
      entry('h-auto', 'height: auto'), entry('h-full', 'height: 100%'),
      entry('h-screen', 'height: 100vh'), entry('h-svh', 'height: 100svh'),
      entry('h-dvh', 'height: 100dvh'), entry('h-min', 'height: min-content'),
      entry('h-max', 'height: max-content'), entry('h-fit', 'height: fit-content'),
      ...SPACING_VALUES.map(v => entry(`min-h-${v}`, `min-height: ${sp(v)}`)),
      entry('min-h-full', 'min-height: 100%'), entry('min-h-screen', 'min-height: 100vh'),
      entry('min-h-svh', 'min-height: 100svh'), entry('min-h-dvh', 'min-height: 100dvh'),
      entry('min-h-min', 'min-height: min-content'), entry('min-h-max', 'min-height: max-content'),
      entry('min-h-fit', 'min-height: fit-content'),
      ...SPACING_VALUES.map(v => entry(`max-h-${v}`, `max-height: ${sp(v)}`)),
      entry('max-h-none', 'max-height: none'), entry('max-h-full', 'max-height: 100%'),
      entry('max-h-screen', 'max-height: 100vh'), entry('max-h-svh', 'max-height: 100svh'),
      entry('max-h-dvh', 'max-height: 100dvh'),
      ...SPACING_VALUES.map(v => entry(`size-${v}`, `width: ${sp(v)}; height: ${sp(v)}`)),
      entry('size-auto', 'width: auto; height: auto'), entry('size-full', 'width: 100%; height: 100%'),
      entry('size-min', 'width: min-content; height: min-content'),
      entry('size-max', 'width: max-content; height: max-content'),
    ],
  },

  // ── Typography ────────────────────────────────────────────────────────────
  {
    category: 'Typography — Font',
    entries: [
      entry('font-sans', 'font-family: var(--font-sans)'),
      entry('font-serif', 'font-family: var(--font-serif)'),
      entry('font-mono', 'font-family: var(--font-mono)'),
      entry('text-xs',  'font-size: 0.75rem; line-height: 1rem'),
      entry('text-sm',  'font-size: 0.875rem; line-height: 1.25rem'),
      entry('text-base','font-size: 1rem; line-height: 1.5rem'),
      entry('text-lg',  'font-size: 1.125rem; line-height: 1.75rem'),
      entry('text-xl',  'font-size: 1.25rem; line-height: 1.75rem'),
      entry('text-2xl', 'font-size: 1.5rem; line-height: 2rem'),
      entry('text-3xl', 'font-size: 1.875rem; line-height: 2.25rem'),
      entry('text-4xl', 'font-size: 2.25rem; line-height: 2.5rem'),
      entry('text-5xl', 'font-size: 3rem; line-height: 1'),
      entry('text-6xl', 'font-size: 3.75rem; line-height: 1'),
      entry('text-7xl', 'font-size: 4.5rem; line-height: 1'),
      entry('text-8xl', 'font-size: 6rem; line-height: 1'),
      entry('text-9xl', 'font-size: 8rem; line-height: 1'),
      entry('font-thin',       'font-weight: 100'), entry('font-extralight', 'font-weight: 200'),
      entry('font-light',      'font-weight: 300'), entry('font-normal',      'font-weight: 400'),
      entry('font-medium',     'font-weight: 500'), entry('font-semibold',    'font-weight: 600'),
      entry('font-bold',       'font-weight: 700'), entry('font-extrabold',   'font-weight: 800'),
      entry('font-black',      'font-weight: 900'),
      entry('italic', 'font-style: italic'), entry('not-italic', 'font-style: normal'),
      entry('antialiased', '-webkit-font-smoothing: antialiased; -moz-osx-font-smoothing: grayscale'),
      entry('subpixel-antialiased', '-webkit-font-smoothing: auto; -moz-osx-font-smoothing: auto'),
    ],
  },
  {
    category: 'Typography — Spacing',
    entries: [
      entry('leading-none',    'line-height: 1'),
      entry('leading-tight',   'line-height: 1.25'),
      entry('leading-snug',    'line-height: 1.375'),
      entry('leading-normal',  'line-height: 1.5'),
      entry('leading-relaxed', 'line-height: 1.625'),
      entry('leading-loose',   'line-height: 2'),
      ...[3,4,5,6,7,8,9,10].map(n => entry(`leading-${n}`, `line-height: ${n * 0.25}rem`)),
      entry('tracking-tighter', 'letter-spacing: -0.05em'),
      entry('tracking-tight',   'letter-spacing: -0.025em'),
      entry('tracking-normal',  'letter-spacing: 0em'),
      entry('tracking-wide',    'letter-spacing: 0.025em'),
      entry('tracking-wider',   'letter-spacing: 0.05em'),
      entry('tracking-widest',  'letter-spacing: 0.1em'),
    ],
  },
  {
    category: 'Typography — Text',
    entries: [
      entry('text-left',    'text-align: left'), entry('text-center', 'text-align: center'),
      entry('text-right',   'text-align: right'), entry('text-justify', 'text-align: justify'),
      entry('text-start',   'text-align: start'), entry('text-end', 'text-align: end'),
      entry('underline',   'text-decoration-line: underline'),
      entry('overline',    'text-decoration-line: overline'),
      entry('line-through','text-decoration-line: line-through'),
      entry('no-underline','text-decoration-line: none'),
      entry('decoration-solid',  'text-decoration-style: solid'),
      entry('decoration-double', 'text-decoration-style: double'),
      entry('decoration-dotted', 'text-decoration-style: dotted'),
      entry('decoration-dashed', 'text-decoration-style: dashed'),
      entry('decoration-wavy',   'text-decoration-style: wavy'),
      ...[0,1,2,4,8].map(n => entry(`decoration-${n}`, `text-decoration-thickness: ${n === 0 ? '0px' : n + 'px'}`)),
      ...['auto',0,1,2,4,8].map(n => entry(`underline-offset-${n}`, `text-underline-offset: ${n === 'auto' ? 'auto' : n === 0 ? '0px' : n + 'px'}`)),
      entry('uppercase', 'text-transform: uppercase'), entry('lowercase', 'text-transform: lowercase'),
      entry('capitalize', 'text-transform: capitalize'), entry('normal-case', 'text-transform: none'),
      entry('truncate', 'overflow: hidden; text-overflow: ellipsis; white-space: nowrap'),
      entry('text-ellipsis', 'text-overflow: ellipsis'), entry('text-clip', 'text-overflow: clip'),
      ...SPACING_VALUES.map(v => entry(`indent-${v}`, `text-indent: ${sp(v)}`)),
      entry('align-baseline',   'vertical-align: baseline'),
      entry('align-top',        'vertical-align: top'),
      entry('align-middle',     'vertical-align: middle'),
      entry('align-bottom',     'vertical-align: bottom'),
      entry('align-text-top',   'vertical-align: text-top'),
      entry('align-text-bottom','vertical-align: text-bottom'),
      entry('whitespace-normal',      'white-space: normal'),
      entry('whitespace-nowrap',      'white-space: nowrap'),
      entry('whitespace-pre',         'white-space: pre'),
      entry('whitespace-pre-line',    'white-space: pre-line'),
      entry('whitespace-pre-wrap',    'white-space: pre-wrap'),
      entry('whitespace-break-spaces','white-space: break-spaces'),
      entry('break-normal', 'overflow-wrap: normal; word-break: normal'),
      entry('break-words', 'overflow-wrap: break-word'),
      entry('break-all', 'word-break: break-all'),
      entry('break-keep', 'word-break: keep-all'),
      entry('list-none', 'list-style-type: none'),
      entry('list-disc', 'list-style-type: disc'),
      entry('list-decimal', 'list-style-type: decimal'),
      entry('list-inside', 'list-style-position: inside'),
      entry('list-outside', 'list-style-position: outside'),
    ],
  },

  // ── Typography — Text Color ───────────────────────────────────────────────
  { category: 'Typography — Text Color', entries: colorEntries('text', 'color') },

  // ── Backgrounds ───────────────────────────────────────────────────────────
  { category: 'Backgrounds — Color', entries: colorEntries('bg', 'background-color') },
  {
    category: 'Backgrounds — Other',
    entries: [
      entry('bg-none', 'background-image: none'),
      ...['t','tr','r','br','b','bl','l','tl'].map(d =>
        entry(`bg-gradient-to-${d}`, `background-image: linear-gradient(to ${d === 't' ? 'top' : d === 'tr' ? 'top right' : d === 'r' ? 'right' : d === 'br' ? 'bottom right' : d === 'b' ? 'bottom' : d === 'bl' ? 'bottom left' : d === 'l' ? 'left' : 'top left'}, var(--tw-gradient-stops))`)),
      entry('bg-auto', 'background-size: auto'), entry('bg-cover', 'background-size: cover'),
      entry('bg-contain', 'background-size: contain'),
      entry('bg-center', 'background-position: center'), entry('bg-top', 'background-position: top'),
      entry('bg-right', 'background-position: right'), entry('bg-bottom', 'background-position: bottom'),
      entry('bg-left', 'background-position: left'),
      entry('bg-repeat', 'background-repeat: repeat'), entry('bg-no-repeat', 'background-repeat: no-repeat'),
      entry('bg-repeat-x', 'background-repeat: repeat-x'), entry('bg-repeat-y', 'background-repeat: repeat-y'),
      entry('bg-fixed', 'background-attachment: fixed'), entry('bg-local', 'background-attachment: local'),
      entry('bg-scroll', 'background-attachment: scroll'),
      entry('bg-clip-border', 'background-clip: border-box'),
      entry('bg-clip-padding', 'background-clip: padding-box'),
      entry('bg-clip-content', 'background-clip: content-box'),
      entry('bg-clip-text', 'background-clip: text; -webkit-background-clip: text'),
    ],
  },
  { category: 'Backgrounds — Gradient From', entries: colorEntries('from', '--tw-gradient-from') },
  { category: 'Backgrounds — Gradient Via', entries: colorEntries('via', '--tw-gradient-via') },
  { category: 'Backgrounds — Gradient To', entries: colorEntries('to', '--tw-gradient-to') },

  // ── Borders ───────────────────────────────────────────────────────────────
  {
    category: 'Borders — Radius',
    entries: [
      ...['none','sm','','md','lg','xl','2xl','3xl','full'].map(s => {
        const cls = s ? `rounded-${s}` : 'rounded';
        const val = s === 'none' ? '0px' : s === 'sm' ? '0.125rem' : s === '' ? '0.25rem' : s === 'md' ? '0.375rem' : s === 'lg' ? '0.5rem' : s === 'xl' ? '0.75rem' : s === '2xl' ? '1rem' : s === '3xl' ? '1.5rem' : '9999px';
        return entry(cls, `border-radius: ${val}`);
      }),
      ...['t','r','b','l','tl','tr','br','bl'].flatMap(side =>
        ['none','sm','','md','lg','xl','2xl','full'].map(s => {
          const cls = s ? `rounded-${side}-${s}` : `rounded-${side}`;
          const val = s === 'none' ? '0px' : s === 'sm' ? '0.125rem' : s === '' ? '0.25rem' : s === 'md' ? '0.375rem' : s === 'lg' ? '0.5rem' : s === 'xl' ? '0.75rem' : s === '2xl' ? '1rem' : '9999px';
          const prop = side === 't' ? 'border-top-left-radius: VAL; border-top-right-radius: VAL' :
                       side === 'r' ? 'border-top-right-radius: VAL; border-bottom-right-radius: VAL' :
                       side === 'b' ? 'border-bottom-right-radius: VAL; border-bottom-left-radius: VAL' :
                       side === 'l' ? 'border-top-left-radius: VAL; border-bottom-left-radius: VAL' :
                       `border-${side === 'tl' ? 'top-left' : side === 'tr' ? 'top-right' : side === 'br' ? 'bottom-right' : 'bottom-left'}-radius: VAL`;
          return entry(cls, prop.replaceAll('VAL', val));
        })
      ),
    ],
  },
  {
    category: 'Borders — Width',
    entries: [
      entry('border', 'border-width: 1px'),
      ...[0,2,4,8].map(n => entry(`border-${n}`, `border-width: ${n}px`)),
      ...['t','r','b','l','x','y'].flatMap(side => [
        entry(`border-${side}`, `border-${side === 'x' ? 'left-width: 1px; border-right' : side === 'y' ? 'top-width: 1px; border-bottom' : side}-width: 1px`),
        ...[0,2,4,8].map(n => entry(`border-${side}-${n}`, `border-${side === 'x' ? 'left-width: ${n}px; border-right' : side === 'y' ? 'top-width: ${n}px; border-bottom' : side}-width: ${n}px`)),
      ]),
    ],
  },
  { category: 'Borders — Color', entries: colorEntries('border', 'border-color') },
  {
    category: 'Borders — Style',
    entries: ['solid','dashed','dotted','double','hidden','none'].map(v => entry(`border-${v}`, `border-style: ${v}`)),
  },
  {
    category: 'Borders — Outline',
    entries: [
      entry('outline-none', 'outline: 2px solid transparent; outline-offset: 2px'),
      entry('outline', 'outline-style: solid'),
      ...[0,1,2,4,8].map(n => entry(`outline-${n}`, `outline-width: ${n}px`)),
      ...colorEntries('outline', 'outline-color'),
      ...['solid','dashed','dotted','double'].map(v => entry(`outline-${v}`, `outline-style: ${v}`)),
      ...[0,1,2,4,8].map(n => entry(`outline-offset-${n}`, `outline-offset: ${n}px`)),
      entry('ring', '--tw-ring-shadow: var(--tw-ring-inset) 0 0 0 calc(3px + var(--tw-ring-offset-width)) var(--tw-ring-color)'),
      ...[0,1,2,4,8].map(n => entry(`ring-${n}`, `--tw-ring-shadow: var(--tw-ring-inset) 0 0 0 calc(${n}px + var(--tw-ring-offset-width)) var(--tw-ring-color)`)),
      entry('ring-inset', '--tw-ring-inset: inset'),
      ...colorEntries('ring', '--tw-ring-color'),
      ...[0,1,2,4,8].map(n => entry(`ring-offset-${n}`, `--tw-ring-offset-width: ${n}px`)),
    ],
  },

  // ── Effects ───────────────────────────────────────────────────────────────
  {
    category: 'Effects — Shadow',
    entries: [
      entry('shadow-sm', 'box-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.05)'),
      entry('shadow', 'box-shadow: 0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)'),
      entry('shadow-md', 'box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)'),
      entry('shadow-lg', 'box-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)'),
      entry('shadow-xl', 'box-shadow: 0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)'),
      entry('shadow-2xl', 'box-shadow: 0 25px 50px -12px rgb(0 0 0 / 0.25)'),
      entry('shadow-inner', 'box-shadow: inset 0 2px 4px 0 rgb(0 0 0 / 0.05)'),
      entry('shadow-none', 'box-shadow: 0 0 #0000'),
    ],
  },
  {
    category: 'Effects — Opacity',
    entries: [0,5,10,15,20,25,30,35,40,45,50,55,60,65,70,75,80,85,90,95,100].map(n =>
      entry(`opacity-${n}`, `opacity: ${n / 100}`)),
  },

  // ── Filters ───────────────────────────────────────────────────────────────
  {
    category: 'Filters',
    entries: [
      entry('blur-none', 'filter: blur(0)'), entry('blur-sm', 'filter: blur(4px)'),
      entry('blur', 'filter: blur(8px)'), entry('blur-md', 'filter: blur(12px)'),
      entry('blur-lg', 'filter: blur(16px)'), entry('blur-xl', 'filter: blur(24px)'),
      entry('blur-2xl', 'filter: blur(40px)'), entry('blur-3xl', 'filter: blur(64px)'),
      ...[0,50,75,90,95,100,105,110,125,150,200].map(n => entry(`brightness-${n}`, `filter: brightness(${n/100})`)),
      ...[0,50,75,100,125,150,200].map(n => entry(`contrast-${n}`, `filter: contrast(${n/100})`)),
      entry('grayscale', 'filter: grayscale(100%)'), entry('grayscale-0', 'filter: grayscale(0)'),
      ...[0,15,30,60,90,180].map(n => entry(`hue-rotate-${n}`, `filter: hue-rotate(${n}deg)`)),
      entry('invert', 'filter: invert(100%)'), entry('invert-0', 'filter: invert(0)'),
      ...[0,50,100,150,200].map(n => entry(`saturate-${n}`, `filter: saturate(${n/100})`)),
      entry('sepia', 'filter: sepia(100%)'), entry('sepia-0', 'filter: sepia(0)'),
      entry('drop-shadow-sm', 'filter: drop-shadow(0 1px 1px rgb(0 0 0 / 0.05))'),
      entry('drop-shadow', 'filter: drop-shadow(0 1px 2px rgb(0 0 0 / 0.1)) drop-shadow(0 1px 1px rgb(0 0 0 / 0.06))'),
      entry('drop-shadow-md', 'filter: drop-shadow(0 4px 3px rgb(0 0 0 / 0.07)) drop-shadow(0 2px 2px rgb(0 0 0 / 0.06))'),
      entry('drop-shadow-lg', 'filter: drop-shadow(0 10px 8px rgb(0 0 0 / 0.04)) drop-shadow(0 4px 3px rgb(0 0 0 / 0.1))'),
      entry('drop-shadow-xl', 'filter: drop-shadow(0 20px 13px rgb(0 0 0 / 0.03)) drop-shadow(0 8px 5px rgb(0 0 0 / 0.08))'),
      entry('drop-shadow-none', 'filter: drop-shadow(0 0 #0000)'),
    ],
  },

  // ── Transitions & Animation ───────────────────────────────────────────────
  {
    category: 'Transitions',
    entries: [
      entry('transition-none', 'transition-property: none'),
      entry('transition-all', 'transition-property: all; transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); transition-duration: 150ms'),
      entry('transition', 'transition-property: color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter; transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); transition-duration: 150ms'),
      entry('transition-colors', 'transition-property: color, background-color, border-color, text-decoration-color, fill, stroke; transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); transition-duration: 150ms'),
      entry('transition-opacity', 'transition-property: opacity; transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); transition-duration: 150ms'),
      entry('transition-shadow', 'transition-property: box-shadow; transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); transition-duration: 150ms'),
      entry('transition-transform', 'transition-property: transform; transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); transition-duration: 150ms'),
      ...[0,75,100,150,200,300,500,700,1000].map(n => entry(`duration-${n}`, `transition-duration: ${n}ms`)),
      entry('ease-linear', 'transition-timing-function: linear'),
      entry('ease-in', 'transition-timing-function: cubic-bezier(0.4, 0, 1, 1)'),
      entry('ease-out', 'transition-timing-function: cubic-bezier(0, 0, 0.2, 1)'),
      entry('ease-in-out', 'transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1)'),
      ...[0,75,100,150,200,300,500,700,1000].map(n => entry(`delay-${n}`, `transition-delay: ${n}ms`)),
      entry('animate-none', 'animation: none'),
      entry('animate-spin', 'animation: spin 1s linear infinite'),
      entry('animate-ping', 'animation: ping 1s cubic-bezier(0, 0, 0.2, 1) infinite'),
      entry('animate-pulse', 'animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite'),
      entry('animate-bounce', 'animation: bounce 1s infinite'),
    ],
  },

  // ── Transforms ────────────────────────────────────────────────────────────
  {
    category: 'Transforms',
    entries: [
      ...[0,50,75,90,95,100,105,110,125,150].flatMap(n => [
        entry(`scale-${n}`, `--tw-scale-x: ${n/100}; --tw-scale-y: ${n/100}; transform: scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))`),
        entry(`scale-x-${n}`, `--tw-scale-x: ${n/100}; transform: scaleX(var(--tw-scale-x))`),
        entry(`scale-y-${n}`, `--tw-scale-y: ${n/100}; transform: scaleY(var(--tw-scale-y))`),
      ]),
      ...[0,1,2,3,6,12,45,90,180].flatMap(n => [
        entry(`rotate-${n}`, `--tw-rotate: ${n}deg; transform: rotate(var(--tw-rotate))`),
        ...(n > 0 ? [entry(`-rotate-${n}`, `--tw-rotate: -${n}deg; transform: rotate(var(--tw-rotate))`)] : []),
      ]),
      ...SPACING_VALUES.map(v => entry(`translate-x-${v}`, `--tw-translate-x: ${sp(v)}; transform: translateX(var(--tw-translate-x))`)),
      ...SPACING_VALUES.map(v => entry(`translate-y-${v}`, `--tw-translate-y: ${sp(v)}; transform: translateY(var(--tw-translate-y))`)),
      entry('translate-x-full', '--tw-translate-x: 100%; transform: translateX(var(--tw-translate-x))'),
      entry('translate-y-full', '--tw-translate-y: 100%; transform: translateY(var(--tw-translate-y))'),
      entry('translate-x-1/2', '--tw-translate-x: 50%; transform: translateX(var(--tw-translate-x))'),
      entry('translate-y-1/2', '--tw-translate-y: 50%; transform: translateY(var(--tw-translate-y))'),
      ...[0,1,2,3,6,12].flatMap(n => [
        entry(`skew-x-${n}`, `--tw-skew-x: ${n}deg; transform: skewX(var(--tw-skew-x))`),
        entry(`skew-y-${n}`, `--tw-skew-y: ${n}deg; transform: skewY(var(--tw-skew-y))`),
      ]),
      ...['center','top','top-right','right','bottom-right','bottom','bottom-left','left','top-left'].map(v =>
        entry(`origin-${v.replace(/ /g,'-')}`, `transform-origin: ${v}`)),
    ],
  },

  // ── Interactivity ─────────────────────────────────────────────────────────
  {
    category: 'Interactivity',
    entries: [
      ...['auto','default','pointer','wait','text','move','help','not-allowed','none','context-menu','progress','cell','crosshair','vertical-text','alias','copy','no-drop','grab','grabbing','all-scroll','col-resize','row-resize','n-resize','e-resize','s-resize','w-resize','ne-resize','nw-resize','se-resize','sw-resize','ew-resize','ns-resize','nesw-resize','nwse-resize','zoom-in','zoom-out']
        .map(v => entry(`cursor-${v}`, `cursor: ${v}`)),
      ...['auto','none','pan-x','pan-left','pan-right','pan-y','pan-up','pan-down','pinch-zoom','manipulation'].map(v =>
        entry(`touch-${v}`, `touch-action: ${v}`)),
      entry('select-none', 'user-select: none'), entry('select-text', 'user-select: text'),
      entry('select-all', 'user-select: all'), entry('select-auto', 'user-select: auto'),
      entry('resize', 'resize: both'), entry('resize-none', 'resize: none'),
      entry('resize-y', 'resize: vertical'), entry('resize-x', 'resize: horizontal'),
      entry('scroll-auto', 'scroll-behavior: auto'), entry('scroll-smooth', 'scroll-behavior: smooth'),
      entry('snap-none', 'scroll-snap-type: none'), entry('snap-x', 'scroll-snap-type: x var(--tw-scroll-snap-strictness)'),
      entry('snap-y', 'scroll-snap-type: y var(--tw-scroll-snap-strictness)'),
      entry('snap-mandatory', '--tw-scroll-snap-strictness: mandatory'),
      entry('snap-proximity', '--tw-scroll-snap-strictness: proximity'),
      entry('snap-start', 'scroll-snap-align: start'), entry('snap-end', 'scroll-snap-align: end'),
      entry('snap-center', 'scroll-snap-align: center'),
      entry('pointer-events-none', 'pointer-events: none'), entry('pointer-events-auto', 'pointer-events: auto'),
      entry('will-change-auto', 'will-change: auto'), entry('will-change-scroll', 'will-change: scroll-position'),
      entry('will-change-contents', 'will-change: contents'), entry('will-change-transform', 'will-change: transform'),
      entry('appearance-none', 'appearance: none'), entry('appearance-auto', 'appearance: auto'),
    ],
  },

  // ── SVG ───────────────────────────────────────────────────────────────────
  {
    category: 'SVG',
    entries: [
      ...colorEntries('fill', 'fill'),
      entry('fill-none', 'fill: none'),
      ...colorEntries('stroke', 'stroke'),
      entry('stroke-none', 'stroke: none'),
      ...[0,1,2].map(n => entry(`stroke-${n}`, `stroke-width: ${n}`)),
    ],
  },

  // ── Accessibility & Misc ──────────────────────────────────────────────────
  {
    category: 'Accessibility',
    entries: [
      entry('sr-only', 'position: absolute; width: 1px; height: 1px; padding: 0; margin: -1px; overflow: hidden; clip: rect(0, 0, 0, 0); white-space: nowrap; border-width: 0'),
      entry('not-sr-only', 'position: static; width: auto; height: auto; padding: 0; margin: 0; overflow: visible; clip: auto; white-space: normal'),
      entry('container', 'width: 100%'),
      entry('table-auto', 'table-layout: auto'), entry('table-fixed', 'table-layout: fixed'),
    ],
  },
];

// ── Génération Markdown ───────────────────────────────────────────────────

const outputPath = process.argv[2];
if (!outputPath) {
  console.error('Usage: node generate-catalog.mjs <output-path>');
  process.exit(1);
}

// Dédupliquer les entrées
const seenClasses = new Set();
for (const group of CATALOG) {
  group.entries = group.entries.filter(e => {
    if (seenClasses.has(e.class)) return false;
    seenClasses.add(e.class);
    return true;
  });
}

const totalClasses = CATALOG.reduce((n, g) => n + g.entries.length, 0);
console.log(`Génération du catalog Tailwind CSS v4...`);
console.log(`${totalClasses} classes dans ${CATALOG.length} catégories`);

const lines = [
  '# Tailwind CSS v4 — Référence complète des classes',
  '',
  `Index de ${totalClasses} classes Tailwind CSS (base, sans variants) avec leur CSS équivalent.`,
  '',
  '> **Tailwind v4** : les valeurs d\'espacement utilisent `calc(var(--spacing) * n)` où `--spacing = 0.25rem` par défaut.',
  '> Les couleurs utilisent des variables CSS : `var(--color-red-500)` etc. (valeurs oklch définies dans la base).',
  '> Utiliser `search_docs` avec un nom de classe (`p-4`), une propriété CSS (`padding`), ou une valeur (`1rem`).',
  '',
];

for (const { category, entries } of CATALOG) {
  if (!entries.length) continue;
  lines.push(`## ${category}`, '');
  lines.push('| Classe | CSS |');
  lines.push('|--------|-----|');
  for (const { class: cls, css } of entries) {
    lines.push(`| \`${cls}\` | \`${css.replace(/\|/g, '\\|')}\` |`);
  }
  lines.push('');
}

mkdirSync(dirname(outputPath), { recursive: true });
writeFileSync(outputPath, lines.join('\n'), 'utf8');
console.log(`catalog.md écrit : ${outputPath}`);
