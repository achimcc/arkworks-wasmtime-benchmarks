use ark_ec::{CurveConfig, Group, AffineRepr, short_weierstrass::SWCurveConfig};
use ark_std::io::Error;

pub fn do_msm_sw(samples: u32) -> Result<(), Error> {
	let g = ark_ed_on_bls12_381::SWAffine::generator();
	let v: Vec<_> = (0..samples).map(|_| g).collect();
	let scalars: Vec<_> = (0..samples)
		.map(|_| ark_ff::Fp::from(2u64))
		.collect();
	let _out = <ark_ed_on_bls12_381::EdwardsConfig as SWCurveConfig>::msm(&v[..], &scalars[..]);
	Ok(())
}

pub fn do_msm_te(samples: u32) -> Result<(), Error> {
	let g = ark_ed_on_bls12_381::EdwardsAffine::generator();
	let v: Vec<_> = (0..samples).map(|_| g).collect();
	let scalars: Vec<_> = (0..samples)
		.map(|_| ark_ff::Fp::from(2u64))
		.collect();
	let _out = <ark_ed_on_bls12_381::JubjubConfig as ark_ec::twisted_edwards::TECurveConfig>::msm(
		&v[..],
		&scalars[..],
	);
	Ok(())
}

pub fn do_mul_affine_sw() -> Result<(), Error> {
	let _out = <ark_ed_on_bls12_381::EdwardsConfig as SWCurveConfig>::mul_affine(
		&ark_ed_on_bls12_381::SWAffine::generator(),
		&[2u64],
	);
	Ok(())
}

pub fn do_mul_affine_te() -> Result<(), Error> {
	let _out =
		<ark_ed_on_bls12_381::EdwardsConfig as SWCurveConfig>::mul_affine(
			&ark_ed_on_bls12_381::SWAffine::generator(),
			&[2u64],
		);
	Ok(())
}

pub fn do_mul_projective_sw() -> Result<(), Error> {
	let _out =
		<ark_ed_on_bls12_381::EdwardsConfig as SWCurveConfig>::mul_projective(
			&ark_ed_on_bls12_381::SWProjective::generator(),
			&[2u64],
		);
	Ok(())
}

pub fn do_mul_projective_te() -> Result<(), Error> {
	let _out = <ark_ed_on_bls12_381::EdwardsConfig as ark_ec::twisted_edwards::TECurveConfig>::mul_projective(
		&ark_ed_on_bls12_381::EdwardsProjective::generator(),
		&[2u64],
	);
	Ok(())
}
