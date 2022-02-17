use hex;
use std::collections::HashMap;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::{stdin, stdout};
use std::iter::Iterator;
use libreria_condivisa::*;
use libreria_condivisa::xor_block;

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

        for x in 0..255 {
            let x = x as u8;
            let res = xor_block(&inp, &vec![x]).unwrap();
            if !printable::is_vec_printable(&res){
                continue;
            }
            let res = String::from_utf8(res).unwrap();
            outputFile.write_all(format!("char '{}' res\"{}\"\n",x as char,res).as_bytes());
        }
    }
    print!("finito");
}
