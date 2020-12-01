use itertools::Itertools;

pub fn part1(xs: &[i32]) -> i32 {
    xs.iter()
        .combinations(2)
        .find(|x| x[0] + x[1] == 2020)
        .expect("No pair of entries add up to 2020...")
        .into_iter()
        .product()

}
