# Simple Smart Contracts

A very simple implementation of a payable `counter` and `temporary-purse` [smart contract](https://en.wikipedia.org/wiki/Smart_contract). `temporary-purse` takes a given amount of motes on installation, deposits it to a temporary purse and safely calls the `counter` contract.

## Usage

### Compile smart contracts

```bash
make build-contracts
```

### Run tests

```bash
make test
```
