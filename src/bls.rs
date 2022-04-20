use bls12_381::hash_to_curve::{ExpandMsgXmd, HashToCurve, HashToField};
use bls12_381::{G1Affine, G1Projective, G2Affine, G2Projective};
pub fn verify(
    msg: Vec<u8>,
    sign_point: &[u8; 48],
    publickey: &[u8; 96],
    domain: &[u8],
    base_point: &[u8; 96],
) -> bool {
    let mut msg_curve: Vec<G1Affine> = vec![G1Affine::generator()];
    let msg_on_curve =
        <G1Projective as HashToCurve<ExpandMsgXmd<sha2_256::Sha256>>>::hash_to_curve(msg, domain);
    G1Projective::batch_normalize(&vec![msg_on_curve], &mut msg_curve);
    let public = G2Affine::from_compressed(publickey).unwrap();
    let p1 = bls12_381::pairing(&msg_curve[0], &public);
    let sign_point = G1Affine::from_compressed(sign_point).unwrap();
    let base_point = G2Affine::from_compressed(base_point).unwrap();
    let p2 = bls12_381::pairing(&sign_point, &base_point);
    return p1 == p2;
}
