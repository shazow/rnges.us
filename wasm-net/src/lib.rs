#[cfg(feature = "browser")]
mod browser;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::task::{Context, Poll};
use std::time::Duration;

use futures::prelude::*;
use libp2p::gossipsub::protocol::MessageId;
use libp2p::gossipsub::{GossipsubEvent, GossipsubMessage, Topic};
use libp2p::wasm_ext;
use libp2p::{gossipsub, identity, PeerId};
use libp2p::{mplex, secio, yamux, Transport};

#[cfg(not(target_os = "unknown"))]
use libp2p::{dns, tcp, websocket};

#[cfg(not(target_os = "unknown"))]
use async_std::{io, task};

use libp2p::core::{self, transport::OptionalTransport};

// This is lifted from the rust libp2p-rs gossipsub and massaged to work with wasm.
// The "glue" to get messages from the browser injected into this service isn't done yet.
pub fn service(
    wasm_external_transport: Option<wasm_ext::ExtTransport>,
    dial: Option<String>,
) -> impl Future<Output = ()> {
    // Create a random PeerId
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local peer id: {:?}", local_peer_id);

    let transport = if let Some(t) = wasm_external_transport {
        OptionalTransport::some(t)
    } else {
        OptionalTransport::none()
    };

    #[cfg(not(target_os = "unknown"))]
    let transport = transport.or_transport({
        let desktop_trans = tcp::TcpConfig::new();
        let desktop_trans =
            websocket::WsConfig::new(desktop_trans.clone()).or_transport(desktop_trans);
        OptionalTransport::some(
            if let Ok(dns) = dns::DnsConfig::new(desktop_trans.clone()) {
                dns.boxed()
            } else {
                desktop_trans.map_err(dns::DnsErr::Underlying).boxed()
            },
        )
    });

    let transport = transport
        .upgrade(core::upgrade::Version::V1)
        .authenticate(secio::SecioConfig::new(local_key))
        .multiplex(core::upgrade::SelectUpgrade::new(
            yamux::Config::default(),
            mplex::MplexConfig::new(),
        ))
        .map(|(peer, muxer), _| (peer, core::muxing::StreamMuxerBox::new(muxer)))
        .timeout(std::time::Duration::from_secs(20));

    // Create a Gossipsub topic
    let topic = Topic::new("rnges.us".into());

    // Create a Swarm to manage peers and events
    let mut swarm = {
        // to set default parameters for gossipsub use:
        // let gossipsub_config = gossipsub::GossipsubConfig::default();

        // To content-address message, we can take the hash of message and use it as an ID.
        let message_id_fn = |message: &GossipsubMessage| {
            let mut s = DefaultHasher::new();
            message.data.hash(&mut s);
            MessageId(s.finish().to_string())
        };

        // set custom gossipsub
        let gossipsub_config = gossipsub::GossipsubConfigBuilder::new()
            .heartbeat_interval(Duration::from_secs(10))
            .message_id_fn(message_id_fn) // content-address messages. No two messages of the
            //same content will be propagated.
            .build();
        // build a gossipsub network behaviour
        let mut gossipsub = gossipsub::Gossipsub::new(local_peer_id.clone(), gossipsub_config);
        gossipsub.subscribe(topic.clone());
        libp2p::Swarm::new(transport, gossipsub, local_peer_id)
    };

    // Listen on all interfaces and whatever port the OS assigns.  Websockt can't receive incoming connections
    // on browser (oops?)
    #[cfg(not(target_os = "unknown"))]
    libp2p::Swarm::listen_on(&mut swarm, "/ip4/0.0.0.0/tcp/0/ws".parse().unwrap()).unwrap();

    // Reach out to another node if specified
    if let Some(to_dial) = dial {
        let dialing = to_dial.clone();
        match to_dial.parse() {
            Ok(to_dial) => match libp2p::Swarm::dial_addr(&mut swarm, to_dial) {
                Ok(_) => println!("Dialed {:?}", dialing),
                Err(e) => println!("Dial {:?} failed: {:?}", dialing, e),
            },
            Err(err) => println!("Failed to parse address to dial: {:?}", err),
        }
    }

    // Read full lines from stdin (Disable for wasm, there is no stdin)
    #[cfg(not(target_os = "unknown"))]
    let mut stdin = io::BufReader::new(io::stdin()).lines();

    let mut listening = false;
    future::poll_fn(move |cx: &mut Context| {
        #[cfg(not(target_os = "unknown"))]
        loop {
            match stdin.try_poll_next_unpin(cx) {
                Poll::Ready(Some(Ok(line))) => swarm.publish(&topic, line.as_bytes()),
                Poll::Ready(Some(Err(_))) => panic!("Stdin errored"),
                Poll::Ready(None) => panic!("Stdin closed"),
                Poll::Pending => break,
            };
        }

        loop {
            match swarm.poll_next_unpin(cx) {
                Poll::Ready(Some(gossip_event)) => match gossip_event {
                    GossipsubEvent::Message(peer_id, id, message) => log::info!(
                        "Got message: {} with id: {} from peer: {:?}",
                        String::from_utf8_lossy(&message.data),
                        id,
                        peer_id
                    ),
                    _ => {}
                },
                Poll::Ready(None) | Poll::Pending => break,
            }
        }

        if !listening {
            for addr in libp2p::Swarm::listeners(&swarm) {
                println!("Listening on {:?}", addr);
                listening = true;
            }
        }

        Poll::Pending
    })
}
