use crate::agent::agent::Agents;

pub struct Market {
    pub agents: Agents,
    pub history: Vec<f32>,
    dynamics: Box<dyn (Fn(f32) -> f32) + Send + Sync>,
}

impl Market {
    pub fn new<F>(dynamics: F) -> Market where F: Fn(f32) -> f32 + Send + Sync + 'static {
        Market {
            agents: Agents::new(),
            history: Vec::new(),
            dynamics: Box::new(dynamics),
        }
    }

    pub fn tick(&mut self) {
        let volume = self.agents.trade_volume(&self.history);
        let response = (self.dynamics)(volume);
        self.history.push(response);
    }
}
