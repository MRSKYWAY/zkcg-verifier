#![cfg(feature = "zk-halo2")]

use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;

use halo2_proofs::{
    circuit::Value,
    plonk::{create_proof, keygen_pk, keygen_vk},
    poly::commitment::Params,
    transcript::{Blake2bWrite, Challenge255},
};
use halo2curves::bn256::{Fr as Fp, G1Affine};
use rand::rngs::OsRng;

use circuits::score_circuit::ScoreCircuit;
use zkcg_verifier::backend_halo2::Halo2Backend;
use zkcg_verifier::backend::ProofBackend;
use zkcg_verifier::engine::PublicInputs;

/// Generate a Halo2 proof (no verification)
fn generate_halo2_proof(
    params: &Params<G1Affine>,
    pk: &halo2_proofs::plonk::ProvingKey<G1Affine>,
) -> Vec<u8> {
    let circuit = ScoreCircuit::<Fp> {
        score: Value::known(Fp::from(500)),
        threshold: Value::known(Fp::from(600)),
    };

    let public_inputs = vec![vec![Fp::from(600)]];
    let instances: Vec<&[Fp]> = public_inputs.iter().map(|v| v.as_slice()).collect();
    let all_instances = vec![instances.as_slice()];

    let mut proof_bytes = Vec::new();
    let mut transcript =
        Blake2bWrite::<_, G1Affine, Challenge255<G1Affine>>::init(&mut proof_bytes);

    create_proof(
        params,
        pk,
        &[circuit],
        &all_instances,
        OsRng,
        &mut transcript,
    )
    .unwrap();

    transcript.finalize().to_vec()
}

/// Shared one-time setup
fn setup() -> (Params<G1Affine>, halo2_proofs::plonk::ProvingKey<G1Affine>, Halo2Backend) {
    let k = 9;
    let params = Params::<G1Affine>::new(k);

    let circuit = ScoreCircuit::<Fp> {
        score: Value::known(Fp::from(500)),
        threshold: Value::known(Fp::from(600)),
    };

    let vk = keygen_vk(&params, &circuit).unwrap();
    let pk = keygen_pk(&params, vk.clone(), &circuit).unwrap();

    let backend = Halo2Backend {
        vk,
        params: params.clone(),
    };

    (params, pk, backend)
}

/// ----------------------
/// Benchmark: VERIFY ONLY
/// ----------------------
fn bench_halo2_verify(c: &mut Criterion) {
    let mut group = c.benchmark_group("halo2_verify");

    let (_params, pk, backend) = setup();
    let proof = generate_halo2_proof(&backend.params, &pk);

    let public_inputs = PublicInputs {
        threshold: 600,
        old_state_root: [0u8; 32],
        nonce: 1,
    };

    group.bench_function("verify", |b| {
        b.iter(|| {
            backend
                .verify(&proof, &public_inputs)
                .expect("halo2 verification failed");
        });
    });

    group.finish();
}

/// ----------------------
/// Benchmark: PROVE ONLY
/// ----------------------
fn bench_halo2_prove(c: &mut Criterion) {
    let mut group = c.benchmark_group("halo2_prove");

    // Proving is expensive → tune Criterion
    group.sample_size(10);
    group.warm_up_time(Duration::from_secs(2));
    group.measurement_time(Duration::from_secs(30));

    let (params, pk, _backend) = setup();

    group.bench_function("prove", |b| {
        b.iter(|| {
            let _proof = generate_halo2_proof(&params, &pk);
        });
    });

    group.finish();
}

/// --------------------------------
/// Benchmark: PROVE + VERIFY (E2E)
/// --------------------------------
fn bench_halo2_prove_and_verify(c: &mut Criterion) {
    let mut group = c.benchmark_group("halo2_prove_and_verify");

    group.sample_size(10);
    group.warm_up_time(Duration::from_secs(2));
    group.measurement_time(Duration::from_secs(30));

    let (params, pk, backend) = setup();

    let public_inputs = PublicInputs {
        threshold: 600,
        old_state_root: [0u8; 32],
        nonce: 1,
    };

    group.bench_function("prove_and_verify", |b| {
        b.iter(|| {
            let proof = generate_halo2_proof(&params, &pk);
            backend
                .verify(&proof, &public_inputs)
                .expect("halo2 verification failed");
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_halo2_verify,
    bench_halo2_prove,
    bench_halo2_prove_and_verify
);

criterion_main!(benches);
