use axum::{routing::post, Router};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

use zkcg_verifier::engine::VerifierEngine;
use zkcg_common::state::ProtocolState;

use api::handler::{submit_proof, AppState};


// Example future switch:
// let backend: Box<dyn ProofBackend> = match backend_type {
//     "stub" => Box::new(StubBackend::default()),
//     "halo2" => Box::new(Halo2Backend),
//     "zkvm" => Box::new(ZkVmBackend),
//     _ => panic!("unknown backend"),
// };

#[tokio::main]
async fn main() {
    #[cfg(feature = "zk-halo2")]
    let backend = Box::new(verifier::backend_halo2::Halo2Backend);

    #[cfg(not(feature = "zk-halo2"))]
    let backend = Box::new(zkcg_verifier::backend_stub::StubBackend::default());

    let engine = VerifierEngine::new(
        ProtocolState::genesis(),
        backend,
    );


    let state = AppState {
        engine: Arc::new(Mutex::new(engine)),
    };

    let app = Router::new()
        .route("/v1/submit-proof", post(submit_proof))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("ZKCG API listening on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}
