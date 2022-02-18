use std::fs;
use hex;
use std::io::{stdin,stdout};
use std::io::Write;
use libreria_condivisa::*;


const FILE_NAME:&str="input.txt";


fn read_yes_or_no(message:&str)-> i8{
    print!("{}[Y/n]",message);
    stdout().flush().unwrap();
    let mut input:[u8;1]=[0;1];
    let mut line=String::new();
    stdin().read_line(&mut line).unwrap(); 
    match (line.trim()) {
        "Y"=>{1} ,
        "n"=>{0},
        _ =>{-1}
    }
}

fn read_input()->Vec<u8>{
    let content=fs::read_to_string(FILE_NAME)
        .expect(format!("{}{}{}","Errore File \"",FILE_NAME,"\" non trovato").as_str());
    
    let inp=hex::decode(content.trim())
        .expect("Errore nel decriptaggio del file");
    inp

}

fn main() {

    let inp=read_input();

    let freq_map=frequency_table::create_frequency_table(&inp);
    let most_freq=freq_map.iter().max_by_key(|entry|entry.1).unwrap().0;
    
    let possible:Vec<u8>=frequency_table::build_frequecy_array(true,false,true,false,false).iter().map(|val|val^most_freq).collect();

    xor_bruteforce::try_all_chars_xor(&inp,possible.into_iter(), &printable::is_vec_printable,&mut |chr:u8,decrypted_message:&Vec<u8>|-> bool {

        let res=String::from_utf8(decrypted_message.to_vec()).unwrap();

        print!("decodifica con '{}', res {}",chr as char,res);
        let mut corretto=false;
        loop {
            match (read_yes_or_no("\nVa bene (Y) o vuoi continuare con il prossimo carattere (n)?")){
                -1=>{continue},
                1=>{corretto=true}
                _=>{corretto=false}
            }
            break;
        }
        if corretto {
            print!("Risultato: {}\n",res);
            true
        }else{
            false
        }
    });

}
