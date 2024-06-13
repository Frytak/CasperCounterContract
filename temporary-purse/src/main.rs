#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;

use casper_types::{runtime_args::RuntimeArgs, runtime_args, ApiError, ContractHash, Key};

use casper_contract::{contract_api::{runtime, system, account}, unwrap_or_revert::UnwrapOrRevert};

const KEY_COUNTER: &str = "counter";

const ENTRY_POINT_INCREMENT: &str = "increment";

#[no_mangle]
pub extern "C" fn call() {
    // Read the `counter`'s ContractHash.
    let contract_hash = {
        let counter_uref = runtime::get_key(KEY_COUNTER).unwrap_or_revert();
        if let Key::Hash(hash) = counter_uref {
            ContractHash::new(hash)
        } else {
            runtime::revert(ApiError::User(1));
        }
    };

    let main_purse = account::get_main_purse();
    let temp_purse = system::create_purse();
    let amount = runtime::get_named_arg("amount");

    system::transfer_from_purse_to_purse(main_purse, temp_purse, amount, None).unwrap_or_revert();

    runtime::call_contract::<()>(
        contract_hash,
        ENTRY_POINT_INCREMENT,
        runtime_args! {
            "purse" => temp_purse
        }
    );
}
