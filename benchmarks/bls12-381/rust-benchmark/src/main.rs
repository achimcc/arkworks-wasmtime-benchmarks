use sightglass_api as bench;

use ark_bls12_381::{Bls12_381, Fr as BlsFr};
use ark_ff::Fp;
use ark_groth16::Groth16;
use ark_serialize::{CanonicalDeserialize, Compress, Validate};
use ark_snark::SNARK;
use ark_std::{io::Error, test_rng, vec::Vec, UniformRand};
use frame_support::assert_ok;
pub use sp_ark_bls12_381::{
	fr::Fr as BlsFrOptimized, Bls12_381 as Bls12_381_Host, G1Affine as G1AffineBls12_381_Host,
	G1Projective as G1ProjectiveBls12_381_Host, G2Affine as G2AffineBls12_381_Host,
	G2Projective as G2ProjectiveBls12_381_Host, HostFunctions as Bls12_381HostFunctions,
};
pub use sp_ark_models::{pairing::Pairing, short_weierstrass::SWCurveConfig, AffineRepr, Group};

pub struct HostBls12_381 {}

impl Bls12_381HostFunctions for HostBls12_381 {
	fn bls12_381_multi_miller_loop(a: Vec<Vec<u8>>, b: Vec<Vec<u8>>) -> Vec<u8> {
		sp_io::elliptic_curves::bls12_381_multi_miller_loop(a, b)
	}
	fn bls12_381_final_exponentiation(f12: Vec<u8>) -> Vec<u8> {
		sp_io::elliptic_curves::bls12_381_final_exponentiation(f12)
	}
	fn bls12_381_msm_g1(bases: Vec<Vec<u8>>, bigints: Vec<Vec<u8>>) -> Vec<u8> {
		sp_io::elliptic_curves::bls12_381_msm_g1(bases, bigints)
	}
	fn bls12_381_mul_projective_g1(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
		sp_io::elliptic_curves::bls12_381_mul_projective_g1(base, scalar)
	}
	fn bls12_381_mul_affine_g1(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
		sp_io::elliptic_curves::bls12_381_mul_affine_g1(base, scalar)
	}
	fn bls12_381_msm_g2(bases: Vec<Vec<u8>>, bigints: Vec<Vec<u8>>) -> Vec<u8> {
		sp_io::elliptic_curves::bls12_381_msm_g2(bases, bigints)
	}
	fn bls12_381_mul_projective_g2(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
		sp_io::elliptic_curves::bls12_381_mul_projective_g2(base, scalar)
	}
	fn bls12_381_mul_affine_g2(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
		sp_io::elliptic_curves::bls12_381_mul_affine_g2(base, scalar)
	}
}

pub type Bls12_381Optimized = Bls12_381_Host<HostBls12_381>;
pub type G1AffineBls12_381 = G1AffineBls12_381_Host<HostBls12_381>;
pub type G2AffineBls12_381 = G2AffineBls12_381_Host<HostBls12_381>;
pub type G1ProjectiveBls12_381 = G1ProjectiveBls12_381_Host<HostBls12_381>;
pub type G2ProjectiveBls12_381 = G2ProjectiveBls12_381_Host<HostBls12_381>;

pub fn do_pairing_optimized() -> Result<(), Error> {
	let _ = Bls12_381Optimized::multi_pairing(
		[G1AffineBls12_381::generator()],
		[G2AffineBls12_381::generator()],
	);
	Ok(())
}

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
    let result = do_pairing_optimized();
    bench::end();

    eprintln!("[pairing_optimized] returned {:?}", result);
}
