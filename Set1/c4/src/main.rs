use hex;
use libreria_condivisa::*;
use std::collections::HashMap;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::{stdin, stdout};
use std::iter::Iterator;

const FILE_NAME: &str = "input.txt";

fn main() {
    let content = fs::read_to_string(FILE_NAME)
        .expect(format!("{}{}{}", "Errore File \"", FILE_NAME, "\" non trovato").as_str());

    let content: Vec<&str> = content.split("\n").collect();
    let mut outputFile = OpenOptions::new()
        .write(true)
        .append(true)
        .create_new(true)
        .open("output.txt")
        .unwrap();

    for (index, row) in content.iter().enumerate() {
        let inp = hex::decode(row.trim()).expect("Errore nel decriptaggio della linea");
        outputFile.write_all(format!("___ line {} ___\n", index).as_bytes());
        let possible_key=0..255;

        xor_bruteforce::try_all_chars_xor(&inp,possible_key, &printable::is_vec_printable,false,&mut |chr:u8,decrypted_message:&Vec<u8>|-> bool {
            outputFile.write_all(format!(" char '{}' decrypted:\"{}\" \n", chr as char,String::from_utf8(decrypted_message.to_vec()).unwrap()).as_bytes());
            false
        });

    }
    print!("finito");
}
