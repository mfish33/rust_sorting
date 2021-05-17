use rust_sorting::*;

use rand::prelude::*;
use std::cmp::Ordering;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;

#[derive(Clone, Debug)]
struct SortEvaluator<T> {
    t: T,
    cmps: Arc<AtomicUsize>,
}

// Turned off comparison counter because of performance impacts
impl<T: PartialEq> PartialEq for SortEvaluator<T> {
    fn eq(&self, other: &Self) -> bool {
        //self.cmps.fetch_add(1, Relaxed);
        self.t == other.t
    }
}
impl<T: Eq> Eq for SortEvaluator<T> {}

impl<T: PartialOrd> PartialOrd for SortEvaluator<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        //self.cmps.fetch_add(1, Relaxed);
        self.t.partial_cmp(&other.t)
    }
}
impl<T: Ord> Ord for SortEvaluator<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        //self.cmps.fetch_add(1, Relaxed);
        self.t.cmp(&other.t)
    }
}

fn main() {
    let mut rand = rand::thread_rng();
    let counter = Arc::new(AtomicUsize::new(0));

    println!("algorithm n comparisons time");
    for &n in &[10_000_000] {
        let mut values = Vec::with_capacity(n);
        for _ in 0..n {
            values.push(SortEvaluator {
                t: rand.gen::<usize>(),
                cmps: Arc::clone(&counter),
            });
        }

        for _ in 0..10 {
            values.shuffle(&mut rand);

            // let took = bench(BubbleSort, &values, &counter);
            // println!("{} {} {} {}", "bubble", n, took.0, took.1);
            // let took = bench(InsertionSort, &values, &counter);
            // println!("{} {} {} {}", "insertion", n, took.0, took.1);
            // let took = bench(SelectionSort, &values, &counter);
            // println!("{} {} {} {}", "selection", n, took.0, took.1);
            // let took = bench(QuickSort, &values, &counter);
            // println!("{} {} {} {}", "quick", n, took.0, took.1);
            let took = bench(ParMergeSort, &values, &counter);
            println!("{} {} {} {}", "ParMerge", n, took.0, took.1);
            // let took = bench(MergeSortPreAlloc, &values, &counter);
            // println!("{} {} {} {}", "MergePreAlloc", n, took.0, took.1);
            let took = bench(MergeSort, &values, &counter);
            println!("{} {} {} {}", "merge", n, took.0, took.1);
            // let took = bench(StdSort, &values, &counter);
            // println!("{} {} {} {}", "StdSorter", n, took.0, took.1);
        }
    }
}

fn bench<T: Sortable, S: Sorter>(
    sorter: S,
    values: &[SortEvaluator<T>],
    counter: &Arc<AtomicUsize>,
) -> (usize, f64) {
    let mut values: Vec<_> = values.to_vec();
    counter.store(0,Relaxed);
    let time = std::time::Instant::now();
    sorter.sort(&mut values);
    let took = time.elapsed();
    let count = counter.load(Relaxed);
    // Can use if on nightly
    // assert!(values.is_sorted());
    for i in 1..values.len() {
        assert!(values[i] >= values[i - 1]);
    }
    (count, took.as_secs_f64())
}