use byteorder::{BigEndian, WriteBytesExt};
pub fn round_to_Bytes(r: u64) -> Vec<u8> {
    let mut wtr = vec![];
    wtr.write_u64::<BigEndian>(r).unwrap();
    wtr
}

pub fn digest_message(msg: &mut Vec<u8>, r: u64) {
    let mut round = round_to_Bytes(r);
    msg.append(&mut round);
}
