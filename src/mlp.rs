use candle_core::{safetensors, Module, Tensor};
use std::collections::HashMap;
use std::path::Path;

pub struct MultiLayerPerceptron<const I: usize, const O: usize> {
    pub device: candle_core::Device,
    pub layers: Vec<candle_nn::Linear>,
}

impl<const I: usize, const O: usize> MultiLayerPerceptron<I, O> {
    pub fn new(
        vs: candle_nn::VarBuilder,
        intern_layers_sizes: &[usize],
    ) -> candle_core::error::Result<Self> {
        let mut nn = Self {
            device: &candle_core::Device.cuda_if_available()?,
            layers: Vec::with_capacity(2 + intern_layers_sizes.len()),
        };
        for (i, w) in intern_layers_sizes.windows(2).enumerate() {
            nn.layers
                .push(candle_nn::linear(w[0], w[1], vs.pp(i.to_string()))?)
        }
        Ok(nn)
    }

    pub fn forward(&self, xs: &candle_core::Tensor) -> candle_core::error::Result<Tensor> {
        let t = self
            .layers
            .iter()
            .try_fold(xs.clone(), |acc, l| l.forward(&acc)?.relu())?;
        Ok(candle_nn::ops::softmax(&t, 1)?)
    }

    pub fn save<P: AsRef<Path>>(&self, p: P) -> candle_core::error::Result<()> {
        safetensors::save(
            &HashMap::from_iter(
                (0..self.layers.len())
                    .map(|i| i.to_string())
                    .zip(self.layers.iter().map(|l| l.weight().clone())),
            ),
            p,
        )?;
        Ok(())
    }
}
