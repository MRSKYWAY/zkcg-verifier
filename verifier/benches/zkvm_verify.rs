#[cfg(feature = "zk-vm")]

use criterion::{criterion_group, criterion_main, Criterion};
use std::time::{Duration, Instant};

use zkcg_common::{state::ProtocolState, types::Commitment};
use zkcg_verifier::engine::{VerifierEngine, PublicInputs};
use zkcg_verifier::backend_zkvm::ZkVmBackend;

use zkcg_zkvm_host::prove;

/// One-shot measurement of zkVM proving time.
/// This runs ONCE when the benchmark binary starts.
fn measure_prove_once() {
    println!("Measuring zkVM prove() once (not a benchmark)...");
    let start = Instant::now();

    let proof = prove(500, 600, [0u8; 32], 1)
        .expect("zkvm proof generation failed");

    let elapsed = start.elapsed();
    println!(
        "prove() took {:?} (proof size = {} bytes)",
        elapsed,
        proof.len()
    );
}

/// Measures **zkVM verification only**
/// Proof is generated once, outside the benchmark.
/// Expected: nanoseconds–microseconds
fn bench_zkvm_verify(c: &mut Criterion) {
    let mut group = c.benchmark_group("zkvm");

    let public_inputs = PublicInputs {
        threshold: 600,
        old_state_root: [0u8; 32],
        nonce: 1,
    };

    let commitment = Commitment([0u8; 32]);

    // 🔑 Generate proof ONCE
    let proof = prove(500, 600, [0u8; 32], 1)
        .expect("zkvm proof generation failed");

    group.bench_function("verify", |b| {
        b.iter(|| {
            let backend = ZkVmBackend;
            let state = ProtocolState::genesis();
            let mut engine = VerifierEngine::new(state, Box::new(backend));

            engine
                .process_transition(&proof, public_inputs, commitment.clone())
                .expect("zkvm verification failed");
        });
    });

    group.finish();
}

// /// Measures **zkVM prove + verify**
// /// This is expensive, so sample size and timing are tuned.
// /// Expected: seconds per iteration
// fn bench_zkvm_prove_and_verify(c: &mut Criterion) {
//     let mut group = c.benchmark_group("zkvm_prove");

//     // 🔒 Expensive operation → reduce samples
//     group.sample_size(10);
//     group.warm_up_time(Duration::from_secs(2));
//     group.measurement_time(Duration::from_secs(60));

//     let public_inputs = PublicInputs {
//         threshold: 600,
//         old_state_root: [0u8; 32],
//         nonce: 1,
//     };

//     let commitment = Commitment([0u8; 32]);

//     group.bench_function("prove_and_verify", |b| {
//         b.iter(|| {
//             let proof = prove(500, 600, [0u8; 32], 1)
//                 .expect("zkvm proof generation failed");

//             let backend = ZkVmBackend;
//             let state = ProtocolState::genesis();
//             let mut engine = VerifierEngine::new(state, Box::new(backend));

//             engine
//                 .process_transition(&proof, public_inputs, commitment.clone())
//                 .expect("zkvm verification failed");
//         });
//     });

//     group.finish();
// }


#[cfg(feature = "zk-vm")]
pub fn debug_double_prove() {
    use std::time::Instant;

    println!("---- zkVM double prove test ----");

    let t1 = Instant::now();
    let _ = prove(500, 600, [0u8; 32], 1).unwrap();
    println!("first prove took {:?}", t1.elapsed());

    let t2 = Instant::now();
    let _ = prove(500, 600, [0u8; 32], 2).unwrap();
    println!("second prove took {:?}", t2.elapsed());
}
/// Criterion entry point
fn benches(c: &mut Criterion) {
    // 👇 run once before any benchmarks
    measure_prove_once();

    // debug_double_prove()
    bench_zkvm_verify(c);
    // bench_zkvm_prove_and_verify(c);
}

criterion_group!(zkvm_benches, benches);
criterion_main!(zkvm_benches);
