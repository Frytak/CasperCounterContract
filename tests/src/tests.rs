#[cfg(test)]
mod tests {
    use casper_engine_test_support::{
        ExecuteRequestBuilder, InMemoryWasmTestBuilder, DEFAULT_ACCOUNT_ADDR,
        PRODUCTION_RUN_GENESIS_REQUEST,
    };
    use casper_types::{runtime_args, ContractHash, RuntimeArgs, U512};

    const COUNTER_WASM: &str = "counter.wasm";
    const COUNTER_CALL_WASM: &str = "temporary-purse.wasm";

    const KEY_CONTRACT: &str = "counter";
    const KEY_COUNT: &str = "count";

    /// Deploys a contract version to the InMemoryWasmTestBuilder
    fn deploy_contract(builder: &mut InMemoryWasmTestBuilder, wasm_code: &str) -> ContractHash {
        let request =
            ExecuteRequestBuilder::standard(*DEFAULT_ACCOUNT_ADDR, wasm_code, runtime_args! {})
                .build();
        builder.exec(request).expect_success().commit();
        get_contract_hash_from_account(builder, KEY_CONTRACT)
    }

    /// Retrieves the contract hash from the default account's storage by a given key
    fn get_contract_hash_from_account(
        builder: &mut InMemoryWasmTestBuilder,
        key: &str,
    ) -> ContractHash {
        builder
            .get_expected_account(*DEFAULT_ACCOUNT_ADDR)
            .named_keys()
            .get(key)
            .expect("must have contract hash key")
            .into_hash()
            .map(ContractHash::new)
            .expect("must get contract hash")
    }

    /// Retrieves the value stored under the `count` key
    fn get_count(builder: &mut InMemoryWasmTestBuilder, contract_hash: ContractHash) -> u64 {
        let count_key = *builder
            .get_contract(contract_hash)
            .expect("this contract should exist")
            .named_keys()
            .get(KEY_COUNT)
            .expect("count uref should exist in the contract named keys");

        builder
            .query(None, count_key, &[])
            .expect("should be stored value.")
            .as_cl_value()
            .expect("should be cl value.")
            .clone()
            .into_t::<u64>()
            .expect("should be u64.")
    }

    #[test]
    fn install_and_test_version1() {
        let mut builder = InMemoryWasmTestBuilder::default();
        builder
            .run_genesis(&PRODUCTION_RUN_GENESIS_REQUEST)
            .commit();

        let contract_v1_hash = deploy_contract(&mut builder, COUNTER_WASM);

        // `count` should be 0 after deploying the contract
        let count = get_count(&mut builder, contract_v1_hash);
        assert_eq!(count, 0);
        
        // Use the `temporary_purse` contract
        let contract_increment_request = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            COUNTER_CALL_WASM,
            runtime_args! {
                "amount" => U512::from(2_000_000_000)
            },
        )
        .build();

        builder
            .exec(contract_increment_request)
            .expect_success()
            .commit();

        // `count` should be 1
        let count = get_count(&mut builder, contract_v1_hash);
        assert_eq!(count, 1);
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
