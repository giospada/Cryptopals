use std::cmp::max;

pub fn xor_block(first: &[u8], second: &[u8]) -> Result<Vec<u8>, &'static str> {
    let first_len = first.len();
    let second_len = second.len();
    if first_len == 0 || second_len == 0 {
        Err("Error one of the array of xor function has length of 0")
    } else {
        let maximum_size = max(first_len, second_len);
        let mut result = Vec::with_capacity(maximum_size);
        for i in 0..maximum_size {
            result.push(first[i % first_len] ^ second[i % second_len]);
        }
        Ok(result)
    }
}
pub fn hamming_distance(first: &[u8], second: &[u8]) -> Result<u32, &'static str> {
    Ok(xor_block(first, second)?
        .iter()
        .map(|xored_byte| xored_byte.count_ones())
        .sum())
}

pub mod frequency_table;
pub mod printable;

#[cfg(test)]
mod tests ;