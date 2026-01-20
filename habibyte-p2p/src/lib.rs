use libp2p::identity::Keypair;
use libp2p::{gossipsub, mdns, noise, swarm::NetworkBehaviour, tcp, yamux, Swarm, SwarmBuilder};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::Duration;
use thiserror::Error;
use tokio::io;

#[derive(Error, Debug)]

pub enum P2PError {
    #[error("Gagal setup Swarm: {0}")]
    SetupError(String),
    #[error("Error Transport: {0}")]
    TransportError(#[from] io::Error),
    #[error("Error Behaviour: {0}")]
    BehaviourError(#[from] libp2p::gossipsub::SubscriptionError),
}

// Struct ini menggabungkan beberapa behaviour jaringan jadi satu logika node.
#[derive(NetworkBehaviour)]
pub struct AppBehaviour {
    // Gossipsub dipakai buat nyebarin pesan (misal: blok baru) ke banyak peer sekaligus.
    pub gossipsub: gossipsub::Behaviour,
    // mDNS buat nemuin node lain secara otomatis di jaringan lokal (tanpa config IP manual).
    pub mdns: mdns::tokio::Behaviour,
}

/// Fungsi utama buat membangun P2P Swarm (Network Engine).
/// Kita konfigurasi transport TCP, enkripsi Noise, dan Multiplexing Yamux di sini.
pub fn create_swarm(keys: Keypair, p2p_port: u16) -> Result<Swarm<AppBehaviour>, P2PError> {
    let mut swarm = SwarmBuilder::with_existing_identity(keys)
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new, // Enkripsi lalu lintas network (biar aman dari sadap)
            yamux::Config::default,
        )
        .map_err(|e| P2PError::SetupError(format!("Gagal config TCP/Noise: {}", e)))?
        .with_behaviour(|key| {
            // Setup Gossipsub (Layer Publikasi Pesan)
            // Kita tentuin ID pesan biar kalo ada pesan ganda bisa difilter.
            let message_id_fn = |message: &gossipsub::Message| {
                let mut s = DefaultHasher::new();
                message.data.hash(&mut s);
                gossipsub::MessageId::from(s.finish().to_string())
            };
            let gossipsub_config = gossipsub::ConfigBuilder::default()
                .heartbeat_interval(Duration::from_secs(10)) // Detak jantung cek koneksi tiap 10 detik
                .validation_mode(gossipsub::ValidationMode::Strict)
                .message_id_fn(message_id_fn)
                .build()
                .map_err(|msg| io::Error::new(io::ErrorKind::Other, msg))?;

            let gossipsub = gossipsub::Behaviour::new(
                gossipsub::MessageAuthenticity::Signed(key.clone()),
                gossipsub_config,
            )
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

            // Setup mDNS (Penemuan Peer Lokal)
            let mdns =
                mdns::tokio::Behaviour::new(mdns::Config::default(), key.public().to_peer_id())?;

            Ok(AppBehaviour { gossipsub, mdns })
        })
        .map_err(|e| P2PError::SetupError(e.to_string()))?
        .build();

    // Node bakal 'dengerin' koneksi masuk di semua interface IP (0.0.0.0) pada port yang ditentukan.
    swarm
        .listen_on(format!("/ip4/0.0.0.0/tcp/{}", p2p_port).parse().unwrap())
        .map_err(|e| P2PError::SetupError(format!("Gagal listen di port: {}", e)))?;

    Ok(swarm)
}
