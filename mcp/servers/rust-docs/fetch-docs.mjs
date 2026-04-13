#!/usr/bin/env node
/**
 * fetch-docs.mjs
 *
 * Reads servers-manifest.json and fetches/processes all documentation sources
 * into /docs/<name>/. Runs inside Docker build (docs-builder stage).
 *
 * Source types:
 *   - git   : clone repo, copy docsPath subfolder
 *   - url   : download single file
 *   - local : copy from /local-docs/<path>
 *
 * Transforms (applied in order):
 *   - strip-mdx        : convert .mdx → .md via remark AST
 *   - generate-catalog  : generate Tailwind CSS class catalog
 *   - split             : split single file into per-heading .md files
 */

import { readFileSync, cpSync, mkdirSync, existsSync } from 'fs';
import { execSync } from 'child_process';
import { resolve } from 'path';

const MANIFEST = 'servers-manifest.json';
const DOCS_DIR = '/docs';
const LOCAL_DOCS = '/local-docs';
const TMP = '/tmp/fetch';

const manifest = JSON.parse(readFileSync(MANIFEST, 'utf8'));

mkdirSync(DOCS_DIR, { recursive: true });
mkdirSync(TMP, { recursive: true });

let total = 0;

for (const entry of manifest.docSources) {
  const { name, source: src } = entry;
  if (!src) {
    console.log(`-- ${name}: no source defined, skipping`);
    continue;
  }

  const outDir = resolve(DOCS_DIR, name);
  console.log(`\n=> ${name} (${src.type})`);

  // ── Fetch ──────────────────────────────────────────────────────────────
  switch (src.type) {
    case 'git': {
      const cloneDir = resolve(TMP, name);
      execSync(`git clone --depth 1 ${src.url} ${cloneDir}`, { stdio: 'inherit' });
      const srcDir = resolve(cloneDir, src.docsPath);
      cpSync(srcDir, outDir, { recursive: true });
      break;
    }

    case 'url': {
      mkdirSync(outDir, { recursive: true });
      const tmpFile = resolve(TMP, `${name}.md`);
      execSync(`curl -fsSL "${src.url}" -o ${tmpFile}`, { stdio: 'inherit' });
      // If no transforms, copy file as-is into output dir
      if (!src.transforms?.length) {
        cpSync(tmpFile, resolve(outDir, `${name}.md`));
      }
      break;
    }

    case 'local': {
      const localPath = resolve(LOCAL_DOCS, src.path);
      if (existsSync(localPath)) {
        cpSync(localPath, outDir, { recursive: true });
      } else {
        console.error(`!! Local docs not found: ${localPath}`);
        process.exit(1);
      }
      break;
    }

    default:
      console.error(`!! Unknown source type: ${src.type}`);
      process.exit(1);
  }

  // ── Transforms ─────────────────────────────────────────────────────────
  if (src.transforms) {
    for (const transform of src.transforms) {
      console.log(`   transform: ${transform}`);
      switch (transform) {
        case 'strip-mdx':
          execSync(`node strip-mdx.mjs ${outDir}`, { stdio: 'inherit' });
          break;

        case 'generate-catalog':
          execSync(`node generate-catalog.mjs ${resolve(outDir, 'catalog.md')}`, {
            stdio: 'inherit',
          });
          break;

        case 'split': {
          const tmpFile = resolve(TMP, `${name}.md`);
          execSync(`node split-daisyui.mjs ${tmpFile} ${outDir}`, { stdio: 'inherit' });
          break;
        }

        default:
          console.error(`!! Unknown transform: ${transform}`);
          process.exit(1);
      }
    }
  }

  total++;
  console.log(`   OK ${name}`);
}

console.log(`\nDone: ${total} doc sources fetched to ${DOCS_DIR}`);
