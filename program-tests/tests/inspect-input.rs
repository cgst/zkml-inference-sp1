use ndarray_npy::WriteNpyExt;
use tempfile::NamedTempFile;

const W: usize = 28;
const H: usize = 28;

#[ignore]
#[test]
fn inspect_input() {
    let src = include_bytes!("../fixtures/trouser_0.jpg");
    let arr = classifier_io::prepare_input_image(src, W, H).unwrap();
    let mut tmp = NamedTempFile::new().unwrap();
    arr.write_npy(&mut tmp).unwrap();
    let (_, path) = tmp.keep().unwrap();
    println!("Wrote prepared input to: {}", path.display());
}
