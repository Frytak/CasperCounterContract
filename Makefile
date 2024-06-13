build-contracts:
	cd counter && cargo build --release --target wasm32-unknown-unknown
	cd temporary-purse && cargo build --release --target wasm32-unknown-unknown

	wasm-strip counter/target/wasm32-unknown-unknown/release/counter.wasm 2>/dev/null | true
	wasm-strip temporary-purse/target/wasm32-unknown-unknown/release/temporary-purse.wasm 2>/dev/null | true

test: build-contracts
	mkdir -p tests/wasm
	cp counter/target/wasm32-unknown-unknown/release/counter.wasm tests/wasm
	cp temporary-purse/target/wasm32-unknown-unknown/release/temporary-purse.wasm tests/wasm
	cd tests && cargo test

clean:
	cd counter && cargo clean
	cd temporary-purse && cargo clean
	cd tests && cargo clean
	rm -rf tests/wasm
