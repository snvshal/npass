// Crypto layer removed: store is saved as cleartext JSON
// Functions remain to keep the Store API stable but are simple pass-throughs.

pub fn encrypt_store(plaintext: &[u8], _password: &str) -> Result<Vec<u8>, ()> {
    Ok(plaintext.to_vec())
}

pub fn decrypt_store(blob: &[u8], _password: &str) -> Result<Vec<u8>, ()> {
    Ok(blob.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pass_through_roundtrip() {
        let data = b"hello";
        let blob = encrypt_store(data, "").expect("encrypt");
        let pt = decrypt_store(&blob, "").expect("decrypt");
        assert_eq!(pt, data);
    }
}
