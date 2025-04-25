# nada

[![nada](https://github.com/bestinslot-xyz/nada-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/bestinslot-xyz/nada-rs/actions)

Compression-focused encoding for zero-heavy Solidity calldata and bytecode.

`nada` provides an efficient way to encode and decode byte arrays where runs of `0x00` are replaced with a compact marker (`0xFF`) followed by the length of the run. This is particularly useful for reducing the size of calldata and bytecode in environments like Ethereum.

## How it Works

- `0xFF 0x00` encodes a literal `0xFF`
- `0xFF N` (1 ≤ N ≤ 255) encodes `N` zero bytes
- All other bytes are passed through unchanged

This encoding helps reduce the size of sequences with a high proportion of zero bytes, which are common in Solidity calldata and bytecode.

### Example

| Input                          | Encoded                          |
|--------------------------------|----------------------------------|
| `[0x01, 0x00, 0x00, 0xFF]`    | `[0x01, 0xFF, 0x02, 0xFF, 0x00]` |


## Installation

To add `nada` to your project, use

```bash
> cargo add nada
```

### Usage
Here is a simple example of how to use the encode and decode functions:

```rust
let data = vec![0x01, 0x00, 0x00, 0xFF];
let encoded = nada::encode(&data);
let decoded = nada::decode(&encoded);

assert_eq!(decoded, Ok(data));
```

`decode` returns a `DecodeError` if the input ends unexpectedly, such as when a `0xFF` marker is found without a following run length byte, indicating incomplete or malformed encoded data.

### License

Apache License, Version 2.0
