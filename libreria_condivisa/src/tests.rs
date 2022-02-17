use super::*;

#[test]
fn xor_test() {
    let i1 = vec![
        28, 1, 17, 0, 31, 1, 1, 0, 6, 26, 2, 75, 83, 83, 80, 9, 24, 28,
    ];
    let i2 = vec![
        104, 105, 116, 32, 116, 104, 101, 32, 98, 117, 108, 108, 39, 115, 32, 101, 121, 101,
    ];
    let res = vec![
        116, 104, 101, 32, 107, 105, 100, 32, 100, 111, 110, 39, 116, 32, 112, 108, 97, 121,
    ];
    assert_eq!(xor_block(&i1, &i2), Ok(res));
}
#[test]
fn hamming_distance_test() {
    assert_eq!(
        hamming_distance(
            &Vec::from("this is a test".as_bytes()),
            &Vec::from("wokka wokka!!!".as_bytes())
        )
        .unwrap(),
        37
    );
}
#[test]
fn create_frequency_table_test() {
    use std::collections::HashMap;

    let mut res = HashMap::new();
    res.insert(10, 2);
    res.insert(20, 1);
    assert_eq!(frequency_table::create_frequency_table(&[10, 10, 20]), res);
}

#[test]
fn printable_test() {
    assert_eq!(printable::is_u8_printable(&('a' as u8)),true);
    assert_eq!(printable::is_u8_printable(&(1 as u8)),false);
    let vect:Vec<u8>=vec![67, 111, 111, 107, 105, 110, 103, 32, 77, 67, 39, 115, 32, 108, 105, 107, 101, 32, 97, 32, 112, 111, 117, 110, 100, 32, 111, 102, 32, 98, 97, 99, 111, 110];
    assert_eq!(printable::is_vec_printable(&vect),true);
}