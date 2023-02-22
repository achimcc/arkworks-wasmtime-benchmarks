#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
mod bls12_377;
mod bls12_381;
mod bw6_761;
mod ed_on_bls12_377;
mod ed_on_bls12_381;
mod groth16;

fn main() {
    let buffer = if std::env::var("WASM_BENCH_USE_SMALL_WORKLOAD").is_ok() {
        eprintln!("[blake3] hashing ./small.input");
        std::fs::read("./small.input").unwrap()
    } else {
        eprintln!("[blake3] hashing ./default.input");
        std::fs::read("./default.input").unwrap()
    };
    eprintln!("[blake3] input size = {}", buffer.len());

    bench::start();
    let result = bls12_381::do_pairing();
    bench::end();

    bench::start();
    let result = bls12_377::do_pairing();
    bench::end();

    bench::start();
    let result = bw6_761::do_pairing();
    bench::end();

    bench::start();
    let result = ed_on_bls12_381::do_mul_affine_sw();
    bench::end();

    bench::start();
    let result = ed_on_bls12_377::do_mul_affine();
    bench::end();

    bench::start();
    let result = groth16::do_verify_groth16();
    bench::end();
}
