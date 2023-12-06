use aoc2023::load_problem_input;

struct Race {
    duration: usize,
    record: usize,
}

impl Race {
    fn wins(&self, seconds_held: usize) -> bool {
        (self.duration - seconds_held) * seconds_held > self.record
    }

    fn ways_to_win(&self) -> usize {
        // The formula for distance traveled is (duration - X) * X where X is seconds held.
        // This must be more than record to win. We can skip 0 and the total duration of the race.
        (1..self.duration)
            .filter(|seconds| self.wins(*seconds))
            .count()
    }
}

fn p1(input: &str) -> usize {
    let races = parse_input(input);

    races.iter().map(|r| r.ways_to_win()).product()
}

fn p2(input: &str) -> usize {
    let race = parse_input_kerning(input);

    race.ways_to_win()
}

fn parse_input_kerning(input: &str) -> Race {
    let duration = input
        .lines()
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .collect::<String>()
        .parse()
        .unwrap();
    let record = input
        .lines()
        .nth(1)
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .collect::<String>()
        .parse()
        .unwrap();

    Race { duration, record }
}

fn parse_input(input: &str) -> Vec<Race> {
    let durations = input
        .lines()
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .map(|d| d.parse().unwrap());
    let records = input
        .lines()
        .nth(1)
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .map(|d| d.parse().unwrap());

    durations
        .zip(records)
        .map(|(duration, record)| Race { duration, record })
        .collect()
}

fn main() {
    let input = load_problem_input(6).unwrap();
    println!("Day Part 1: {}", p1(&input));
    println!("Day Part 2: {}", p2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    use aoc2023::load_test_input;

    #[test]
    fn d4p1() {
        let input = load_test_input(6).unwrap();

        assert_eq!(288, p1(&input));
    }

    #[test]
    fn d4p2() {
        let input = load_test_input(6).unwrap();

        assert_eq!(71503, p2(&input));
    }
}
