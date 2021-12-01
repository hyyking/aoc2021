pub fn solve(measurements: &[usize], window: usize) -> usize {
    let measurements = measurements
        .windows(window)
        .map(<[_]>::iter)
        .map(Iterator::sum);

    measurements
        .clone()
        .zip(measurements.skip(1))
        .filter(|(prev, curr): &(usize, usize)| curr > prev)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn load_data() -> Vec<usize> {
        include_str!("../input.txt")
            .lines()
            .map(|l| l.parse::<usize>().unwrap())
            .collect()
    }

    #[test]
    fn part1() {
        let result = solve(&load_data()[..], 1);
        assert_eq!(result, 1832)
    }

    #[test]
    fn part2() {
        let result = solve(&load_data()[..], 3);
        assert_eq!(result, 1858)
    }
}
