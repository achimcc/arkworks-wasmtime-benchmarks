#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;

use ark_std::io::Error;
use ark_ec::{AffineRepr, pairing::Pairing};

pub fn do_pairing() -> Result<(), Error> {
	let _ = ark_bls12_381::Bls12_381::multi_pairing(
		[ark_bls12_381::G1Affine::generator()],
		[ark_bls12_381::G2Affine::generator()],
	);
	Ok(())
}

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
    let result = do_pairing();
    bench::end();

    eprintln!("[pairing_optimized] returned {:?}", result);
}
