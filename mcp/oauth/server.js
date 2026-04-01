const express = require('express');
const crypto = require('crypto');

const app = express();
app.use(express.json());
app.use(express.urlencoded({ extended: true }));

const ISSUER = process.env.OAUTH_ISSUER;
const ACCESS_TOKEN = process.env.MCP_API_KEY;

if (!ISSUER || !ACCESS_TOKEN) {
  console.error('Missing OAUTH_ISSUER or MCP_API_KEY');
  process.exit(1);
}

// In-memory stores (reset on container restart)
const clients = new Map(); // client_id → { redirect_uris }
const codes = new Map();   // code → { client_id, redirect_uri, code_challenge, used }

const esc = (s) =>
  String(s || '').replace(/[&<>"']/g, (c) =>
    ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#x27;' }[c])
  );

// RFC 9728 — OAuth 2.0 Protected Resource Metadata
app.get('/.well-known/oauth-protected-resource', (req, res) => {
  res.json({
    resource: ISSUER,
    authorization_servers: [ISSUER],
  });
});

// RFC 8414 — OAuth 2.0 Authorization Server Metadata
app.get('/.well-known/oauth-authorization-server', (req, res) => {
  res.json({
    issuer: ISSUER,
    authorization_endpoint: `${ISSUER}/oauth/authorize`,
    token_endpoint: `${ISSUER}/oauth/token`,
    registration_endpoint: `${ISSUER}/oauth/register`,
    response_types_supported: ['code'],
    grant_types_supported: ['authorization_code'],
    code_challenge_methods_supported: ['S256'],
    token_endpoint_auth_methods_supported: ['none'],
  });
});

// RFC 7591 — Dynamic Client Registration
app.post('/oauth/register', (req, res) => {
  const client_id = crypto.randomBytes(16).toString('hex');
  const { redirect_uris = [], client_name = 'MCP Client' } = req.body;
  clients.set(client_id, { redirect_uris, client_name });
  res.status(201).json({
    client_id,
    redirect_uris,
    client_name,
    token_endpoint_auth_method: 'none',
  });
});

// Authorization endpoint — page de confirmation unique utilisateur
app.get('/oauth/authorize', (req, res) => {
  const { client_id, redirect_uri, response_type, code_challenge, code_challenge_method, state } = req.query;

  if (response_type !== 'code' || code_challenge_method !== 'S256' || !code_challenge) {
    return res.status(400).json({ error: 'invalid_request' });
  }

  const code = crypto.randomBytes(16).toString('hex');
  codes.set(code, { client_id, redirect_uri, code_challenge, used: false });

  res.send(`<!DOCTYPE html>
<html lang="fr">
<head>
  <meta charset="UTF-8">
  <title>MCP — Autorisation</title>
  <style>
    body { font-family: sans-serif; max-width: 360px; margin: 80px auto; text-align: center; color: #1a1a1a; }
    h2 { font-size: 1.4rem; margin-bottom: .5rem; }
    p  { color: #555; margin-bottom: 1.5rem; }
    button {
      padding: 10px 28px; background: #0057b8; color: #fff;
      border: none; border-radius: 6px; font-size: 1rem; cursor: pointer;
    }
    button:hover { background: #004a9e; }
  </style>
</head>
<body>
  <h2>MCP — Autorisation</h2>
  <p>Autoriser l'accès à vos MCP servers personnels&nbsp;?</p>
  <form method="GET" action="/oauth/approve">
    <input type="hidden" name="code" value="${esc(code)}">
    <input type="hidden" name="redirect_uri" value="${esc(redirect_uri)}">
    <input type="hidden" name="state" value="${esc(state)}">
    <button type="submit">Autoriser</button>
  </form>
</body>
</html>`);
});

// Redirection après approbation
app.get('/oauth/approve', (req, res) => {
  const { code, redirect_uri, state } = req.query;
  try {
    const url = new URL(redirect_uri);
    url.searchParams.set('code', code);
    if (state) url.searchParams.set('state', state);
    res.redirect(url.toString());
  } catch {
    res.status(400).json({ error: 'invalid_redirect_uri' });
  }
});

// Token endpoint — échange code + code_verifier contre access_token
app.post('/oauth/token', (req, res) => {
  const { grant_type, code, redirect_uri, client_id, code_verifier } = req.body;

  if (grant_type !== 'authorization_code') {
    return res.status(400).json({ error: 'unsupported_grant_type' });
  }

  const entry = codes.get(code);
  if (!entry || entry.used || entry.client_id !== client_id || entry.redirect_uri !== redirect_uri) {
    return res.status(400).json({ error: 'invalid_grant' });
  }

  // PKCE S256
  const challenge = crypto.createHash('sha256').update(code_verifier).digest('base64url');
  if (challenge !== entry.code_challenge) {
    return res.status(400).json({ error: 'invalid_grant' });
  }

  entry.used = true;

  res.json({
    access_token: ACCESS_TOKEN,
    token_type: 'Bearer',
    expires_in: 31536000, // 1 an
  });
});

// Validation interne pour nginx auth_request
app.get('/oauth/validate', (req, res) => {
  const auth = req.headers['authorization'];
  res.sendStatus(auth === `Bearer ${ACCESS_TOKEN}` ? 200 : 401);
});

app.listen(8080, () => console.log(`OAuth server listening — issuer: ${ISSUER}`));
