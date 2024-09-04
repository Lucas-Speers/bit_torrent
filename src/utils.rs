
pub fn vec_to_string(vec: &[u8]) -> String {
    vec.iter().map(|c| {*c as char}).collect()
}