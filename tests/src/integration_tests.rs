#[cfg(test)]
mod tests {
    use casper_contract::contract_api::runtime;
    use casper_engine_test_support::{Code, Error, SessionBuilder, TestContextBuilder, Value};
    use casper_types::{
        account::AccountHash, runtime_args, BlockTime, PublicKey, RuntimeArgs, SecretKey, U512,
    };

    const MY_ACCOUNT: [u8; 32] = [7u8; 32];
    // Define `KEY` constant to match that in the contract.
    const KEY: &str = "stringvalue";
    const VALUE: &str = "hello world";
    const RUNTIME_ARG_NAME: &str = "message";
    const CONTRACT_WASM: &str = "contract.wasm";

    #[test]
    fn should_store_hello_world() {
        let secret_key = SecretKey::ed25519_from_bytes(MY_ACCOUNT).unwrap();
        let public_key = PublicKey::from(&secret_key);
        let account_addr = AccountHash::from(&public_key);

        let mut context = TestContextBuilder::new()
            .with_public_key(public_key, U512::from(500_000_000_000_000_000u64))
            .build();

        // The test framework checks for compiled Wasm files in '<current working dir>/wasm'.  Paths
        // relative to the current working dir (e.g. 'wasm/contract.wasm') can also be used, as can
        // absolute paths.
        let session_code = Code::from(CONTRACT_WASM);
        let session_args = runtime_args! {
            RUNTIME_ARG_NAME => VALUE,
        };
        let session = SessionBuilder::new(session_code, session_args)
            .with_address(account_addr)
            .with_authorization_keys(&[account_addr])
            .build();

        let result_of_query = context.run(session);
        let value_result = result_of_query
            .query(account_addr, &[KEY.to_string()])
            .unwrap();
        let value_result1 = result_of_query
            .query(account_addr, &["access".to_string()])
            .unwrap();

        // let returned_value = result_of_query.expect("should be a value");

        // let expected_value = Value::from_t(VALUE.to_string()).expect("should construct Value");
        // assert_eq!(expected_value, returned_value);

        // let actual_block_time: BlockTime = runtime::get_blocktime();
        // let b: u64 = actual_block_time.into();

        let returned_value: i32 = value_result.into_t().unwrap();
        let returned_value2: u8 = value_result1.into_t().unwrap();

        println!("I ma here {}", returned_value);
        println!("I ma here2 {}", returned_value2);
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
