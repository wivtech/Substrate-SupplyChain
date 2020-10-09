# Wiv blockchain backend

## Running node

### Run project with docker

```bash
# Run node without re-compiling
./scripts/docker_run.sh ./target/release/node-template --dev --ws-external

# Purge the local dev chain
./scripts/docker_run.sh ./target/release/node-template purge-chain --dev
```


### Run project without docker


```bash
# Compile node
cargo build --release

# Run node without re-compiling
./target/release/node-template --dev
```

## Deploy token contract


### Building contract

Token contract code is in `/smartcontracts/token`.

Run these commands to build contract

```bash
# Compile contract
cargo +nightly contract build

# Generate metadata
cargo +nightly contract generate-metadata
```

### Deploying contract to node

You can deploy contract using [Polkadot Apps](https://polkadot.js.org/apps/).

1. Uploading contract metadata
Please select generated contract wasm and metadata to upload.
<img src="https://res.cloudinary.com/soapbravowork/image/upload/v1602202707/Wiv%20Wiki/0_Uploading_contract_code_lccir9.png" alt="Upload contract metadata">

2. Deploying uploaded contract code
Click Deploy button to deploy contract code.
Please make sure to provide enough endowment.
<img src="https://res.cloudinary.com/soapbravowork/image/upload/v1602202707/Wiv%20Wiki/1_Deploying_Contract_peqg7h.png" alt="Deploy contract code">

3. Copy contract address and update frontend configuration according to the environment using.

