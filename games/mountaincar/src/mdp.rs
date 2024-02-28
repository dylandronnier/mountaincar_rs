use std::fmt::{self, Debug, Display};

use bevy::app::Plugin;
use candle_core::Tensor;

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

// A trait for Markov Decision processes
pub trait MarkovDecisionProcess {
    // Action type
    type Action: Debug + PartialEq;

    // Reset the MDP to the initial state
    fn reset(&mut self);

    // Step the
    fn step(&mut self, a: Self::Action, t: f32) -> Result<f32, NotAllowed<Self::Action>>;

    // Indicate if the MDP is at the terminal state.
    fn is_finished(&self) -> bool;

    // The feature
    fn feature(&self) -> Tensor;
}

pub trait Render<T>
where
    T: MarkovDecisionProcess,
{
    type Plugin: Plugin;
}

pub trait Agent<T>
where
    T: MarkovDecisionProcess,
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
