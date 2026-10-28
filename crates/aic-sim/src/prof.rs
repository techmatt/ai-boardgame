//! Opt-in timing of the search's hot calls: build with `--features profile`
//! and `experiment` prints where the agents' time went (rules apply, the
//! state copy included; legal-action generation; the heuristic;
//! determinization). Off, [`time`] is the call itself and costs nothing.
//!
//! Each worker thread adds to its own counters and merges them into the
//! process total with [`flush`] when it runs out of games.

#[derive(Clone, Copy)]
pub enum Part {
    Apply,
    Legal,
    Eval,
    Determinize,
}

#[cfg_attr(not(feature = "profile"), allow(dead_code))]
pub const PARTS: [&str; 4] = ["apply", "legal", "eval", "determinize"];

#[cfg(feature = "profile")]
mod on {
    use std::cell::RefCell;
    use std::sync::Mutex;

    pub type Totals = [(f64, u64); super::PARTS.len()];

    thread_local! {
        pub static LOCAL: RefCell<Totals> = const { RefCell::new([(0.0, 0); super::PARTS.len()]) };
    }
    pub static TOTAL: Mutex<Totals> = Mutex::new([(0.0, 0); super::PARTS.len()]);
}

/// Runs `f`, adding its time to `part` when profiling.
#[inline(always)]
pub fn time<R>(part: Part, f: impl FnOnce() -> R) -> R {
    #[cfg(feature = "profile")]
    {
        let t = std::time::Instant::now();
        let r = f();
        let ms = t.elapsed().as_secs_f64() * 1000.0;
        on::LOCAL.with(|l| {
            let e = &mut l.borrow_mut()[part as usize];
            e.0 += ms;
            e.1 += 1;
        });
        r
    }
    #[cfg(not(feature = "profile"))]
    {
        let _ = part;
        f()
    }
}

/// Merges this thread's counters into the process total.
pub fn flush() {
    #[cfg(feature = "profile")]
    on::LOCAL.with(|l| {
        let mut l = l.borrow_mut();
        let mut t = on::TOTAL.lock().expect("profile lock");
        for (a, b) in t.iter_mut().zip(l.iter_mut()) {
            a.0 += b.0;
            a.1 += b.1;
            *b = (0.0, 0);
        }
    });
}

/// The process total, `(part, ms, calls)`; `None` unless built with `profile`.
pub fn totals() -> Option<Vec<(&'static str, f64, u64)>> {
    #[cfg(feature = "profile")]
    {
        let t = on::TOTAL.lock().expect("profile lock");
        Some(PARTS.iter().zip(t.iter()).map(|(n, (ms, c))| (*n, *ms, *c)).collect())
    }
    #[cfg(not(feature = "profile"))]
    None
}
