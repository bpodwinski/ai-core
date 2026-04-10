#!/usr/bin/env node
/**
 * Génère les fichiers de configuration MCP pour Claude Code et Codex CLI
 * à partir de mcp/servers-manifest.json.
 *
 * Usage :
 *   node mcp/generate-configs.mjs            # écrit dans dist/
 *   node mcp/generate-configs.mjs --check    # vérifie que dist/ est à jour (exit 1 si diff)
 */

import { readFileSync, writeFileSync, mkdirSync, existsSync } from 'fs';
import { resolve, dirname } from 'path';
import { fileURLToPath } from 'url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const checkMode = process.argv.includes('--check');

// ── Source de vérité ─────────────────────────────────────────────────────────
const manifest = JSON.parse(
  readFileSync(resolve(__dirname, 'servers-manifest.json'), 'utf8')
);
const { baseUrl, servers } = manifest;

// ── Claude Code : .mcp.json ───────────────────────────────────────────────────
// Utilise le flux OAuth 2.1 PKCE intégré à Claude Code (pas d'auth explicite).
// Copier ce fichier à la racine du projet sous le nom .mcp.json
const claudeConfig = {
  mcpServers: Object.fromEntries(
    servers.map(s => [
      s.name,
      {
        type: 'http',
        url: `${baseUrl}/${s.name}/mcp`
      }
    ])
  )
};

// ── Codex CLI : codex-mcp.json ────────────────────────────────────────────────
// Utilise un Bearer token explicite via la variable d'environnement MCP_API_KEY.
// Copier ce fichier dans ~/.codex/mcp_servers.json
// (ou merger dans ~/.codex/config.json sous la clé "mcpServers")
const codexConfig = {
  mcpServers: Object.fromEntries(
    servers.map(s => [
      s.name,
      {
        type: 'http',
        url: `${baseUrl}/${s.name}/mcp`,
        headers: {
          Authorization: 'Bearer ${MCP_API_KEY}'
        }
      }
    ])
  )
};

// ── Écriture ──────────────────────────────────────────────────────────────────
const distDir = resolve(__dirname, '../dist');

const outputs = [
  { path: resolve(distDir, 'claude-mcp.json'), data: claudeConfig },
  { path: resolve(distDir, 'codex-mcp.json'),  data: codexConfig  }
];

if (checkMode) {
  let dirty = false;
  for (const { path, data } of outputs) {
    const expected = JSON.stringify(data, null, 2) + '\n';
    if (!existsSync(path)) {
      console.error(`MISSING: ${path}`);
      dirty = true;
    } else {
      const current = readFileSync(path, 'utf8');
      if (current !== expected) {
        console.error(`OUT OF DATE: ${path}`);
        dirty = true;
      }
    }
  }
  if (dirty) {
    console.error('\nRun: node mcp/generate-configs.mjs');
    process.exit(1);
  }
  console.log('dist/ configs are up to date.');
  process.exit(0);
}

mkdirSync(distDir, { recursive: true });
for (const { path, data } of outputs) {
  writeFileSync(path, JSON.stringify(data, null, 2) + '\n');
  console.log(`Written: ${path}`);
}

console.log(`\n${servers.length} servers — configs générées dans dist/`);
console.log(`\nTéléchargement :`);
console.log(`  # Claude Code`);
console.log(`  curl -fsSL https://raw.githubusercontent.com/bpodwinski/ai-core/main/dist/claude-mcp.json > .mcp.json`);
console.log(`\n  # Codex CLI`);
console.log(`  curl -fsSL https://raw.githubusercontent.com/bpodwinski/ai-core/main/dist/codex-mcp.json > ~/.codex/mcp_servers.json`);
