use alloy_sol_types::SolType;
use clap::{Parser, ValueEnum};
use classifier_io::{prepare_input_image, PublicValuesStruct};
use serde::{Deserialize, Serialize};
use sp1_sdk::{HashableKey, ProverClient, SP1ProofWithPublicValues, SP1Stdin, SP1VerifyingKey};
use tracing::instrument;

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const PROGRAM_ELF: &[u8] = include_bytes!("../../../elf/riscv32im-succinct-zkvm-elf");

/// The arguments for the command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    command: Command,
    #[clap(long, required = true)]
    input_image: String,
}

#[derive(Clone, Debug, ValueEnum)]
#[clap(rename_all = "kebab-case")]
enum Command {
    Execute,
    Prove,
    Evm,
}

#[instrument(skip_all)]
fn execute(client: ProverClient, stdin: SP1Stdin, program_input: &[u8]) {
    let (output, report) = client.execute(PROGRAM_ELF, stdin).run().unwrap();
    tracing::info!(
        total_instruction_count = report.total_instruction_count(),
        "success"
    );
    // Check the output.
    let decoded = PublicValuesStruct::abi_decode(output.as_slice(), true).unwrap();
    assert_eq!(decoded.label, get_expected_label(program_input));
    tracing::info!("output values are correct");
    println!("{decoded:#?}");
}

#[instrument(skip_all)]
fn generate_proof(client: ProverClient, stdin: SP1Stdin) {
    // Set up the program for proving.
    let (pk, vk) = client.setup(PROGRAM_ELF);
    // Generate the proof.
    let proof = client
        .prove(&pk, stdin)
        .run()
        .expect("failed to generate proof");
    tracing::info!("proof generated");
    // Verify the proof.
    client.verify(&proof, &vk).expect("failed to verify proof");
    tracing::info!("proof verified");
    // Print public values.
    let decoded = PublicValuesStruct::abi_decode(proof.public_values.as_slice(), true).unwrap();
    println!("{decoded:#?}");
}

#[instrument(skip_all)]
fn generate_evm_proof(client: ProverClient, stdin: SP1Stdin) {
    let (pk, vk) = client.setup(PROGRAM_ELF);
    let proof = client
        .prove(&pk, stdin)
        .plonk()
        .run()
        .expect("failed to generate proof");
    tracing::info!("proof generated");
    write_plonk_test_fixture(&proof, &vk);
}

/// Write test fixture to stdout; copy it into your (Forge) project.
fn write_plonk_test_fixture(proof: &SP1ProofWithPublicValues, vk: &SP1VerifyingKey) {
    let bytes = proof.public_values.as_slice();
    let PublicValuesStruct { ipfsUri, label } =
        PublicValuesStruct::abi_decode(bytes, false).unwrap();
    let fixture = SP1ProofFixture {
        ipfs_uri: ipfsUri,
        label,
        program_vkey: vk.bytes32().to_string(),
        public_values: format!("0x{}", hex::encode(bytes)),
        proof: format!("0x{}", hex::encode(proof.bytes())),
    };
    println!("{}", serde_json::to_string_pretty(&fixture).unwrap());
}

fn main() {
    // Set up the logger.
    sp1_sdk::utils::setup_logger();

    // Parse the command line arguments.
    let args = Args::parse();

    // Setup the prover client.
    let client = ProverClient::new();

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();
    let image_file_content = std::fs::read(args.input_image).unwrap();
    tracing::debug!(size = image_file_content.len(), "read input image");
    stdin.write_slice(&image_file_content);

    match args.command {
        Command::Execute => execute(client, stdin, &image_file_content),
        Command::Prove => generate_proof(client, stdin),
        Command::Evm => generate_evm_proof(client, stdin),
    }
}

fn get_expected_label(image_file_content: &[u8]) -> u8 {
    let input = prepare_input_image(
        image_file_content,
        classifier::INPUT_WIDTH,
        classifier::INPUT_HEIGHT,
    )
    .unwrap();
    classifier::predict_mlp(input)
}

/// A fixture that can be used to test the verification of SP1 zkVM proofs inside Solidity.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SP1ProofFixture {
    ipfs_uri: String,
    label: u8,
    program_vkey: String,
    public_values: String,
    proof: String,
}
