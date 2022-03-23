use base64;

pub fn base_to_blob(base_string: String) -> Result<Vec<u8>, &'static str>{
    base64::decode(base_string)
    .map_err(|_| "cannot convert string to blob")
}