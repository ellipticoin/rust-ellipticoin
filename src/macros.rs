#[macro_export]
macro_rules! pay {
    ($api:expr, $token:expr, $recipient:expr, $amount:expr) => {
        token::transfer_from(
            $api,
            $token,
            Address::Contract((SYSTEM_ADDRESS, CONTRACT_NAME.to_string())),
            $recipient,
            $amount,
        )
    };
}

#[macro_export]
macro_rules! charge {
    ($api:expr, $token:expr, $recipient:expr, $amount:expr) => {
        token::transfer_from(
            $api,
            $token,
            $recipient,
            Address::Contract((SYSTEM_ADDRESS, CONTRACT_NAME.to_string())),
            $amount,
        )
    };
}
