# ZK-ML Inference with SP1

The goal of this project is to demonstrate the use of [SP1](https://github.com/succinctlabs/sp1)
to perform provable ML inference.

The project consists of an SP1 program that receives an input image and emits:

  1. An [IPFS content identifier](https://docs.ipfs.tech/concepts/content-addressing/) (CID).
  2. A class prediction using a simple MLP (multi-layer perceptron) model.

We use the [Succinct Prover Network](https://docs.succinct.xyz/generating-proofs/prover-network.html)
to generate a cryptographic proof that binds the prediction to the IPFS CID. Thus, we can prove
(cryptographically) that an image stored at that IPFS CID has a certain class prediction according
to our model.

## Application

A potential use case is to perform basic computer vision tasks such as classification, recognition,
transformations, etc. on IPFS-backed NFT images and verify the results onchain.

SP1 can generate cryptographic proofs that are small in size and cheap to verify onchain with
[`SP1Verifier`](https://github.com/succinctlabs/sp1-contracts/tree/main). EVM contracts can verify
inference proofs (for ~400k gas) and match the NFT's `tokenURI()` property (e.g. `ipfs://...`).

## Limitations

This is a proof of concept with a toy model. It's really not useful for real-world
applications. More capable vision models might be impractical to prove in this way.

## Model

We use a simple MLP (multi-layer perceptron) trained on the [Fashion MNIST](https://github.com/zalandoresearch/fashion-mnist)
dataset. It can classify images into 10 classes (e.g. t-shirt, trouser, pullover, etc.). We
replicate the model in Rust because it has to run inside the SP1 VM.

![Fashion MNIST](https://github.com/zalandoresearch/fashion-mnist/raw/master/doc/img/fashion-mnist-sprite.png)

For simplicity, we use pre-trained model weights from [Hugging Face](https://huggingface.co/sadhaklal/mlp-fashion-mnist/tree/main).
Training the model is outside the scope of this project.

## Requirements

- [Rust](https://rustup.rs/)
- [SP1](https://succinctlabs.github.io/sp1/getting-started/install.html)

## Running the Project

### Execute (without generating a proof)

To run the program without generating a proof:

```sh
export RUST_LOG=info

cargo run --release -- execute --input-image examples/sneaker_0.jpg
```

This will execute the program and display the public output.

Example output:

```text
PublicValuesStruct {
    ipfsUri: "ipfs://QmUEDCvfPn59SKP6Ub4ynQmHsZy5FwHXB8fypqV8ydkFdU",
    label: 7,
}
```

The image isn't uploaded to IPFS, and uploading isn't necessary for generating the proof. The
content identifier (CID) is derived from the image file; should anyone upload the file to IPFS,
the CID will be the same.

### Generate a Proof

Using Succinct's [Prover Network](https://docs.succinct.xyz/generating-proofs/prover-network.html)
is generally much, much faster than generating a proof locally.

```sh
export RUST_LOG=info
export SP1_PRIVATE_KEY=0x...

SP1_PROVER=network cargo run --release -- prove --input-image examples/sneaker_0.jpg
```

### Generate an EVM-Compatible (PLONK) Proof

To generate a PLONK proof that is small enough to be verified on-chain and verifiable by the EVM:

```sh
SP1_PROVER=network cargo run --release -- evm --input-image examples/sneaker_0.jpg
```

This command also generates a fixture that can be used to test the verification of SP1 zkVM proofs
inside Solidity.

To retrieve the `programVKey` for the on-chain contract, run the following command:

```sh
cargo run -- v-key
```
