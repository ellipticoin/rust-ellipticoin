extern crate ellipticoin_macros;
#[macro_use]
extern crate lazy_static;
extern crate sha2;
pub extern crate wasm_rpc;
extern crate wasm_rpc_macros;

pub use ellipticoin_macros::*;
pub mod constants;
pub mod contract_functions;
pub mod helpers;
use helpers::sha256;
use wasm_rpc::{
    error::Error,
    serde::{de::DeserializeOwned, Deserialize, Serialize},
    serde_cbor::{from_slice, to_vec, Value},
};
pub use wasm_rpc::{pointer, serde_cbor};
pub use wasm_rpc_macros::{export, export_native};

pub trait MemoryAPI {
    fn get(&mut self, key: &[u8]) -> Vec<u8>;
    fn set(&mut self, key: &[u8], value: &[u8]);
}

pub trait StorageAPI {
    fn get(&mut self, key: &[u8]) -> Vec<u8>;
    fn set(&mut self, key: &[u8], value: &[u8]);
}

pub trait ToKey {
    fn to_key(self) -> Vec<u8>;
}

impl ToKey for &[u8] {
    fn to_key(self) -> Vec<u8> {
        self.to_vec()
    }
}

impl ToKey for Vec<u8> {
    fn to_key(self) -> Vec<u8> {
        self
    }
}

impl ToKey for Address {
    fn to_key(mut self) -> Vec<u8> {
        sha256(self.to_vec()).to_vec()
    }
}

macro_rules! replace_expr {
    ($_t:tt $sub:ty) => {
        $sub
    };
}

macro_rules! impl_to_key_tuple {
    ( $( $name:ident )+ ) => {
        impl<T: ToKey> ToKey for ($(replace_expr!(($name) T),)+)
        {
            fn to_key(self) -> Vec<u8> {
                let ($($name,)+) = self;
                vec![$($name.to_key(),)+].concat()
            }
        }
    };
}

impl_to_key_tuple! { a }
impl_to_key_tuple! { a b }
impl_to_key_tuple! { a b c }
impl_to_key_tuple! { a b c d }
impl_to_key_tuple! { a b c d e }
impl_to_key_tuple! { a b c d e f }
impl_to_key_tuple! { a b c d e f g }
impl_to_key_tuple! { a b c d e f g h }
impl_to_key_tuple! { a b c d e f g h i }
impl_to_key_tuple! { a b c d e f g h i j }
impl_to_key_tuple! { a b c d e f g h i j k }
impl_to_key_tuple! { a b c d e f g h i j k l }

#[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct Token {
    pub issuer: Address,
    pub token_id: [u8; 32],
}

impl Into<Vec<u8>> for Token {
    fn into(mut self) -> Vec<u8> {
        [self.issuer.to_vec(), self.token_id.to_vec()].concat()
    }
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
pub enum Address {
    PublicKey([u8; 32]),
    Contract(([u8; 32], String)),
}

impl Address {
    pub fn to_vec(&mut self) -> Vec<u8> {
        match self {
            Address::PublicKey(address) => address.to_vec(),
            Address::Contract((legislator, name)) => [&legislator[..], name.as_bytes()].concat(),
        }
    }

    pub fn as_public_key(&mut self) -> Option<[u8; 32]> {
        match self {
            Address::PublicKey(address) => Some(*address),
            _ => None,
        }
    }
}

pub trait API: MemoryAPI + StorageAPI {
    fn contract_address(&self) -> ([u8; 32], String);
    fn sender(&self) -> [u8; 32];
    fn caller(&self) -> Address;
    fn call<D: DeserializeOwned>(
        &mut self,
        legislator: [u8; 32],
        contract_name: &str,
        function_name: &str,
        arguments: Vec<Value>,
    ) -> Result<D, Box<Error>>;
    fn get_memory<K: Into<Vec<u8>>, V: DeserializeOwned>(
        &mut self,
        key: K,
    ) -> Result<V, serde_cbor::Error> {
        from_slice(&MemoryAPI::get(self, &key.into()))
    }

    fn set_memory<K: Into<Vec<u8>>, V: Serialize>(&mut self, key: K, value: V) {
        MemoryAPI::set(self, &key.into(), &to_vec(&value).unwrap())
    }

    fn get_storage<K: Into<Vec<u8>>, V: DeserializeOwned>(
        &mut self,
        key: K,
    ) -> Result<V, serde_cbor::Error> {
        from_slice(&StorageAPI::get(self, &key.into()))
    }

    fn set_storage<K: Into<Vec<u8>>, V: Serialize>(&mut self, key: K, value: V) {
        StorageAPI::set(self, &key.into(), &to_vec(&value).unwrap())
    }

    fn get_code(&mut self, contract_address: &[u8]) -> Vec<u8> {
        StorageAPI::get(self, contract_address)
    }

    fn set_code(&mut self, contract_address: &[u8], value: &[u8]) {
        StorageAPI::set(self, contract_address, value)
    }
}
