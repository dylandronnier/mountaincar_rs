use candle_core::{safetensors, Module, Tensor};
use candle_nn::VarMap;
use std::collections::HashMap;
use std::path::Path;

struct MultiLayerPerceptron {
    layers: Vec<candle_nn::Linear>,
}

impl MultiLayerPerceptron {
    pub fn new(vs: candle_nn::VarBuilder, topology: &[usize]) -> candle_core::error::Result<Self> {
        let mut nn = Self {
            layers: Vec::with_capacity(topology.len()),
        };
        for (i, w) in topology.windows(2).enumerate() {
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
