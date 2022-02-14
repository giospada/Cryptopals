use std::fs;
use hex;
use std::cmp::{max};
use std::collections::HashMap;
use std::io::{stdin,stdout};
use std::io::Write;


const FILE_NAME:&str="input.txt";
const FREQ_TABLE:[char;26]=['e','t','a','o','i','n','s','h','r','d','l','c','u','m','w','f','g','y','p','b','v','k','j','x','q','z'];


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

fn xor(inp1:&Vec<u8>,inp2:&Vec<u8>)-> Vec<u8>{
    let lunghezza_massima=max(inp1.len(),inp2.len());
    let mut vettore:Vec<u8>=Vec::with_capacity(lunghezza_massima);
    for x in 0..lunghezza_massima {
        vettore.push(inp1[x%inp1.len()]^inp2[x%inp2.len()]);
    }
    vettore
}

fn u8_to_freq(inp:&Vec<u8>)->HashMap<u8,u8>{
    let mut freq_inp:HashMap<u8,u8>=HashMap::new();

    for x in inp{
        let count=freq_inp.entry(*x).or_insert(0);
        *count+=1;
    }
    freq_inp
}

fn build_frequecy_array(lower:bool,upper:bool,spaces:bool)->Vec<u8>{
    let mut len=0;
    if lower {len+=FREQ_TABLE.len();}
    if upper {len+=FREQ_TABLE.len();}
    if spaces {len+=2;}

    let mut byte_freq_table:Vec<u8>=Vec::with_capacity(FREQ_TABLE.len());
    if spaces { byte_freq_table.push('\n' as u8);byte_freq_table.push(' ' as u8); }
    for current_char in FREQ_TABLE {
        if lower {byte_freq_table.push(current_char as u8);}
        if upper {byte_freq_table.push((current_char as u8)-32);}
    }

    byte_freq_table
}

fn main() {

    let content=fs::read_to_string(FILE_NAME)
        .expect(format!("{}{}{}","Errore File \"",FILE_NAME,"\" non trovato").as_str());
    
    let content=content.split("\n");
    
    

    let inp=hex::decode(content.trim())
        .expect("Errore nel decriptaggio del file");
    
    
    for x in 0..255 {
        let x=x as u8;
        let res=xor(&inp,&vec![x]);
        let res=String::from_utf8(res);

        if let Err(e) = res {
            print!("Errore nella decodifica con il carattere '{}' passiamo direttamente al prossimo\n",x as char);
            continue;
        }
        let res=res.unwrap();
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
