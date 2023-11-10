use crate::mlp::MultiLayerPerceptron;
use std::fmt::{self, Debug, Display};

use std::mem;
use strum::EnumCount;

#[derive(Debug, Clone)]
pub struct NotAllowed<A>
where
    A: Debug,
{
    pub a: A,
}

impl<A> fmt::Display for NotAllowed<A>
where
    A: Display + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Action {} is not allowed in the current state.", self.a)
    }
}

pub trait Mdp<const D: usize> {
    type Action: Debug + PartialEq + EnumCount;
    fn reset(&mut self);
    fn step(&mut self, a: Self::Action, t: f32) -> Result<f32, NotAllowed<Self::Action>>;
    fn is_finished(&self) -> bool;
    fn feature(&self) -> [f32; D];
}

pub trait Agent<const D: usize, T>
where
    T: Mdp<D>,
{
    fn policy(&self, s: &T) -> T::Action;
    fn play_game(&self, e: &mut T) -> Result<f32, NotAllowed<T::Action>> {
        let mut reward = 0.0;
        while !e.is_finished() {
            let a = self.policy(e);
            reward += e.step(a, 0.1)?;
        }
        Ok(reward)
    }
}

impl<const D: usize, T> Agent<D, T>
    for MultiLayerPerceptron<D, { mem::variant_count::<<T as Mdp>::Action>() }>
where
    T: Mdp<D>,
{
    fn policy(&self, s: &T) -> <T as Mdp<D>>::Action {
        let i = self
            .forward(&candle_core::Tensor::from_slice(&s.feature(), D, &self.device).unwrap())
            .unwrap()
            .argmax(0)
            .unwrap()
            .to_scalar::<u32>()
            .unwrap() as u8;
        i as <T as Mdp<D>>::Action
    }
}

#[test]
fn test() {}
