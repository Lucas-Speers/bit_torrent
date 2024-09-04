#![feature(iter_advance_by)]
#![allow(unused)]

mod bencoding;
mod client;
mod utils;

use std::{collections::HashMap, env, fmt::Debug, fs::{self, File}, io::{self, Write}, sync::Mutex};
use client::Client;
use rand::Rng;
use sha1_smol::Sha1;

fn file_size(bytes: u64) -> String {
    match bytes {
        0..1_000 => format!("{} B", bytes),
        1_000..1_000_000 => format!("{} KB", (bytes as f64/100.0).round() / 10.0),
        1_000_000..1_000_000_000 => format!("{} MB", (bytes as f64/100_000.0).round() / 10.0),
        _ => format!("{} GB", (bytes as f64/10_000_000.0).round() / 100.0),
    }    
}

fn main() -> Result<(), ()> {
    let args: Vec<String> = env::args().collect();
    
    let mut file_name = String::new();
    
    if let Some(x) = args.get(1) {
        file_name = x.to_string();
    } else {
        println!("Enter the name of the .torrent file");
        io::stdin().read_line(&mut file_name).unwrap();
        file_name.pop();
    }
    
    println!("USING TORRENT FILE: \"{file_name}\"");
    
    let file = fs::read(file_name).unwrap();
    let data = bencoding::decoder(&file[..]);

    
    println!("READ FILE SUCCESSFULY");

    let announce = data.get("announce")?.get_string()?;
    println!("1");
    let info = data.get("info")?;
    println!("1");
    let piece_length = info.get("piece length")?.get_int()?;
    println!("1");
    let pieces = info.get("pieces")?.get_string()?;
    println!("1");
    let name = info.get("name")?.get_string()?;
    println!("1");
    let length = info.get("length")?.get_int()?;
    println!("1");

    let peer_id: String = String::from("-RB0001-")+
        &(0..12)
        .map(|_| {rand::thread_rng().gen_range(0..=9)})
        .map(|c| {c.to_string()})
        .collect::<String>();

    println!("ANOUNCE SERVER:     \"{announce}\"");
    println!("FILE NAME:          \"{name}\"");
    println!("FILE SIZE:          {}", file_size(length));

    println!("INFO: {:?}", info);

    let info_str = bencoding::encoder(&info);
    println!("{}", info_str.chars().last().unwrap());
    println!("{}", Sha1::from(&info_str).digest().to_string());

    let client = Client::new(
        announce,
        peer_id,
        bencoding::get_info_hash(),
        length as usize
    );
    client.start();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use bencoding::{decoder, DataType::*};

    #[test]
    fn it_works() {
        assert_eq!(decoder("i42e".as_bytes()), Int(42));
        bencoding::decoder("12:Hello There!".as_bytes());
        bencoding::decoder("l4:spam4:eggsi123ee".as_bytes());
        bencoding::decoder("d3:cow3:moo4:spam4:eggse".as_bytes());
    }
}