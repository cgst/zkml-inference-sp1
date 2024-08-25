use crate::nn::{argmax, relu, Linear, Scalar};
use ndarray::prelude::*;
use ndarray_npy::ReadNpyError;

#[derive(Debug)]
pub struct MLP2<S> {
    pub input_dim: usize,
    pub output_classes: usize,
    pub fc_1: Linear<S>,
    pub fc_2: Linear<S>,
    pub fc_3: Linear<S>,
}

impl<S: Scalar> MLP2<S> {
    pub fn new(input_dim: usize, output_classes: usize, hidden_dim: [usize; 2]) -> Self {
        Self {
            input_dim,
            output_classes,
            fc_1: Linear::new(input_dim, hidden_dim[0], true),
            fc_2: Linear::new(hidden_dim[0], hidden_dim[1], true),
            fc_3: Linear::new(hidden_dim[1], output_classes, true),
        }
    }

    pub fn forward(&self, x: Array1<S>) -> Array1<S> {
        assert_eq!(x.dim(), self.input_dim);
        let x = relu(self.fc_1.forward(x));
        let x = relu(self.fc_2.forward(x));
        self.fc_3.forward(x)
    }

    pub fn predict(&self, x: Array1<S>) -> u8 {
        argmax(self.forward(x)) as u8
    }

    pub fn load_npy(&mut self, weight: [&[u8]; 3], bias: [&[u8]; 3]) -> Result<(), ReadNpyError> {
        self.fc_1.load_npy(weight[0], Some(bias[0]))?;
        self.fc_2.load_npy(weight[1], Some(bias[1]))?;
        self.fc_3.load_npy(weight[2], Some(bias[2]))?;
        Ok(())
    }
}
