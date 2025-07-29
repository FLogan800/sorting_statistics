use crate::DistributionType;
use fastrand;

pub fn generate_data(distribution_type: &DistributionType, n: usize) -> Vec<u32> {
    match distribution_type {
        DistributionType::Random => generate_random(n),
        DistributionType::Reversed => generate_reversed(n),
        DistributionType::Repeated => generate_repeated(n),
        DistributionType::Sorted => generate_sorted(n),
        DistributionType::NearlySorted => generate_nearly_sorted(n),
    }
}

fn generate_random(n: usize) -> Vec<u32> {
    let mut data: Vec<u32> = (0..n as u32).collect();

    fastrand::shuffle(&mut data);

    data
}

fn generate_reversed(n: usize) -> Vec<u32> {
    (0..n).rev().map(|x| x as u32).collect()
}

fn generate_repeated(n: usize) -> Vec<u32> {
    let repeat_count = 10;
    let max_number = n / 10;

    let mut data = Vec::<u32>::new();

    for i in 0..max_number {
        for _ in 0..repeat_count {
            data.push(i as u32);
        }
    }

    fastrand::shuffle(&mut data);

    data
}

fn generate_sorted(n: usize) -> Vec<u32> {
    (0..n).map(|x| x as u32).collect()
}

fn generate_nearly_sorted(n: usize) -> Vec<u32> {
    let num_changes = n / 10;

    let mut data: Vec<u32> = (0..n as u32).collect();

    for _ in 0..num_changes {
        let i = fastrand::usize(..n);
        let j = fastrand::usize(..n);

        data.swap(i, j);
    }

    data
}
