/**
 * generate-catalog.cjs
 *
 * Génère un catalog.md de toutes les classes Tailwind CSS (base sans variants)
 * avec leur équivalent CSS, organisé par catégorie.
 *
 * Utilise PostCSS + Tailwind v3 avec safelist universelle pour extraire
 * toutes les utilities et les formater en tableau Markdown.
 *
 * Usage: node generate-catalog.cjs <output-path>
 */

const postcss = require('postcss');
const tailwindcss = require('tailwindcss');
const fs = require('fs');
const path = require('path');

// ── Inférence de catégorie depuis la propriété CSS principale ──────────────

const PROP_TO_CATEGORY = {
  // Layout
  display: 'Layout — Display',
  position: 'Layout — Position',
  top: 'Layout — Position',
  right: 'Layout — Position',
  bottom: 'Layout — Position',
  left: 'Layout — Position',
  inset: 'Layout — Position',
  'z-index': 'Layout — Z-Index',
  float: 'Layout — Float',
  clear: 'Layout — Float',
  visibility: 'Layout — Visibility',
  overflow: 'Layout — Overflow',
  'overflow-x': 'Layout — Overflow',
  'overflow-y': 'Layout — Overflow',
  overscroll: 'Layout — Overscroll',
  'overscroll-behavior': 'Layout — Overscroll',
  'box-sizing': 'Layout — Box Sizing',
  isolation: 'Layout — Isolation',
  'object-fit': 'Layout — Object Fit',
  'object-position': 'Layout — Object Position',
  'aspect-ratio': 'Layout — Aspect Ratio',
  columns: 'Layout — Columns',
  // Flexbox
  flex: 'Flexbox',
  'flex-direction': 'Flexbox',
  'flex-wrap': 'Flexbox',
  'flex-grow': 'Flexbox',
  'flex-shrink': 'Flexbox',
  order: 'Flexbox — Order',
  'flex-basis': 'Flexbox — Basis',
  // Grid
  'grid-template-columns': 'Grid',
  'grid-template-rows': 'Grid',
  'grid-column': 'Grid — Column',
  'grid-row': 'Grid — Row',
  'grid-auto-flow': 'Grid — Auto Flow',
  'grid-auto-columns': 'Grid',
  'grid-auto-rows': 'Grid',
  // Alignment
  'align-items': 'Flexbox & Grid — Alignment',
  'align-content': 'Flexbox & Grid — Alignment',
  'align-self': 'Flexbox & Grid — Alignment',
  'justify-content': 'Flexbox & Grid — Alignment',
  'justify-items': 'Flexbox & Grid — Alignment',
  'justify-self': 'Flexbox & Grid — Alignment',
  'place-content': 'Flexbox & Grid — Alignment',
  'place-items': 'Flexbox & Grid — Alignment',
  'place-self': 'Flexbox & Grid — Alignment',
  // Gap
  gap: 'Spacing — Gap',
  'column-gap': 'Spacing — Gap',
  'row-gap': 'Spacing — Gap',
  // Padding
  padding: 'Spacing — Padding',
  'padding-top': 'Spacing — Padding',
  'padding-right': 'Spacing — Padding',
  'padding-bottom': 'Spacing — Padding',
  'padding-left': 'Spacing — Padding',
  'padding-inline': 'Spacing — Padding',
  'padding-block': 'Spacing — Padding',
  'padding-inline-start': 'Spacing — Padding',
  'padding-inline-end': 'Spacing — Padding',
  // Margin
  margin: 'Spacing — Margin',
  'margin-top': 'Spacing — Margin',
  'margin-right': 'Spacing — Margin',
  'margin-bottom': 'Spacing — Margin',
  'margin-left': 'Spacing — Margin',
  'margin-inline': 'Spacing — Margin',
  'margin-block': 'Spacing — Margin',
  'margin-inline-start': 'Spacing — Margin',
  'margin-inline-end': 'Spacing — Margin',
  // Sizing
  width: 'Sizing — Width',
  'max-width': 'Sizing — Width',
  'min-width': 'Sizing — Width',
  height: 'Sizing — Height',
  'max-height': 'Sizing — Height',
  'min-height': 'Sizing — Height',
  // Typography
  'font-family': 'Typography — Font Family',
  'font-size': 'Typography — Font Size',
  'font-weight': 'Typography — Font Weight',
  'font-style': 'Typography — Font Style',
  'font-smoothing': 'Typography — Font Smoothing',
  '-webkit-font-smoothing': 'Typography — Font Smoothing',
  'font-variant-numeric': 'Typography — Font Variant',
  'letter-spacing': 'Typography — Letter Spacing',
  'line-height': 'Typography — Line Height',
  'list-style-type': 'Typography — Lists',
  'list-style-position': 'Typography — Lists',
  'text-align': 'Typography — Text Align',
  'text-decoration-line': 'Typography — Text Decoration',
  'text-decoration': 'Typography — Text Decoration',
  'text-decoration-color': 'Typography — Text Decoration',
  'text-decoration-style': 'Typography — Text Decoration',
  'text-decoration-thickness': 'Typography — Text Decoration',
  'text-underline-offset': 'Typography — Text Decoration',
  'text-transform': 'Typography — Text Transform',
  'text-overflow': 'Typography — Text Overflow',
  'text-indent': 'Typography — Text Indent',
  'vertical-align': 'Typography — Vertical Align',
  'white-space': 'Typography — Whitespace',
  'word-break': 'Typography — Word Break',
  'overflow-wrap': 'Typography — Word Break',
  hyphens: 'Typography — Hyphens',
  color: 'Typography — Text Color',
  // Backgrounds
  'background-color': 'Backgrounds — Color',
  'background-image': 'Backgrounds — Image',
  'background-size': 'Backgrounds — Size',
  'background-position': 'Backgrounds — Position',
  'background-repeat': 'Backgrounds — Repeat',
  'background-attachment': 'Backgrounds — Attachment',
  'background-clip': 'Backgrounds — Clip',
  'background-origin': 'Backgrounds — Origin',
  'background-opacity': 'Backgrounds — Opacity',
  // Borders
  'border-radius': 'Borders — Radius',
  'border-top-left-radius': 'Borders — Radius',
  'border-top-right-radius': 'Borders — Radius',
  'border-bottom-left-radius': 'Borders — Radius',
  'border-bottom-right-radius': 'Borders — Radius',
  'border-width': 'Borders — Width',
  'border-top-width': 'Borders — Width',
  'border-right-width': 'Borders — Width',
  'border-bottom-width': 'Borders — Width',
  'border-left-width': 'Borders — Width',
  'border-color': 'Borders — Color',
  'border-style': 'Borders — Style',
  'border-collapse': 'Borders — Collapse',
  'outline-width': 'Borders — Outline',
  'outline-color': 'Borders — Outline',
  'outline-style': 'Borders — Outline',
  'outline-offset': 'Borders — Outline',
  // Effects
  'box-shadow': 'Effects — Shadow',
  opacity: 'Effects — Opacity',
  'mix-blend-mode': 'Effects — Blend Mode',
  'background-blend-mode': 'Effects — Blend Mode',
  // Filters
  filter: 'Filters',
  'backdrop-filter': 'Filters — Backdrop',
  // Transitions
  transition: 'Transitions',
  'transition-property': 'Transitions',
  'transition-duration': 'Transitions',
  'transition-timing-function': 'Transitions',
  'transition-delay': 'Transitions',
  animation: 'Animation',
  // Transforms
  transform: 'Transforms',
  'transform-origin': 'Transforms — Origin',
  // Interactivity
  cursor: 'Interactivity — Cursor',
  'pointer-events': 'Interactivity',
  resize: 'Interactivity — Resize',
  'scroll-behavior': 'Interactivity — Scroll',
  'scroll-margin': 'Interactivity — Scroll',
  'scroll-padding': 'Interactivity — Scroll',
  'scroll-snap-type': 'Interactivity — Snap',
  'scroll-snap-align': 'Interactivity — Snap',
  'scroll-snap-stop': 'Interactivity — Snap',
  'user-select': 'Interactivity — Select',
  'touch-action': 'Interactivity — Touch',
  'will-change': 'Interactivity — Will Change',
  appearance: 'Interactivity — Appearance',
  // SVG
  fill: 'SVG',
  stroke: 'SVG',
  'stroke-width': 'SVG',
  // Accessibility
  'clip-path': 'Accessibility',
};

