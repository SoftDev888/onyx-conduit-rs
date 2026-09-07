fn running(values: &[i64]) -> Vec<i64> {
    let mut total = 0;
    values
        .iter()
        .map(|one| {
            total += one;
            total
        })
        .collect()
}

fn main() {
    println!("{:?}", running(&[3, 1, 4, 1, 5]));
}
