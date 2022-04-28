use bls12_381::hash_to_curve::{ExpandMsgXmd, HashToCurve, HashToField};
use bls12_381::{G1Affine, G1Projective, G2Affine, G2Projective};
pub fn verify(
    msg: Vec<u8>,
    sign_point: &[u8; 48],
    publickey: &[u8; 96],
    domain: &[u8],
    base_point: &[u8; 96],
) -> Result<bool, String> {
    let mut msg_curve: Vec<G1Affine> = vec![G1Affine::generator()];
    let msg_on_curve =
        <G1Projective as HashToCurve<ExpandMsgXmd<sha2_256::Sha256>>>::hash_to_curve(msg, domain);
    G1Projective::batch_normalize(&vec![msg_on_curve], &mut msg_curve);
    let public = G2Affine::from_compressed(publickey);
    if public.is_none().unwrap_u8() == 1 {
        return Err("Invalid pulickey".to_string());
    }
    let p1 = bls12_381::pairing(&msg_curve[0], &public.unwrap());
    let sign_point = G1Affine::from_compressed(sign_point);
    if sign_point.is_none().unwrap_u8() == 1 {
        return Err("Invalid sign_point".to_string());
    }
    let base_point = G2Affine::from_compressed(base_point);
    if base_point.is_none().unwrap_u8() == 1 {
        return Err("Invalid base_point".to_string());
    }
    let p2 = bls12_381::pairing(&sign_point.unwrap(), &base_point.unwrap());
    return Ok(p1 == p2);
}
