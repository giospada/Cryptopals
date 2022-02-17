use hex;
use std::cmp::max;
use std::collections::HashMap;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::{stdin, stdout};
use std::iter::Iterator;
use libreria_condivisa::*;

const FILE_NAME: &str = "input.txt";

fn main() {
    let content = fs::read_to_string(FILE_NAME)
        .expect(format!("{}{}{}", "Errore File \"", FILE_NAME, "\" non trovato").as_str());

    let key="ICE".as_bytes();

    let content=content.as_bytes();
    let res=xor_block(&key,&content).unwrap();
    print!("{}\n",hex::encode(res));

}
