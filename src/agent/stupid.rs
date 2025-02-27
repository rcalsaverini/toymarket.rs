use crate::agent::agent::Agent;
use rand::prelude::*;

#[derive(Clone)]
pub struct StupidAgent;

impl Agent for StupidAgent {
    fn decide(&self, _history: &[f32]) -> f32 {
        let mut rng = rand::rng();
        (rng.random::<f32>() - 0.5).signum()
    }

    fn learn(&mut self, _history: &[f32]) {}
}
