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



fn calculate_hamming_distance_on_block(content: &Vec<u8>, space: u8, competenza: u8) -> f64 {
    let mut count = 0;
    for x in 1..competenza {
        for c in 0..space {
            count += (content[(c) as usize] ^ content[(c + x * space) as usize]).count_ones();
        }
    }
    (count as f64) / (space as f64)
}


fn main() {
    let content = fs::read_to_string(FILE_NAME)
        .expect(format!("{}{}{}", "Errore File \"", FILE_NAME, "\" non trovato").as_str());

    let content = content.replace("\n", "");
    let content = base64::decode_block(content.as_str()).unwrap();

    let probably_key_len;
    {
        let mut all_distance:Vec<(f64,u8)>=Vec::new();
        for space in 2..30 {
            let temp=calculate_hamming_distance_on_block(&content, space, 4);
            all_distance.push((temp,space))
        }
        all_distance.sort_by(|a,b|a.0.partial_cmp(&b.0).unwrap());
        probably_key_len=all_distance[0].1;
        print!("{:?}",all_distance);
    }

    print!("la key probabile è di lunghezza:{}\n",probably_key_len);

}

