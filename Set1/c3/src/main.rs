use std::fs;
use hex;
use std::cmp::{min};

const FILE_NAME:&str="input.txt";

fn xor(inp1:&Vec<u8>,inp2:&Vec<u8>)-> Vec<u8>{
    let lunghezza_massima=min(inp1.len(),inp2.len());
    let mut vettore:Vec<u8>=Vec::with_capacity(lunghezza_massima);
    for x in 0..lunghezza_massima {
        vettore.push(inp1[x%inp1.len()]^inp2[x%inp2.len()]);
    }
    vettore
}

fn main() {
    let content=fs::read_to_string(FILE_NAME)
        .expect(format!("{}{}{}","Errore File \"",FILE_NAME,"\" non trovato").as_str());
    

    
    let inp1=hex::decode(content.trim())
        .expect("Errore nel decriptaggio del file");
    

    let res=xor(&inp1,&vec![10]);
    
    let res=hex::encode(res);

    print!("Risultato: {}\n",res);
}
