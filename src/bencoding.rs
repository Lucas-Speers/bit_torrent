use std::{fmt::Debug, sync::Mutex};

use sha1_smol::Sha1;

use crate::utils;

static INFO_HASH: Mutex<String> = Mutex::new(String::new());

/// Bencoding datatypes
#[derive(PartialEq, Eq, Clone)]
pub(crate) enum DataType {
    Int(u64),
    Str(Vec<u8>),
    List(Vec<DataType>),
    Dict(Vec<(Vec<u8>, DataType)>),
}

impl DataType {
    pub fn get(&self, key: &str) -> Result<DataType, ()> {
        let key = key.chars().map(|c| {c as u8}).collect::<Vec<u8>>();
        let mut output: Result<DataType, ()> = Err(()); 
        if let DataType::Dict(x) = self {
            for item in x {
                if item.0 == key {
                    output = Ok(item.1.clone());
                }
            }
        }

        output
    }

    pub fn get_string(&self) -> Result<String, ()> {
        if let DataType::Str(x) = self {
            return Ok(x.iter().map(|c| {*c as char}).collect());
        } else {
            Err(())
        }
    }

    pub fn get_int(&self) -> Result<u64, ()> {
        if let DataType::Int(x) = self {
            return Ok(*x);
        } else {
            Err(())
        }
    }
}

impl Debug for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let DataType::Int(x) = self {
            write!(f, "Int({x})").unwrap();
        } else if let DataType::Str(x) = self {
            write!(f, "Str(\"").unwrap();
            if x.len() > 100 {
                write!(f, "OVER 100 CHARACTERS STRING OMITTED").unwrap();
            } else {
                for c in x {
                    write!(f, "{}", *c as char).unwrap();
                }
            }
            write!(f, "\")").unwrap();
        } else if let DataType::List(x) = self {
            write!(f, "List(\n").unwrap();
            for item in x {
                write!(f, "{item:?}\n").unwrap();
            }
            write!(f, ")").unwrap();
        } else if let DataType::Dict(x) = self {
            write!(f, "Dict(\n").unwrap();
            for pair in x {
                for c in &pair.0 {
                    write!(f, "{}", *c as char).unwrap();
                }
                write!(f, ": {:?}\n", pair.1).unwrap();
            }
            write!(f, ")").unwrap();
        }
        Ok(())
    }
}

/// returns the number and how many chars it used
fn decode_int(str: &[u8]) -> (u64, usize) {
    let mut chars = str.iter().enumerate(); // (usize, char)
    let mut number: u64 = 0;
    loop {
        let char = chars.next().unwrap();
        if (*char.1 as char == ':') | (*char.1 as char == 'e') {
            return (number, char.0+1);
        }
        else {
            number *= 10;
            number += (char.1 - 48) as u64;
        }
    }
}

/// returns the string and how many chars it used
fn decode_str(str: &[u8]) -> (Vec<u8>, usize) {
    let int = decode_int(str);

    let mut chars = str.iter();
    chars.advance_by(int.1).unwrap();

    let string = chars.take(int.0 as usize).map(|x| *x).collect::<Vec<u8>>();
    let len = string.len();

    (string, int.1+len)
}

fn decoder_with_len(input_str: &[u8]) -> (DataType, usize) {
    match *input_str.iter().next().unwrap() as char {
        'i' => { // integer
            let int = decode_int(&input_str[1..]);
            (DataType::Int(int.0), int.1+1)
        },
        'l' => { // list of elements
            let mut output = Vec::new();
            let mut index = 1;
            loop {
                if *input_str.iter().nth(index).unwrap() as char == 'e' { break; }
                let value = decoder_with_len(&input_str[index..]);
                output.push(value.0);
                index += value.1;
            }
        
            (DataType::List(output), index+1)
        },
        'd' => {
            let mut output = Vec::new();
            let mut index = 1;
            loop {
                if *input_str.iter().nth(index).unwrap() as char == 'e' { break; }
                let key = decode_str(&input_str[index..]);
                index += key.1;
                if key.0 == vec!['p' as u8, 'e' as u8, 'e' as u8, 'r' as u8, 's' as u8] {
                    // hacky workaround
                    
                }
                let value = decoder_with_len(&input_str[index..]);
                if key.0 == vec!['i' as u8, 'n' as u8, 'f' as u8, 'o' as u8] {
                    let data = &input_str[index..index+value.1];
                    *INFO_HASH.lock().unwrap() = utils::url_encode_bytes(&Sha1::from(data).digest().bytes());
                }
                output.push((key.0, value.0));
                index += value.1;
            }
        
            (DataType::Dict(output), index+1)
        },
        '0'..='9' => { // str
            let str = decode_str(input_str);
            (DataType::Str(str.0), str.1)
        },
        _ => {
            panic!("Invalid Bencoding");
        }
    }
}

/// Reads the Bencoding file
pub fn decoder(input_str: &[u8]) -> DataType {
    decoder_with_len(input_str).0
}

pub fn get_info_hash() -> String {
    INFO_HASH.lock().unwrap().to_owned()
}

// pub fn encoder(data: &DataType) -> String {
//     let mut output = String::new();
//     match data {
//         DataType::Int(x) => {
//             output.push('i');
//             output.push_str(&x.to_string());
//             output.push('e');
//         },
//         DataType::Str(x) => {
//             output.push_str(&x.len().to_string());
//             output.push(':');
//             output.push_str(
//                 &x.iter()
//                 .map(|c| {*c as char})
//                 .collect::<String>()
//             )
//         },
//         DataType::List(x) => {
//             output.push('l');
//             for item in x {
//                 output.push_str(&encoder(item));
//             }
//             output.push('e');
//         },
//         DataType::Dict(x) => {
//             output.push('d');
//             for item in x {
//                 output.push_str(&encoder(&DataType::Str(item.0.clone())));
//                 output.push_str(&encoder(&item.1));
//             }
//             output.push('e');
//         },
//     }

//     output
// }