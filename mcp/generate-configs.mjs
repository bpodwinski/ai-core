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
const { baseUrl, servers, external = [] } = manifest;

// ── Claude Code : claude-mcp.json → .mcp.json ────────────────────────────────
// OAuth 2.1 PKCE géré nativement par Claude Code pour les serveurs self-hosted.
// Serveurs externes : pas d'auth, URL directe.
const claudeConfig = {
  mcpServers: {
    // Self-hosted (OAuth)
    ...Object.fromEntries(
      servers.map(s => [
        s.name,
        { type: 'http', url: `${baseUrl}/${s.name}/mcp` }
      ])
    ),
    // External (pas d'auth)
    ...Object.fromEntries(
      external.map(s => [
        s.name,
        { type: 'http', url: s.url }
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
    '# Auth self-hosted — OAuth PKCE : codex mcp login <server-name>',
    '',
  ].join('\n');

  const selfHostedBlocks = servers.map(s => [
    `[mcp_servers.${s.name}]`,
    `url = "${baseUrl}/${s.name}/mcp"`,
    `# description = "${s.description}"`,
    '',
  ].join('\n'));

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

  return header
    + selfHostedBlocks.join('\n')
    + externalHeader
    + externalBlocks.join('\n');
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

const total = servers.length + external.length;
console.log(`\n${servers.length} self-hosted + ${external.length} external = ${total} servers`);
