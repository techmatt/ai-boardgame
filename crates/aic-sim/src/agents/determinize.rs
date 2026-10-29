//! Determinization: the agents' answer to hidden information. A state holds
//! the true order of every shuffled deck and the dice (`rng`) that will
//! shuffle them again; a search that simulated the state as it is would
//! peek. Before simulating, an agent replaces everything the printed game
//! hides with a guess drawn from its own RNG, consistent with what players
//! can see:
//!
//! - the player-card, threat and Unity/Discord decks: their contents are
//!   known (every card not seen elsewhere), their order is not, so they are
//!   shuffled;
//! - the adversary deck: its size and the stage of each position are public
//!   (the stages are stacked), next round's card (`adversary.next`) is face
//!   up, and the top `adversary.peek` cards are shown (Recon Sweep). Every
//!   other position gets a card of its stage drawn from the printed cards of
//!   that stage not seen face up, current, in the discard or the peek: the
//!   game may deal a subset of each stage, so which ones is hidden too;
//! - every RNG stream is reseeded, so later reshuffles are guesses too.
//!
//! A pending choice's snapshot gets the same guess, so answering it replays
//! the partly resolved step consistently.

use aic_core::modules::adversary;
use aic_core::{GameState, Rules};
use rand::seq::SliceRandom;
use rand::RngCore;
use rand_chacha::ChaCha8Rng;

/// Piles whose order is hidden but whose contents are known.
pub(crate) const SHUFFLED: [&str; 8] = [
    "cards.deck.1",
    "cards.deck.2",
    "threats.deck.1",
    "threats.deck.2",
    "alignment.deck.unity.1",
    "alignment.deck.unity.2",
    "alignment.deck.discord.1",
    "alignment.deck.discord.2",
];

/// Replaces the hidden parts of `g` with a sample drawn from `rng`.
pub fn determinize(rules: &Rules, g: &mut GameState, rng: &mut ChaCha8Rng) {
    crate::prof::time(crate::prof::Part::Determinize, || guess(rules, g, rng))
}

fn guess(rules: &Rules, g: &mut GameState, rng: &mut ChaCha8Rng) {
    let original = g.clone();
    for key in SHUFFLED {
        if let Some(pile) = g.piles.get_mut(key) {
            // Sorted first, so the guess depends on the contents (public)
            // and the agent's RNG, never on the true order.
            pile.sort_unstable();
            pile.shuffle(rng);
        }
    }
    resample_adversary_deck(rules, g, rng);
    let seeds: Vec<(String, u64)> = g.rng.keys().map(|k| (k.clone(), rng.next_u64())).collect();
    for (purpose, seed) in &seeds {
        g.rng
            .insert(purpose.clone(), aic_core::rng::Stream::new(*seed, purpose));
    }
    if let Some(p) = g.pending.as_mut() {
        let snap = std::sync::Arc::make_mut(&mut p.snapshot);
        // Usually the step that asked has not drawn, so its snapshot's
        // hidden piles are the same as the state's: give them the same
        // guess. A step may draw before it asks: then the snapshot's pile is the drawn
        // cards (now public) on top of the state's, and gets them on top of
        // the same guess, so replaying draws them again. Any other
        // difference gets its own guess.
        for key in SHUFFLED.iter().copied().chain([adversary::DECK]) {
            let (was, now) = (snap.pile(key), original.pile(key));
            if was == now {
                match g.piles.get(key) {
                    Some(v) => snap.piles.insert(key.to_string(), v.clone()),
                    None => snap.piles.remove(key),
                };
            } else if was.len() > now.len() && was.ends_with(now) {
                let mut pile = was[..was.len() - now.len()].to_vec();
                pile.extend(g.piles.get(key).into_iter().flatten().cloned());
                snap.piles.insert(key.to_string(), pile);
            } else if was.len() > now.len() && was.starts_with(now) {
                let mut pile: Vec<String> = g.piles.get(key).cloned().unwrap_or_default();
                pile.extend(was[now.len()..].iter().cloned());
                snap.piles.insert(key.to_string(), pile);
            } else if key == adversary::DECK {
                resample_adversary_deck(rules, snap, rng);
            } else if let Some(pile) = snap.piles.get_mut(key) {
                pile.sort_unstable();
                pile.shuffle(rng);
            }
        }
        for (purpose, seed) in &seeds {
            snap.rng
                .insert(purpose.clone(), aic_core::rng::Stream::new(*seed, purpose));
        }
    }
}

