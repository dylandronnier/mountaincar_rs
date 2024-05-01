use std::{
    error::Error,
    fmt::{self, Debug, Display},
    u32,
};

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

    // One step forward for the Markov decision process
    fn step(&mut self, a: Self::Action, t: f32) -> Result<f32, Box<dyn Error>>;

    // Indicate if the MDP is at the terminal state.
    fn is_finished(&self) -> bool;

    // The feature
    fn feature(&self) -> Tensor;
}

pub trait Agent<T>
where
    T: MarkovDecisionProcess,
{
    fn policy(&self, s: &T) -> Result<T::Action, Box<dyn Error>>;
    fn play_game(&self, e: &mut T, time_step: Option<f32>) -> Result<f32, Box<dyn Error>> {
        let mut total_reward = 0.0;
        let time_step = time_step.unwrap_or(0.1);
        while !e.is_finished() {
            let a = self.policy(e)?;
            total_reward += e.step(a, time_step)?;
        }
        Ok(total_reward)
    }
    fn evaluate(
        &self,
        e: &mut T,
        nb_games: Option<u32>,
        time_step: Option<f32>,
    ) -> Result<f32, Box<dyn Error>> {
        let n = nb_games.unwrap_or(1_000);
        let mut res = 0.0;
        for _ in 1..n {
            e.reset();
            res += self.play_game(e, time_step)?;
        }
        Ok(res)
    }
}
