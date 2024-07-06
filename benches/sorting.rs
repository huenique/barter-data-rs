use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use rand::Rng;
use serde::Deserialize;
use serde::Serialize;
use std::cmp::Ordering;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub struct Level {
    pub price: f64,
    pub amount: f64,
}

impl Eq for Level {}

impl Ord for Level {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other)
            .unwrap_or_else(|| panic!("{:?}.partial_cmp({:?}) impossible", self, other))
    }
}

#[derive(Debug)]
pub struct OrderBookSide {
    pub levels: Vec<Level>,
    pub side: Side,
}

#[derive(Debug)]
pub enum Side {
    Buy,
    Sell,
}

impl OrderBookSide {
    pub fn new(side: Side, levels: Vec<Level>) -> Self {
        Self { levels, side }
    }

    pub fn sort(&mut self) {
        quicksort_iterative(&mut self.levels);

        if let Side::Buy = self.side {
            self.levels.reverse();
        }
    }

    pub fn sort_unstable(&mut self) {
        self.levels.sort_unstable();

        if let Side::Buy = self.side {
            self.levels.reverse();
        }
    }
}

fn partition(arr: &mut [Level], low: isize, high: isize) -> isize {
    let pivot = arr[high as usize];
    let mut i = low - 1;

    for j in low..high {
        if arr[j as usize].price <= pivot.price {
            i += 1;
            arr.swap(i as usize, j as usize);
        }
    }

    arr.swap((i + 1) as usize, high as usize);
    i + 1
}

fn quicksort_iterative(arr: &mut [Level]) {
    let len = arr.len();
    let mut stack = Vec::with_capacity(len);

    stack.push((0, len as isize - 1));

    while let Some((low, high)) = stack.pop() {
        if low < high {
            let p = partition(arr, low, high);
            stack.push((low, p - 1));
            stack.push((p + 1, high));
        }
    }
}

fn generate_random_levels(n: usize) -> Vec<Level> {
    let mut rng = rand::thread_rng();
    (0..n)
        .map(|_| Level {
            price: rng.gen_range(0.0..100000.0),
            amount: rng.gen_range(0.0..10.0),
        })
        .collect()
}

fn criterion_benchmark(c: &mut Criterion) {
    let levels = generate_random_levels(10000);
    let mut order_book_side = OrderBookSide::new(Side::Sell, levels.clone());

    c.bench_function("sort_unstable", |b| {
        b.iter(|| {
            order_book_side.levels = levels.clone();
            order_book_side.sort_unstable();
        })
    });

    c.bench_function("quicksort_iterative", |b| {
        b.iter(|| {
            order_book_side.levels = levels.clone();
            order_book_side.sort();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
