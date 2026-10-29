| cell | variant | d | agent | games | win % | rounds | first new corruption (mean round; % of games) | cities corrupted at end | destroyed per war round | Unity tips | Discord tips | std actions/game | ends |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0 | c2 | 0 | greedy | 16 | 100 | 6.0 | 4.0; 44% | 0.00 | 0.32 | 1.94 | 6.31 | 14.12 | win purge 16 |
| 1 | c2 | 0 | engine | 16 | 100 | 7.3 | 4.8; 81% | 0.00 | 0.65 | 2.19 | 9.62 | 13.81 | win purge 16 |
| 2 | c2 | 4 | greedy | 16 | 0 | 3.4 | 2.3; 69% | 2.38 | 0.22 | 0.00 | 4.62 | 4.56 | loss supply 16 |
| 3 | c2 | 4 | engine | 16 | 0 | 3.2 | 2.3; 75% | 2.50 | 0.18 | 0.06 | 4.12 | 4.38 | loss supply 16 |
| 4 | c3 | 0 | greedy | 16 | 100 | 5.1 | 4.1; 44% | 0.00 | 1.80 | 1.06 | 5.44 | 15.38 | win purge 16 |
| 5 | c3 | 0 | engine | 16 | 100 | 6.8 | 4.3; 69% | 0.00 | 2.15 | 1.56 | 9.25 | 18.69 | win purge 16 |
| 6 | c3 | 4 | greedy | 16 | 6 | 3.8 | 2.1; 94% | 2.94 | 2.05 | 0.06 | 4.94 | 7.38 | loss supply 15, win purge 1 |
| 7 | c3 | 4 | engine | 16 | 6 | 3.4 | 2.1; 94% | 3.12 | 1.82 | 0.06 | 3.75 | 6.19 | loss supply 15, win purge 1 |

Adversary ambassadors on the board after the corrupt step, mean over games still running (games):

| cell | variant | d | agent | r1 | r2 | r3 | r4 | r5 | r6 | r7 | r8 | r9 | r10 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0 | c2 | 0 | greedy | 6.8 (16) | 8.2 (16) | 6.4 (16) | 8.0 (16) | 7.9 (12) | 7.6 (7) | 6.8 (5) | 6.5 (4) | 4.0 (3) | 7.0 (1) |
| 1 | c2 | 0 | engine | 6.8 (16) | 8.2 (16) | 6.4 (16) | 7.7 (16) | 8.3 (15) | 7.7 (14) | 5.8 (10) | 4.5 (8) | 3.8 (4) | 7.0 (2) |
| 2 | c2 | 4 | greedy | 8.8 (16) | 11.2 (16) | 12.3 (7) | 15.0 (1) |  |  |  |  |  |  |
| 3 | c2 | 4 | engine | 8.8 (16) | 11.1 (15) | 11.4 (7) | 14.5 (2) |  |  |  |  |  |  |
| 4 | c3 | 0 | greedy | 10.2 (16) | 11.9 (16) | 10.1 (16) | 12.7 (15) | 12.6 (9) | 10.6 (5) | 16.0 (2) | 4.5 (2) |  |  |
| 5 | c3 | 0 | engine | 10.2 (16) | 11.6 (16) | 10.9 (16) | 12.9 (16) | 14.2 (14) | 12.4 (14) | 11.2 (11) | 2.0 (3) | 0.0 (1) | 3.0 (1) |
| 6 | c3 | 4 | greedy | 13.2 (16) | 16.1 (16) | 21.7 (9) | 21.5 (2) | 20.0 (1) | 20.0 (1) | 5.0 (1) |  |  |  |
| 7 | c3 | 4 | engine | 13.2 (16) | 15.2 (15) | 22.2 (5) | 20.0 (1) | 22.0 (1) | 17.0 (1) | 5.0 (1) |  |  |  |

Standard actions per game, by kind:

- 0 c2 d0 greedy: remove 8.62, move 2.19, data_centre 1.69, align 1.62
- 1 c2 d0 engine: remove 6.31, align 3.25, data_centre 2.31, move 1.75, sell 1.19, station 0.12, ambassador 0.06
- 2 c2 d4 greedy: remove 2.69, move 1.81, align 0.06
- 3 c2 d4 engine: remove 2.88, move 1.38, sell 0.25, align 0.12
- 4 c3 d0 greedy: remove 9.38, move 4.19, data_centre 1.44, align 0.38
- 5 c3 d0 engine: remove 9.31, align 3.44, move 3.25, data_centre 2.69, sell 0.81
- 6 c3 d4 greedy: remove 4.06, move 3.25, align 0.06
- 7 c3 d4 engine: remove 3.00, move 2.50, align 0.44, data_centre 0.25, sell 0.19
