# Design principles

The standards every module and every rules change is judged against.

## Pillars

- **Cooperative, against an escalating adversary** (Spirit Island × Terraforming Mars).
- **A real, printable board game.** Every rule is runnable by hand, and the digital
  version adds nothing a physical copy couldn't have.
- **Easy to model, with a predictable future.** The adversary seems smart but moves few
  tokens, makes no free choices, and its threats are visible ahead.
- **Adaptation over one strong general strategy**, especially at high difficulty.
- **Light, rewarding cooperation.** Helping is optional and limited, costs the helper,
  and nets more than it costs. Engines are never tightly interlinked.
- **Scale "for each country"**, with as few "X per player" rules as possible.
- **Complexity earns its place.**

## Difficulty targets

- **Difficulty 0:** random play almost never wins; moderately skilled play essentially
  always wins.
- **Top difficulty:** the strongest agent wins consistently, but narrowly.
- **In between:** win rate falls smoothly with difficulty, a stronger team always does
  better, and the skill gap widens as difficulty rises.

## Design lenses for every module

- strategic: situational, never always right
- modelable without heavy mental simulation
- engine depth
- fun
- table cost: tokens moved, upkeep steps
- scales naturally across 1–5 players
- rewards adaptation
- pulls players together
- readable timing
- printable

## Measures per module, over many simulated games

- win rate by skill tier × difficulty, and the skill gap at each level;
- the effect of removing the module;
- decision closeness (few forced moves);
- pick and play rates (no dead cards, no must-picks);
- build variety among wins, and an adaptation index (a fixed build loses ground against
  some adversaries);
- win probability by round (uncertain into the late game) and margin of victory (mostly
  narrow);
- game length (8–10 rounds) and adversary operations per round (low);
- help and counter usage;
- luck share (most outcome variance comes from decisions, not the shuffle).

Agents get no information or help the printed game couldn't give players.

The agents (random, greedy, beam, rollout) and how each measure is computed are in the
README (*Agents*, *Experiments*); `aic-sim experiment` writes them up as a cycle report.
