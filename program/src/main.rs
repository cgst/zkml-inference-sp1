#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_sol_types::SolType;
use classifier::{INPUT_HEIGHT, INPUT_WIDTH};
use classifier_io::{generate_ipfs_cid_v0, ipfs_uri, prepare_input_image, PublicValuesStruct};

pub fn main() {
    let image_file_content = sp1_zkvm::io::read_vec();
    let input = prepare_input_image(&image_file_content, INPUT_WIDTH, INPUT_HEIGHT)
        .expect("prepare input image");
    let label = classifier::predict_mlp(input);
    let public_out = PublicValuesStruct::abi_encode(&PublicValuesStruct {
        ipfsUri: ipfs_uri(generate_ipfs_cid_v0(&image_file_content)),
        label,
    });
    sp1_zkvm::io::commit_slice(&public_out);
}
