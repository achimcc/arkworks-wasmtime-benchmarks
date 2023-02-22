use ark_std::io::Error;
use ark_ec::pairing::Pairing;
use ark_ec::{Group, AffineRepr, models::short_weierstrass::SWCurveConfig};

pub fn do_msm_g1(samples: u32) -> Result<(), Error> {
	let g = ark_bw6_761::G1Affine::generator();
	let v: Vec<_> = (0..samples).map(|_| g).collect();
	let scalars: Vec<_> = (0..samples)
		.map(|_| ark_ff::Fp::from(2u64))
		.collect();
	let _out = <ark_bw6_761::g1::Config as SWCurveConfig>::msm(&v[..], &scalars[..]);

	Ok(())
}

pub fn do_msm_g2(samples: u32) -> Result<(), Error> {
	let g = ark_bw6_761::G2Affine::generator();
	let v: Vec<_> = (0..samples).map(|_| g).collect();
	let scalars: Vec<_> = (0..samples)
		.map(|_| ark_ff::Fp::from(2u64))
		.collect();
	let _out = <ark_bw6_761::g2::Config as SWCurveConfig>::msm(&v[..], &scalars[..]);
	Ok(())
}

pub fn do_mul_affine_g1() -> Result<(), Error> {
	let _out = <ark_bw6_761::g1::Config as SWCurveConfig>::mul_affine(
		&ark_bw6_761::G1Affine::generator(),
		&[2u64],
	);
	Ok(())
}

pub fn do_mul_projective_g1() -> Result<(), Error> {
	let _out = <ark_bw6_761::g1::Config as SWCurveConfig>::mul_projective(
		&ark_bw6_761::G1Projective::generator(),
		&[2u64],
	);
	Ok(())
}

pub fn do_mul_affine_g2() -> Result<(), Error> {
	let _out = <ark_bw6_761::g2::Config as SWCurveConfig>::mul_affine(
		&ark_bw6_761::G2Affine::generator(),
		&[2u64],
	);
	Ok(())
}

pub fn do_mul_projective_g2() -> Result<(), Error> {
	let _out = <ark_bw6_761::g2::Config as SWCurveConfig>::mul_projective(
		&ark_bw6_761::G2Projective::generator(),
		&[2u64],
	);
	Ok(())
}

pub fn do_pairing() -> Result<(), Error> {
	let _out = ark_bw6_761::BW6_761::multi_pairing(
		[ark_bw6_761::G1Affine::generator()],
		[ark_bw6_761::G2Affine::generator()],
	);
	Ok(())
}