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
    let key_path = match std::env::var("XQR_KEY_PATH") {
        Ok(path) => path,
        _ => "key.pub".to_string(),
    };
    println!("using key path {:?}", key_path);
    let k = std::fs::read(key_path)?;

    let k = SomePublicKey::from_pem(&k)?;
    let k = WithKid::new_with_thumbprint_id(k)?;
    println!("using key {:?}", k);

    let k_public_jwk = k.public_key_to_jwk()?;
    let jwks = JwkSet {
        keys: vec![k_public_jwk],
    };

    let state = Arc::new(AppState { jwks });

    let app = Router::new()
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
