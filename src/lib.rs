extern crate ellipticoin_macros;
#[macro_use]
extern crate lazy_static;
extern crate sha2;
pub extern crate wasm_rpc;
extern crate wasm_rpc_macros;

pub use ellipticoin_macros::*;
pub mod constants;
pub mod helpers;
use helpers::db_key;
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

impl Into<Vec<u8>> for Address {
    fn into(mut self) -> Vec<u8> {
        self.to_vec()
    }
}


pub trait API: MemoryAPI + StorageAPI {
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
        contract_address: ([u8; 32], &'static str),
        key: K,
    ) -> Result<V, serde_cbor::Error> {
        from_slice(&MemoryAPI::get(self, &db_key(&contract_address, &key.into())))
    }

    fn set_memory<K: Into<Vec<u8>>, V: Serialize>(&mut self, 

        contract_address: ([u8; 32], &'static str),
key: K, value: V) {
        MemoryAPI::set(self, &db_key(&contract_address, &key.into()), &to_vec(&value).unwrap())
    }

    fn get_storage<K: Into<Vec<u8>>, V: DeserializeOwned>(
        &mut self,
        contract_address: ([u8; 32], &'static str),
        key: K,
    ) -> Result<V, serde_cbor::Error> {
        from_slice(&StorageAPI::get(self, &db_key(&contract_address, &key.into())))
    }

    fn set_storage<K: Into<Vec<u8>>, V: Serialize>(&mut self,
        contract_address: ([u8; 32], &'static str),

key: K, value: V) {
        StorageAPI::set(self, &db_key(&contract_address, &key.into()), &to_vec(&value).unwrap())
    }
}
