# XDR Decoder

Used to convert XDR contract_id into hash and vice versa.

## Usage

Build binary

```bash
cargo build --release
```

Copy the binary into your `$PATH`

```bash
cp ./target/release/xdr_decoder /usr/local/bin/
```

Convert contract id

```bash
xdr_decoder hash-xdr <contract_id hash> -> contract_id XDR
xdr_decoder xdr-hash <contract_id XDR> -> contract_id hash
```