const CATEGORY_ORDER = [
  'Layout — Display',
  'Layout — Position',
  'Layout — Z-Index',
  'Layout — Overflow',
  'Layout — Overscroll',
  'Layout — Float',
  'Layout — Visibility',
  'Layout — Box Sizing',
  'Layout — Isolation',
  'Layout — Object Fit',
  'Layout — Object Position',
  'Layout — Aspect Ratio',
  'Layout — Columns',
  'Flexbox',
  'Flexbox — Order',
  'Flexbox — Basis',
  'Grid',
  'Grid — Column',
  'Grid — Row',
  'Grid — Auto Flow',
  'Flexbox & Grid — Alignment',
  'Spacing — Gap',
  'Spacing — Padding',
  'Spacing — Margin',
  'Sizing — Width',
  'Sizing — Height',
  'Typography — Font Family',
  'Typography — Font Size',
  'Typography — Font Weight',
  'Typography — Font Style',
  'Typography — Font Smoothing',
  'Typography — Font Variant',
  'Typography — Letter Spacing',
  'Typography — Line Height',
  'Typography — Lists',
  'Typography — Text Align',
  'Typography — Text Color',
  'Typography — Text Decoration',
  'Typography — Text Transform',
  'Typography — Text Overflow',
  'Typography — Text Indent',
  'Typography — Vertical Align',
  'Typography — Whitespace',
  'Typography — Word Break',
  'Typography — Hyphens',
  'Backgrounds — Color',
  'Backgrounds — Image',
  'Backgrounds — Size',
  'Backgrounds — Position',
  'Backgrounds — Repeat',
  'Backgrounds — Attachment',
  'Backgrounds — Clip',
  'Backgrounds — Origin',
  'Borders — Radius',
  'Borders — Width',
  'Borders — Color',
  'Borders — Style',
  'Borders — Outline',
  'Borders — Collapse',
  'Effects — Shadow',
  'Effects — Opacity',
  'Effects — Blend Mode',
  'Filters',
  'Filters — Backdrop',
  'Transitions',
  'Animation',
  'Transforms',
  'Transforms — Origin',
  'Interactivity — Cursor',
  'Interactivity — Resize',
  'Interactivity — Select',
  'Interactivity — Touch',
  'Interactivity — Scroll',
  'Interactivity — Snap',
  'Interactivity — Will Change',
  'Interactivity — Appearance',
  'Interactivity',
  'SVG',
  'Accessibility',
  'Autres',
];

