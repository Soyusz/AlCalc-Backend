use argonautica::Hasher;

pub fn hash_password(password: String) -> Option<String> {
    let mut hash_secret = std::env::var("HASH_SECRET").unwrap_or(String::from("sample salt"));
    let mut hasher = Hasher::default();
    let hash = hasher
        .with_password(password)
        .with_secret_key(hash_secret)
        .hash();
    
    match hash {
        Ok(s) => Some(s),
        Err(_) => None
    }
}