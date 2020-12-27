# Substrate Node - Wiv Supply Chain

A Substrate node including Wiv Supply Chain Pallet
The WiV Supply Chain Pallet allows users to securely collect and store information about each unique asset’s provenance and transaction history, whilst holding and insuring the unique asset in a professionally managed storage facility.


## Getting Started

In brief, you should install Rust Language a few required packages and it will build with 2 simple commands.
You must have knowledge about command line usage.
The node should work in any operating system compatible with the [Rust](https://www.rust-lang.org/) programming language.

### Rust Setup

Setup instructions for working with the [Rust](https://www.rust-lang.org/) programming language can
be found at the
[Substrate Developer Hub](https://substrate.dev/docs/en/knowledgebase/getting-started). Follow those
steps to install [`rustup`](https://rustup.rs/) and configure the Rust toolchain to default to the
latest stable version.

### Makefile

This project uses a [Makefile](Makefile) to document helpful commands and make it easier to execute
them. Get started by running these [`make`](https://www.gnu.org/software/make/manual/make.html)
targets:

1. `make init` - Run the [init script](scripts/init.sh) to configure the Rust toolchain for
   [WebAssembly compilation](https://substrate.dev/docs/en/knowledgebase/getting-started/#webassembly-compilation).
1. `make run` - Build and launch this project in development mode.

The init script and Makefile both specify the version of the
[Rust nightly compiler](https://substrate.dev/docs/en/knowledgebase/getting-started/#rust-nightly-toolchain)
that this project depends on.

### Build

The `make run` command will perform an initial build. Use the following command to build the node
without launching it:

```sh
make build
```

### Embedded Docs

Once the project has been built, the following command can be used to explore all parameters and
subcommands:

```sh
./target/release/node-wivsupplychain -h
```

## Run

The `make run` command will launch a temporary node and its state will be discarded after you
terminate the process. After the project has been built, there are other ways to launch the node.

## Interaction with the blockchain
For a raw interface, you can install the front end user interface for Substrate project.
you need Yarn, which itself requires Node.js. If you don't have these tools, you may install them from these instructions:

Install [Node.js](https://nodejs.org/en/download/)
Install [Yarn](https://yarnpkg.com/lang/en/docs/install/)

Clone the code from github with the following commands:
```sh
git clone -b v2.0.0 --depth 1 https://github.com/substrate-developer-hub/substrate-front-end-template
cd substrate-front-end-template
yarn install
```

### Node

A blockchain node is an application that allows users to participate in a blockchain network.
Substrate-based blockchain nodes expose a number of capabilities:

-   Networking: Substrate nodes use the [`libp2p`](https://libp2p.io/) networking stack to allow the
    nodes in the network to communicate with one another.
-   Consensus: Blockchains must have a way to come to
    [consensus](https://substrate.dev/docs/en/knowledgebase/advanced/consensus) on the state of the
    network. Substrate makes it possible to supply custom consensus engines and also ships with
    several consensus mechanisms that have been built on top of
    [Web3 Foundation research](https://research.web3.foundation/en/latest/polkadot/NPoS/index.html).
-   RPC Server: A remote procedure call (RPC) server is used to interact with Substrate nodes.

After the node has been [built](#build), refer to the embedded documentation to learn more about the
capabilities and configuration parameters that it exposes:

```shell
./target/release/node-wivsupplychain --help
```

### Run in Docker

First, install [Docker](https://docs.docker.com/get-docker/) and
[Docker Compose](https://docs.docker.com/compose/install/).

Then run the following command to start a single node development chain.

```bash
./scripts/docker_run.sh
```

This command will firstly compile your code, and then start a local development network. You can
also replace the default command (`cargo build --release && ./target/release/node-wivsupplychain --dev --ws-external`)
by appending your own. A few useful ones are as follow.

```bash
# Run Substrate node without re-compiling
./scripts/docker_run.sh ./target/release/node-wivsupplychain --dev --ws-external

# Purge the local dev chain
./scripts/docker_run.sh ./target/release/node-wivsupplychain purge-chain --dev

# Check whether the code is compilable
./scripts/docker_run.sh cargo check
```
