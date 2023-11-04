use std::fmt;

#[derive(Debug, Clone)]
pub struct UndefinedAction<A>
where
    A: fmt::Debug,
{
    pub a: A,
}

impl<A> fmt::Display for UndefinedAction<A>
where
    A: fmt::Display + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Action {} is not permitted in the current state.",
            self.a
        )
    }
}

pub trait Mdp {
    type Action;
    fn reset(&mut self);
    fn step(&mut self, a: Self::Action, t: f32) -> Result<f32, UndefinedAction<Self::Action>>
    where
        <Self as Mdp>::Action: fmt::Debug;
    fn is_finished(&self) -> bool;
}

pub trait Agent<T>
where
    T: Mdp,
{
    fn policy(&self, s: &T) -> T::Action;
    fn play_game(&self, e: &mut T) -> Result<f32, UndefinedAction<T::Action>>
    where
        <T as Mdp>::Action: fmt::Debug,
    {
        let mut reward = 0.0;
        while !e.is_finished() {
            let a = self.policy(e);
            reward += e.step(a, 0.1)?;
        }
        Ok(reward)
    }
}

#[test]
fn test() {}
