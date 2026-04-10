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

// ── Claude Code : claude-mcp.json → .mcp.json ────────────────────────────────
// OAuth 2.1 PKCE géré nativement par Claude Code (pas d'auth explicite).
// Copier à la racine du projet : curl … > .mcp.json
const claudeConfig = {
  mcpServers: Object.fromEntries(
    servers.map(s => [
      s.name,
      { type: 'http', url: `${baseUrl}/${s.name}/mcp` }
    ])
  )
};

// ── Codex CLI : codex-config.toml → ~/.codex/config.toml ─────────────────────
// Format TOML avec [mcp_servers.<name>] tables.
// Deux options d'auth :
//   - bearer_token_env_var : token statique dans MCP_API_KEY
//   - OAuth PKCE           : après ajout, lancer `codex mcp login <name>`
function generateCodexToml() {
  const header = [
    '# Codex CLI — MCP servers configuration',
    '# Source : https://raw.githubusercontent.com/bpodwinski/ai-core/main/dist/codex-config.toml',
    '#',
    '# Installation :',
    '#   curl -fsSL https://raw.githubusercontent.com/bpodwinski/ai-core/main/dist/codex-config.toml \\',
    '#        >> ~/.codex/config.toml',
    '#',
    '# Auth (choisir une option) :',
    '#   Option A — Bearer token : export MCP_API_KEY=<your-key>',
    '#   Option B — OAuth PKCE   : codex mcp login <server-name>',
    '',
  ].join('\n');

  const blocks = servers.map(s => [
    `[mcp_servers.${s.name}]`,
    `url = "${baseUrl}/${s.name}/mcp"`,
    `bearer_token_env_var = "MCP_API_KEY"`,
    `# description = "${s.description}"`,
    '',
  ].join('\n'));

  return header + blocks.join('\n');
}

// ── Écriture ──────────────────────────────────────────────────────────────────
const distDir = resolve(__dirname, '../dist');

const outputs = [
  {
    path: resolve(distDir, 'claude-mcp.json'),
    content: () => JSON.stringify(claudeConfig, null, 2) + '\n',
  },
  {
    path: resolve(distDir, 'codex-config.toml'),
    content: generateCodexToml,
  },
];

if (checkMode) {
  let dirty = false;
  for (const { path, content } of outputs) {
    const expected = content();
    if (!existsSync(path)) {
      console.error(`MISSING: ${path}`);
      dirty = true;
    } else if (readFileSync(path, 'utf8') !== expected) {
      console.error(`OUT OF DATE: ${path}`);
      dirty = true;
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
for (const { path, content } of outputs) {
  writeFileSync(path, content());
  console.log(`Written: ${path}`);
}

console.log(`\n${servers.length} servers — configs générées dans dist/`);
