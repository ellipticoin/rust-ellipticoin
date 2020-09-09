use crate::{Address, Token};
use helpers::zero_pad_vec;
use std::convert::TryInto;

pub const SYSTEM_ADDRESS: [u8; 32] = [0; 32];

lazy_static! {
    pub static ref ELC: Token = Token {
        issuer: Address::Contract((SYSTEM_ADDRESS, "Ellipticoin".to_string())),
        id: zero_pad_vec("ELC".as_bytes(), 32)[..].try_into().unwrap()
    };
}
