use std::fmt::Write;

use crate::{bencoding::{self, DataType}, utils};

pub struct Client {
    announce: String,
    peer_id: String,
    info_hash: String,
    file_length: usize,
}

impl Client {
    pub fn new(announce: String, peer_id: String, info_hash: String, file_length: usize) -> Client {
        Client {announce, peer_id, info_hash, file_length}
    }
    pub fn connect(
        &self,
        first: bool,
        uploaded: u64,
        downloaded: u64,
    ) -> Result<DataType, ()> {

        let url_builder = ureq::get(&self.announce)
            .query("peer_id", &self.peer_id)
            .query("uploaded", "0")
            .query("downloaded", "0")
            .query("left", &self.file_length.to_string())
            .query("port", "6889")
            .query("compact", "1");
        
        let url = url_builder.url().to_owned() + "&info_hash=" + &self.info_hash;
    
        println!("{}", url);

        let body = ureq::get(&url)
            .call().unwrap()
            .into_string().unwrap();

        println!("{body}");
    
        
    
        Ok(bencoding::decoder(body.as_bytes()))
    }
    pub fn start(&self) {
        let r = self.connect(true, 0, 0).unwrap();

        println!("{r:?}");
    }
}

pub fn url_encode_bytes(content: &[u8]) -> String {
    let mut out: String = String::new();

    for byte in content.iter() {
        match *byte as char {
            '0'..='9' | 'a'..='z' | 'A'..='Z' | '.' | '-' | '_' | '~' => out.push(*byte as char),
            _ => write!(&mut out, "%{:02X}", byte).unwrap(),
        };
    }

    out
}

fn url_encode(params: &[(&[u8], &[u8])]) -> String {
    let mut output = String::new();

    for param in params {
        output.push_str(&url_encode_bytes(param.0));
        output.push('=');
        output.push_str(&url_encode_bytes(param.1));
        output.push('&');
    }
    output.pop();

    output
}

