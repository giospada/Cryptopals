use std::fs;
use hex;
use openssl::base64;

const FILE_NAME:&str="input.txt";

fn main() {
    let content=fs::read_to_string(FILE_NAME)
        .expect(format!("{}{}{}","Errore File \"",FILE_NAME,"\" non trovato").as_str());
    
    let res=hex::decode(content.trim())
        .expect("Errore nel decriptaggio del file");
    
    let res=base64::encode_block(&res);
    

    print!("{}\n",res);
}
