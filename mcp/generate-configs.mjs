#!/usr/bin/env node
/**
 * Génère les fichiers de configuration MCP pour Claude Code et Codex CLI
 * à partir de mcp/servers-manifest.json.
 *
 * Usage :
 *   node mcp/generate-configs.mjs   # écrit dans dist/
 *
 * Les fichiers générés sont publiés automatiquement via GitHub Actions
 * comme assets de la release "mcp-configs".
 */

import { readFileSync, writeFileSync, mkdirSync } from 'fs';
import { resolve, dirname } from 'path';
import { fileURLToPath } from 'url';

const __dirname = dirname(fileURLToPath(import.meta.url));

// ── Source de vérité ─────────────────────────────────────────────────────────
const manifest = JSON.parse(
  readFileSync(resolve(__dirname, 'servers-manifest.json'), 'utf8')
);
const { baseUrl, mcpEndpoint, mcpName, docSources = [], external = [], stdio = [] } = manifest;

// ── Claude Code : claude-mcp.json → .mcp.json ────────────────────────────────
// Multi-tenant : un seul endpoint pour toutes les doc sources.
// OAuth 2.1 PKCE géré nativement par Claude Code.
const claudeConfig = {
  mcpServers: {
    // Self-hosted multi-tenant docs server
    [mcpName]: { type: 'http', url: `${baseUrl}${mcpEndpoint}` },
    // External HTTP (pas d'auth)
    ...Object.fromEntries(
      external.map(s => [
        s.name,
        { type: 'http', url: s.url }
      ])
    ),
    // Stdio (process local via npx/node/etc.)
    ...Object.fromEntries(
      stdio.map(s => [
        s.name,
        { type: 'stdio', command: s.command, args: s.args }
      ])
    ),
  }
};

// ── Codex CLI : codex-config.toml → ~/.codex/config.toml ─────────────────────
function generateCodexToml() {
  const header = [
    '# Codex CLI — MCP servers configuration',
    '# Source : https://github.com/bpodwinski/ai-core/releases/latest/download/codex-config.toml',
    '#',
    '# Installation :',
    '#   curl -fsSL https://github.com/bpodwinski/ai-core/releases/latest/download/codex-config.toml \\',
    '#        >> ~/.codex/config.toml',
    '#',
    '# Auth : codex mcp login docs',
    '',
    `[mcp_servers.${mcpName}]`,
    `url = "${baseUrl}${mcpEndpoint}"`,
    `# Multi-tenant docs server — categories: ${docSources.map(s => s.name).join(', ')}`,
    '',
  ].join('\n');

  const externalHeader = external.length > 0
    ? '# ── External MCP servers ─────────────────────────────────────────────────────\n\n'
    : '';

  const externalBlocks = external.map(s => {
    const lines = [
      `[mcp_servers.${s.name}]`,
      `url = "${s.url}"`,
    ];
    if (s.bearer_token_env_var) {
      lines.push(`bearer_token_env_var = "${s.bearer_token_env_var}"`);
    }
    lines.push(`# description = "${s.description}"`);
    lines.push('');
    return lines.join('\n');
  });

  return header + externalHeader + externalBlocks.join('\n');
}

// ── Écriture ──────────────────────────────────────────────────────────────────
const distDir = resolve(__dirname, '../dist');
mkdirSync(distDir, { recursive: true });

const outputs = [
  { path: resolve(distDir, 'claude-mcp.json'), content: JSON.stringify(claudeConfig, null, 2) + '\n' },
  { path: resolve(distDir, 'codex-config.toml'), content: generateCodexToml() },
];

for (const { path, content } of outputs) {
  writeFileSync(path, content);
  console.log(`Written: ${path}`);
}

const total = 1 + external.length + stdio.length; // 1 multi-tenant + external + stdio
console.log(`\n1 multi-tenant (${docSources.length} doc sources) + ${external.length} external + ${stdio.length} stdio = ${total} servers`);
