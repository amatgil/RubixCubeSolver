
use std::fmt::Display;

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use tubaitu::cycle_items_safe;
use tubaitu::cycle_items_unchecked;
use tubaitu::cycle_items_old;

const TS: [[usize; 5]; 3] = [
    [1, 2, 3, 4, 5],
    [2, 1, 6, 4, 3],
    [4, 2, 3, 1, 5],
];

const IDXS: [[usize; 4]; 4] = [
    [0, 1, 2, 3],
    [3, 2, 1, 0],
    [3, 1, 0, 2],
    [1, 2, 3, 0],
];

#[derive(Clone, Copy)]
struct Pair {
    t: [usize; 5],
    idx: [usize; 4]
}

impl Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	let mut out = String::new();
	out.push_str("t-");
	for thing in self.t { out.push_str(&format!("{thing}")); }
	out.push_str("_and_");
	for thing in self.idx { out.push_str(&format!("{thing}")); }

	write!(f, "{out}")
    }
}

pub fn cyclings(c: &mut Criterion) {
    let mut group = c.benchmark_group("cyclings");
    let mut i = 0;
    for t in TS {
	for idx in IDXS {
	    let input = Pair { t, idx };
	    group.bench_with_input(
		BenchmarkId::new("Old", i),
		&input,
		|b, inp| b.iter(|| cycle_items_old(&mut black_box(inp.t.clone()), black_box(inp.idx)))
	    );
	    group.bench_with_input(
		BenchmarkId::new("Safe", i),
		&input,
		|b, inp| b.iter(|| cycle_items_safe(&mut black_box(inp.t.clone()), black_box(inp.idx)))
	    );
	    group.bench_with_input(
		BenchmarkId::new("Unchecked", i),
		&input,
		|b, inp| b.iter(|| cycle_items_unchecked(&mut black_box(inp.t.clone()), black_box(inp.idx)))
	    );
	    i += 1;
	}
    }
    group.finish();
}

criterion_group!(benches, cyclings);
criterion_main!(benches);
