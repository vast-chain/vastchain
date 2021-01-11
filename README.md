
## Table of Contents

1. [Description](#chapter-001)
2. [Technical Overview](#chapter-002)
3. [Building](#chapter-003)<br>
  3.1 [Building Dependencies](#chapter-0031)<br>
  3.2 [Building from Source Code](#chapter-0032)<br>
  3.3 [Simple One-Line Installer for Mac and Linux](#chapter-0033)<br>
  3.4 [Starting Vast](#chapter-0034)
4. [Testing](#chapter-004)
5. [Documentation](#chapter-005)
6. [Toolchain](#chapter-006)
7. [Community](#chapter-007)
8. [Contributing](#chapter-008)
9. [License](#chapter-009)


## 1. Description <a id="chapter-001"></a>

**Built for mission-critical use**: Miners, service providers, and exchanges need fast synchronisation and maximum uptime. Vast provides the core infrastructure essential for speedy and reliable services.

- Clean, modular codebase for easy customisation
- Advanced CLI-based client
- Minimal memory and storage footprint
- Synchronise in hours, not days with Warp Sync
- Modular for light integration into your service or product

## 2. Technical Overview <a id="chapter-002"></a>

Vast's goal is to be the fastest, lightest, and most secure Vast client. We are developing Vast using the sophisticated and cutting-edge **Rust programming language**. Vast is licensed under the GPLv3 and can be used for all your Vast needs.

By default, Vast runs a JSON-RPC HTTP server on port `:8545` and a Web-Sockets server on port `:8546`. This is fully configurable and supports a number of APIs.

If you run into problems while using Vast, check out the [wiki for documentation](https://wiki.vast.io/), feel free to [file an issue in this repository](https://github.com/vasttech/vast-vast/issues/new), or hop on our [Gitter](https://gitter.im/vasttech/vast) or [Riot](https://riot.im/app/#/group/+vast:matrix.vast.io) chat room to ask a question. We are glad to help! **For security-critical issues**, please refer to the security policy outlined in [SECURITY.md](SECURITY.md).

Vast's current beta-release is 2.6. You can download it at [the releases page](https://github.com/vasttech/vast-vast/releases) or follow the instructions below to build from source. Please, mind the [CHANGELOG.md](CHANGELOG.md) for a list of all changes between different versions.

## 3. Building <a id="chapter-003"></a>

### 3.1 Build Dependencies <a id="chapter-0031"></a>

Vast requires **latest stable Rust version** to build.

We recommend installing Rust through [rustup](https://www.rustup.rs/). If you don't already have `rustup`, you can install it like this:

- Linux:
  ```bash
  $ curl https://sh.rustup.rs -sSf | sh
  ```

  Vast also requires `gcc`, `g++`, `pkg-config`, `file`, `make`, and `cmake` packages to be installed.

- OSX:
  ```bash
  $ curl https://sh.rustup.rs -sSf | sh
  ```

  `clang` is required. It comes with Xcode command line tools or can be installed with homebrew.

- Windows:
  Make sure you have Visual Studio 2015 with C++ support installed. Next, download and run the `rustup` installer from
  https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe, start "VS2015 x64 Native Tools Command Prompt", and use the following command to install and set up the `msvc` toolchain:
  ```bash
  $ rustup default stable-x86_64-pc-windows-msvc
  ```

Once you have `rustup` installed, then you need to install:
* [Perl](https://www.perl.org)
* [Yasm](https://yasm.tortall.net)

Make sure that these binaries are in your `PATH`. After that, you should be able to build Vast from source.

### 3.2 Build from Source Code <a id="chapter-0032"></a>

```bash
# download Vast code
$ git clone https://github.com/vasttech/vast-vast
$ cd vast-vast

# build in release mode
$ cargo build --release --features final
```

This produces an executable in the `./target/release` subdirectory.

Note: if cargo fails to parse manifest try:

```bash
$ ~/.cargo/bin/cargo build --release
```

Note, when compiling a crate and you receive errors, it's in most cases your outdated version of Rust, or some of your crates have to be recompiled. Cleaning the repository will most likely solve the issue if you are on the latest stable version of Rust, try:

```bash
$ cargo clean
```

This always compiles the latest nightly builds. If you want to build stable or beta, do a

```bash
$ git checkout stable
```

or

```bash
$ git checkout beta
```

### 3.3 Simple One-Line Installer for Mac and Linux <a id="chapter-0033"></a>

```bash
bash <(curl https://get.vast.io -L)
```

The one-line installer always defaults to the latest beta release. To install a stable release, run:

```bash
bash <(curl https://get.vast.io -L) -r stable
```

### 3.4 Starting Vast <a id="chapter-0034"></a>

#### Manually

To start Vast manually, just run

```bash
$ ./target/release/vast
```

so Vast begins syncing the Vast blockchain.

#### Using `systemd` service file

To start Vast as a regular user using `systemd` init:

1. Copy `./scripts/vast.service` to your
`systemd` user directory (usually `~/.config/systemd/user`).
2. Copy release to bin folder, write `sudo install ./target/release/vast /usr/bin/vast`
3. To configure Vast, write a `/etc/vast/config.toml` config file, see [Configuring Vast](https://vasttech.github.io/wiki/Configuring-Vast) for details.

## 4. Testing <a id="chapter-004"></a>

Download the required test files: `git submodule update --init --recursive`. You can run tests with the following commands:

* **All** packages
  ```
  cargo test --all
  ```

* Specific package
  ```
  cargo test --package <spec>
  ```

Replace `<spec>` with one of the packages from the [package list](#package-list) (e.g. `cargo test --package evmbin`).

You can show your logs in the test output by passing `--nocapture` (i.e. `cargo test --package evmbin -- --nocapture`)

## 5. Documentation <a id="chapter-005"></a>

Official website: https://vast.io

Be sure to [check out our wiki](https://wiki.vast.io) for more information.

### Viewing documentation for Vast packages

You can generate documentation for Vast Rust packages that automatically opens in your web browser using [rustdoc with Cargo](https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html#using-rustdoc-with-cargo) (of the The Rustdoc Book), by running the the following commands:

* **All** packages
  ```
  cargo doc --document-private-items --open
  ```

* Specific package
  ```
  cargo doc --package <spec> -- --document-private-items --open
  ```

Use`--document-private-items` to also view private documentation and `--no-deps` to exclude building documentation for dependencies.

Replacing `<spec>` with one of the following from the details section below (i.e. `cargo doc --package vast-vast --open`):

<a id="package-list"></a>
**Package List**
<details><p>

* Vast (VastCore) Client Application
  ```bash
  vast-vast
  ```
* Vast Account Management, Key Management Tool, and Keys Generator
  ```bash
  vastcore-accounts, vastkey-cli, vaststore, vaststore-cli
  ```
* Vast Chain Specification
  ```bash
  chainspec
  ```
* Vast CLI Signer Tool & RPC Client
  ```bash
  cli-signer vast-rpc-client
  ```
* Vast Vastash & ProgPoW Implementations
  ```bash
  vastash
  ```
* Vast (VastCore) Library
  ```bash
  vastcore
  ```
  * Vast Blockchain Database, Test Generator, Configuration,
Caching, Importing Blocks, and Block Information
    ```bash
    vastcore-blockchain
    ```
  * Vast (VastCore) Contract Calls and Blockchain Service & Registry Information
    ```bash
    vastcore-call-contract
    ```
  * Vast (VastCore) Database Access & Utilities, Database Cache Manager
    ```bash
    vastcore-db
    ```
  * Vast Virtual Machine (EVM) Rust Implementation
    ```bash
    evm
    ```
  * Vast (VastCore) Light Client Implementation
    ```bash
    vastcore-light
    ```
  * Vast Smart Contract based Node Filter, Manage Permissions of Network Connections
    ```bash
    node-filter
    ```
  * Vast Private Transactions
    ```bash
    vastcore-private-tx
    ```
  * Vast (VastCore) Client & Network Service Creation & Registration with the I/O Subsystem
    ```bash
    vastcore-service
    ```
  * Vast (VastCore) Blockchain Synchronization
    ```bash
    vastcore-sync
    ```
  * Vast Common Types
    ```bash
    common-types
    ```
  * Vast Virtual Machines (VM) Support Library
    ```bash
    vm
    ```
  * Vast WASM Interpreter
    ```bash
    wasm
    ```
  * Vast WASM Test Runner
    ```bash
    pwasm-run-test
    ```
  * Vast EVM Implementation
    ```bash
    evmbin
    ```
  * Vast IPFS-compatible API
    ```bash
    vast-ipfs-api
    ```
  * Vast JSON Deserialization
    ```bash
    vastjson
    ```
  * Vast State Machine Generalization for Consensus Engines
    ```bash
    vast-machine
    ```
* Vast (VastCore) Miner Interface
  ```bash
  vastcore-miner vast-local-store price-info vastcore-stratum using_queue
  ```
* Vast (VastCore) Logger Implementation
  ```bash
  vastcore-logger
  ```
* C bindings library for the Vast client
  ```bash
  vast-clib
  ```
* Vast JSON-RPC Servers
  ```bash
  vast-rpc
  ```
* Vast (VastCore) Secret Store
  ```bash
  vastcore-secretstore
  ```
* Vast Updater Service
  ```bash
  vast-updater vast-hash-fetch
  ```
* Vast Core Libraries (Vast Util)
  ```bash
  vastcore-bloom-journal blooms-db dir eip-712 fake-fetch fastmap fetch vastcore-io
  journaldb keccak-hasher len-caching-lock macros memory-cache memzero
  migration-rocksdb vastcore-network vastcore-network-devp2p panic_hook
  patricia-trie-vast registrar rlp_compress rlp_derive vast-runtime stats
  time-utils triehash-vast unexpected vast-version
  ```

</p></details>

### Contributing to documentation for Vast packages

[Document source code](https://doc.rust-lang.org/1.9.0/book/documentation.html) for Vast packages by annotating the source code with documentation comments.

Example (generic documentation comment):
```markdown
/// Summary
///
/// Description
///
/// # Panics
///
/// # Errors
///
/// # Safety
///
/// # Examples
///
/// Summary of Example 1
///
/// ```rust
/// // insert example 1 code here for use with documentation as tests
/// ```
///
```

## 6. Toolchain <a id="chapter-006"></a>

In addition to the Vast client, there are additional tools in this repository available:

- [evmbin](./evmbin) - Vast EVM Implementation.
- [vaststore](./accounts/vaststore) - Vast Key Management.
- [vastkey](./accounts/vastkey) - Vast Keys Generator.

The following tool is available in a separate repository:
- [vastabi](https://github.com/vasttech/vastabi) - Vast Encoding of Function Calls. [Docs here](https://crates.io/crates/vastabi)
- [whisper](https://github.com/vasttech/whisper) - Vast Whisper-v2 PoC Implementation.

## 7. Community <a id="chapter-007"></a>

### Join the chat!

Questions? Get in touch with us on Gitter:
[![Gitter: Vast](https://img.shields.io/badge/gitter-vast-4AB495.svg)](https://gitter.im/vasttech/vast)
[![Gitter: Vast.js](https://img.shields.io/badge/gitter-vast.js-4AB495.svg)](https://gitter.im/vasttech/vast.js)
[![Gitter: Vast/Miners](https://img.shields.io/badge/gitter-vast/miners-4AB495.svg)](https://gitter.im/vasttech/vast/miners)
[![Gitter: Vast-PoA](https://img.shields.io/badge/gitter-vast--poa-4AB495.svg)](https://gitter.im/vasttech/vast-poa)

Alternatively, join our community on Matrix:
[![Riot: +Vast](https://img.shields.io/badge/riot-%2Bvast%3Amatrix.vast.io-orange.svg)](https://riot.im/app/#/group/+vast:matrix.vast.io)

## 8. Contributing <a id="chapter-008"></a>

An introduction has been provided in the ["So You Want to be a Core Developer" presentation slides by Hernando Castano](http://tiny.cc/contrib-to-vast-vast). Additional guidelines are provided in [CONTRIBUTING](./.github/CONTRIBUTING.md).

### Contributor Code of Conduct

[CODE_OF_CONDUCT](./.github/CODE_OF_CONDUCT.md)

## 9. License <a id="chapter-009"></a>

[LICENSE](./LICENSE)
