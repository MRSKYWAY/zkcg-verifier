use std::time::Instant;

use rand::thread_rng;
use rand_distr::{Distribution, Normal};

use halo2_proofs::{
    arithmetic::Field,
    circuit::Value,
    plonk::{create_proof, keygen_pk, keygen_vk},
    poly::commitment::Params,
    transcript::{Blake2bWrite, Challenge255},
};
use halo2curves::bn256::{Fr, G1Affine};
use rand::rngs::OsRng;

use circuits::score_circuit::ScoreCircuit;
use zkcg_verifier::{
    backend::ProofBackend,
    backend_halo2::Halo2Backend,
    engine::PublicInputs,
};

fn main() {
    let num_loans = 1_000;
    let threshold = 600u64;

    // --- Generate synthetic scores ---
    let mut rng = thread_rng();
    let normal = Normal::new(715.0, 100.0).unwrap();

    let scores: Vec<u64> = (0..num_loans)
        .map(|_| {
            let s: f64 = normal.sample(&mut rng);
            s.clamp(300.0, 850.0) as u64
        })
        .collect();

    // --- CRS / params ---
    let k = 9;
    let params: Params<G1Affine> = Params::new(k);

    // --- Verifier backend (shared VK) ---
    let dummy = ScoreCircuit::<Fr> {
        score: Value::known(Fr::ZERO),
        threshold: Value::known(Fr::ZERO),
    };
    let vk = keygen_vk(&params, &dummy).unwrap();
    let backend = Halo2Backend {
        vk,
        params: params.clone(),
    };

    println!("DeFi Lending Simulation (Halo2)");
    println!("Loans evaluated: {}", num_loans);

    // --- PROVING ---
    let prove_start = Instant::now();

    let proofs: Vec<Vec<u8>> = scores
        .iter()
        .map(|&score| generate_proof(score, threshold, &params))
        .collect();

    let prove_time = prove_start.elapsed();

    // --- VERIFICATION ---
    let verify_start = Instant::now();

    let approvals = proofs
        .iter()
        .filter(|proof| {
            let inputs = PublicInputs {
                threshold,
                old_state_root: [0u8; 32], // unused by Halo2 backend
                nonce: 1,                  // unused by Halo2 backend
            };
            backend.verify(proof, &inputs).is_ok()
        })
        .count();

    let verify_time = verify_start.elapsed();
    let total_time = prove_time + verify_time;

    println!(
        "Approvals: {} ({:.1}%)",
        approvals,
        approvals as f64 / num_loans as f64 * 100.0
    );

    println!("\nHalo2 Performance (real execution):");
    println!("- Prove total:  {:?}", prove_time);
    println!("- Verify total: {:?}", verify_time);
    println!(
        "- Throughput:   {:.2} TPS",
        num_loans as f64 / total_time.as_secs_f64()
    );
}

fn generate_proof(
    score: u64,
    threshold: u64,
    params: &Params<G1Affine>,
) -> Vec<u8> {
    let circuit = ScoreCircuit::<Fr> {
        score: Value::known(Fr::from(score)),
        threshold: Value::known(Fr::from(threshold)),
    };

    let vk = keygen_vk(params, &circuit).unwrap();
    let pk = keygen_pk(params, vk, &circuit).unwrap();

    let public_inputs = vec![vec![Fr::from(threshold)]];
    let instance_slices: Vec<&[Fr]> =
        public_inputs.iter().map(|v| v.as_slice()).collect();
    let all_instances: Vec<&[&[Fr]]> =
        vec![instance_slices.as_slice()];

    let mut transcript =
        Blake2bWrite::<_, G1Affine, Challenge255<G1Affine>>::init(Vec::new());

    create_proof(
        params,
        &pk,
        &[circuit],
        &all_instances,
        OsRng,
        &mut transcript,
    )
    .unwrap();

    transcript.finalize()
}
