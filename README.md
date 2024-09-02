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

### Generate a proof

```sh
export RUST_LOG=info
export SP1_PRIVATE_KEY=0x...

SP1_PROVER=network cargo run --release -- prove --input-image examples/sneaker_0.jpg
```

### Generate an EVM-compatible proof

Generate a (PLONK) proof that can be verified onchain:

```sh
SP1_PROVER=network cargo run --release -- evm --input-image examples/sneaker_0.jpg
```

This will output a test fixture that can be used to test the verification of SP1 zkVM proofs inside Solidity. Copy the fixture into your Solidity project; e.g., [Foundry](https://github.com/foundry-rs).

Example output:

```text
{
  "ipfsUri": "ipfs://QmUEDCvfPn59SKP6Ub4ynQmHsZy5FwHXB8fypqV8ydkFdU",
  "label": 7,
  "programVkey": "0x006c7912eb97da386a236d3498aa13dda77bff1b6f2ac5476290fe2f5f49131d",
  "publicValues": "0x0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000070000000000000000000000000000000000000000000000000000000000000035697066733a2f2f516d554544437666506e3539534b5036556234796e516d48735a793546774858423866797071563879646b4664550000000000000000000000",
  "proof": "0xc430ff7f2715b253f7429645dc1fbb0a2fccffa7ef6817cef8396b7903933cc41243c7700a1923cb78994a44040007c7e020e775b84b1435fcdb1b21f293cfec61c924fd10bb84694267113fbf2e77c77aae6019ddb13e6c2a0b0d7cc78334f6b139b44c2a2f64db16b5014c06a26c59103db06c3c39d68c1bbdd4a297192b9f5d09225a016ef0fa394d76b803711d97f5acc86e5eed08efbfa5509eeff299a5fc39c7200fcbf0e84da4935e71814d7f585d3c4d7e4df28fa4e0c06bfad90ea3d96a5a05125160643075fc1a4f1cb458c516328e96b63666ebe71e96dfd266698416a03e1784d5b386686f9cc7e92c8d0963eaf9e1997c143ff2558f2467607f5f737ef9026d012c3f90c463edd5c41473ff4dea29fb51280e8fd026f00ee7935bb7f9270d5abb063623551667b1fe141eaf0401288e453f91222da87eb0ad40eb0c42d023a43b11fefb3d1e48485663fa217b028ce5e1445046c0b8b788d33dcd4063da21668f490a488125d06437a5259b1c270c9ba351b1bc3cb320f76913cac35fe621d48d6dcd864c54ae35b143e8475f4ed8cda5617b4a4c3172790cf507a498c604a362a30237718dc67b02476d9a961162e0a21b1a5982b9b165eddab37e51b71b4ede759d7899d2986ea630f1a63133b04b890cdb38cb9e583424a95835090527511cafea7edb50dd70d7589ff2fb7b242e2450c8d4235c31f2e34a1f60851c03acad45b71ca32effede607c3ce5ab65b30271aeaa0e3f5f2d3f09f32f1aba60e1f9943be62e31577b1b67edea6726b876479e1ce5660882158af3661916b3c0b3a3ca2a67583532c6a037a53dabcd9f06c2e0ee7035c6f8f9c06028fde82c91a91daaf21de1f39f0b14e799b7426d4d7d7308cc5529a89adcb48957442df7015f7243058304066914f52ed23a5bd7cec74783c1b792f2a6c0e7dd7f14b717a01681ef929d22cb6daf0e599fce92f3a6dd9e6e81d87a85616792a20ad89f0361211bbafa6730e1baa33db14ccb3e13579ba432c8a38ef10211a4da687293e0821cab726ff08296212ced04661be1041f6f50e21ff96d88bed8e6c8889fc7a8120e42fa4e9099b9be6f2f477634207a2d6b49b793985d075cae5e69d19add27103e36e4a9d952a864742623db9843b39a082100c70ea41ef09edb9ffbf794c612feed2201c7ff657eee5ab7ae15c18230c4cbd9c3e14c500ea99ce563d692b9b"
}
```

## Workspace

- `classifier`: implements MLP
- `classifier-io`: preprocesses input image and generates IPFS CID
- `program`: ZKP SP1 program
- `program-tests`: integration tests for `program`
- `script`: CLI entry point to execute and prove `program`
- `examples`: example input images
- `notebooks`: one-off Jupyter notebooks to export the model weights and inspect inputs
