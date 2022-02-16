use hex;
use openssl::base64;
use std::cmp::max;
use std::collections::HashMap;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::{stdin, stdout};
use std::iter::Iterator;

const FILE_NAME: &str = "input.txt";
const LETTER: [char; 26] = [
    'e', 't', 'a', 'o', 'i', 'n', 's', 'h', 'r', 'd', 'l', 'c', 'u', 'm', 'w', 'f', 'g', 'y', 'p',
    'b', 'v', 'k', 'j', 'x', 'q', 'z',
];
const PUNCTUATION: [char; 32] = [
    '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<', '=',
    '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~',
];

fn read_yes_or_no(message: &str) -> i8 {
    print!("{}[Y/n]", message);
    stdout().flush().unwrap();
    let mut input: [u8; 1] = [0; 1];
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    match line.trim() {
        "Y" => 1,
        "n" => 0,
        _ => -1,
    }
}

fn xor(inp1: &Vec<u8>, inp2: &Vec<u8>) -> Vec<u8> {
    let len1 = inp1.len();
    let len2 = inp2.len();
    let lunghezza_massima = max(len1, len2);
    let mut vettore: Vec<u8> = Vec::with_capacity(lunghezza_massima);
    for x in 0..lunghezza_massima {
        vettore.push(inp1[x % len1] ^ inp2[x % len2]);
    }
    vettore
}

fn u8_to_freq(inp: &Vec<u8>) -> HashMap<u8, u8> {
    let mut freq_inp: HashMap<u8, u8> = HashMap::new();

    for x in inp {
        let count = freq_inp.entry(*x).or_insert(0);
        *count += 1;
    }
    freq_inp
}

fn build_frequecy_array(lower: bool, upper: bool, spaces: bool, punctuation: bool) -> Vec<u8> {
    let mut len = 0;
    if lower {
        len += LETTER.len();
    }
    if upper {
        len += LETTER.len();
    }
    if spaces {
        len += 2;
    }

    let mut byte_freq_table: Vec<u8> = Vec::with_capacity(len);
    if spaces {
        byte_freq_table.push('\n' as u8);
        byte_freq_table.push(' ' as u8);
    }
    if lower || upper {
        for current_char in &LETTER {
            if lower {
                byte_freq_table.push(*current_char as u8);
            }
            if upper {
                byte_freq_table.push((*current_char as u8) - 32);
            }
        }
    }
    if punctuation {
        for current_char in &PUNCTUATION {
            byte_freq_table.push(*current_char as u8);
        }
    }

    byte_freq_table
}

fn hamming_distance(inp1: &Vec<u8>, inp2: &Vec<u8>) -> u32 {
    xor(inp1, inp2).iter().map(|curr| curr.count_ones()).sum()
}

fn calculate_hamming_distance_on_block(content: &Vec<u8>, space: u8, competenza: u8) -> f64 {
    let mut count = 0;
    for x in 1..competenza {
        for c in 0..space {
            count += (content[(c) as usize] ^ content[(c + x * space) as usize]).count_ones();
        }
    }
    (count as f64) / (competenza as f64)
}

fn main() {
    let content = fs::read_to_string(FILE_NAME)
        .expect(format!("{}{}{}", "Errore File \"", FILE_NAME, "\" non trovato").as_str());

    let content = content.replace("\n", "");
    let content = base64::decode_block(content.as_str()).unwrap();

    let mut probably_keysize = 0;

    {
        let mut minore: f64 = 9.0;
        for space in 2..30 {
            let temp=calculate_hamming_distance_on_block(&content, space, 4);
            if minore > temp {
                minore=temp;
                probably_keysize=space;
            }
        }
    }
    print!("la key probabile è di lunghezza:{}\n",probably_keysize);

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hamming_distance_test() {
        assert_eq!(
            hamming_distance(
                &Vec::from("this is a test".as_bytes()),
                &Vec::from("wokka wokka!!!".as_bytes())
            ),
            37
        );
    }
}
