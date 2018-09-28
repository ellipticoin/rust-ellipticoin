#![no_std]
#![feature(alloc)]
extern crate alloc;
extern crate wasm_rpc;
use alloc::string::String;
use alloc::vec::Vec;
use core::intrinsics::transmute;

use wasm_rpc::{Bytes, Dereferenceable, Referenceable};

extern "C" {
    fn _sender() -> *const u8;
    fn _block_hash() -> *const u8;
    fn _block_winner() -> *const u8;
    fn _secp256k1_recover(message: *const u8, signature: *const u8, recovery_id: u8) -> *const u8;
    fn _read(key: *const u8) -> *const u8;
    fn _call(
        code: *const u8,
        method: *const u8,
        params: *const u8,
        storage_context: *const u8,
    ) -> *const u8;
    fn _write(key: *const u8, value: *const u8);
}
pub fn read_u32<K: Into<Vec<u8>>>(key: K) -> u32 {
    read(key.into()).value()
}

pub fn read_u64<K: Into<Vec<u8>>>(key: K) -> u64 {
    read(key.into()).value()
}

pub fn read_int<K: Into<Vec<u8>>>(key: K) -> u64 {
    read_u64(key)
}

pub fn write_u32<K: Into<Vec<u8>>>(key: K, value: u32) {
    write(
        key.into(),
        unsafe { transmute::<u32, [u8; 4]>(value) }.to_vec(),
    );
}

pub fn write_u64<K: Into<Vec<u8>>>(key: K, value: u64) {
    write(
        key.into(),
        unsafe { transmute::<u64, [u8; 8]>(value) }.to_vec(),
    );
}

pub fn write_int<K: Into<Vec<u8>>>(key: K, value: u64) {
    write(
        key.into(),
        unsafe { transmute::<u64, [u8; 8]>(value) }.to_vec(),
    );
}

pub fn block_hash() -> Vec<u8> {
    unsafe { _block_hash().as_raw_bytes() }
}

pub fn block_winner() -> Vec<u8> {
    unsafe { _block_winner().as_raw_bytes() }
}

pub fn secp256k1_recover(message: Vec<u8>, signature: Vec<u8>, recovery_id: u8) -> Vec<u8> {
    unsafe {
        _secp256k1_recover(message.as_pointer(), signature.as_pointer(), recovery_id).as_raw_bytes()
    }
}

pub fn read<K: Into<Vec<u8>>>(key: K) -> Vec<u8> {
    unsafe { _read(key.into().as_pointer()).as_raw_bytes() }
}

pub fn sender() -> Vec<u8> {
    unsafe { _sender().as_raw_bytes() }
}

pub fn update<K: Into<Vec<u8>> + Clone>(key: K, update_function: &Fn(Vec<u8>) -> Vec<u8>) {
    let value_before = read(key.clone());
    let value_after = update_function(value_before);
    write(key, value_after);
}

pub fn write<K: Into<Vec<u8>>, V: Into<Vec<u8>>>(key: K, value: V) {
    unsafe {
        _write(key.into().as_pointer(), value.into().as_pointer());
    }
}

pub fn call(code: Vec<u8>, method: String, params: Vec<u8>, storage_context: Vec<u8>) -> Vec<u8> {
    unsafe {
        _call(
            code.as_pointer(),
            method.as_pointer(),
            params.as_pointer(),
            storage_context.as_pointer(),
        ).as_raw_bytes()
    }
}
