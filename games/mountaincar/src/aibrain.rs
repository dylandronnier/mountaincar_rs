use std::collections::HashMap;
use std::convert::TryFrom;
use std::ops::Add;

use candle_core::{Device, Tensor};
use rfd::FileDialog;
use std::error::Error;

use bevy::prelude::*;

use crate::{
    mdp::Agent,
    mountaincar::{self, MountainAction, MountainCar},
};

#[derive(Resource)]
pub struct SimpleMap {
    q_left: Tensor,
    q_nothing: Tensor,
    q_right: Tensor,
}

impl<T: mountaincar::Ground> Agent<MountainCar<T>> for SimpleMap {
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

impl TryFrom<&mut HashMap<String, Tensor>> for SimpleMap {
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
        Ok(SimpleMap {
            q_left: q1,
            q_nothing: q2,
            q_right: q3,
        })
    }
}

pub fn load_tensor(mut commands: Commands) {
    // Select the device. Try GPU and pick CPU if not found.
    let device = match Device::cuda_if_available(0) {
        Ok(d) => {
            info!("Loading AI brain on the GPU.");
            d
        }
        Err(_) => {
            info!("Failed to find a GPU. Loading AI brain on the CPU.");
            Device::Cpu
        }
    };

    // Picking the file storing the brain
    let Some(file) = FileDialog::new()
        .add_filter("Safetensor file", &["safetensors"])
        .pick_file()
    else {
        error!("No file picked");
        return;
    };
    let Ok(mut h) = candle_core::safetensors::load(file, &device) else {
        error!("Invalid file");
        return;
    };
    let Ok(brain) = SimpleMap::try_from(&mut h) else {
        error!("Invalid");
        return;
    };
    commands.insert_resource(brain);
}
