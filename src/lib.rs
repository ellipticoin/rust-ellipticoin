extern crate alloc;
extern crate wasm_rpc;
extern crate wasm_rpc_macros;

pub use wasm_rpc::{Bytes, Dereferenceable, FromBytes, Referenceable, ToBytes, Value};
pub use wasm_rpc_macros::export;
use std::mem::transmute;
pub mod error;

extern "C" {
    fn __address() -> *const u8;
    fn __sender() -> *const u8;
    fn __block_hash() -> *const u8;
    fn __block_number() -> *const u8;
    fn __block_winner() -> *const u8;
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

pub trait Keyable {
    fn to_key(self: Self) -> Vec<u8>;
}

impl Keyable for Vec<u8> {
    fn to_key(self: Self) -> Vec<u8> {
        self
    }
}

impl Keyable for &str {
    fn to_key(self: Self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}


pub fn get_memory<K: Keyable,V: FromBytes,>(key: K) -> V {
    let v: Vec<u8> = unsafe { __get_memory(key.to_key().as_pointer()) }.as_raw_bytes();
    FromBytes::from_bytes(v)
}

pub fn set_memory<K: Keyable, V: ToBytes>(key: K, value: V) {
    unsafe { __set_memory(key.to_key().as_pointer(), value.to_bytes().as_pointer()) }
}

pub fn get_storage<V: FromBytes, K: Keyable>(key: K) -> V {
    let v: Vec<u8> = unsafe { __get_storage(key.to_key().as_pointer()) }.as_raw_bytes();
    FromBytes::from_bytes(v)
}

pub fn set_storage<K: Keyable, V: ToBytes>(key: K, value: V) {
    unsafe { __set_storage(key.to_key().as_pointer(), value.to_bytes().as_pointer()) }
}

pub fn address() -> Vec<u8> {
    let v: Vec<u8> = unsafe { __block_winner() }.as_raw_bytes();
    v
}

pub fn block_winner() -> Vec<u8> {
    let v: Vec<u8> = unsafe { __block_winner() }.as_raw_bytes();
    v
}


pub fn contract_address() -> Vec<u8> {
    unsafe { __contract_address().as_raw_bytes() }
}

pub fn sender() -> Vec<u8> {
    unsafe { __sender().as_raw_bytes() }
}


pub fn caller() -> Vec<u8> {
    unsafe { __caller().as_raw_bytes() }
}

pub fn block_number() -> u64 {
    unsafe { __block_number() }.to_i64() as u64
}


pub fn call(contract_address: Vec<u8>, function_name: &str, arguments: Vec<Value>) -> (u32, Value) {
    from_bytes(unsafe {
        __call(
            contract_address.as_pointer(),
            function_name.to_string().as_pointer(),
            arguments.as_pointer(),
        )
    }.as_raw_bytes())
}

pub type Result = (u32, Value);
pub fn vm_panic() -> Result {
    (1, "vm panic 2".to_string().into())
}

pub fn from_bytes(bytes: Vec<u8>) -> Result {
    if bytes.len() == 0 {
        (1, "vm bytes length is zero".to_string().into())
    } else {
        let bytes_clone = bytes.clone();
        let (return_code_bytes, return_value_bytes) = bytes_clone.split_at(4);
        let mut return_code_bytes_fixed: [u8; 4] = Default::default();
        if bytes.len() == 0 {
            (1, "vm error".to_string().into())
        } else {
            return_code_bytes_fixed.copy_from_slice(&return_code_bytes[0..4]);
            let return_code: u32 = unsafe { transmute(return_code_bytes_fixed) };

            let return_value: Value = wasm_rpc::from_slice(return_value_bytes).expect("failed to parse return_value");

            (return_code, return_value)
        }
    }
}
