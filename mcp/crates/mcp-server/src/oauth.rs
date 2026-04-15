use axum::{
    extract::{Form, Query, State},
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse, Response},
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
    time::{Duration, Instant},
};

const AUTHORIZE_HTML: &str = include_str!("views/authorize.html");
const APPROVED_HTML: &str = include_str!("views/approved.html");
const AUTHORIZE_CSS: &str = "\
*,*::before,*::after{box-sizing:border-box;margin:0;padding:0}\
body{min-height:100vh;display:flex;align-items:center;justify-content:center;\
background:radial-gradient(ellipse at top,#1e1b4b 0%,#0d0d14 60%);\
font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,sans-serif;\
color:#e2e8f0;padding:1rem}\
.card{background:rgba(255,255,255,.04);border:1px solid rgba(255,255,255,.08);\
border-radius:20px;padding:2.5rem 2rem;max-width:420px;width:100%;\
box-shadow:0 0 0 1px rgba(99,102,241,.1),0 20px 60px rgba(0,0,0,.6),inset 0 1px 0 rgba(255,255,255,.06)}\
.icon-wrap{width:60px;height:60px;border-radius:16px;\
background:linear-gradient(135deg,#7c3aed,#6366f1);\
display:flex;align-items:center;justify-content:center;\
margin:0 auto 1.5rem;color:#fff;box-shadow:0 8px 24px rgba(124,58,237,.4)}\
.icon-wrap--success{background:linear-gradient(135deg,#059669,#10b981);\
box-shadow:0 8px 24px rgba(16,185,129,.4)}\
h1{font-size:1.5rem;font-weight:700;text-align:center;color:#f1f5f9;\
margin-bottom:.5rem;letter-spacing:-.02em}\
.desc{text-align:center;color:#94a3b8;font-size:.9rem;line-height:1.6;margin-bottom:1.75rem}\
.permissions{background:rgba(99,102,241,.06);border:1px solid rgba(99,102,241,.15);\
border-radius:12px;padding:1.125rem;margin-bottom:1.75rem}\
.perm-header{font-size:.7rem;font-weight:700;text-transform:uppercase;\
letter-spacing:.1em;color:#6366f1;margin-bottom:.875rem}\
.perm-item{display:flex;align-items:center;gap:.625rem;padding:.375rem 0;\
font-size:.85rem;color:#cbd5e1}\
.perm-item svg{flex-shrink:0;color:#6366f1}\
.perm-item+.perm-item{border-top:1px solid rgba(99,102,241,.1);margin-top:.375rem;padding-top:.75rem}\
.btn{display:block;width:100%;padding:.9rem;border:none;border-radius:12px;\
font-size:.95rem;font-weight:600;cursor:pointer;transition:all .2s;\
text-align:center;letter-spacing:-.01em}\
.btn-primary{background:linear-gradient(135deg,#7c3aed,#6366f1);color:#fff}\
.btn-primary:hover{transform:translateY(-2px);box-shadow:0 12px 30px rgba(99,102,241,.45)}\
.btn-primary:active{transform:translateY(0)}\
.footer{text-align:center;font-size:.75rem;color:#475569;margin-top:1.5rem}\
.spinner{width:32px;height:32px;border:3px solid rgba(16,185,129,.2);\
border-top-color:#10b981;border-radius:50%;\
animation:spin .8s linear infinite;margin:1.5rem auto 0}\
@keyframes spin{to{transform:rotate(360deg)}}\
.check-anim{animation:pop .4s cubic-bezier(.175,.885,.32,1.275)}\
@keyframes pop{0%{transform:scale(0)}100%{transform:scale(1)}}\
";

pub struct OAuthState {
    issuer: String,
    access_token: String,
    allowed_origins: Vec<String>,
    codes: Mutex<HashMap<String, CodeEntry>>,
}

const CODE_TTL: Duration = Duration::from_secs(600); // 10 min

#[derive(Clone, Debug)]
struct CodeEntry {
    client_id: String,
    redirect_uri: String,
    code_challenge: String,
    used: bool,
    created_at: Instant,
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
        .route("/oauth/style.css", get(authorize_css))
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
            created_at: Instant::now(),
        },
    );

    let state_val = q.state.unwrap_or_default();
    let html = AUTHORIZE_HTML
        .replace("{{CODE}}", &esc_html(&code))
        .replace("{{REDIRECT_URI}}", &esc_html(&redirect_uri))
        .replace("{{STATE}}", &esc_html(&state_val));
    Html(html).into_response()
}

async fn authorize_css() -> impl IntoResponse {
    (
        [(axum::http::header::CONTENT_TYPE, "text/css; charset=utf-8")],
        AUTHORIZE_CSS,
    )
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
    let html = APPROVED_HTML.replace("{{REDIRECT_URL}}", &esc_html(&url));
    Html(html).into_response()
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
    // Nettoyage des codes expirés ou déjà utilisés
    codes.retain(|_, e| !e.used && e.created_at.elapsed() < CODE_TTL);
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
