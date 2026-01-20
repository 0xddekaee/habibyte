use libp2p::identity::Keypair;
use libp2p::{gossipsub, mdns, noise, swarm::NetworkBehaviour, tcp, yamux, Swarm, SwarmBuilder};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::Duration;
use thiserror::Error;
use tokio::io;

#[derive(Error, Debug)]
pub enum P2PError {
    #[error("Swarm setup failed: {0}")]
    SetupError(String),
    #[error("Transport error: {0}")]
    TransportError(#[from] io::Error),
    #[error("Behaviour error: {0}")]
    BehaviourError(#[from] libp2p::gossipsub::SubscriptionError),
}

#[derive(NetworkBehaviour)]
pub struct AppBehaviour {
    pub gossipsub: gossipsub::Behaviour,
    pub mdns: mdns::tokio::Behaviour,
}

/// Initializes a new P2P Swarm with Gossipsub and mDNS.
pub fn create_swarm(keys: Keypair, p2p_port: u16) -> Result<Swarm<AppBehaviour>, P2PError> {
    let mut swarm = SwarmBuilder::with_existing_identity(keys)
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )
        .map_err(|e| P2PError::SetupError(format!("TCP/Noise config failed: {}", e)))?
        .with_behaviour(|key| {
            // Setup Gossipsub
            let message_id_fn = |message: &gossipsub::Message| {
                let mut s = DefaultHasher::new();
                message.data.hash(&mut s);
                gossipsub::MessageId::from(s.finish().to_string())
            };
            let gossipsub_config = gossipsub::ConfigBuilder::default()
                .heartbeat_interval(Duration::from_secs(10))
                .validation_mode(gossipsub::ValidationMode::Strict)
                .message_id_fn(message_id_fn)
                .build()
                .map_err(|msg| io::Error::new(io::ErrorKind::Other, msg))?;

            let gossipsub = gossipsub::Behaviour::new(
                gossipsub::MessageAuthenticity::Signed(key.clone()),
                gossipsub_config,
            )
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

            // Setup mDNS
            let mdns =
                mdns::tokio::Behaviour::new(mdns::Config::default(), key.public().to_peer_id())?;

            Ok(AppBehaviour { gossipsub, mdns })
        })
        .map_err(|e| P2PError::SetupError(e.to_string()))?
        .build();

    // Listen on all interfaces
    swarm
        .listen_on(format!("/ip4/0.0.0.0/tcp/{}", p2p_port).parse().unwrap())
        .map_err(|e| P2PError::SetupError(format!("Listen failed: {}", e)))?;

    Ok(swarm)
}
