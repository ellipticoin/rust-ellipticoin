pub const SYSTEM_ADDRESS: [u8; 32] = [0; 32];
pub const SYSTEM_CONTRACT_NAME: &str = "Ellipticoin";
lazy_static! {
    pub static ref SYSTEM_CONTRACT_ADDRESS: Vec<u8> = [
        SYSTEM_ADDRESS.to_vec(),
        SYSTEM_CONTRACT_NAME.as_bytes().to_vec(),
    ]
    .concat();
}
