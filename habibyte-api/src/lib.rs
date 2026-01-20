use habibyte_ledger::{Ledger, Transaction};
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::Filter;

type SharedLedger = Arc<RwLock<Ledger>>;

pub async fn start_api_server(port: u16, ledger: SharedLedger) {
    // Helper to pass ledger state to filters
    let ledger_filter = warp::any().map(move || ledger.clone());

    // GET /health
    let health_route = warp::path("health").map(|| {
        warp::reply::json(&serde_json::json!({
            "status": "ok",
            "system": "Habibyte Node",
            "version": "0.1.0"
        }))
    });

    // GET /blocks
    let blocks_route = warp::path("blocks")
        .and(warp::get())
        .and(ledger_filter.clone())
        .then(|ledger: SharedLedger| async move {
            let state = ledger.read().await;
            warp::reply::json(&state.chain)
        });

    // POST /transaction
    // Example: curl -X POST -H "Content-Type: application/json" -d '...' ...
    let transaction_route = warp::path("transaction")
        .and(warp::post())
        .and(warp::body::json())
        .and(ledger_filter.clone())
        .then(|tx: Transaction, ledger: SharedLedger| async move {
            let mut state = ledger.write().await;

            // In a real system you would validate and add to mempool.
            // For now, we'll just auto-mine it into a new block for demo purposes.
            state.add_block(vec![tx], "API_NODE".to_string());

            warp::reply::json(&serde_json::json!({
                "status": "success",
                "message": "Transaction added and mined (Demo Mode)"
            }))
        });

    let routes = health_route.or(blocks_route).or(transaction_route);

    // Log startup
    // We use println here or log crate. main.rs initializes log, so log crate would work but println is fine for start msg.
    log::info!("Layanan API aktif pada port {}", port);

    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}