fn resample_adversary_deck(rules: &Rules, g: &mut GameState, rng: &mut ChaCha8Rng) {
    let w = rules.world();
    let content = w.content;
    let stage =
        |id: &str| content.adversary[content.adversary_index(id).expect("adversary card")].stage;
    let deck = g.pile(adversary::DECK).to_vec();
    if deck.is_empty() {
        return;
    }
    let peek = (g.var(adversary::PEEK).max(0) as usize).min(deck.len());
    let mut seen: Vec<&str> = deck[..peek].iter().map(|s| s.as_str()).collect();
    for key in [adversary::NEXT, adversary::CURRENT, adversary::DISCARD] {
        seen.extend(g.pile(key).iter().map(|s| s.as_str()));
    }
    let mut out = deck[..peek].to_vec();
    let mut pools: Vec<Vec<String>> = (1..=3u8)
        .map(|s| {
            let mut v: Vec<String> = content
                .adversary
                .iter()
                .filter(|c| c.stage == s && !seen.contains(&c.id.as_str()))
                .map(|c| c.id.clone())
                .collect();
            v.shuffle(rng);
            v
        })
        .collect();
    for id in &deck[peek..] {
        let s = stage(id) as usize;
        // The pool always has a card: the true one is in it.
        out.push(pools[s - 1].pop().unwrap_or_else(|| id.clone()));
    }
    g.piles.insert(adversary::DECK.to_string(), out);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agents::tests::{config_rules, rules};
    use rand::SeedableRng;

    #[test]
    fn keeps_public_information() {
        for d in [0, 4] {
            let r = rules(d);
            for seed in 0..20 {
                let g = r.new_game(seed);
                let mut h = g.clone();
                determinize(&r, &mut h, &mut ChaCha8Rng::seed_from_u64(seed + 99));
                assert_eq!(g.hexes, h.hexes);
                assert_eq!(g.vars, h.vars);
                assert_eq!(g.seats, h.seats);
                for (k, v) in &g.piles {
                    let mut a = v.clone();
                    let mut b = h.pile(k).to_vec();
                    if k == adversary::DECK {
                        // Same stage at every position.
                        let w = r.world();
                        let st = |id: &String| {
                            w.content.adversary[w.content.adversary_index(id).unwrap()].stage
                        };
                        assert_eq!(
                            a.iter().map(st).collect::<Vec<_>>(),
                            b.iter().map(st).collect::<Vec<_>>()
                        );
                        continue;
                    }
                    if !SHUFFLED.contains(&k.as_str()) {
                        assert_eq!(&a, &b, "{k}");
                    }
                    a.sort();
                    b.sort();
                    assert_eq!(a, b, "{k}");
                }
                // Legal actions are the same: the agent faces the same decision.
                assert_eq!(r.legal_actions(&g), r.legal_actions(&h));
            }
        }
    }

    /// Two tables that differ only in what is hidden (deck order, which
    /// adversary cards were dealt, the dice) give the same guess.
    #[test]
    fn guess_ignores_hidden_information() {
        let r = rules(0);
        for seed in 0..10 {
            let a = r.new_game(seed);
            // Swap a hidden adversary card for an unseen one of its stage.
            let w = r.world();
            let st = |id: &str| w.content.adversary[w.content.adversary_index(id).unwrap()].stage;
            let last = a.pile(adversary::DECK).last().unwrap().clone();
            let all: Vec<String> = a.piles.values().flatten().cloned().collect();
            let other = w
                .content
                .adversary
                .iter()
                .find(|c| c.stage == st(&last) && !all.contains(&c.id))
                .map(|c| c.id.clone());
            // The same hidden difference on the table and, mid-resolution,
            // on the pending choice's snapshot (one table, seen twice).
            let hide = |g: &mut GameState| {
                for key in SHUFFLED {
                    if let Some(p) = g.piles.get_mut(key) {
                        p.reverse();
                    }
                }
                for (purpose, s) in g.rng.iter_mut() {
                    *s = aic_core::rng::Stream::new(seed + 1000, purpose);
                }
                if let (Some(other), Some(deck)) = (&other, g.piles.get_mut(adversary::DECK)) {
                    for id in deck.iter_mut().filter(|id| **id == last) {
                        *id = other.clone();
                    }
                }
            };
            let mut b = a.clone();
            hide(&mut b);
            if let Some(p) = b.pending.as_mut() {
                hide(std::sync::Arc::make_mut(&mut p.snapshot));
            }
            assert_ne!(a, b);
            let mut ga = a.clone();
            let mut gb = b.clone();
            determinize(&r, &mut ga, &mut ChaCha8Rng::seed_from_u64(5));
            determinize(&r, &mut gb, &mut ChaCha8Rng::seed_from_u64(5));
            assert_eq!(ga, gb, "seed {seed}");
        }
    }

    /// In the middle of a resolution (a pending choice), answering on the
    /// guess plays out on the same public table as answering on the truth,
    /// up to the next hidden draw (the round's end, or a tip).
    /// Fast lane: seeds 0..8; the slow lane plays 8..40.
    #[test]
    fn pending_choices_replay_on_the_guess() {
        for name in ["baseline", "shared_row"] {
            pending_choices_replay(&config_rules(name, 0), 0..8);
        }
    }

    #[test]
    #[ignore = "slow lane: more seeds of the pending-choice guess check"]
    fn pending_choices_replay_on_the_guess_more_seeds() {
        for name in ["baseline", "shared_row"] {
            pending_choices_replay(&config_rules(name, 0), 8..40);
        }
    }

    fn pending_choices_replay(r: &Rules, seeds: std::ops::Range<u64>) {
        let mut found = 0;
        let mut legal = Vec::new();
        for seed in seeds {
            let mut g = r.new_game(seed);
            let mut k = seed;
            while !g.is_over() {
                r.legal_actions_into(&g, &mut legal);
                if g.pending.is_some() {
                    found += 1;
                    let mut h = g.clone();
                    determinize(r, &mut h, &mut ChaCha8Rng::seed_from_u64(k));
                    for a in &legal {
                        let mut x = g.clone();
                        let mut y = h.clone();
                        r.apply_mut(&mut x, a, None).unwrap();
                        r.apply_mut(&mut y, a, None)
                            .expect("the guess accepts every answer");
                        // Until the round ends or a bucket tips (a tip
                        // draws from a hidden Unity or Discord deck).
                        let tips = |s: &GameState| {
                            (s.var("alignment.unity_tips"), s.var("alignment.discord_tips"))
                        };
                        if x.round == g.round && tips(&x) == tips(&g) && tips(&y) == tips(&g) {
                            assert_eq!(
                                (&x.hexes, &x.vars, &x.seats),
                                (&y.hexes, &y.vars, &y.seats),
                                "seed {seed}, {a:?}"
                            );
                        }
                    }
                }
                k = k
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                let i = (k >> 33) as usize % legal.len();
                r.apply_mut(&mut g, &legal[i].clone(), None).unwrap();
            }
        }
        assert!(found > 20, "only {found} pending choices met");
    }

    #[test]
    fn hides_the_order() {
        let r = rules(0);
        let g = r.new_game(7);
        let differ = (0..10).any(|i| {
            let mut h = g.clone();
            determinize(&r, &mut h, &mut ChaCha8Rng::seed_from_u64(i));
            h.pile("cards.deck.1") != g.pile("cards.deck.1")
        });
        assert!(differ);
    }
}
