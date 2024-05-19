<p align="center">
  <img src="logo.png" alt="Logo">
</p>

# Clarity - Circuit Source Code Lookup

## Overview

When exploring smart contract platforms like Etherscan, users often encounter contracts identified by cryptographic hashes, such as those used in Cairo programs on StarkNet OS. Retrieving the corresponding source code can be difficult.

This project aims to provide developer-friendly tooling that allows users to look up the source code of a circuit from its identifier. The identifier can be a Program Hash in the case of Cairo programs, a verifying key in Halo2, gnark, Noir, etc.

## Features

- **Lookup by Circuit Hash**: Retrieve the source code using cryptographic identifiers.
- **Support for Multiple Platforms**: Cairo programs (StarkNet), Halo2, gnark, Noir, and more.
- **Developer-Friendly**: Easy-to-use tooling for developers working with smart contracts and cryptographic programs.
