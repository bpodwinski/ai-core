use axum::{
    extract::{Form, Query, State},
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse, Redirect, Response},
    routing::{get, post},
    Json, Router,
};
use base64::Engine;
use rand::RngCore;
use serde::Deserialize;
use sha2::Digest;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

const AUTHORIZE_HTML: &str = include_str!("views/authorize.html");

pub struct OAuthState {
    issuer: String,
    access_token: String,
    allowed_origins: Vec<String>,
    codes: Mutex<HashMap<String, CodeEntry>>,
}

#[derive(Clone, Debug)]
struct CodeEntry {
    client_id: String,
    redirect_uri: String,
    code_challenge: String,
    used: bool,
}

impl OAuthState {
    pub fn from_env() -> anyhow::Result<Arc<Self>> {
        let issuer = std::env::var("OAUTH_ISSUER")
            .map_err(|_| anyhow::anyhow!("OAUTH_ISSUER env var is required"))?;
        let access_token = std::env::var("MCP_API_KEY")
            .map_err(|_| anyhow::anyhow!("MCP_API_KEY env var is required"))?;
        let allowed_origins = std::env::var("OAUTH_ALLOWED_ORIGINS")
            .unwrap_or_default()
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        Ok(Arc::new(Self {
            issuer,
            access_token,
            allowed_origins,
            codes: Mutex::new(HashMap::new()),
        }))
    }

    fn is_allowed_redirect(&self, uri: &str) -> bool {
        if self.allowed_origins.is_empty() {
            return true;
        }
        let origin = match extract_origin(uri) {
            Some(o) => o,
            None => return false,
        };
        self.allowed_origins
            .iter()
            .any(|a| origin == *a || uri.starts_with(a))
    }
}

pub fn router(state: Arc<OAuthState>) -> Router {
    Router::new()
        .route(
            "/.well-known/oauth-protected-resource",
            get(protected_resource),
        )
        .route(
            "/.well-known/oauth-authorization-server",
            get(auth_server_metadata),
        )
        .route("/oauth/register", post(register))
        .route("/oauth/authorize", get(authorize))
        .route("/oauth/approve", get(approve))
        .route("/oauth/token", post(token))
        .route("/oauth/validate", get(validate))
        .with_state(state)
}

async fn protected_resource(State(s): State<Arc<OAuthState>>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "resource": s.issuer,
        "authorization_servers": [s.issuer],
    }))
}

async fn auth_server_metadata(State(s): State<Arc<OAuthState>>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "issuer": s.issuer,
        "authorization_endpoint": format!("{}/oauth/authorize", s.issuer),
        "token_endpoint": format!("{}/oauth/token", s.issuer),
        "registration_endpoint": format!("{}/oauth/register", s.issuer),
        "response_types_supported": ["code"],
        "grant_types_supported": ["authorization_code"],
        "code_challenge_methods_supported": ["S256"],
        "token_endpoint_auth_methods_supported": ["none"],
    }))
}

#[derive(Deserialize)]
struct RegisterReq {
    #[serde(default)]
    redirect_uris: Vec<String>,
    #[serde(default = "default_client_name")]
    client_name: String,
}

fn default_client_name() -> String {
    "MCP Client".to_string()
}

async fn register(Json(req): Json<RegisterReq>) -> (StatusCode, Json<serde_json::Value>) {
    let client_id = random_hex(16);
    (
        StatusCode::CREATED,
        Json(serde_json::json!({
            "client_id": client_id,
            "redirect_uris": req.redirect_uris,
            "client_name": req.client_name,
            "token_endpoint_auth_method": "none",
        })),
    )
}

#[derive(Deserialize)]
struct AuthorizeQuery {
    client_id: Option<String>,
    redirect_uri: Option<String>,
    response_type: Option<String>,
    code_challenge: Option<String>,
    code_challenge_method: Option<String>,
    state: Option<String>,
}

async fn authorize(State(s): State<Arc<OAuthState>>, Query(q): Query<AuthorizeQuery>) -> Response {
    if q.response_type.as_deref() != Some("code")
        || q.code_challenge_method.as_deref() != Some("S256")
    {
        return error_json(StatusCode::BAD_REQUEST, "invalid_request");
    }
    let challenge = match q.code_challenge {
        Some(c) if !c.is_empty() => c,
        _ => return error_json(StatusCode::BAD_REQUEST, "invalid_request"),
    };
    let redirect_uri = match q.redirect_uri {
        Some(u) => u,
        None => return error_json(StatusCode::BAD_REQUEST, "invalid_request"),
    };
    if !s.is_allowed_redirect(&redirect_uri) {
        return error_json(StatusCode::BAD_REQUEST, "invalid_redirect_uri");
    }

    let code = random_hex(16);
    s.codes.lock().unwrap().insert(
        code.clone(),
        CodeEntry {
            client_id: q.client_id.unwrap_or_default(),
            redirect_uri: redirect_uri.clone(),
            code_challenge: challenge,
            used: false,
        },
    );

    let state_val = q.state.unwrap_or_default();
    let html = AUTHORIZE_HTML
        .replace("{{CODE}}", &esc_html(&code))
        .replace("{{REDIRECT_URI}}", &esc_html(&redirect_uri))
        .replace("{{STATE}}", &esc_html(&state_val));
    Html(html).into_response()
}

