#[derive(Debug, Default, Copy, Clone)]
pub struct Operation {
    x: isize,
    y: isize,
}

impl Operation {
    pub fn merge(self, rhs: &Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
    pub fn solution(self) -> isize {
        self.x * self.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn load_data() -> Vec<Operation> {
        include_str!("../input.txt")
            .lines()
            .map(|l| {
                let mut l = l.split(" ");
                let operation = l.next().unwrap();
                let digit = l.next().unwrap().parse::<isize>().unwrap();
                match operation {
                    "forward" => Operation { x: digit, y: 0 },
                    "up" => Operation { y: -digit, x: 0 },
                    "down" => Operation { y: digit, x: 0 },
                    _ => unreachable!(),
                }
            })
            .collect()
    }

    #[test]
    fn part1() {
        let data = load_data();
        assert_eq!(
            data.iter()
                .fold(Operation::default(), Operation::merge)
                .solution(),
            1524750
        );
    }

    #[test]
    fn part2() {
        let data = load_data();
        let sol = data
            .iter()
            .fold((Operation::default(), 0), |(prev, depth), curr| {
                (Operation::merge(prev, curr), depth + curr.x * prev.y)
            });
        assert_eq!(sol.0.x * sol.1, 1592426537);
    }
}
