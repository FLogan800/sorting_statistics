use sorting_algorithm::*;
use strum_macros::EnumIter;

mod data_generator;

#[derive(EnumIter)]
enum Algorithm {
    BubbleSort,
    GnomeSort,
    InsertionSort,
    QuickSort,
    SelectionSort,
    ShellSort,
}

#[derive(EnumIter)]
enum DistributionType {
    Random,
    Repeated,
    Reversed,
    Sorted,
    NearlySorted,
}

fn main() {
    println!("Hello, world!");
}
