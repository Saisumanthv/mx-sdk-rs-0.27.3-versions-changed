# Preparations

# Setting up the local testnet

The following examples rely on having a [local testnet](https://docs.dharitri.com/developers/setup-local-testnet/) up and running.

# Installing @dharitrinetwork/erdjs globally

```bash
cd ./code/dharitri-sdk-erdjs
npm run compile && npm test && npm install -g
```

# How to start a node terminal

By exporting `NODE_PATH`, the node terminal should have access to `erdjs`.
Open a terminal and enter the following:

```bash
cd ./code/dharitri-wasm-rs
export NODE_PATH=$HOME/.nvm/versions/node/$(node --version)/lib/node_modules
node --experimental-repl-await
```

# Basic DCT usage

- [Fungible Tokens (FTs)](dct-FT-fungible-tokens.md)
- [Semi-Fungible Tokens (SFTs)](dct-SFT-semi-fungible-tokens.md)
- [Non-Fungible Tokens (NFTs)](dct-NFT-non-fungible-tokens.md)

# Smart contract examples

- Adder [interaction](../../../contracts/examples/adder/interaction/Adder.erdjs.md)
- Crowdfunding DCT [MOAX interaction](../../../contracts/examples/crowdfunding-dct/interaction/Crowdfunding-moax.erdjs.md), [DCT interaction](../../../contracts/examples/crowdfunding-dct/interaction/Crowdfunding-dct.erdjs.md)
- Multisig [MOAX adder interaction](../../../contracts/examples/multisig/interaction/Multisig-adder-moax.erdjs.md)
- Ping-pong [MOAX interaction](../../../contracts/examples/ping-pong-moax/interaction/Ping-pong-moax.erdjs.md)
