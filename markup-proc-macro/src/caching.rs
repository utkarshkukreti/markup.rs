use proc_macro::TokenStream;

#[cfg(not(feature = "caching"))]
pub fn cached(tokens: TokenStream, f: impl FnOnce(TokenStream) -> TokenStream) -> TokenStream {
    f(tokens)
}

#[cfg(feature = "caching")]
pub fn cached(tokens: TokenStream, f: impl FnOnce(TokenStream) -> TokenStream) -> TokenStream {
    use sha2::{Digest, Sha256};
    use std::fs;

    let dir = dirs::cache_dir().unwrap().join("markup-rs");
    let _ = fs::create_dir(&dir);

    let input = tokens.to_string();
    let mut hasher = Sha256::new();
    hasher.input(input);
    let hash = hasher.result();
    let path = dir.join(format!("{:x}", hash));

    if let Ok(cached) = fs::read_to_string(&path) {
        if let Ok(cached) = cached.parse() {
            return cached;
        }
    }

    let output = f(tokens);
    if fs::write(&path, output.to_string()).is_err() {
        eprintln!("warning: cannot write to cache dir");
    }
    output
}
