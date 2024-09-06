use std::fs;

use rand::Rng;

use crate::{bencoding::{self, DataType}, utils};

pub struct TorrentClient {
    file_name: String,
    announce: String,
    info: DataType,
    piece_length: u64,
    pieces: String,
    name: String,
    length: u64,
    peer_id: String,
}

impl TorrentClient {
    pub fn new() -> Result<TorrentClient, ()> {
        let file_name = utils::get_torrent_file();
        println!("USING TORRENT FILE: \"{}\"", file_name);
    
        let file = fs::read(&file_name).unwrap();
        let data = bencoding::decoder(&file[..]);
        
        println!("READ FILE SUCCESSFULY");
    
        let announce = data.get("announce")?.get_string()?;
        let info = data.get("info")?;
        let piece_length = info.get("piece length")?.get_int()?;
        let pieces = info.get("pieces")?.get_string()?;
        let name = info.get("name")?.get_string()?;
        let length = info.get("length")?.get_int()?;
    
        let peer_id: String = String::from("-RB0001-")+
            &(0..12)
            .map(|_| {rand::thread_rng().gen_range(0..=9)})
            .map(|c| {c.to_string()})
            .collect::<String>();
    
        println!("ANOUNCE SERVER:     \"{announce}\"");
        println!("FILE NAME:          \"{name}\"");
        println!("FILE SIZE:          {}", utils::file_size(length));
    
        println!("INFO: {:?}", info);

        Ok(TorrentClient {file_name, announce, info, piece_length, pieces, name, length, peer_id })
    }

    pub fn connect(&self) -> Result<(), ()> {

        let url_builder = ureq::get(&self.announce)
            .query("peer_id", &self.peer_id)
            .query("uploaded", "0")
            .query("downloaded", "0")
            .query("left", &self.length.to_string())
            .query("port", "6889");
            // .query("compact", "0");
        
        let url = url_builder.url().to_owned() + "&info_hash=" + &bencoding::get_info_hash();
    
        println!("{}", url);

        let body = ureq::get(&url)
            .call().unwrap()
            .into_string().unwrap();
        
        let body = body.as_bytes();

        println!("{body:?}");
        std::fs::write("tmp", &body).expect("Unable to write file");
    
        let decoded_body = bencoding::decoder(body);

        println!("{decoded_body:?}");

        Ok(())
    }
}