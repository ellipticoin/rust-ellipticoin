pub extern crate wasm_rpc;
extern crate wasm_rpc_macros;
pub mod constants;
pub use wasm_rpc::{pointer, serde_cbor};
use wasm_rpc::{
    error::Error,
    serde::{de::DeserializeOwned, Serialize, Deserialize},
    serde_cbor::{from_slice, to_vec, Value},
};
pub use wasm_rpc_macros::export;
pub use wasm_rpc_macros::export_native;

pub trait MemoryAPI {
    fn get(&mut self, key: &[u8]) -> Vec<u8>;
    fn set(&mut self, key: &[u8], value: &[u8]);
}

pub trait StorageAPI  {
    fn get(&mut self, key: &[u8]) -> Vec<u8>;
    fn set(&mut self, key: &[u8], value: &[u8]);
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
pub enum Address {
    PublicKey([u8; 32]),
    Contract([u8; 32], String),
}

impl Address {
    pub fn to_vec(&mut self) -> Vec<u8> {
        match self {
            Address::PublicKey(address) => address.to_vec(),
            Address::Contract(legislator, name) => [&legislator[..], name.as_bytes()].concat(),
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
    fn get_memory<V: DeserializeOwned>(&mut self, key: &[u8]) -> Result<V, serde_cbor::Error> {
        from_slice(&MemoryAPI::get(self, key))
    }

    fn set_memory<V: Serialize>(&mut self, key: &[u8], value: V) {
        MemoryAPI::set(self, key, &to_vec(&value).unwrap())
    }

    fn get_storage<V: DeserializeOwned>(&mut self, key: &[u8]) -> Result<V, serde_cbor::Error> {
        from_slice(&StorageAPI::get(self, key))
    }

    fn set_storage<V: Serialize>(&mut self, key: &[u8], value: V) {
        StorageAPI::set(self, key, &to_vec(&value).unwrap())
    }

    fn get_code(&mut self, contract_address: &[u8]) -> Vec<u8> {
        StorageAPI::get(self, contract_address)
    }

    fn set_code(&mut self, contract_address: &[u8], value: &[u8]) {
        StorageAPI::set(self, contract_address, value)
    }
}
