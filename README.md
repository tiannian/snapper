# Snapper Framework

Smart contract development framework in rust.

## Project

Add these folder and files in cargo project.

- contracts: Contracts files
- Snapper.toml: Config file for snapper.
- bins: Some scripts.
- tests: Tests
- src: library

## cargo snapper

### Init project

```shell
$ cargo snapper init

$ cargo snapper new
```

### Build and Run Project

```shell
$ cargo buid

$ cargo run --bin <bin name>
```

### Manage bins

```shell
$ cargo snapper bin new <name>

$ cargo snapper bin remove <name>
```

## Snapper.toml

```toml
[solidity]
version = [""]
viaIR = true

optimizer.enable = true

[library."Locker.sol"]
LockerLib = "0x1234567890"

[networks.localhost]
url = ""
accounts = ""
```

## Generated content

Contract compile result will generate into `target/snapper`

- abi.json
- metadata.json
- undeploy.bytecode
- deployed.bytecode

## Related Project

- snapper-providers
- snapper-ethers
- snapper-signers
- snapper-solc
- snapper-node
- cargo-snapper
