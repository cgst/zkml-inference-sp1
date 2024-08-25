use ndarray::{prelude::*, ScalarOperand};
use ndarray_npy::{ReadNpyError, ReadNpyExt, ReadableElement};
use num_traits::Float;

#[derive(Debug)]
pub struct Linear<S> {
    pub input_dim: usize,
    pub output_dim: usize,
    pub weight: Array2<S>,
    pub bias: Option<Array1<S>>,
}

impl<S: Scalar> Linear<S> {
    pub fn new(input_dim: usize, output_dim: usize, bias: bool) -> Self {
        Self {
            input_dim,
            output_dim,
            weight: Array2::zeros((input_dim, output_dim)),
            bias: bias.then(|| Array1::zeros(output_dim)),
        }
    }

    pub fn forward(&self, x: Array1<S>) -> Array1<S> {
        assert_eq!(x.dim(), self.input_dim);
        if let Some(bias) = &self.bias {
            x.dot(&self.weight) + bias
        } else {
            x.dot(&self.weight)
        }
    }

    pub fn load_npy(&mut self, weight: &[u8], bias: Option<&[u8]>) -> Result<(), ReadNpyError> {
        self.weight = Array2::<S>::read_npy(weight)?;
        assert_eq!(self.bias.is_some(), bias.is_some());
        if let Some(bias) = bias {
            self.bias = Some(Array1::<S>::read_npy(bias)?);
        }
        Ok(())
    }
}

pub fn relu<S: Scalar>(x: Array1<S>) -> Array1<S> {
    x.mapv(|elem| match elem < S::zero() {
        true => S::zero(),
        false => elem,
    })
}

pub fn argmax<S: Scalar>(x: Array1<S>) -> usize {
    x.into_iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .unwrap()
        .0
}

pub trait Scalar: Float + ScalarOperand + ReadableElement {}

impl Scalar for f32 {}
impl Scalar for f64 {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relu() {
        assert_eq!(relu(Array1::from_elem(10, 0.0)), Array1::from_elem(10, 0.0));
        assert_eq!(relu(Array1::from_elem(10, 1.0)), Array1::from_elem(10, 1.0));
        assert_eq!(
            relu(Array1::from_elem(10, -1.0)),
            Array1::from_elem(10, 0.0)
        );
    }

    #[test]
    fn test_argmax() {
        assert_eq!(
            argmax(Array1::from_shape_vec((5,), vec![-5.0, 13.0, 37.0, 0.0, -1e5]).unwrap()),
            2
        );
    }
}
