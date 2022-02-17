use std::collections::HashMap;

pub fn create_frequency_table(inp: &[u8]) -> HashMap<u8, u8> {
    let mut freq_inp: HashMap<u8, u8> = HashMap::new();

    for x in inp {
        let count = freq_inp.entry(*x).or_insert(0);
        *count += 1;
    }
    freq_inp
}

pub fn build_frequecy_array(lower: bool, upper: bool, spaces: bool,number:bool, punctuation: bool) -> Vec<u8> {
    const LETTER: [char; 26] = [
        'e', 't', 'a', 'o', 'i', 'n', 's', 'h', 'r', 'd', 'l', 'c', 'u', 'm', 'w', 'f', 'g', 'y',
        'p', 'b', 'v', 'k', 'j', 'x', 'q', 'z',
    ];
    const NUMBER: [char; 10] = [
        '0','1','2','3','4','5','6','7','8','9'
    ];
    const PUNCTUATION: [char; 32] = [
        '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<',
        '=', '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~',
    ];

    let mut len = 0;
    if lower {
        len += LETTER.len();
    }
    if upper {
        len += LETTER.len();
    }
    if number {
        len += NUMBER.len();
    }
    if punctuation {
        len += PUNCTUATION.len();
    }
    if spaces {
        len += 2;
    }

    let mut byte_freq_table: Vec<u8> = Vec::with_capacity(len);

    if lower || upper {
        for current_char in LETTER {
            if lower {
                byte_freq_table.push(current_char as u8);
            }
            if upper {
                byte_freq_table.push((current_char as u8) - 32);
            }
        }
    }
    if spaces {
        byte_freq_table.push('\n' as u8);
        byte_freq_table.push(' ' as u8);
    }
    if number {
        for current_char in NUMBER {
            byte_freq_table.push(current_char as u8);
        }
    }
    if punctuation {
        for current_char in PUNCTUATION {
            byte_freq_table.push(current_char as u8);
        }
    }

    byte_freq_table
}
