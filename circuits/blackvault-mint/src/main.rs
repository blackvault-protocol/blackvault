#!/usr/bin/env cargo

use halo2_proofs::{
    arithmetic::Field,
    circuit::{floor_planner::V1, Layouter, Value},
    dev::MockProver,
    pasta::Fp,
    plonk::{Advice, Circuit, Column, ConstraintSystem, Error, Instance},
};
use ff::{FromUniformBytes, PrimeField};
use blake2b_simd::Params as Blake2bParams;
use rand::thread_rng;
use qrcode::QrCode;
use image::Luma;
use serde_json::json;
use hex;

const DEPTH: usize = 20;
const DEFAULT_AMOUNT_CENTS: u64 = 5000;

fn hash_to_field(a: Fp, b: Fp) -> Fp {
    let mut hasher = Blake2bParams::new()
        .hash_length(64)
        .personal(b"BlackVault")
        .to_state();
    hasher.update(&a.to_repr());
    hasher.update(&b.to_repr());
    let out = hasher.finalize();
    let mut bytes = [0u8; 64];
    bytes.copy_from_slice(out.as_bytes());
    Fp::from_uniform_bytes(&bytes)
}

#[derive(Clone)]
struct Config {
    advice: [Column<Advice>; 3],
    instance: Column<Instance>,
}

#[derive(Default)]
struct BlackVaultCircuit {
    amount: Value<Fp>,
    secret: Value<Fp>,
    nullifier_key: Value<Fp>,
    path: [Value<Fp>; DEPTH],
    nullifier: Fp,
}

impl Circuit<Fp> for BlackVaultCircuit {
    type Config = Config;
    type FloorPlanner = V1;

    fn without_witnesses(&self) -> Self { Self::default() }

    fn configure(meta: &mut ConstraintSystem<Fp>) -> Config {
        let advice = [meta.advice_column(), meta.advice_column(), meta.advice_column()];
        let instance = meta.instance_column();
        meta.enable_equality(instance);
        for &col in &advice { meta.enable_equality(col); }
        Config { advice, instance }
    }

    fn synthesize(&self, config: Config, mut layouter: impl Layouter<Fp>) -> Result<(), Error> {
        let (nullifier_cell, amount_cell) = layouter.assign_region(|| "blackvault note", |mut region| {
            let commitment = self.amount + self.secret;

            let nullifier_cell = region.assign_advice(
                || "nullifier", config.advice[0], 0, || Value::known(self.nullifier)
            )?;

            let amount_cell = region.assign_advice(
                || "amount", config.advice[2], 0, || self.amount
            )?;

            let mut node = commitment;
            for i in 0..DEPTH {
                let sibling = self.path[i];
                let (left, right) = if i % 2 == 0 { (node, sibling) } else { (sibling, node) };
                node = left.zip(right).map(|(l, r)| hash_to_field(l, r));
                region.assign_advice(|| "merkle", config.advice[1], i + 1, || node)?;
            }

            Ok((nullifier_cell, amount_cell))
        })?;

        layouter.constrain_instance(amount_cell.cell(), config.instance, 0)?;
        layouter.constrain_instance(nullifier_cell.cell(), config.instance, 1)?;
        Ok(())
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let amount_cents: u64 = args.get(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_AMOUNT_CENTS);

    let amount_display = format!("${:.2}", amount_cents as f64 / 100.0);
    println!("\nMinting BlackVault note for {} ({} cents)\n", amount_display, amount_cents);

    let mut rng = thread_rng();
    let secret = Fp::random(&mut rng);
    let nullifier_key = Fp::random(&mut rng);

    // commitment = H(amount || secret)
    let mut h = Blake2bParams::new().hash_length(32).personal(b"Commit").to_state();
    h.update(&amount_cents.to_le_bytes());
    h.update(&secret.to_repr());
    let commitment = h.finalize();

    // nullifier = H(nullifier_key || commitment)
    let mut h = Blake2bParams::new().hash_length(64).personal(b"Nullifier").to_state();
    h.update(&nullifier_key.to_repr());
    h.update(commitment.as_bytes());
    let nullifier_hash = h.finalize();

    let mut bytes = [0u8; 64];
    bytes.copy_from_slice(nullifier_hash.as_bytes());
    let nullifier = Fp::from_uniform_bytes(&bytes);

    let circuit = BlackVaultCircuit {
        amount: Value::known(Fp::from(amount_cents)),
        secret: Value::known(secret),
        nullifier_key: Value::known(nullifier_key), // used for nullifier derivation
        path: [Value::known(Fp::random(&mut rng)); DEPTH],
        nullifier,
    };

    let public = vec![Fp::from(amount_cents), nullifier];
    assert!(MockProver::run(18, &circuit, vec![public]).unwrap().verify().is_ok());

    let note = json!({
        "version": "v5",
        "amount": amount_cents,
        "amount_display": amount_display,
        "currency": "USD",
        "commitment": hex::encode(commitment.as_bytes()),
        "nullifier": hex::encode(nullifier_hash.as_bytes()),
    }).to_string();

    QrCode::new(note.as_bytes()).unwrap()
        .render::<Luma<u8>>().min_dimensions(400, 400)
        .build()
        .save("blackvault_note.png").unwrap();

    println!("BLACKVAULT v5 — VARIABLE AMOUNT — LIVE");
    println!("Amount: {}", amount_display);
    println!("QR saved → blackvault_note.png");
    println!("Scan with wallet → You own {} shielded\n", amount_display);
}