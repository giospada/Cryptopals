use hex;
use std::fs;
use libreria_condivisa::xor_block;

const FILE_NAME: &str = "input.txt";


fn read_input() -> (Vec<u8>, Vec<u8>) {
    let content = fs::read_to_string(FILE_NAME)
        .expect(format!("{}{}{}", "Errore File \"", FILE_NAME, "\" non trovato").as_str());

    let content = content.split("\n").collect::<Vec<&str>>();

    let inp1 = hex::decode(content[0].trim())
        .expect("Errore nel decriptaggio del file, della prima linea");
    let inp2 = hex::decode(content[1].trim())
        .expect("Errore nel decriptaggio del file, della seconda linea");

    (inp1, inp2)
}

fn main() {
    let (inp1, inp2) = read_input();

    let res = xor_block(&inp1, &inp2).unwrap();

    let res = hex::encode(res);

    print!("Risultato: {}\n", res);
}
