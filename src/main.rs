use std::collections::HashMap;


/// Bencoding datatypes
enum DataType {
    Int(u32),
    Str(String),
    List(Vec<DataType>),
    Dict(HashMap<String, DataType>),
}

fn main() {
    println!("Hello, world!");
}
