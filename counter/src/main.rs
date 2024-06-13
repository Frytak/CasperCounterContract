#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;

use alloc::{
    string::String,
    vec::Vec,
};

use casper_contract::{
    contract_api::{runtime, storage, system},
    unwrap_or_revert::UnwrapOrRevert,
};

use casper_types::{
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, NamedKeys},
    CLType, CLValue, ApiError, URef, runtime_args, RuntimeArgs
};

const KEY_CONTRACT: &str = "counter";
const KEY_CONTRACT_PURSE: &str = "counter_purse";
const KEY_COUNT: &str = "count";

const ENTRY_POINT_INIT: &str = "init";
const ENTRY_POINT_GET: &str = "get";
const ENTRY_POINT_INCREMENT: &str = "increment";

#[no_mangle]
pub extern "C" fn get() {
    let count_uref = runtime::get_key(KEY_COUNT)
        .unwrap_or_revert()
        .into_uref()
        .unwrap_or_revert();

    let count: u64 = storage::read(count_uref)
        .unwrap_or_revert()
        .unwrap_or_revert();

    runtime::ret(CLValue::from_t(count).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn increment() {
    // Cost of 2 CSPRs
    let cost = casper_types::U512::from(2 * 1_000_000_000);

    let count_uref = runtime::get_key(KEY_COUNT)
        .unwrap_or_revert()
        .into_uref()
        .unwrap_or_revert();

    let contract_purse_uref = runtime::get_key(KEY_CONTRACT_PURSE)
        .unwrap_or_revert()
        .into_uref()
        .unwrap_or_revert();

    let user_purse_uref = runtime::get_named_arg::<URef>("purse");
    if !user_purse_uref.access_rights().is_writeable() {
        runtime::revert(ApiError::User(1));
    }

    let balance = system::get_purse_balance(user_purse_uref).unwrap_or_revert();
    if balance < cost {
        runtime::revert(ApiError::User(2));
    }

    let transfer_result = system::transfer_from_purse_to_purse(
        user_purse_uref,
        contract_purse_uref,
        cost,
        None
    );

    match transfer_result {
        Ok(_) => { storage::add(count_uref, 1); }
        Err(err) => { runtime::revert(err); }
    }
}

#[no_mangle]
pub extern "C" fn init() {
    if runtime::get_key(KEY_CONTRACT_PURSE).is_some() {
        runtime::revert(ApiError::User(3));
    }

    runtime::put_key(KEY_CONTRACT_PURSE, system::create_purse().into());
}

#[no_mangle]
pub extern "C" fn call() {
    let mut keys = NamedKeys::new();
    keys.insert(String::from(KEY_COUNT), storage::new_uref(0u64).into());

    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_INIT,
        Vec::new(),
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract
    ));

    entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_GET,
        Vec::new(),
        CLType::U64,
        EntryPointAccess::Public,
        EntryPointType::Contract
    ));

    entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_INCREMENT,
        Vec::new(),
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract
    ));

    let (contract_hash, _) = storage::new_locked_contract(entry_points, Some(keys), None, None);

    runtime::put_key(KEY_CONTRACT, contract_hash.into());

    runtime::call_contract::<()>(contract_hash, ENTRY_POINT_INIT, runtime_args! {});
}
