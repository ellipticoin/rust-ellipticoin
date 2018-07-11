extern crate wasm_rpc;
use wasm_rpc::*;

extern {
    fn _sender() -> *const u8;
    fn _block_hash() -> *const u8;
    fn read(key: *const u8) -> *const u8;
    fn _call(code: *const u8, method: *const u8, params: *const u8, storage_context: *const u8) -> *const u8;
    fn write(key: *const u8, value: *const u8);
}

pub struct ElipitcoinBlockchain {}

impl BlockChain for ElipitcoinBlockchain {
    fn block_hash(&self) -> Vec<u8> {
        unsafe {
            _block_hash().as_raw_bytes()
        }
    }

    fn read(&self, key: Vec<u8>) -> Vec<u8> {
      unsafe {
        read(key.as_pointer()).as_raw_bytes()
      }
    }

    fn sender(&self) -> Vec<u8> {
        unsafe {
            _sender().as_raw_bytes()
        }
    }

    fn write(&self, key: Vec<u8>, value: Vec<u8>) {
        unsafe {
            write(
                key.as_pointer(),
                value.as_pointer(),
            );
        }
    }

    fn call(&self, code: Vec<u8>, method: String, params: Vec<u8>, storage_context: Vec<u8>) -> Vec<u8> {
        unsafe {
            _call(
                code.as_pointer(),
                method.as_pointer(),
                params.as_pointer(),
                storage_context.as_pointer()
            ).as_raw_bytes()
        }
    }
}

pub trait BlockChain {
    fn read(&self, key: Vec<u8>) -> Vec<u8>;
    fn write(&self, key: Vec<u8>, value: Vec<u8>);
    fn block_hash(&self) -> Vec<u8>;
    fn sender(&self) -> Vec<u8>;
    fn call(&self, code: Vec<u8>, method: String, params: Vec<u8>, storage_context: Vec<u8>) -> Vec<u8>;

    fn read_u32(&self, key: Vec<u8>) -> u32 {
        self.read(key).value()
    }

    fn read_u64(&self, key: Vec<u8>) -> u64 {
        self.read(key).value()
    }

    fn write_u64(&self, key: Vec<u8>, value: u64) {
        FromBytes::from_u64(value);
        self.write(key, FromBytes::from_u64(value));
    }
}
