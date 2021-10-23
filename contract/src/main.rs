#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::{String, ToString};

use casper_contract::contract_api::{
    runtime::{self, remove_key},
    storage,
};
use casper_types::{
    bytesrepr::FromBytes, AccessRights, ContractHash, Key, RuntimeArgs, URef, U256,
    UREF_ADDR_LENGTH,
};
use core::convert::{TryFrom, TryInto};

const REPLACEMENT_DATA: &str = "bawitdaba";
const ARG_CONTRACT_HASH: &str = "contract_hash";

#[no_mangle]
pub extern "C" fn call() {
    // let a = storage::new_uref(1);
    // a.with_access_rights(AccessRights::NONE);
    // storage::add(a, 2);

    // // let c = URef::new(b.addr(), AccessRights::NONE);

    // runtime::put_key("stringvalue", a.into());

    let a = storage::new_uref(1);
    let b = a.with_access_rights(AccessRights::READ);
    let bit = b.access_rights().bits();

    runtime::put_key("access", storage::new_uref(bit).into());
    // runtime::remove_key("stringvalue");
    runtime::put_key("stringvalue", b.into());

    let ccc = runtime::get_key("stringvalue").unwrap();

    let aaa: URef = runtime::get_key("stringvalue").unwrap().try_into().unwrap();
    // storage::add(aaa, 2);

    // let a: i32 = storage::read(aaa).unwrap().unwrap();
}
