use axum::{routing::get, Router, Json};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use serde::Serialize;

#[derive(Serialize)]
struct InventoryItem {
    product_id: i32,
    stock: i32,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/inventory", get(get_inventory))
        .route("/", get(|| async { "API de Inventario Activa 🦀" }))
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("Servicio escuchando en {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Función que devuelve los datos
async fn get_inventory() -> Json<Vec<InventoryItem>> {
    let inventory = vec![
        InventoryItem { product_id: 1, stock: 5 },    
        InventoryItem { product_id: 2, stock: 50 },   
        InventoryItem { product_id: 3, stock: 15 },   
        InventoryItem { product_id: 4, stock: 10 },   
        InventoryItem { product_id: 5, stock: 7 },    
    ];
    Json(inventory)
}