#[derive(Deserialize)]
struct ApproveQuery {
    code: String,
    redirect_uri: String,
    #[serde(default)]
    state: String,
}

async fn approve(Query(q): Query<ApproveQuery>) -> Response {
    if !(q.redirect_uri.starts_with("http://") || q.redirect_uri.starts_with("https://")) {
        return error_json(StatusCode::BAD_REQUEST, "invalid_redirect_uri");
    }
    let sep = if q.redirect_uri.contains('?') {
        '&'
    } else {
        '?'
    };
    let mut url = format!(
        "{}{}code={}",
        q.redirect_uri,
        sep,
        urlencoding::encode(&q.code)
    );
    if !q.state.is_empty() {
        url.push_str("&state=");
        url.push_str(&urlencoding::encode(&q.state));
    }
    Redirect::to(&url).into_response()
}

#[derive(Deserialize)]
struct TokenReq {
    grant_type: Option<String>,
    code: Option<String>,
    redirect_uri: Option<String>,
    client_id: Option<String>,
    code_verifier: Option<String>,
}

async fn token(State(s): State<Arc<OAuthState>>, Form(req): Form<TokenReq>) -> Response {
    if req.grant_type.as_deref() != Some("authorization_code") {
        return error_json(StatusCode::BAD_REQUEST, "unsupported_grant_type");
    }
    let redirect_uri = match req.redirect_uri {
        Some(r) => r,
        None => return error_json(StatusCode::BAD_REQUEST, "invalid_request"),
    };
    if !s.is_allowed_redirect(&redirect_uri) {
        return error_json(StatusCode::BAD_REQUEST, "invalid_redirect_uri");
    }
    let code = match req.code {
        Some(c) => c,
        None => return error_json(StatusCode::BAD_REQUEST, "invalid_grant"),
    };
    let client_id = req.client_id.unwrap_or_default();
    let verifier = match req.code_verifier {
        Some(v) => v,
        None => return error_json(StatusCode::BAD_REQUEST, "invalid_grant"),
    };

    let mut codes = s.codes.lock().unwrap();
    let entry = match codes.get_mut(&code) {
        Some(e) => e,
        None => return error_json(StatusCode::BAD_REQUEST, "invalid_grant"),
    };
    if entry.used || entry.client_id != client_id || entry.redirect_uri != redirect_uri {
        return error_json(StatusCode::BAD_REQUEST, "invalid_grant");
    }

    let hash = sha2::Sha256::digest(verifier.as_bytes());
    let expected = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(hash);
    if expected != entry.code_challenge {
        return error_json(StatusCode::BAD_REQUEST, "invalid_grant");
    }
    entry.used = true;
    drop(codes);

    Json(serde_json::json!({
        "access_token": s.access_token,
        "token_type": "Bearer",
        "expires_in": 3600,
    }))
    .into_response()
}

async fn validate(State(s): State<Arc<OAuthState>>, headers: HeaderMap) -> StatusCode {
    let expected = format!("Bearer {}", s.access_token);
    match headers.get("authorization").and_then(|v| v.to_str().ok()) {
        Some(auth) if auth == expected => StatusCode::OK,
        _ => StatusCode::UNAUTHORIZED,
    }
}

fn random_hex(n: usize) -> String {
    let mut buf = vec![0u8; n];
    rand::rngs::OsRng.fill_bytes(&mut buf);
    let mut out = String::with_capacity(n * 2);
    for b in &buf {
        out.push_str(&format!("{:02x}", b));
    }
    out
}

fn esc_html(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#x27;"),
            _ => out.push(c),
        }
    }
    out
}

fn extract_origin(uri: &str) -> Option<String> {
    let (scheme, rest) = if let Some(r) = uri.strip_prefix("https://") {
        ("https://", r)
    } else if let Some(r) = uri.strip_prefix("http://") {
        ("http://", r)
    } else {
        return None;
    };
    let host_end = rest.find('/').unwrap_or(rest.len());
    if host_end == 0 {
        return None;
    }
    Some(format!("{scheme}{}", &rest[..host_end]))
}

fn error_json(status: StatusCode, err: &str) -> Response {
    (status, Json(serde_json::json!({ "error": err }))).into_response()
}
