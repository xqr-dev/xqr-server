use axum::response::Html;
use axum::{extract::State, response::IntoResponse, routing::get, Json, Router};
use jwtk::{
    jwk::{JwkSet, WithKid},
    PublicKeyToJwk, SomePublicKey,
};
use std::{net::Ipv4Addr, sync::Arc};

struct AppState {
    jwks: JwkSet,
}

#[tokio::main]
async fn main() -> jwtk::Result<()> {
    let key = match std::env::var("XQR_KEY") {
        Ok(pub_key) => {
            println!("using key from env");
            pub_key.as_bytes().to_vec()
        }
        _ => {
            let key_path = match std::env::var("XQR_KEY_PATH") {
                Ok(path) => path,
                _ => "key.pub".to_string(),
            };
            println!("using key path {:?}", key_path);
            std::fs::read(key_path)?
        }
    };

    let key = SomePublicKey::from_pem(&key)?;
    let key = WithKid::new_with_thumbprint_id(key)?;
    println!("using key {:?}", key);

    let k_public_jwk = key.public_key_to_jwk()?;
    let jwks = JwkSet {
        keys: vec![k_public_jwk],
    };

    let state = Arc::new(AppState { jwks });

    let app = Router::new()
        .route("/", get(home))
        .route("/.well-known/jwks.json", get(jwks_handler))
        .with_state(state);

    axum::Server::bind(&(Ipv4Addr::from(0), 3000).into())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn jwks_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    Json(&state.jwks).into_response()
}

async fn home() -> impl IntoResponse {
    Html(
        r#"
    <html>
        <head>
            <title>XQR Code Server Demo</title>
            <style>body{text-align:center}</style>
        </head>
        <body>
            <p>This is a demo server for eXtended QR (XQR) Codes.</p>
            <p>See <a href="https://github.com/xqr-dev/xqr-server">GitHub</a> for more information.</p>
        </body>
    </html>
    "#
        .to_string(),
    )
}
