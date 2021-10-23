#![no_std]
#![no_main]

extern crate alloc;

use core::convert::TryFrom;

use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    account::AccountHash,
    contracts::{
        EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, NamedKeys,
        CONTRACT_INITIAL_VERSION,
    },
    runtime_args, BlockTime, CLType, ContractHash, ContractPackageHash, ContractVersion, Key,
    RuntimeArgs, U256,
};

const PACKAGE_HASH_KEY: &str = "package_hash_key";
const PACKAGE_ACCESS_KEY: &str = "package_access_key";
const CONTRACT_HASH_KEY: &str = "contract_hash_key";
const CONTRACT_CODE: &str = "contract_code_test";
const SESSION_CODE: &str = "session_code_test";
const NEW_KEY: &str = "new_key";
const NAMED_KEY: &str = "contract_named_key";
const CONTRACT_VERSION: &str = "contract_version";

#[no_mangle]
pub extern "C" fn call() {
    // Session contract

    let a: U256 = U256::one();
    let value: String = a.to_string();

    let actual_block_time: BlockTime = runtime::get_blocktime();
    let b: u64 = actual_block_time.into();

    runtime::put_key("stringvalue", storage::new_uref(value).into());
    runtime::put_key("blocktimevalue", storage::new_uref(b).into());

    let a = Key::from_formatted_str("hash-xxxx").unwrap();
    let b = Key::from(a);
    let c = b;
}
