pub fn solve1(measurements: &[usize]) -> usize {
    if measurements.len() < 2 {
        return 0;
    }
    measurements
        .iter()
        .zip(&measurements[1..])
        .filter(|(prev, curr)| curr > prev)
        .count()
}

pub fn solve2(measurements: &[usize]) -> usize {
    if measurements.len() < 3 {
        return 0;
    }
    let measurements = measurements
        .iter()
        .zip(&measurements[1..])
        .zip(&measurements[2..])
        .map(|((a, b), c)| a + b + c);

    measurements
        .clone()
        .zip(measurements.skip(1))
        .filter(|(prev, curr)| curr > prev)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;
    use std::str::FromStr;

    fn load_data() -> Vec<usize> {
        let f = BufReader::new(File::open("input.txt").unwrap());
        let v: Vec<_> = f.lines().map(Result::unwrap).collect();
        v.iter()
            .map(String::as_str)
            .map(usize::from_str)
            .map(Result::unwrap)
            .collect()
    }

    #[test]
    fn part1() {
        let result = solve1(&load_data()[..]);
        assert_eq!(result, 1832)
    }

    #[test]
    fn part2() {
        let result = solve2(&load_data()[..]);
        assert_eq!(result, 1858)
    }
}
