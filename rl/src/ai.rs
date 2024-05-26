use crate::mdp::MarkovDecisionProcess;
use candle_core::{Device, Tensor};
use std::{collections::HashMap, convert::TryFrom, error::Error, path::PathBuf, u32};

/// Agent trait for implementing AI that plays a game.
pub trait Agent<T>
where
    T: MarkovDecisionProcess,
{
    /// The only function to define for implementing the trait. Take action given the state of a
    /// Markov decision state.
    fn policy(&self, s: &T) -> Result<T::Action, Box<dyn Error>>;

    /// Play the game until the end and return the total reward.
    fn play_game(&self, e: &mut T, time_step: Option<f32>) -> Result<f32, Box<dyn Error>> {
        let mut total_reward = 0.0;
        let time_step = time_step.unwrap_or(0.1);
        while !e.is_finished() {
            let a = self.policy(e)?;
            total_reward += e.step(a, time_step)?;
        }
        Ok(total_reward)
    }

    /// Monte-Carlo evaluation of the performance of the agent.
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

/// Sub-trait that implements agents loading from safetensors file.
pub trait FileLoader<T: MarkovDecisionProcess>:
    Agent<T> + for<'a> TryFrom<&'a mut HashMap<String, Tensor>>
{
    /// Function that implements the loading of file.
    fn from_file(file: PathBuf) -> Result<Self, ()> {
        // Select the device. Try GPU and pick CPU if not found.
        let device = Device::cuda_if_available(0).unwrap_or(Device::Cpu);

        let Ok(mut h) = candle_core::safetensors::load(file, &device) else {
            return Err(());
        };
        let Ok(brain) = Self::try_from(&mut h) else {
            return Err(());
        };
        Ok(brain)
    }
}
