use candle_core::Tensor;
use std::{error::Error, fmt::Debug};

/// A trait for implementing Markov Decision processes.
pub trait MarkovDecisionProcess {
    /// Action type.
    type Action: Debug + PartialEq;

    /// Reset the MDP to its initial state.
    fn reset(&mut self);

    /// Take one step forward for the Markov decision process. As the function simulates a
    /// continuous dynamics, the time step is alsso given as an argument of the function.
    fn step(&mut self, action: Self::Action, time_step: f32) -> Result<f32, Box<dyn Error>>;

    /// Indicate if the MDP has reached the terminal state.
    fn is_finished(&self) -> bool;

    /// Return the current set of the MDP as a feature tensor.
    fn feature(&self) -> Tensor;
}
