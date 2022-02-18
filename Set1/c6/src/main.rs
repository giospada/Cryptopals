use libreria_condivisa::*;
use openssl::base64;
use std::fs;
use std::io::Write;
use std::io::{stdin, stdout};

const FILE_NAME: &str = "input.txt";

fn print_and_read(message: &str) -> String {
    print!("{}", message);
    stdout().flush().unwrap();
    let mut input: [u8; 1] = [0; 1];
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    String::from(line.trim())
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

fn probably_key(content: &Vec<u8>) -> u8 {
    let probably_key_len;
    let mut all_distance: Vec<(f64, u8)> = Vec::new();
    for space in 2..30 {
        let temp = calculate_hamming_distance_on_block(&content, space, 4);
        all_distance.push((temp, space));
    }
    all_distance.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    probably_key_len = all_distance[0].1;
    probably_key_len
}

fn read_input() -> Vec<u8> {
    let content = fs::read_to_string(FILE_NAME)
        .expect(format!("{}{}{}", "Errore File \"", FILE_NAME, "\" non trovato").as_str());

    let content = content.replace("\n", "");
    let content = base64::decode_block(content.as_str()).unwrap();
    content
}

fn print_char(chr: u8) -> Option<char> {
    if printable::is_u8_printable(&chr) && !vec!['\n', '\r', '\t'].contains(&(chr as char)) {
        Some(chr as char)
    } else {
        None
    }
}

fn print_content(keys: &Vec<u8>, content: &Vec<u8>, current_key: u8) {
    let current_key = current_key as usize;
    let mut i = 0;
    let mut all_printable=true;
    while i < content.len() {
        let to_print = print_char(content[i]);
        if let Some(printable_char) = to_print {
            if i % keys.len() == current_key {
                print!("\x1b[32m{}\x1b[0m", printable_char);
            } else {
                print!("{}", printable_char);
            }
        } else {
            print!("\x1b[91m#\x1b[0m");
            all_printable&=i % keys.len() != current_key ;
        }
        i += 1;
    }
    print!("\nsono tutte printabili ({})\n",all_printable);
}

fn main() {
    let content = read_input();
    let probably_key_len = probably_key(&content);

    print!("la key probabile è di lunghezza:{}\n", probably_key_len);

    let mut keys: Vec<u8> = vec![0; probably_key_len as usize];

    let mut current_key = 0;
    let mut add=0;
    loop {
        if add!=0 {
            keys[current_key]+=add;
            add=0;
        }
        print!("Current key:{}\n",current_key);
        print_content(&keys, &content, current_key as u8);
        match print_and_read("(l:prossima key,h:prev key,j:next decode,k:prev decode)\n").as_str() {
            "l" => { current_key+=1; }
            "h" => { current_key-=1; }
            "j" => { add+=1;}
            "k" => { add-=1;}
            _ => {}
        }
        current_key=current_key%(keys.len());
    }
}
