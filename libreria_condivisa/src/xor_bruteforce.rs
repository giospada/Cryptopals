use super::*;

pub fn decrypt_one_char_xor<T: Fn(&[u8]) -> bool>(
    inp: &[u8],
    x: u8,
    is_accetable: &T,
) -> Option<Vec<u8>> {
    let res = xor_block(inp, &vec![x]).unwrap();
    if !is_accetable(&res) {
        None
    } else {
        Some(res)
    }
}

pub fn try_all_chars_xor<T, F, C>(
    inp: &[u8],
    to_try: F,
    is_accetable: &T,
    mut exec_on_succesful:C,
) where
    T: Fn(&[u8]) -> bool,
    F: Iterator<Item = u8>,
    C: FnMut(u8, &Vec<u8>) -> bool,
{
    for x in to_try {
        let res = decrypt_one_char_xor(&inp, x, is_accetable);
        if let Some(p) = res {
            if exec_on_succesful(x, &p) {
                break;
            }
        }
    }
}
