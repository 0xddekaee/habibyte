use clap::Parser;
use habibyte_ledger::Ledger;
use libp2p::{futures::StreamExt, gossipsub, identity, mdns, swarm::SwarmEvent};
use log::info;
use std::sync::Arc;
use tokio::signal;
use tokio::sync::RwLock;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Port untuk P2P networking
    #[arg(long, default_value_t = 6000)]
    p2p_port: u16,

    /// Port untuk API (RPC/REST)
    #[arg(long, default_value_t = 8080)]
    api_port: u16,

    /// Mode Validator (apakah node ini boleh membuat blok)
    #[arg(long, default_value_t = false)]
    validator: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Default log settings
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
    let args = Cli::parse();

    info!("Sistem Node Habibyte memulai inisialisasi...");

    // ---------------------------------------------------------
    // 1. Inisialisasi Ledger (Buku Besar)
    // ---------------------------------------------------------
    // Kita bikin instance ledger baru di memori.
    // Genesis block (blok pertama) otomatis dibuat di sini.
    let ledger = Ledger::new();
    info!(
        "Ledger terinisialisasi. Genesis Hash: {}",
        ledger.chain[0].hash
    );
    
    // Kita bungkus ledger pakai Arc<RwLock> biar aman diakses banyak thread (API & P2P).
    let shared_ledger = Arc::new(RwLock::new(ledger));

    // ---------------------------------------------------------
    // 2. Menjalankan Layanan API
    // ---------------------------------------------------------
    // API jalan di background task (asynchronous) biar gak nge-blok proses utama.
    let api_ledger = shared_ledger.clone();
    let api_port = args.api_port;
    tokio::spawn(async move {
        habibyte_api::start_api_server(api_port, api_ledger).await;
    });

    // ---------------------------------------------------------
    // 3. Menjalankan Layanan P2P (Jaringan)
    // ---------------------------------------------------------
    // Generate identitas kriptografi unik buat node ini.
    let id_keys = identity::Keypair::generate_ed25519();
    let local_peer_id = libp2p::PeerId::from(id_keys.public());
    info!("Identitas Lokal Node (PeerId): {}", local_peer_id);

    // Setup swarm libp2p pake modul eksternal habibyte_p2p.
    let mut swarm = habibyte_p2p::create_swarm(id_keys, args.p2p_port)?;

    // Subscribe ke topik global buat dengerin broadcast blok/transaksi.
    let topic = gossipsub::IdentTopic::new("habibyte-global");
    swarm.behaviour_mut().gossipsub.subscribe(&topic)?;

    info!("Node beroperasi normal. Menunggu sinyal sistem...");
    info!(
        "Layanan P2P aktif pada port {}. API pada port {}",
        args.p2p_port, api_port
    );

    // ---------------------------------------------------------
    // 4. Main Event Loop (Jantung Sistem)
    // ---------------------------------------------------------
    // Loop ini bakal jalan terus buat handle event network atau sinyal shutdown.
    loop {
        tokio::select! {
            // Tangkap sinyal Ctrl+C dari user buat matiin node baik-baik.
            _ = signal::ctrl_c() => {
                info!("Sinyal penghentian diterima. Mematikan layanan...");
                break;
            }
            // Handle event yang masuk dari network (Swarm).
            event = swarm.select_next_some() => {
                match event {
                    SwarmEvent::NewListenAddr { address, .. } => {
                       info!("Mendengarkan koneksi P2P di alamat: {}", address);
                    }
                    // Kalo ada peer baru kedetect lewat mDNS (lokal network).
                    SwarmEvent::Behaviour(habibyte_p2p::AppBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                        for (peer_id, _addr) in list {
                            info!("Peer Baru Ditemukan (mDNS): {}", peer_id);
                            // Langsung masukin ke daftar peer gossipsub biar bisa tukeran data.
                            swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                        }
                    }
                    // Kalo peer ilang/disconnect.
                    SwarmEvent::Behaviour(habibyte_p2p::AppBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
                         for (peer_id, _addr) in list {
                            info!("Peer Terputus (mDNS): {}", peer_id);
                            swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
                        }
                    }
                    // Kalo ada pesan masuk (Blok baru / Transaksi).
                    SwarmEvent::Behaviour(habibyte_p2p::AppBehaviourEvent::Gossipsub(gossipsub::Event::Message { propagation_source: peer_id, message_id: _, message })) => {
                         info!("Pesan diterima dari {}: {:?}", peer_id, String::from_utf8_lossy(&message.data));
                    }
                    _ => {}
                }
            }
        }
    }

    info!("Sistem dimatikan.");
    Ok(())
}
