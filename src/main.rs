#![feature(iter_advance_by)]

mod bencoding;
mod utils;
mod torrent;

use torrent::TorrentClient;

fn main() -> Result<(), ()> {

    let client = TorrentClient::new()?;

    client.connect().unwrap();

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