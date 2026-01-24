use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};


#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Serialize)]
struct Item {
    id: i32,
    name: String,
    quantity: i32,
    status: String,
}

async fn auth_middleware(req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req.headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let token = if auth_header.starts_with("Bearer ") {
        &auth_header[7..]
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let secret = "esta_es_una_clave_muy_secreta_shhh"; 
    
    let mut validation = Validation::new(Algorithm::HS256);
    validation.leeway = 60; 

    match decode::<Claims>(token, &DecodingKey::from_secret(secret.as_bytes()), &validation) {
        Ok(_) => {
            Ok(next.run(req).await)
        }
        Err(err) => {
            println!("❌ ERROR DE AUTH: {:?}", err);
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}


async fn root() -> &'static str {
    "¡Servicio de Inventario (Rust) activo! 🦀"
}

async fn get_inventory() -> impl IntoResponse {
    let inventory = vec![
        Item { id: 101, name: "Teclado Mecánico".to_string(), quantity: 15, status: "En Stock".to_string() },
        Item { id: 102, name: "Monitor 24p".to_string(), quantity: 8, status: "Pocas Unidades".to_string() },
        Item { id: 103, name: "Silla Gamer".to_string(), quantity: 0, status: "Agotado".to_string() },
        Item { id: 104, name: "Mouse RGB".to_string(), quantity: 50, status: "En Stock".to_string() },
    ];

    Json(inventory)
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let protected_routes = Router::new()
        .route("/inventory", get(get_inventory))
        .route_layer(middleware::from_fn(auth_middleware));

    let public_routes = Router::new()
        .route("/", get(root));

    let app = public_routes
        .merge(protected_routes)
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8002));
    println!("🚀 Servicio de Inventario (Rust) escuchando en {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}