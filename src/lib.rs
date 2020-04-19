extern crate alloc;
extern crate wasm_rpc;
extern crate wasm_rpc_macros;

pub use wasm_rpc::{abort,Referenceable,Dereferenceable, Bytes, FromBytes, ToBytes, Value, error, BTreeMap, value};
pub use wasm_rpc_macros::export;
pub use wasm_rpc::serde_bytes;

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

pub fn get_memory<K: ToBytes,V: FromBytes,>(key: K) -> V {
    let v: Vec<u8> = unsafe { __get_memory(key.to_bytes().as_pointer()) }.as_raw_bytes();
    FromBytes::from_bytes(v)
}

pub fn set_memory<K: ToBytes, V: ToBytes>(key: K, value: V) {
    unsafe { __set_memory(key.to_bytes().as_pointer(), value.to_bytes().as_pointer()) }
    // unsafe { __set_memory(1 as *const u8, 1 as *const u8) }
}

pub fn get_storage<K: ToBytes, V: FromBytes>(key: K) -> V {
    let v: Vec<u8> = unsafe { __get_storage(key.to_bytes().as_pointer()) }.as_raw_bytes();
    FromBytes::from_bytes(v)
}

pub fn set_storage<K: ToBytes, V: ToBytes>(key: K, value: V) {
    unsafe { __set_storage(key.to_bytes().as_pointer(), value.to_bytes().as_pointer()) }
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
    unsafe { FromBytes::from_bytes(__block_number().as_raw_bytes()) }
}


pub fn call(contract_address: Vec<u8>, function_name: &str, arguments: Vec<Value>) -> Value {
    FromBytes::from_bytes(unsafe {
        __call(
            contract_address.as_pointer(),
            function_name.to_string().as_pointer(),
            arguments.as_pointer(),
        )
    }.as_raw_bytes())
}

pub type Result = (u32, Value);
