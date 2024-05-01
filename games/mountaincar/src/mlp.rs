use candle_core::{safetensors, Module, Tensor};
use itertools::Itertools;
use rl::{Agent, MarkovDecisionProcess};
use std::convert::TryFrom;
use std::error::Error;
use std::iter::FromIterator;
use std::path::Path;
use std::usize;
use std::{collections::HashMap, u32};

use crate::aibrain::EncodedAgent;
use crate::mountaincar::{self, MountainAction, MountainCar};

pub struct MultiLayerPerceptron<const I: usize, const O: usize> {
    pub layers: Vec<candle_nn::Linear>,
}

impl<const I: usize, const O: usize> MultiLayerPerceptron<I, O> {
    pub fn new(
        vs: candle_nn::VarBuilder,
        intern_layers_sizes: &[usize],
    ) -> candle_core::error::Result<Self> {
        let mut nn = Self {
            layers: Vec::with_capacity(2 + intern_layers_sizes.len()),
        };

        // Push the first internal layer
        nn.layers.push(candle_nn::linear(
            I,
            intern_layers_sizes[0],
            vs.pp(0.to_string()),
        )?);

        // Push the
        for (i, w) in intern_layers_sizes.windows(2).enumerate() {
            nn.layers
                .push(candle_nn::linear(w[0], w[1], vs.pp((i + 1).to_string()))?)
        }
        nn.layers.push(candle_nn::linear(
            *intern_layers_sizes.last().unwrap(),
            O,
            vs.pp(intern_layers_sizes.len().to_string()),
        )?);

        Ok(nn)
    }

    pub fn forward(&self, xs: candle_core::Tensor) -> candle_core::error::Result<Tensor> {
        let n = self.layers.len();
        let logits = self.layers[..n - 1]
            .iter()
            .try_fold(xs, |acc, l| l.forward(&acc)?.relu())?;
        let logits = self.layers[n - 1].forward(&logits)?;
        Ok(logits)
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

impl<T: mountaincar::Ground> Agent<MountainCar<T>> for MultiLayerPerceptron<2, 3> {
    fn policy(&self, e: &MountainCar<T>) -> Result<MountainAction, Box<dyn Error>> {
        let logits = self.forward(e.feature())?;
        let probs = candle_nn::ops::softmax(&logits, 1)?;
        let i_max = probs.argmax(0)?.to_scalar::<u32>()?;
        if i_max == 0 {
            Ok(MountainAction::Left)
        } else if i_max == 2 {
            Ok(MountainAction::Right)
        } else {
            Ok(MountainAction::DoNothing)
        }
    }
}

impl<const I: usize, const O: usize> TryFrom<&mut HashMap<String, Tensor>>
    for MultiLayerPerceptron<I, O>
{
    type Error = &'static str;

    fn try_from(h: &mut HashMap<String, Tensor>) -> Result<Self, Self::Error> {
        let mut mlp = MultiLayerPerceptron::<I, O> {
            layers: Vec::with_capacity(h.len()),
        };

        for (w, b) in h.keys().cloned().sorted().tuples() {
            mlp.layers.push(candle_nn::Linear::new(
                h.remove(&w).unwrap(),
                Some(h.remove(&b).unwrap()),
            ));
        }
        Ok(mlp)
    }
}

impl EncodedAgent for MultiLayerPerceptron<2, 3> {}
