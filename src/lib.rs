extern crate alloc;
extern crate wasm_rpc;
extern crate wasm_rpc_macros;

pub use wasm_rpc::{Bytes, Dereferenceable, FromBytes, Referenceable, ToBytes, Value};
pub use wasm_rpc_macros::export;

pub mod error;

extern "C" {
    fn __sender() -> *const u8;
    fn __block_hash() -> *const u8;
    fn __block_number() -> *const u8;
    fn __block_winner() -> *const u8;
    fn __call(
        code: *const u8,
        method: *const u8,
        params: *const u8,
        storage_context: *const u8,
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

pub fn block_winner() -> Vec<u8> {
    let v: Vec<u8> = unsafe { __block_winner() }.as_raw_bytes();
    v
}


pub fn sender() -> Vec<u8> {
    unsafe { __sender().as_raw_bytes() }
}

pub fn block_number() -> u64 {
    unsafe { __block_number() }.to_i64() as u64
}


pub fn call(code: Vec<u8>, method: String, params: Vec<u8>, storage_context: Vec<u8>) -> Vec<u8> {
    unsafe {
        __call(
            code.as_pointer(),
            method.as_pointer(),
            params.as_pointer(),
            storage_context.as_pointer(),
        )
        .as_raw_bytes()
    }
}
