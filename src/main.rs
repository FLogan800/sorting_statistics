use core::num;
use std::time::Instant;

use sorting_algorithm::*;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

mod data_generator;

#[derive(EnumIter, PartialEq, Display)]
enum Algorithm {
    Bubble,
    CocktailShaker,
    Comb,
    Gnome,
    Insertion,
    Merge,
    Quick,
    Selection,
    Shell,
}

#[derive(EnumIter, Display)]
enum DistributionType {
    Random,
    Repeated,
    Reversed,
    Sorted,
    NearlySorted,
}

fn main() {
    let num_iterations = 10;

    let slow_algorithms = [
        Algorithm::Bubble,
        Algorithm::Gnome,
        Algorithm::CocktailShaker,
        Algorithm::Insertion,
        Algorithm::Selection,
        Algorithm::Shell,
    ];

    let data_sizes = [10, 100, 1_000, 10_000, 100_000, 1_000_000, 10_000_000, 100_000_000];

    for _ in 0..num_iterations {
        for n in data_sizes {
            for distribution in DistributionType::iter() {
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
                }
            }
        }
    }
}

fn run_algorithm<T: Ord + Clone>(mut data: &mut [T], algorithm: &Algorithm) {
    match algorithm {
        Algorithm::Bubble => bubble_sort::sort(&mut data),
        Algorithm::CocktailShaker => cocktail_shaker_sort::sort(&mut data),
        Algorithm::Comb => comb_sort::sort(&mut data),
        Algorithm::Gnome => gnome_sort::sort(&mut data),
        Algorithm::Insertion => insertion_sort::sort(&mut data),
        Algorithm::Merge => merge_sort::sort(&mut data),
        Algorithm::Quick => quick_sort::sort(&mut data),
        Algorithm::Selection => selection_sort::sort(&mut data),
        Algorithm::Shell => shell_sort::sort(&mut data),
    }
}
