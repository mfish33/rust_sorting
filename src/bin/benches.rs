use rust_sorting::*;

use rand::prelude::*;

fn main() {
    let mut rand = rand::thread_rng();

    println!("algorithm n time");
    for &n in &[0, 100, 10_000, 1_000_000, 10_000_000] {
        let mut values = Vec::with_capacity(n);
        for _ in 0..n {
            values.push(rand.gen::<usize>());
        }

        for _ in 0..3 {
            values.shuffle(&mut rand);

            // let took = bench(BubbleSort, &values);
            // println!("{} {} {}", "bubble", n, took);
            // let took = bench(InsertionSort, &values);
            // println!("{} {} {}", "insertion", n, took);
            // let took = bench(SelectionSort, &values);
            // println!("{} {} {}", "selection", n, took);
            // let took = bench(QuickSort, &values);
            // println!("{} {} {}", "quick", n, took);
            let took = bench(RayonSort, &values);
            println!("{} {} {}", "RayonSort", n, took);
            let took = bench(ParMergeSort, &values);
            println!("{} {} {}", "ParMerge", n, took);
            // let took = bench(MergeSortPreAlloc, &values);
            // println!("{} {} {}", "MergePreAlloc", n, took);
            let took = bench(MergeSort, &values);
            println!("{} {} {}", "merge", n, took);
            let took = bench(StdSort, &values);
            println!("{} {} {}", "StdSorter", n, took);
        }
    }
}

fn bench<T: Sortable, S: Sorter>(
    sorter: S,
    values: &[T]
) -> f64 {
    let mut values: Vec<_> = values.to_vec();
    let time = std::time::Instant::now();
    sorter.sort(&mut values);
    let took = time.elapsed();
    // Can use if on nightly
    // assert!(values.is_sorted());
    for i in 1..values.len() {
        assert!(values[i] >= values[i - 1]);
    }
    took.as_secs_f64()
}