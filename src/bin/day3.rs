use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

use aoc2023::{load_problem_input, Point};

fn main() {
    let input = load_problem_input(3).unwrap();
    println!("Day 3 Part 1: {}", p1(&input));
    println!("Day 3 Part 2: {}", p2(&input));
}

#[derive(Debug)]
struct Number {
    value: usize,
    points: Vec<Point>,
}

fn p1(input: &str) -> i64 {
    let now = Instant::now();

    let (numbers, symbols) = parse_input(input);

    // Calculate the sum of any number that's adjacent to a symbol
    let mut sum = 0;
    for number in &numbers {
        let mut found = false;
        for point in &number.points {
            if !found {
                for adj in point.adjacent_diaganoal() {
                    if symbols.contains_key(&adj) {
                        sum += number.value;
                        found = true;
                        break;
                    }
                }
            }
        }
    }

    let elapsed = now.elapsed();
    println!("{:?}", elapsed);

    sum as i64
}

fn p2(input: &str) -> i64 {
    let now = Instant::now();

    // Only consider gears '*'
    let (numbers, mut symbols) = parse_input(input);
    symbols.retain(|_, v| *v == '*');

    let mut gear_ratio: i64 = 0;

    for point in symbols.keys() {
        let mut adjacent_numbers = HashSet::new();

        for adj_p in point.adjacent_diaganoal() {
            // If a number contains one of the adjacent points of the gear, count it
            for number in &numbers {
                if number.points.contains(&adj_p) {
                    // The use of the hash set here works only because the input doesn't have
                    // a gear next to two distinct instances of the same number
                    adjacent_numbers.insert(number.value as i64);
                }
            }
        }

        if adjacent_numbers.len() == 2 {
            gear_ratio += adjacent_numbers.iter().product::<i64>();
        }
    }

    let elapsed = now.elapsed();
    println!("{:?}", elapsed);

    gear_ratio
}

fn parse_input(input: &str) -> (Vec<Number>, HashMap<Point, char>) {
    let mut numbers = Vec::new();
    let mut symbols = HashMap::new();

    for (row, line) in input.lines().enumerate() {
        let mut chars = line.chars().peekable();

        // We could use enumerate here but that makes some of the conditionals wonky later,
        // so we'll just track column manually.
        let mut col = 0;

        while chars.peek().is_some() {
            let c = chars.next().unwrap();

            if c.is_ascii_digit() {
                // Gather the digits of the number
                let mut s = String::new();
                s.push(c);
                while chars.peek().is_some() && chars.peek().unwrap().is_ascii_digit() {
                    s.push(chars.next().unwrap());
                }

                // Each digit is a point on the graph that we'll want to track
                let mut points = Vec::new();
                for x in col..col + s.len() {
                    points.push(Point::new(x as i64, row as i64));
                }

                numbers.push(Number {
                    value: s.parse().unwrap(),
                    points,
                });

                col += s.len();
            } else if c != '.' {
                symbols.insert(Point::new(col as i64, row as i64), c);
                col += 1;
            } else {
                col += 1;
            }
        }
    }

    (numbers, symbols)
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc2023::load_test_input;

    #[test]
    fn d3_p1() {
        let input = load_test_input(3).unwrap();

        assert_eq!(4361, p1(&input));
    }

    #[test]
    fn d3_p2() {
        let input = load_test_input(3).unwrap();

        assert_eq!(467835, p2(&input));
    }
}
