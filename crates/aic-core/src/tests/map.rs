use super::*;

#[test]
fn every_layout_builds_and_every_piece_follows_the_design() {
    let content = crate::content();
    for n in 1..=4u8 {
        let map = Map::build(content, n).unwrap();
        assert_eq!(map.hexes.len(), 16 * n as usize);
        for c in 0..n as usize {
            let p = &content.map.countries[c];
            let at = |num: u8| map.numbered(c, num);
            assert_eq!(p.cities.len(), 3, "country {c}");
            assert!(p.corrupted.len() >= 2, "country {c}: two cities to corrupt at difficulty 4");
            assert_eq!(p.stations.len(), 1, "country {c}");
            assert_eq!(p.factories.len(), 2, "country {c}");
            assert_eq!(p.data_centres.len(), 2, "country {c}");
            assert!(p.ocean.len() <= 2, "country {c}");
            let count = |k: HexKind| map.countries[c].hexes.iter().filter(|&&h| kind(&map, h) == k).count();
            assert_eq!(count(HexKind::City), 3);
            assert_eq!(count(HexKind::Ocean), p.ocean.len());
            for &f in &p.factories {
                let h = at(f);
                assert!(
                    map.hexes[h].neighbours.iter().any(|&x| map.hexes[x].country as usize == c && kind(&map, x) == HexKind::City),
                    "country {c}: factory #{f} touches no city of its piece"
                );
            }
            for &o in &p.ocean {
                assert!(map.hexes[at(o)].neighbours.len() < 6, "country {c}: ocean #{o} is inland with {n} countries");
            }
            let [a, b] = [at(p.data_centres[0]), at(p.data_centres[1])];
            assert!(map.adjacent(a, b), "country {c}: the starting data centres are apart");
        }
    }
}

#[test]
fn every_front_has_a_base_on_each_side_and_every_country_a_front() {
    let content = crate::content();
    for n in 1..=4u8 {
        let map = Map::build(content, n).unwrap();
        let land = |h: usize| kind(&map, h) != HexKind::Ocean;
        for f in &map.fronts {
            assert_ne!(f.countries[0], f.countries[1]);
            for i in 0..2 {
                let (b, me, other) = (f.bases[i], f.countries[i], f.countries[1 - i]);
                let cell = &map.hexes[b];
                let p = &content.map.countries[me as usize];
                assert_eq!((cell.country, cell.kind, cell.facing), (me, HexKind::Base, Some(other)));
                assert_eq!(p.kind_at(cell.number), HexKind::Open, "{n} countries: base on a printed city or ocean");
                for list in [&p.stations, &p.factories, &p.data_centres] {
                    assert!(!list.contains(&cell.number), "{n} countries: base on a setup building");
                }
                assert!(cell.neighbours.iter().any(|&x| map.hexes[x].country == other && land(x)));
            }
        }
        // Exactly the pairs of countries that touch by land have a front.
        for a in 0..n {
            for b in a + 1..n {
                let touch = (0..map.hexes.len()).any(|h| {
                    map.hexes[h].country == a
                        && land(h)
                        && map.hexes[h].neighbours.iter().any(|&x| map.hexes[x].country == b && land(x))
                });
                let front = map.fronts.iter().any(|f| f.countries == [a, b]);
                assert_eq!(touch, front, "{n} countries: {a}-{b}");
            }
        }
        assert_eq!(map.hexes.iter().filter(|c| c.kind == HexKind::Base).count(), 2 * map.fronts.len());
        if n == 1 {
            assert!(map.fronts.is_empty());
        }
        if n == 2 || n == 3 {
            for c in 0..n {
                assert!(map.fronts.iter().any(|f| f.countries.contains(&c)), "{n} countries: country {c} has no front");
            }
        }
    }
}

#[test]
fn an_ocean_edge_is_not_a_front() {
    let mut content = crate::content().clone();
    let map = Map::build(&content, 2).unwrap();
    assert_eq!(map.fronts.len(), 1);
    // Flood country 1's border: the two pieces now meet only across ocean.
    let border: Vec<u8> = map.countries[1]
        .hexes
        .iter()
        .filter(|&&h| map.hexes[h].neighbours.iter().any(|&x| map.hexes[x].country == 0))
        .map(|&h| map.hexes[h].number)
        .collect();
    let p = &mut content.map.countries[1];
    p.cities.retain(|n| !border.contains(n));
    p.ocean.extend(border);
    let flooded = Map::build(&content, 2).unwrap();
    assert!(flooded.fronts.is_empty());
    assert!(flooded.hexes.iter().all(|c| c.kind != HexKind::Base && c.facing.is_none()));
}
