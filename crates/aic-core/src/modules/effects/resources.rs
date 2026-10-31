//! Trades, tokens and what a player's effects build or harden.

use super::*;

pub(super) fn transfer(g: &mut GameState, cx: &mut Cx, ctx: &Ctx, seat: usize, resource: Resource, max: i64, bonus: i64) -> Res {
        if let Some(o) = ctx.other {
            let most = max.min(g.resource(seat, resource)).max(0);
            let opts = (0..=most)
                .map(|k| opt(format!("give {}", resource.amount(k))))
                .collect();
            let prompt = format!("Seat {seat}: give how much {resource} to seat {o}?");
            let k = cx.choose(seat, &prompt, opts)?.unwrap_or(0) as i64;
            if k > 0 {
                cx.add_resource(g, seat, resource, -k);
                cx.add_resource(g, o, resource, k + bonus);
            }
        }
    Ok(())
}

pub(super) fn convert(g: &mut GameState, cx: &mut Cx, seat: usize, from: Resource, to: Resource, max: i64, rate: i64) -> Res {
        let most = max.min(g.resource(seat, from)).max(0);
        let opts = (0..=most)
            .map(|k| opt(format!("convert {}", from.amount(k))))
            .collect();
        let prompt = format!("Seat {seat}: convert how much {from} into {to}?");
        let k = cx.choose(seat, &prompt, opts)?.unwrap_or(0) as i64;
        if k > 0 {
            cx.add_resource(g, seat, from, -k);
            cx.add_resource(g, seat, to, k * rate);
        }
    Ok(())
}

pub(super) fn gain_per_tokens(g: &mut GameState, cx: &mut Cx, seat: usize, resource: Resource, kind: TokenKind, per: i64, max: i64) {
    let w = cx.world;
        let held: i64 = cards::token_cards(&w, g, seat, Some(kind))
            .iter()
            .map(|c| g.var(&cards::tokens_key(seat, c)))
            .sum();
        cx.add_resource(g, seat, resource, (held / per.max(1)).min(max));
}

pub(super) fn cash_out(g: &mut GameState, cx: &mut Cx, seat: usize, kind: TokenKind, resource: Resource, max: i64) {
    let w = cx.world;
        let mut n = 0;
        for c in cards::token_cards(&w, g, seat, Some(kind)) {
            let mut k = g.var(&cards::tokens_key(seat, &c));
            if max > 0 {
                k = k.min(max - n);
            }
            cards::add_tokens(g, cx, seat, &c, -k);
            n += k;
        }
        cx.add_resource(g, seat, resource, n * (n + 1) / 2);
}

pub(super) fn build_factory(g: &mut GameState, cx: &mut Cx, seat: usize) -> Res {
    let w = cx.world;
        let hexes = factory_sites(&w, g, seat);
        let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
        let prompt = format!("Seat {seat}: build a factory on which zone?");
        if let Some(i) = cx.choose(seat, &prompt, opts)? {
            cx.add_tokens(g, hexes[i], FACTORY, 1);
            cx.add_tokens(g, hexes[i], &cube(seat), 1);
            cx.tally(g, "board.factories_built.player", 1);
        }
    Ok(())
}

pub(super) fn harden(g: &mut GameState, cx: &mut Cx, seat: usize, count: i64, in_country: CountryRule) -> Res {
    let w = cx.world;
        let cs = harden_countries(&w, g, seat, in_country);
        let opts = cs.iter().map(|&c| country_opt(&w, c)).collect();
        let prompt = format!("Seat {seat}: harden {count} buildings in which country?");
        if let Some(i) = cx.choose(seat, &prompt, opts)? {
            // 022: only buildings a token can protect, each at most once
            // ("harden 2 buildings" is two buildings; fewer if fewer exist).
            let mut hexes: Vec<usize> = w.map.countries[cs[i]]
                .hexes
                .iter()
                .copied()
                .filter(|&h| board::hardenable(g, h) && in_reach(&w, g, seat, h))
                .collect();
            for k in 0..count {
                let opts = hexes.iter().map(|&h| hex_opt(&w, h)).collect();
                let prompt = format!(
                    "Seat {seat}: hardening {} of {count} on which building?",
                    k + 1
                );
                let Some(j) = cx.choose(seat, &prompt, opts)? else {
                    break;
                };
                cx.add_tokens(g, hexes.remove(j), HARDEN, 1);
            }
        }
    Ok(())
}
