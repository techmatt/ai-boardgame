| cell | variant | d | agent | games | win % | rounds | Unity tips | Discord tips | ends |
|---|---|---|---|---|---|---|---|---|---|
| 0 | c2 | 0 | greedy | 16 | 0 | 5.1 | 0.81 | 8.06 | loss supply 16 |
| 1 | c2 | 0 | engine | 16 | 0 | 6.6 | 1.44 | 10.12 | loss supply 13, loss deck_empty 3 |
| 2 | c2 | 4 | greedy | 16 | 0 | 2.6 | 0.00 | 3.06 | loss supply 16 |
| 3 | c2 | 4 | engine | 16 | 0 | 2.7 | 0.06 | 2.81 | loss supply 16 |
| 4 | c3 | 0 | greedy | 16 | 0 | 7.1 | 0.88 | 12.00 | loss supply 16 |
| 5 | c3 | 0 | engine | 16 | 0 | 7.7 | 1.44 | 12.00 | loss supply 14, loss deck_empty 2 |
| 6 | c3 | 4 | greedy | 16 | 0 | 2.7 | 0.06 | 2.31 | loss supply 16 |
| 7 | c3 | 4 | engine | 16 | 0 | 2.7 | 0.06 | 2.12 | loss supply 16 |

Per game (means over the cell): presence placed (std action / all routes); node peak per board (mean of each board's peak; the highest board); nodes made / removed; factories built player / adversary / neutral; destroyed; owned at the end player / adversary / neutral; hardware made / bought / spent; adversary compute made.

| cell | variant | d | agent | presence | node peak/board (max) | nodes made / removed | factories built P/A/N | destroyed | owned at end P/A/N | hardware made / bought / spent | adv. compute |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 0 | c2 | 0 | greedy | 3.62 / 9.00 | 3.38 (8) | 5.50 / 1.75 | 1.12 / 3.12 / 4.50 | 6.31 | 0.62 / 3.12 / 2.81 | 2.06 / 0.00 / 0.00 | 21.7 |
| 1 | c2 | 0 | engine | 2.75 / 8.69 | 3.31 (6) | 5.88 / 2.94 | 1.56 / 3.31 / 6.69 | 9.38 | 0.62 / 3.12 / 2.25 | 3.19 / 0.56 / 0.25 | 28.4 |
| 2 | c2 | 4 | greedy | 1.62 / 5.62 | 2.81 (5) | 3.81 / 0.06 | 0.75 / 2.12 / 2.00 | 2.38 | 0.62 / 2.00 / 3.75 | 0.75 / 0.00 / 0.00 | 10.1 |
| 3 | c2 | 4 | engine | 0.94 / 5.12 | 2.88 (6) | 3.75 / 0.12 | 0.69 / 2.19 / 2.00 | 2.38 | 0.56 / 2.06 / 3.81 | 0.81 / 0.00 / 0.00 | 10.5 |
| 4 | c3 | 0 | greedy | 4.75 / 14.94 | 3.96 (9) | 10.62 / 3.88 | 1.56 / 6.00 / 12.44 | 17.81 | 0.69 / 5.88 / 1.81 | 3.88 / 0.00 / 0.00 | 51.8 |
| 5 | c3 | 0 | engine | 4.62 / 15.00 | 3.98 (11) | 11.38 / 4.75 | 3.31 / 6.00 / 13.12 | 19.12 | 1.12 / 5.88 / 2.19 | 5.06 / 0.38 / 0.38 | 53.7 |
| 6 | c3 | 4 | greedy | 2.31 / 7.81 | 2.60 (6) | 5.12 / 0.19 | 0.62 / 2.69 / 2.88 | 3.81 | 0.62 / 2.69 / 5.06 | 0.69 / 0.00 / 0.00 | 15.3 |
| 7 | c3 | 4 | engine | 1.94 / 7.38 | 2.60 (7) | 4.94 / 0.25 | 0.62 / 2.69 / 2.88 | 3.56 | 0.62 / 2.69 / 5.31 | 0.69 / 0.00 / 0.00 | 15.1 |

Standard actions per game, by kind:

- 0 c2 d0 greedy: remove 3.88, presence 3.62, move 2.12, node 1.75, align 0.19
- 1 c2 d0 engine: remove 4.69, ambassador 4.44, node 2.94, presence 2.75, move 2.00, align 1.12, sell 0.62, hardware 0.56, station 0.12
- 2 c2 d4 greedy: remove 1.69, presence 1.62, move 0.44, node 0.06
- 3 c2 d4 engine: remove 1.69, presence 0.94, move 0.69, node 0.12, sell 0.06
- 4 c3 d0 greedy: remove 11.56, presence 4.75, move 4.56, node 3.88, align 0.38
- 5 c3 d0 engine: remove 10.00, ambassador 5.56, node 4.75, presence 4.62, move 3.62, align 1.69, sell 1.44, hardware 0.38, station 0.19
- 6 c3 d4 greedy: presence 2.31, remove 1.19, move 0.56, node 0.19, align 0.06
- 7 c3 d4 engine: presence 1.94, remove 0.94, move 0.88, node 0.25, sell 0.12, align 0.06
