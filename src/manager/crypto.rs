use std::num::NonZeroU32;

use data_encoding::HEXUPPER;
use ring::{
    digest::{self},
    pbkdf2::{self, PBKDF2_HMAC_SHA512},
    rand::{self, SecureRandom},
};

pub fn gen_salt() -> String {
    const LEN: usize = digest::SHA512_OUTPUT_LEN;
    let mut salt = [0u8; LEN];
    let _ = rand::SystemRandom::new().fill(&mut salt);
    HEXUPPER.encode(&salt)
}

pub fn hash(input: &str, master: &str, salt: &str) -> String {
    let iter = NonZeroU32::new(100000).unwrap();
    let mut out = input.as_bytes().to_owned();
    pbkdf2::derive(
        PBKDF2_HMAC_SHA512,
        iter,
        salt.as_bytes(),
        master.as_bytes(),
        &mut out,
    );

    HEXUPPER.encode(&out)
}

pub fn check_hash(input: &str, hash: &str, salt: &str) -> bool {
    let iter = NonZeroU32::new(100000).unwrap();
    let hash = HEXUPPER.decode(hash.as_bytes()).unwrap();
    pbkdf2::verify(
        PBKDF2_HMAC_SHA512,
        iter,
        salt.as_bytes(),
        input.as_bytes(),
        hash.as_ref(),
    ).is_ok()
}
