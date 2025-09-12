use std::{error::Error, time::Instant};

use sorting_algorithm::{bogo_sort::sort, *};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

mod data_generator;

#[derive(EnumIter, PartialEq, Display)]
enum Algorithm {
    Bubble,
    CocktailShaker,
    Comb,
    Gnome,
    Heap,
    Insertion,
    Merge,
    Quick,
    Selection,
    Shell,
    Sort, // Rust's built in sorting algorithm
}

#[derive(EnumIter, Display)]
enum DistributionType {
    Random,
    Repeated,
    Reversed,
    Sorted,
    NearlySorted,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path("sorting_statistics.csv")?;
    wtr.write_record(&[
        "Algorithm",
        "Distribution Type",
        "n",
        "Time Elapsed (Seconds)",
    ])?;

    let num_iterations = 100;

    let slow_algorithms = [
        Algorithm::Bubble,
        Algorithm::Gnome,
        Algorithm::CocktailShaker,
        Algorithm::Insertion,
        Algorithm::Selection,
        Algorithm::Shell,
    ];

    let data_sizes = [
        10,
        100,
        1_000,
        10_000,
        100_000,
        1_000_000,
        10_000_000,
        100_000_000,
    ];

    for n in data_sizes {
        for distribution in DistributionType::iter() {
            for _ in 0..num_iterations {
                let data = data_generator::generate_data(&distribution, n);

                for algorithm in Algorithm::iter() {
                    if slow_algorithms.contains(&algorithm) && n >= 1_000_000 {
                        continue;
                    }
                    let mut data_clone = data.clone();

                    let start = Instant::now();
                    run_algorithm(&mut data_clone, &algorithm);
                    let time_elapsed = start.elapsed().as_secs_f64();

                    println!("{}, {}, {n}, {time_elapsed}", &algorithm, &distribution);

                    if !data_clone.is_sorted() {
                        panic!("Data not sorted with {} {} {n}", &algorithm, &distribution);
                    }

                    wtr.write_record(&[
                        algorithm.to_string(),
                        distribution.to_string(),
                        n.to_string(),
                        time_elapsed.to_string(),
                    ])?;
                }
            }
        }
        wtr.flush()?;
    }

    Ok(())
}

fn run_algorithm<T: Ord + Clone>(mut data: &mut [T], algorithm: &Algorithm) {
    match algorithm {
        Algorithm::Bubble => bubble_sort::sort(&mut data),
        Algorithm::CocktailShaker => cocktail_shaker_sort::sort(&mut data),
        Algorithm::Comb => comb_sort::sort(&mut data),
        Algorithm::Gnome => gnome_sort::sort(&mut data),
        Algorithm::Heap => heap_sort::sort(&mut data),
        Algorithm::Insertion => insertion_sort::sort(&mut data),
        Algorithm::Merge => merge_sort::sort(&mut data),
        Algorithm::Quick => quick_sort::sort(&mut data),
        Algorithm::Selection => selection_sort::sort(&mut data),
        Algorithm::Shell => shell_sort::sort(&mut data),
        Algorithm::Sort => data.sort(),
    }
}
