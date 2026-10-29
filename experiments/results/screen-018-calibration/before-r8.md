# Before: r8 (017) quick screen, `screen-017-projects.json` (scripts/board/screen_017.py)

| cell | variant | d | agent | games | win % | rounds | Unity tips | Discord tips | ends |
|---|---|---|---|---|---|---|---|---|---|
| 0 | c2 | 0 | greedy | 16 | 0 | 4.1 | 0.94 | 7.00 | loss supply 16 |
| 1 | c2 | 0 | engine | 16 | 0 | 3.8 | 1.12 | 5.81 | loss supply 16 |
| 2 | c2 | 4 | greedy | 16 | 0 | 2.2 | 0.12 | 2.81 | loss supply 16 |
| 3 | c2 | 4 | engine | 16 | 0 | 2.4 | 0.38 | 2.88 | loss supply 16 |
| 4 | c3 | 0 | greedy | 16 | 0 | 4.8 | 0.75 | 7.25 | loss supply 16 |
| 5 | c3 | 0 | engine | 16 | 0 | 4.8 | 1.19 | 7.00 | loss supply 16 |
| 6 | c3 | 4 | greedy | 16 | 0 | 2.3 | 0.12 | 2.25 | loss supply 16 |
| 7 | c3 | 4 | engine | 16 | 0 | 2.3 | 0.19 | 2.56 | loss supply 16 |

Per game (means over the cell): projects resolved cheap / big; adversary compute made / blocked; Discord / Unity tips; games with a Unity tip; troop advances (moves) / advance strikes / their destructions; production-strike destructions; first project round (mean over games with one, range; games with none).

| cell | variant | d | agent | projects cheap / big | compute made / blocked | compute a round | Discord / Unity tips | games with Unity tip | advances / strikes / destroyed | strike destroyed | first project round |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 0 | c2 | 0 | greedy | 1.38 / 0.00 | 23.4 / 0.0 | 5.4 | 7.00 / 0.94 | 9/16 | 12.6 / 25.3 / 9.2 | 4.1 | 3.2 (3–4); none 4 |
| 1 | c2 | 0 | engine | 0.81 / 0.00 | 15.4 / 0.0 | 4.1 | 5.81 / 1.12 | 8/16 | 12.2 / 18.3 / 8.9 | 3.4 | 3.5 (3–4); none 5 |
| 2 | c2 | 4 | greedy | 0.50 / 0.00 | 13.4 / 0.0 | 5.7 | 2.81 / 0.12 | 2/16 | 6.3 / 7.7 / 4.2 | 1.6 | 2.0 (2–2); none 10 |
| 3 | c2 | 4 | engine | 0.56 / 0.00 | 13.6 / 0.0 | 5.5 | 2.88 / 0.38 | 5/16 | 6.4 / 7.1 / 4.3 | 1.8 | 2.2 (2–3); none 8 |
| 4 | c3 | 0 | greedy | 1.75 / 0.00 | 43.2 / 0.0 | 8.8 | 7.25 / 0.75 | 9/16 | 19.7 / 28.9 / 15.3 | 5.6 | 3.1 (3–4); none 1 |
| 5 | c3 | 0 | engine | 1.56 / 0.00 | 36.2 / 0.0 | 7.3 | 7.00 / 1.19 | 11/16 | 21.9 / 37.2 / 14.7 | 6.1 | 3.5 (3–5); none 2 |
| 6 | c3 | 4 | greedy | 0.50 / 0.00 | 19.4 / 0.0 | 8.2 | 2.25 / 0.12 | 2/16 | 4.7 / 6.4 / 4.5 | 2.1 | 2.0 (2–2); none 8 |
| 7 | c3 | 4 | engine | 0.56 / 0.00 | 19.5 / 0.0 | 8.2 | 2.56 / 0.19 | 2/16 | 6.9 / 8.5 / 5.1 | 2.6 | 2.1 (2–3); none 8 |

Standard actions by kind and cards played, per game (the strategy-health gate):

| cell | align | move | node | presence | remove | sell | cards played | drain / discard |
|---|---|---|---|---|---|---|---|---|
| 0 | 0.0 | 1.4 | 1.8 | 2.0 | 2.1 | 0.0 | 8.0 | 0.00 / 0.00 |
| 1 | 0.4 | 0.6 | 2.3 | 1.7 | 1.1 | 0.5 | 11.9 | 0.00 / 0.00 |
| 2 | 0.1 | 0.2 | 0.3 | 0.9 | 0.9 | 0.0 | 3.9 | 0.00 / 0.00 |
| 3 | 0.1 | 0.6 | 0.8 | 0.9 | 0.4 | 0.1 | 5.9 | 0.00 / 0.00 |
| 4 | 0.1 | 2.2 | 1.9 | 3.6 | 4.0 | 0.0 | 13.7 | 0.00 / 0.00 |
| 5 | 1.2 | 1.2 | 3.7 | 2.7 | 1.8 | 0.9 | 22.9 | 0.00 / 0.00 |
| 6 | 0.1 | 0.6 | 0.2 | 0.9 | 1.2 | 0.0 | 5.6 | 0.00 / 0.00 |
| 7 | 0.0 | 0.7 | 0.8 | 1.3 | 0.6 | 0.1 | 8.4 | 0.00 / 0.00 |
