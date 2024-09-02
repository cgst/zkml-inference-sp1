use ipfs_unixfs::file::adder::FileAdder;
use tiny_cid::{Cid, Version, DAG_PROTOBUF};
use tiny_multihash::{Code, MultihashCode};

/// Generate v0 IPFS CID from file content, assuming DAG-PB codec and SHA2-256 hashing algorithm.
/// Under these assumptions, the CID is deterministic and ~unique per file.
pub fn generate_ipfs_cid_v0(file_conten: &[u8]) -> String {
    let mut adder = FileAdder::default();
    adder.push(file_conten).0.for_each(|_| {}); // Consume
    let (_, unixfs_repr) = adder.finish().last().unwrap();
    let hash = Code::Sha2_256.digest(&unixfs_repr);
    let cid = Cid::new(Version::V0, DAG_PROTOBUF, hash).unwrap();
    cid.to_string()
}

pub fn ipfs_uri(cid: impl AsRef<str>) -> String {
    format!("ipfs://{}", cid.as_ref())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipfs_cid_derivation() {
        assert_eq!(
            generate_ipfs_cid_v0(include_bytes!("test-fixtures/example.jpg")),
            "QmUEDCvfPn59SKP6Ub4ynQmHsZy5FwHXB8fypqV8ydkFdU"
        );
        assert_eq!(
            generate_ipfs_cid_v0(include_bytes!("test-fixtures/example.png")),
            "QmWtxXq4tEAQ7RpgRcDe89X664HHFtwa8kqYxHsHCUe9Zt"
        );
    }
}
