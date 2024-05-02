use candle_core::Tensor;
use rl::{Agent, FileLoader};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::error::Error;
use std::ops::Add;

use crate::mountaincar::{self, Ground, MountainAction, MountainCar};

pub struct Tabular {
    q_left: Tensor,
    q_nothing: Tensor,
    q_right: Tensor,
}

impl<T: mountaincar::Ground> Agent<MountainCar<T>> for Tabular {
    fn policy(&self, e: &MountainCar<T>) -> Result<MountainAction, Box<dyn Error>> {
        let i = e.pos.div_euclid(0.05).clamp(0.0, 9.0) as usize;
        let j = 9 - e.speed.add(0.15).div_euclid(0.03).clamp(0.0, 9.0) as usize;
        let q_l = self.q_left.get(i)?.get(j)?.to_scalar::<f64>()?;
        let q_n = self.q_nothing.get(i)?.get(j)?.to_scalar::<f64>()?;
        let q_r = self.q_right.get(i)?.get(j)?.to_scalar::<f64>()?;
        if q_l >= q_n && q_l >= q_r {
            Ok(MountainAction::Left)
        } else if q_r >= q_n && q_r >= q_l {
            Ok(MountainAction::Right)
        } else {
            Ok(MountainAction::DoNothing)
        }
    }
}

impl TryFrom<&mut HashMap<String, Tensor>> for Tabular {
    type Error = &'static str;

    fn try_from(h: &mut HashMap<String, Tensor>) -> Result<Self, Self::Error> {
        let Some(q1) = h.remove("q_left") else {
            return Err("Q_left not in");
        };
        let Some(q2) = h.remove("q_nothing") else {
            return Err("Q_nothing not in");
        };
        let Some(q3) = h.remove("q_right") else {
            return Err("Q_right not in");
        };
        Ok(Tabular {
            q_left: q1,
            q_nothing: q2,
            q_right: q3,
        })
    }
}

impl<T: Ground> FileLoader<MountainCar<T>> for Tabular {}
