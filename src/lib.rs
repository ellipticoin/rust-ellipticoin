#[macro_use]
extern crate lazy_static;
pub extern crate wasm_rpc;
extern crate wasm_rpc_macros;
pub mod constants;
pub use wasm_rpc::{pointer, serde_cbor};
use wasm_rpc::{
    serde::{de::DeserializeOwned, Serialize},
    serde_cbor::{from_slice, to_vec, Value},
};
pub use wasm_rpc_macros::export;
pub use wasm_rpc_macros::export_native;

extern "C" {
    fn __address() -> *const u8;
    fn __sender() -> *const u8;
    fn __contract_address() -> *const u8;
    fn __caller() -> *const u8;
    fn __call(
        contract_address: *const u8,
        function_name: *const u8,
        arguments: *const u8,
    ) -> *const u8;
    fn __get_memory(key: *const u8) -> *const u8;
    fn __set_memory(key: *const u8, value: *const u8);
    fn __get_storage(key: *const u8) -> *const u8;
    fn __set_storage(key: *const u8, value: *const u8);
}

pub trait Contract {
    fn get_memory_raw(&mut self, key: &[u8]) -> Vec<u8>;
    fn set_memory_raw(&mut self, key: &[u8], value: &[u8]);
    fn get_storage_raw(&mut self, key: &[u8]) -> Vec<u8>;
    fn set_storage_raw(&mut self, key: &[u8], value: &[u8]);
    fn contract_address(&self) -> Vec<u8>;
    fn sender(&self) -> Vec<u8>;
    fn caller(&self) -> Vec<u8>;
    fn call<D: DeserializeOwned + 'static + std::convert::From<Value>>(
        contract_address: Vec<u8>,
        function_name: &str,
        arguments: Vec<Value>,
    ) -> Result<D, serde_cbor::Error>;
    fn get_memory<V: DeserializeOwned>(&mut self, key: &[u8]) -> Result<V, serde_cbor::Error> {
        from_slice(&self.get_memory_raw(key))
    }

    fn set_memory<V: Serialize>(&mut self, key: &[u8], value: V) {
        self.set_memory_raw(key, &to_vec(&value).unwrap())
    }

    fn get_storage<V: DeserializeOwned>(&mut self, key: &[u8]) -> Result<V, serde_cbor::Error> {
        from_slice(&self.get_storage_raw(key))
    }

    fn set_storage<V: Serialize>(&mut self, key: &[u8], value: V) {
        self.set_storage_raw(key, &to_vec(&value).unwrap())
    }
}

pub fn get_memory<V: DeserializeOwned>(key: &[u8]) -> Result<V, serde_cbor::Error> {
    pointer::to_value::<V>(unsafe { __get_memory(pointer::from_bytes(key)) })
}

pub fn set_memory<V: Serialize>(key: &[u8], value: V) {
    unsafe { __set_memory(pointer::from_bytes(key), pointer::from_value::<V>(&value)) }
}

pub fn get_storage<V: DeserializeOwned>(key: &[u8]) -> Result<V, serde_cbor::Error> {
    pointer::to_value::<V>(unsafe { __get_storage(pointer::from_bytes(key)) })
}

pub fn set_storage<V: Serialize>(key: &[u8], value: V) {
    unsafe { __set_storage(pointer::from_bytes(key), pointer::from_value::<V>(&value)) }
}

pub fn contract_address() -> Vec<u8> {
    pointer::to_bytes(unsafe { __contract_address() }).to_vec()
}

pub fn sender() -> Vec<u8> {
    pointer::to_bytes(unsafe { __sender() }).to_vec()
}

pub fn caller() -> Vec<u8> {
    pointer::to_bytes(unsafe { __caller() }).to_vec()
}

pub fn call<D: DeserializeOwned + 'static + std::convert::From<Value>>(
    contract_address: Vec<u8>,
    function_name: &str,
    arguments: Vec<Value>,
) -> Result<D, serde_cbor::Error> {
    pointer::to_value(unsafe {
        __call(
            pointer::from_value(&Value::Bytes(contract_address)),
            pointer::from_value(&Value::Text(function_name.to_string())),
            pointer::from_value::<Vec<Value>>(&arguments.into()),
        )
    })
}
