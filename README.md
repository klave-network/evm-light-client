# EVM Light Client Application

This Rust-based template provides a foundation for developing an EVM-compatible Light Client application. It enables efficient interaction with Ethereum-based blockchains, focusing on lightweight on-chain data verification and synchronization.

## Usage

The EVM Light Client application allows users to initialize and update light client instances, facilitating the retrieval and verification of blockchain headers and blocks. Leveraging confidential computing, the application ensures that operations are secure and data integrity is maintained. On-chain data verification is supported, while cross-chain functionalities are not included in this template.

## Prerequisites

To build and use this template, ensure the following tools are installed:

- **Rust Toolchain**: Includes `rust`, `rustup`, and `cargo`. [Install Rust](https://www.rust-lang.org/tools/install)
- **Cargo Component**: Install via `cargo install cargo-component`
- **WASM Target**: Add the target with `rustup target add wasm32-unknown-unknown`

## Wasm Component

Klave applications are built as WebAssembly (WASM) components. The following methods are implemented and exposed in the `evm-light-client` component:

### Light Client Management
- `register-routes`: Registers the available routes.
- `light-client-init`: Initializes the light client.
- `light-client-update`: Updates the light client with the latest headers.
- `light-client-update-for-block-number`: Updates the light client for a specific block number.
- `light-client-update-for-period`: Updates the light client for a specific period.
- `light-client-update-for-slot`: Updates the light client for a specific slot.
- `light-client-fetch-header-from-slot`: Fetches the header from a specific slot.
- `light-client-fetch-block-from-slot`: Fetches the block from a specific slot.
- `light-client-persist`: Persists the current state of the light client.

## Deploying Your App on Klave

To deploy your application on Klave:

1. **Build the Application**:
   ```sh
   cargo component build --target wasm32-unknown-unknown --release
   ```
   This command generates the WASM files in the `target/wasm32-unknown-unknown/release/` directory.

2. **Deploy to Klave**: Follow the deployment instructions provided in the [Klave documentation](https://docs.klave.com/deployment).

## Authors

This template is created by Klave and Secretarium team members, with contributions from:

- Jeremie Labbe ([@jlabbeklavo](https://github.com/jlabbeklavo)) - Klave | Secretarium

For more information and support, refer to the [Klave documentation](https://docs.klave.com) or contact the authors.
