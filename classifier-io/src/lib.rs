mod input;
mod ipfs;

use alloy_sol_types::sol;

sol! {
    #[derive(Debug)]
    struct PublicValuesStruct {
        string ipfsUri;
        uint8 label;
    }
}

pub use input::prepare_input_image;
pub use ipfs::{generate_ipfs_cid_v0, ipfs_uri};
