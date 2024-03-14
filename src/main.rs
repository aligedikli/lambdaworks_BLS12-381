use lambdaworks_math::cyclic_group::IsGroup;
use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381::curve::BLS12381Curve;
use lambdaworks_math::elliptic_curve::short_weierstrass::point::ShortWeierstrassProjectivePoint;
use lambdaworks_math::elliptic_curve::traits::IsEllipticCurve;
use lambdaworks_math::traits::{AsBytes, ByteConversion};
use lambdaworks_math::unsigned_integer::element::U256;

pub fn compute_public_key(secret_key: U256) -> ShortWeierstrassProjectivePoint<BLS12381Curve> {
    let bls_curve_generator = BLS12381Curve::generator();
    let public_key = bls_curve_generator.operate_with_self(secret_key);

    public_key
}

fn main() {
    let secret_key = U256::from_hex_unchecked("6C616D6264617370");

    let public_key = compute_public_key(secret_key);

    let public_key_bytes = public_key.as_bytes();

    let public_key_hex = U256::from_bytes_be(&public_key_bytes).expect("Public key conversion failed").to_hex();


    println!("Public key computed from {:?} is: {:?}", secret_key.to_hex(), public_key_hex); // EFC2D10AD531CEBF2B8C7B4325BC93ED91E6477D260304C1F9ECC7BA0E6F5711
    }
