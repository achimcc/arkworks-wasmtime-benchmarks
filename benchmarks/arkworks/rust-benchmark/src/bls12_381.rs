use ark_ff::Fp;
use ark_std::{io::Error, UniformRand};
use ark_ec::{Group, AffineRepr};

pub fn do_pairing() -> Result<(), Error> {
	let _ = ark_bls12_381::Bls12_381::multi_pairing(
		[ark_bls12_381::G1Affine::generator()],
		[ark_bls12_381::G2Affine::generator()],
	);
	Ok(())
}

pub fn do_msm_g1(samples: u32) -> Result<(), Error> {
	let mut rng = test_rng();
	let g = ark_bls12_381::g1::G1Affine::rand(&mut rng);
	let v: Vec<_> = (0..samples).map(|_| g).collect();
	let scalars: Vec<_> = (0..samples)
		.map(|_| {
			<ark_bls12_381::g1::Config as ark_ec::models::CurveConfig>::ScalarField::rand(&mut rng)
		})
		.collect();
	let _out = <ark_bls12_381::g1::Config as SWCurveConfig>::msm(&v[..], &scalars[..]);
	Ok(())
}

pub fn do_msm_g2(samples: u32) -> Result<(), Error> {
	let mut rng = test_rng();
	let g = ark_bls12_381::g2::G2Affine::rand(&mut rng);
	let v: Vec<_> = (0..samples).map(|_| g).collect();
	let scalars: Vec<_> = (0..samples)
		.map(|_| {
			<ark_bls12_381::g2::Config as ark_ec::models::CurveConfig>::ScalarField::rand(&mut rng)
		})
		.collect();
	let _out = <ark_bls12_381::g2::Config as SWCurveConfig>::msm(&v[..], &scalars[..]);
	Ok(())
}

pub fn do_mul_affine_g1() -> Result<(), Error> {
	let _out = <ark_bls12_381::g1::Config as SWCurveConfig>::mul_affine(
		&ark_bls12_381::G1Affine::generator(),
		&[2u64],
	);
	Ok(())
}

pub fn do_mul_projective_g1() -> Result<(), Error> {
	let _out = <ark_bls12_381::g1::Config as SWCurveConfig>::mul_projective(
		&ark_bls12_381::G1Projective::generator(),
		&[2u64],
	);
	Ok(())
}

pub fn do_mul_affine_g2() -> Result<(), Error> {
	let _out = <ark_bls12_381::g2::Config as SWCurveConfig>::mul_affine(
		&ark_bls12_381::G2Affine::generator(),
		&[2u64],
	);
	Ok(())
}

pub fn do_mul_projective_g2() -> Result<(), Error> {
	let _out = <ark_bls12_381::g2::Config as SWCurveConfig>::mul_projective(
		&ark_bls12_381::G2Projective::generator(),
		&[2u64],
	);
	Ok(())
}
