# zkML Inference with SP1

Perform verifiable ML inference tasks (e.g., image classification) with [SP1](https://github.com/succinctlabs/sp1).

This project contains an SP1 program that receives an input image (JPEG, PNG, etc.) and outputs:

  1. An image class prediction using a simple MLP (multi-layer perceptron) model.
  2. An [IPFS content identifier](https://docs.ipfs.tech/concepts/content-addressing/) (CID).

We use the [Succinct Prover Network](https://docs.succinct.xyz/generating-proofs/prover-network.html)
to generate a cryptographic proof that binds the class prediction to the IPFS CID. Thus, we prove (cryptographically) that an IPFS image (e.g., an NFT) has a certain class prediction.
Moreover, the proof can be verified onchain by an EVM app.

## Sounds familiar?

There are a few [other projects](https://github.com/worldcoin/awesome-zkml?tab=readme-ov-file#codebases) that
do ML inference in ZK proofs. What's different about this one is we don't design or use ZK circuits
explicitly. Instead, we use a general-purpose programming language (e.g., Rust) and leave it to
SP1 to generate the ZK proofs.

_**If you can write Rust, you can write ZKP programs!**_

## Application

Perform basic computer vision tasks such as classification, recognition, transformations, etc. on
IPFS-backed NFT images and verify the results onchain. In this repo, we implement a simple
image classifier based on the [Fashion MNIST](https://github.com/zalandoresearch/fashion-mnist)
dataset, but other tasks can be implemented in a similar way.

The IPFS CID acts both as a serving URI (e.g., what to display) and as a commitment of the image
content. An onchain contract must match the NFT's `tokenURI()` property (e.g. `ipfs://...`) with
the proof.

SP1 can generate cryptographic proofs that are small in size and cheap to verify onchain with
[`SP1Verifier`](https://github.com/succinctlabs/sp1-contracts/tree/main).

## Limitations

The toy model used here is inadequate for real-world applications. Larger and more capable vision
models, on the other hand, might be impractical to implement this way.

You must use Succinct's [Prover Network](https://docs.succinct.xyz/generating-proofs/prover-network.html)
to generate proofs; it would be too slow to generate proofs locally with commodity hardware.

The IPFS CID must be generated with the exact same specifications: v0 + DAG-PB + SHA2-256.

## Model

We use a simple MLP (multi-layer perceptron) trained on the [Fashion MNIST](https://github.com/zalandoresearch/fashion-mnist)
dataset. It projects images to 10 classes (e.g. t-shirt, trouser, pullover, etc.).

For simplicity, we use pre-trained model weights from [Hugging Face](https://huggingface.co/sadhaklal/mlp-fashion-mnist/tree/main).
Training the model is outside the scope of this project.

![Fashion MNIST](https://github.com/zalandoresearch/fashion-mnist/raw/master/doc/img/fashion-mnist-sprite.png)

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

See [table of class labels](https://github.com/zalandoresearch/fashion-mnist?tab=readme-ov-file#labels).

The script doesn't upload to IPFS, and uploading isn't necessary to generate the proof. The
content identifier (CID) is derived from the image file; should anyone upload the file to IPFS,
the CID will be the same.

### Prove

```sh
export RUST_LOG=info
export SP1_PRIVATE_KEY=0x...

SP1_PROVER=network cargo run --release -- prove --input-image examples/sneaker_0.jpg
```

### Prove for EVM

Generate a (PLONK) proof that can be verified onchain:

```sh
SP1_PROVER=network cargo run --release -- evm --input-image examples/sneaker_0.jpg
```

## Workspace

- `classifier`: implements MLP
- `classifier-io`: preprocesses input image and generates IPFS CID
- `program`: ZKP SP1 program
- `program-tests`: integration tests for `program`
- `script`: CLI entry point to execute and prove `program`
- `examples`: example input images
- `notebooks`: one-off Jupyter notebooks to export the model weights and inspect inputs
