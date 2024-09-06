use std::fmt::Write;

pub fn file_size(bytes: u64) -> String {
    match bytes {
        0..1_000 => format!("{} B", bytes),
        1_000..1_000_000 => format!("{} KB", (bytes as f64/100.0).round() / 10.0),
        1_000_000..1_000_000_000 => format!("{} MB", (bytes as f64/100_000.0).round() / 10.0),
        _ => format!("{} GB", (bytes as f64/10_000_000.0).round() / 100.0),
    }    
}

/// This will atempt to gather the filename from `env::args()`
pub fn get_torrent_file() -> String {
    let args: Vec<String> = std::env::args().collect();
    
    let file_name;
    
    if let Some(x) = args.get(1) {
        file_name = x.to_string();
    } else {
        println!("Usage wrong");
        std::process::exit(0);
    }

    let path = std::path::Path::new(&file_name);

    if !path.exists() {
        println!("File does not exist");
        std::process::exit(1);
    }

    file_name

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