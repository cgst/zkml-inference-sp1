mod mlp;
mod nn;

pub use mlp::MLP;
use ndarray::*;
use ndarray_npy::ReadNpyError;

pub const INPUT_WIDTH: usize = 28;
pub const INPUT_HEIGHT: usize = 28;
pub const OUTPUT_CLASSES: usize = 10;

pub fn predict_mlp(x: Array2<f32>) -> u8 {
    assert_eq!(x.shape(), &[INPUT_HEIGHT, INPUT_WIDTH]);
    build_mlp()
        .expect("build model")
        .predict(Array::from_iter(x))
}

fn build_mlp() -> Result<MLP<f32>, ReadNpyError> {
    let mut model = MLP::<f32>::new(INPUT_HEIGHT * INPUT_WIDTH, OUTPUT_CLASSES, [300, 100]);
    model.load_npy(
        [
            include_bytes!("../weights/mlp2/fc1_weight.npy"),
            include_bytes!("../weights/mlp2/fc2_weight.npy"),
            include_bytes!("../weights/mlp2/fc3_weight.npy"),
        ],
        [
            include_bytes!("../weights/mlp2/fc1_bias.npy"),
            include_bytes!("../weights/mlp2/fc2_bias.npy"),
            include_bytes!("../weights/mlp2/fc3_bias.npy"),
        ],
    )?;
    Ok(model)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mlp_build() {
        assert!(build_mlp().is_ok());
    }

    #[test]
    fn mlp_predict_doesnt_panic() {
        predict_mlp(Array2::from_elem((INPUT_HEIGHT, INPUT_WIDTH), 0.1337));
    }
}
