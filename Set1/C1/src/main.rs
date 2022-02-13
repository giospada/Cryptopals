use std::fs;
use hex;
use openssl::base64;


const FILE_NAME:&str="input.txt";





fn main() {
    let content=fs::read_to_string(FILE_NAME.as_ref())
        .expect(format!("{}{}{}","Errore File \"",FILE_NAME,"\" non trovato"));
    
    let res=hex::decode(content).expect("Errore nel decriptaggio del file");
    
    
    //let res=base64::encode_block(res);
    

    //print!("{}\n",res);
}
