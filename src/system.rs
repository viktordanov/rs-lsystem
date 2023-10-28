use std::collections::HashMap;

use crate::Arena;
use crate::token::TokenId;

#[derive(Clone, Debug)]
pub struct LSystem {
    arena: Arena,
    axiom: Vec<TokenId>,
    rules_map: HashMap<TokenId, Vec<TokenId>>,
    state: Vec<TokenId>,
    steps: usize,
}

impl LSystem {
    pub(crate) fn new(
        arena: Arena,
        axiom: Vec<TokenId>,
        rules_map: HashMap<TokenId, Vec<TokenId>>,
    ) -> Self {
        Self {
            arena,
            axiom: axiom.clone(),
            rules_map,
            state: axiom,
            steps: 0,
        }
    }

    pub fn reset(&mut self) {
        self.state = self.axiom.clone();
        self.steps = 0;
    }

    pub fn step(&mut self) {
        let mut next_state = Vec::new();

        for id in self.state.iter() {
            next_state.extend(self.rules_map[id].clone());
        }

        self.state = next_state;
        self.steps += 1;
    }

    pub fn step_by(&mut self, n: usize) {
        for _ in 0..n {
            self.step();
        }
    }

    pub fn steps(&self) -> usize {
        self.steps
    }
    pub fn render(&self) -> String {
        self.state
            .iter()
            .map(|id| self.arena.get_token(id).unwrap().name())
            .collect::<Vec<_>>()
            .join("")
    }

    pub fn get_state(&self) -> &[TokenId] {
        &self.state
    }
}
