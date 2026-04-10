/**
 * strip-mdx.mjs
 *
 * Converts MDX files to clean Markdown by removing JSX nodes via remark AST.
 * Reads .mdx files, strips all JSX (imports, exports, components), and writes
 * clean .md files alongside the originals.
 *
 * Usage: node strip-mdx.mjs <docs-directory>
 *
 * What is removed:
 *   - mdxjsEsm        : import/export statements
 *   - mdxFlowExpression / mdxTextExpression : JS expressions {foo}
 *   - mdxJsxFlowElement / mdxJsxTextElement : JSX components (<Figure>, <Example>, etc.)
 *     → replaced by their children so code blocks inside are preserved
 *
 * What is kept:
 *   - Headings (##, ###)
 *   - Paragraphs / prose
 *   - Code blocks (```html, ```css, etc.)
 *   - Lists, tables
 */

import { remark } from 'remark';
import remarkMdx from 'remark-mdx';
import { visit, SKIP } from 'unist-util-visit';
import { remove } from 'unist-util-remove';
import { readFileSync, writeFileSync, readdirSync, statSync } from 'fs';
import { join, extname } from 'path';

function stripMdxPlugin() {
  return (tree) => {
    // Supprimer les imports/exports ESM en entier
    remove(tree, 'mdxjsEsm');

    // Supprimer les expressions JS de type {expression}
    remove(tree, 'mdxFlowExpression');
    remove(tree, 'mdxTextExpression');

    // Unwrapper les éléments JSX : remplacer le nœud par ses enfants
    // Ainsi <Figure>...code block...</Figure> → ...code block...
    visit(tree, (node, index, parent) => {
      if (
        parent &&
        typeof index === 'number' &&
        (node.type === 'mdxJsxFlowElement' || node.type === 'mdxJsxTextElement')
      ) {
        parent.children.splice(index, 1, ...(node.children || []));
        return [SKIP, index];
      }
    });
  };
}

const processor = remark().use(remarkMdx).use(stripMdxPlugin);

let processed = 0;
let errors = 0;

function processDir(dir) {
  for (const entry of readdirSync(dir)) {
    const fullPath = join(dir, entry);
    if (statSync(fullPath).isDirectory()) {
      processDir(fullPath);
    } else if (extname(entry) === '.mdx') {
      const outPath = fullPath.slice(0, -4) + '.md';
      try {
        const content = readFileSync(fullPath, 'utf8');
        const result = processor.processSync(content);
        writeFileSync(outPath, String(result));
        processed++;
      } catch (err) {
        console.error(`  ✗ ${entry}: ${err.message}`);
        errors++;
      }
    }
  }
}

const docsDir = process.argv[2];
if (!docsDir) {
  console.error('Usage: node strip-mdx.mjs <docs-directory>');
  process.exit(1);
}

console.log(`Traitement MDX → Markdown dans : ${docsDir}`);
processDir(docsDir);
console.log(`Terminé : ${processed} fichiers convertis, ${errors} erreurs.`);
