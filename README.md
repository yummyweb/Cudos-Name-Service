# Cudos Name Service

This is a sample application for the Cudos blockchain. This application demonstrates basic features of CosmWasm and Cudos. To learn how to deploy this application on Cudos, please see `INSTRUCTIONS.md`. To compile this smart contract, do so using `rust-optimiser`, as given below:

```
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.12.6
```

This application is inspired by ENS and other name service applications on the blockchain. It contains a global state, `REGISTRY` which is a key-value store, with the key type being `&str` and the value type is `Domain`, where `Domain` is a `struct` as given below:

```
pub struct Domain {
    pub owner: Addr,
    pub name: String,
    pub ttl: u8,
    pub text_record: TextRecord,
}
```

Each `Domain` contains a text record of the type, `TextRecord`, which is given below:

```
pub struct TextRecord {
    pub url: String,
    pub avatar: String,
    pub email: String,
}
```

