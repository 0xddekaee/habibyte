use habibyte_ledger::{Ledger, Transaction};
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::Filter;

type SharedLedger = Arc<RwLock<Ledger>>;

pub async fn start_api_server(port: u16, ledger: SharedLedger) {
    // Helper buat nyuntikkin state ledger ke dalam handler route.
    let ledger_filter = warp::any().map(move || ledger.clone());

    // GET /health
    // Endpoint buat cek apakah node masih idup atau pingsan.
    let health_route = warp::path("health").map(|| {
        warp::reply::json(&serde_json::json!({
            "status": "ok",
            "system": "Habibyte Node",
            "version": "0.1.0"
        }))
    });

    // GET /blocks
    // Endpoint buat ngambil seluruh history blockchain yang ada di node ini.
    let blocks_route = warp::path("blocks")
        .and(warp::get())
        .and(ledger_filter.clone())
        .then(|ledger: SharedLedger| async move {
            let state = ledger.read().await;
            warp::reply::json(&state.chain)
        });

    // POST /transaction
    // Endpoint buat submit transaksi baru.
    // Contoh: curl -X POST -H "Content-Type: application/json" -d '...' ...
    let transaction_route = warp::path("transaction")
        .and(warp::post())
        .and(warp::body::json())
        .and(ledger_filter.clone())
        .then(|tx: Transaction, ledger: SharedLedger| async move {
            let mut state = ledger.write().await;

            // Di sistem real, harusnya masuk mempool dulu buat divalidasi validator.
            // Tapi buat demo ini, kita langsung auto-mine jadi blok baru aja.
            state.add_block(vec![tx], "API_NODE".to_string());

            warp::reply::json(&serde_json::json!({
                "status": "success",
                "message": "Transaksi diterima dan telah dimining (Mode Demo)"
            }))
        });

    let routes = health_route.or(blocks_route).or(transaction_route);

    // Logging tanda API udah siap melayani request.
    log::info!("Layanan API aktif pada port {}", port);

    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}
