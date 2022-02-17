use std::fs;
use hex;
use std::io::{stdin,stdout};
use std::io::Write;
use libreria_condivisa::{xor_block,printable::*,frequency_table::*};


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

    let freq_map=create_frequency_table(&inp);
    let most_freq=freq_map.iter().max_by_key(|entry|entry.1).unwrap().0;
    
    for x in build_frequecy_array(true,false,true,false,false) {
        let key=x^most_freq;
        let res=xor_block(&inp,&vec![key]).unwrap();

        if !is_vec_printable(&res) {
            println!("skipping '{}'",x as char);
            continue;
        }

        let res=String::from_utf8(res).unwrap();


        print!("decodifica con '{}', res {}",x as char,res);
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
            return;
        }
    }
    print!("Risultato non trovato");
}
