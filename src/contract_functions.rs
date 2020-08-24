use constants::SYSTEM_ADDRESS;
use serde_cbor::value::to_value;
use wasm_rpc::error::Error;
use Address;
use Token;
pub fn pay<API: crate::API>(
    api: &mut API,
    token: Token,
    address: Address,
    amount: u64,
) -> Result<(), Box<Error>> {
    api.call::<Result<(), Box<Error>>>(
        SYSTEM_ADDRESS,
        "Token",
        "transfer_from",
        vec![
            to_value(token.clone()).unwrap(),
            to_value(Address::Contract(api.contract_address())).unwrap(),
            to_value(address).unwrap(),
            to_value(amount).unwrap(),
        ],
    )?
}

pub fn charge<API: crate::API>(
    api: &mut API,
    token: Token,
    address: Address,
    amount: u64,
) -> Result<(), Box<Error>> {
    api.call::<Result<(), Box<Error>>>(
        SYSTEM_ADDRESS,
        "Token",
        "transfer_from",
        vec![
            to_value(token.clone()).unwrap(),
            to_value(address.clone()).unwrap(),
            to_value(Address::Contract(api.contract_address())).unwrap(),
            to_value(amount).unwrap(),
        ],
    )??;
    Ok(())
}
