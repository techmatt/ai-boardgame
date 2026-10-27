//! Team agents for simulation. One agent drives every seat and sees only
//! what the printed game shows (the whole state is open information).

use aic_core::{Action, Agent, GameState, Rules};
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;

/// Uniform over legal actions. Seeded per game, so results do not depend on
/// thread count or game order. A floor, not a model of a weak player.
pub struct RandomAgent(ChaCha8Rng);

impl RandomAgent {
    pub fn new(game_seed: u64) -> Self {
        let mut rng = ChaCha8Rng::seed_from_u64(game_seed);
        // A stream no engine purpose uses, so the agent never mirrors the game's dice.
        rng.set_stream(0xa6e7_a6e7);
        RandomAgent(rng)
    }
}

impl Agent for RandomAgent {
    fn choose(&mut self, _rules: &Rules, _state: &GameState, legal: &[Action]) -> usize {
        (self.0.next_u64() % legal.len() as u64) as usize
    }
}
