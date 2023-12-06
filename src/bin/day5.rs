use aoc2023::load_problem_input;

struct Translation(Vec<Range>);

impl Translation {
    fn new() -> Self {
        Translation(Vec::new())
    }

    fn mapping(&self, s: usize) -> usize {
        for range in &self.0 {
            if let Some(m) = range.mapping(s) {
                return m;
            }
        }

        s
    }
}
struct Almanac {
    translations: Vec<Translation>,
}

impl Almanac {
    fn min_location_range(&self, start: usize, length: usize) -> usize {
        (start..start + length)
            .map(|s| self.find_location(s))
            .min()
            .unwrap()
    }

    fn min_location_slice(&self, seeds: &[usize]) -> usize {
        seeds.iter().map(|s| self.find_location(*s)).min().unwrap()
    }

    fn find_location(&self, seed: usize) -> usize {
        let mut current = seed;
        for translation in &self.translations {
            current = translation.mapping(current);
        }

        current
    }
}

struct Range {
    source: usize,
    destination: usize,
    length: usize,
}

impl Range {
    fn mapping(&self, s: usize) -> Option<usize> {
        if s >= self.source && s < self.source + self.length {
            return Some(self.destination + (s - self.source));
        }

        None
    }
}

impl From<&str> for Almanac {
    fn from(s: &str) -> Self {
        let mut translations: Vec<Translation> = Vec::new();

        let mut current: Translation = Translation::new();

        // Here lies more hacky AoC parsing - avert your eyes
        for line in s.lines().skip(2).filter(|l| !l.contains("map")) {
            if line.is_empty() {
                if !current.0.is_empty() {
                    translations.push(current);
                }
                current = Translation::new();
            } else {
                let values: Vec<usize> = line
                    .split_ascii_whitespace()
                    .map(|d| d.parse().unwrap())
                    .collect();
                current.0.push(Range {
                    source: values[1],
                    destination: values[0],
                    length: values[2],
                });
            }
        }

        Almanac { translations }
    }
}

fn seed_data(input: &str) -> Vec<usize> {
    // I mean it's kind of beautiful in its own ugly way, right?
    input
        .lines()
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn p1(input: &str) -> usize {
    let seeds = seed_data(input);
    let almanac = Almanac::from(input);

    almanac.min_location_slice(&seeds)
}

fn p2(input: &str) -> usize {
    let seeds = seed_data(input);
    let almanac = Almanac::from(input);

    // From inspecting the input, I don't think we want to try to build the whole slice of
    // possible seeds like part one, so let's operate on the seed range definition instead
    seeds
        .chunks(2)
        .map(|sr| almanac.min_location_range(sr[0], sr[1]))
        .min()
        .unwrap()
}

fn main() {
    let input = load_problem_input(5).unwrap();
    println!("Part 1: {}", p1(&input));
    println!("Part 2: {}", p2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    use aoc2023::load_test_input;

    #[test]
    fn d5p1() {
        let input = load_test_input(5).unwrap();

        assert_eq!(35, p1(&input));
    }

    #[test]
    fn d5p2() {
        let input = load_test_input(5).unwrap();

        assert_eq!(46, p2(&input));
    }
}
