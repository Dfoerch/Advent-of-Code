use itertools::Itertools;

pub fn part2(xs: &[i32]) -> i32 {
    xs.iter()
        .combinations(3)
        .find(|x| x[0] + x[1] + x[2] == 2020)
        .expect("No pair of entries add up to 2020...")
        .into_iter()
        .product()

}
