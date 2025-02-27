use crate::agent::agent::Agent;
use nalgebra::SVector;

#[derive(Clone)]
pub struct PerceptronAgent<const D: usize> {
    pub weights: SVector<f32, D>,
    lambda: f32,
}

impl<const D: usize> PerceptronAgent<D> {
    fn take_history(history: &[f32]) -> SVector<f32, D> {
        let mut data = [0.0; D];
        for (i, &value) in history.iter().rev().take(D).enumerate() {
            data[D - 1 - i] = value;
        }
        SVector::from_row_slice(&data)
    }

    pub fn new(lambda: f32) -> Self {
        Self { weights: SVector::from_element(0.0), lambda }
    }
}

impl<const D: usize> Agent for PerceptronAgent<D> {
    fn decide(&self, history: &[f32]) -> f32 {
        let input = PerceptronAgent::take_history(history);
        let dot = self.weights.dot(&input);
        if dot == 0.0 {
            crate::agent::stupid::StupidAgent.decide(history)
        } else if dot > 0.0 {
            1.0
        } else {
            -1.0
        }
    }

    fn learn(&mut self, history: &[f32]) {
        let input = PerceptronAgent::take_history(&history[..history.len() - 1]);
        let actual_output = history[history.len() - 1];
        let prediction = self.weights.dot(&input).signum();
        let error = actual_output - prediction;
        self.weights += self.lambda * error * input;
    }
}