function inferCategory(declarations) {
  for (const decl of declarations) {
    const prop = decl.split(':')[0].trim().toLowerCase();
    if (PROP_TO_CATEGORY[prop]) return PROP_TO_CATEGORY[prop];
  }
  return 'Autres';
}

// ── Génération du CSS via PostCSS + Tailwind v3 ────────────────────────────

async function generateCatalog(outputPath) {
  console.log('Génération CSS avec Tailwind v3 + PostCSS...');

  const result = await postcss([
    tailwindcss({
      content: [{ raw: '' }],
      safelist: [{ pattern: /.*/ }],
      corePlugins: { preflight: false },
    }),
  ]).process('@tailwind utilities;', { from: undefined });

  const css = result.css;
  console.log(`CSS généré : ${(css.length / 1024).toFixed(0)} KB`);

  // ── Parsing : uniquement les sélecteurs simples (pas de variants) ──────
  // Exclut : .hover\:p-4, .md\:flex, .dark\:bg-gray-800, etc.
  const catalog = new Map();

  // Regex : classe simple sans backslash ni deux-points dans le nom
  const ruleRegex = /^\.([\w/-]+)\s*\{([^}]+)\}/gm;

  for (const match of css.matchAll(ruleRegex)) {
    const className = match[1];
    const body = match[2].trim();

    // Exclure les classes vides ou les pseudo-classes imbriquées
    if (!body || className.includes(':')) continue;

    const declarations = body
      .split(';')
      .map((d) => d.trim())
      .filter(Boolean);

    const category = inferCategory(declarations);
    if (!catalog.has(category)) catalog.set(category, []);

    // Condensé en une ligne pour le tableau
    const cssOneLine = declarations.join('; ');
    catalog.get(category).push({ class: className, css: cssOneLine });
  }

  // ── Génération Markdown ────────────────────────────────────────────────
  const totalClasses = [...catalog.values()].reduce((n, v) => n + v.length, 0);
  console.log(`${totalClasses} classes trouvées dans ${catalog.size} catégories`);

  const lines = [
    '# Tailwind CSS — Référence complète des classes',
    '',
    `Index de toutes les classes Tailwind CSS (base, sans variants) avec leur CSS équivalent. ${totalClasses} classes au total.`,
    '',
    '> Utiliser `search_docs` avec un nom de classe (ex: `p-4`), une propriété CSS (ex: `padding`), ou une valeur (ex: `1rem`) pour trouver la classe correspondante.',
    '',
  ];

  // Trier les catégories selon CATEGORY_ORDER
  const sortedCategories = [...catalog.keys()].sort((a, b) => {
    const ia = CATEGORY_ORDER.indexOf(a);
    const ib = CATEGORY_ORDER.indexOf(b);
    if (ia === -1 && ib === -1) return a.localeCompare(b);
    if (ia === -1) return 1;
    if (ib === -1) return -1;
    return ia - ib;
  });

  for (const category of sortedCategories) {
    const entries = catalog.get(category);
    lines.push(`## ${category}`, '');
    lines.push('| Classe | CSS |');
    lines.push('|--------|-----|');
    for (const { class: cls, css } of entries) {
      // Échapper les pipes dans le CSS pour le tableau Markdown
      const safeCss = css.replace(/\|/g, '\\|');
      lines.push(`| \`${cls}\` | \`${safeCss}\` |`);
    }
    lines.push('');
  }

  fs.mkdirSync(path.dirname(outputPath), { recursive: true });
  fs.writeFileSync(outputPath, lines.join('\n'), 'utf8');
  console.log(`catalog.md écrit dans : ${outputPath}`);
}

// ── Entrée ─────────────────────────────────────────────────────────────────

const outputPath = process.argv[2];
if (!outputPath) {
  console.error('Usage: node generate-catalog.cjs <output-path>');
  process.exit(1);
}

generateCatalog(outputPath).catch((err) => {
  console.error('Erreur :', err.message);
  process.exit(1);
});
