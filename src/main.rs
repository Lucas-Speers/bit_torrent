#![feature(iter_advance_by)]

use std::{fs::{self, File}, io::Read};

/// Bencoding datatypes
#[derive(Debug)]
enum DataType {
    Int(i32),
    Str(String),
    List(Vec<DataType>),
    Dict(Vec<(String, DataType)>),
}

/// returns the number and how many chars it used
fn decode_int(str: &str, delim: char) -> (i32, usize) {
    let mut chars = str.chars().enumerate(); // (usize, char)
    let mut number: i32 = 0;
    loop {
        let char = chars.next().unwrap();
        if char.1 == delim { return (number, char.0+1); }
        if char.1 == '-' { number *= -1; }
        else {
            number *= 10;
            number += char.1.to_digit(10).unwrap() as i32;
        }
    }
}

/// returns the string and how many chars it used
fn decode_str(str: &str) -> (String, usize) {
    let int = decode_int(str, ':');
    
    let mut chars = str.chars();
    chars.advance_by(int.1).unwrap();
    
    let string = chars.take(int.0 as usize).collect::<String>();
    let len = string.len();
    
    (string, int.1+len)
}

/// Warning! Does not check the validity of the input
fn decoder(input_str: &str) -> (DataType, usize) {
    match input_str.chars().next().unwrap() {
        'i' => { // integer
            let int = decode_int(&input_str[1..], 'e');
            (DataType::Int(int.0), int.1+1)
        },
        'l' => { // list of elements
            let mut output = Vec::new();
            let mut index = 1;
            loop {
                if input_str.chars().nth(index).unwrap() == 'e' { break; }
                let value = decoder(&input_str[index..]);
                output.push(value.0);
                index += value.1;
            }
            
            (DataType::List(output), index+1)
        },
        'd' => {
            let mut output = Vec::new();
            let mut index = 1;
            loop {
                if input_str.chars().nth(index).unwrap() == 'e' { break; }
                let key = decode_str(&input_str[index..]);
                index += key.1;
                let value = decoder(&input_str[index..]);
                output.push((key.0, value.0));
                index += value.1;
            }
            
            (DataType::Dict(output), index+1)
        },
        _ => { // str
            let str = decode_str(input_str);
            (DataType::Str(str.0), str.1)
        },
    }
}

fn main() {
    let file = File::open("archlinux.torrent").unwrap().bytes();
    // dbg!(decoder(file))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        dbg!(decoder("i42e"));
        dbg!(decoder("i-235e"));
        dbg!(decoder("12:Hello There!"));
        dbg!(decoder("l4:spam4:eggsi123ee"));
        dbg!(decoder("d3:cow3:moo4:spam4:eggse"));
    }
}