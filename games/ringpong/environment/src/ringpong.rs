use std::{convert::TryFrom, error::Error, f32::consts::PI};

use bevy::math::Vec2;
use candle_core::Tensor;
use rand::Rng;
use rl::mdp::MarkovDecisionProcess;

// "Ring Pong" decision process

pub const RADIUS: f32 = 300.0;
pub const THETA: f32 = PI / 12.0;
pub const SPEED: f32 = 100.0;

pub struct RingPong {
    pub ball_pos: Vec2,
    pub ball_speed: Vec2,
    pub paddle_angle: f32,
}

#[derive(Default, Debug, PartialEq)]
pub enum RingPongAction {
    Left = -1,
    Right = 1,
    #[default]
    DoNothing = 0,
}

impl RingPong {
    pub fn new() -> Self {
        let v_x = rand::thread_rng().gen_range(-1.0..1.0);
        RingPong {
            ball_pos: Vec2::new(0.0, 0.0),
            ball_speed: Vec2::new(v_x, f32::sqrt(1.0 - v_x.powi(2))),
            paddle_angle: 0.0,
        }
    }
}

impl Default for RingPong {
    fn default() -> Self {
        RingPong {
            ball_pos: Vec2::new(0.0, 0.0),
            ball_speed: Vec2::new(1.0, 0.0),
            paddle_angle: 0.0,
        }
    }
}

impl MarkovDecisionProcess for RingPong {
    type Action = RingPongAction;

    fn reset(&mut self) {
        let v_x = rand::thread_rng().gen_range(-1.0..1.0);
        self.ball_pos = Vec2::new(0.0, 0.0);
        self.ball_speed = Vec2::new(v_x, f32::sqrt(1.0 - v_x.powi(2)));
        self.paddle_angle = 0.0;
    }
    fn is_finished(&self) -> bool {
        self.ball_pos.length() > RADIUS
    }
    fn step(&mut self, action: Self::Action, time_step: f32) -> Result<f32, Box<dyn Error>> {
        self.paddle_angle += time_step * action as i8 as f32;
        let pad_extremite_1 = RADIUS * Vec2::from_angle(self.paddle_angle - THETA);
        let pad_extremite_2 = RADIUS * Vec2::from_angle(self.paddle_angle + THETA);
        let t = (pad_extremite_1 - self.ball_pos).perp_dot(pad_extremite_2 - pad_extremite_1)
            / (SPEED * time_step * self.ball_speed).perp_dot(pad_extremite_2 - pad_extremite_1);
        if t > 0.0 && t < 1.0 {
            self.ball_pos += SPEED * t * time_step * self.ball_speed;
            self.ball_speed =
                Vec2::from_angle(2.0 * self.paddle_angle + PI - self.ball_speed.to_angle());
            self.ball_pos += SPEED * (1.0 - t) * time_step * self.ball_speed;
        } else {
            self.ball_pos += SPEED * time_step * self.ball_speed;
        }
        Ok(1.0) // Reward 1 at each step for surviving
    }

    fn feature(&self) -> Tensor {
        Tensor::try_from(vec![
            self.ball_pos.x,
            self.ball_pos.y,
            self.ball_speed.x,
            self.ball_speed.y,
            self.paddle_angle,
        ])
        .unwrap()
    }
}
