use color_eyre::eyre::{Ok, Result};
use libp2p::{Swarm, noise, ping, swarm::NetworkBehaviour, tcp, yamux};
use std::time::Duration;

pub fn build_swarm() -> Result<Swarm<impl NetworkBehaviour>> {
    let mut swarm = libp2p::SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_behaviour(|_| ping::Behaviour::default())?
        .with_swarm_config(|cfg| cfg.with_idle_connection_timeout(Duration::from_secs(u64::MAX))) // Allows us to observe pings indefinitely.
        .build();
    Ok(swarm)
}
