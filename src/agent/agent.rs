use std::sync::{ Arc, Mutex };

pub trait Agent: Send + Sync {
    fn decide(&self, history: &[f32]) -> f32;
    fn learn(&mut self, history: &[f32]);
}

pub struct Agents {
    agents: Vec<Arc<Mutex<dyn Agent>>>,
}

impl Agents {
    pub fn new() -> Self {
        Self { agents: Vec::new() }
    }

    pub fn register<A: Agent + 'static>(&mut self, agent: A) {
        self.agents.push(Arc::new(Mutex::new(agent)));
    }

    pub fn trade_volume(&self, history: &[f32]) -> f32 {
        self.agents
            .iter()
            .map(|agent| agent.lock().unwrap().decide(history))
            .sum()
    }

    pub fn learn(&self, history: &[f32]) {
        for agent in &self.agents {
            agent.lock().unwrap().learn(history);
        }
    }
}

impl Agents {
    pub fn populate_market(&mut self, perceptron_count: usize, stupid_count: usize) {
        for _ in 0..perceptron_count {
            self.register(crate::agent::perceptron::PerceptronAgent::<2>::new(0.01));
        }
        for _ in 0..stupid_count {
            self.register(crate::agent::stupid::StupidAgent);
        }
    }
}
