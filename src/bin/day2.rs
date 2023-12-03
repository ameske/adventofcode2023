use aoc2023::load_problem_input;

#[derive(Debug)]
struct Game {
    id: i64,
    sets: Vec<CubeSet>,
}

impl From<&str> for Game {
    fn from(s: &str) -> Game {
        let parts: Vec<&str> = s.split(':').map(|s| s.trim()).collect::<Vec<&str>>();

        let id = parts[0]
            .split_ascii_whitespace()
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();

        let sets = parts[1].split(';');
        let sets = sets.map(CubeSet::from).collect();

        Game { id, sets }
    }
}

impl Game {
    fn is_possible(&self, red: usize, green: usize, blue: usize) -> bool {
        self.sets
            .iter()
            .all(|set| set.is_possible(red, green, blue))
    }

    fn min_cubes_required(&self) -> CubeSet {
        let mut min = CubeSet::default();

        for set in &self.sets {
            if set.red > min.red {
                min.red = set.red;
            }

            if set.green > min.green {
                min.green = set.green;
            }

            if set.blue > min.blue {
                min.blue = set.blue;
            }
        }

        min
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
struct CubeSet {
    red: usize,
    green: usize,
    blue: usize,
}

impl From<&str> for CubeSet {
    fn from(s: &str) -> CubeSet {
        let cubes: Vec<&str> = s.split(',').map(|cube| cube.trim()).collect();

        let mut set = CubeSet::default();

        for cube in cubes {
            let number: usize = cube
                .split_ascii_whitespace()
                .nth(0)
                .unwrap()
                .parse()
                .unwrap();

            let color = cube.split_ascii_whitespace().nth(1).unwrap();
            match color {
                "red" => set.red = number,
                "green" => set.green = number,
                "blue" => set.blue = number,
                _ => panic!("unexpected color: {}", color),
            }
        }

        set
    }
}

impl CubeSet {
    fn is_possible(&self, red: usize, green: usize, blue: usize) -> bool {
        self.red <= red && self.green <= green && self.blue <= blue
    }

    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

fn d2_p1(input: &str) -> i64 {
    input
        .lines()
        .map(Game::from)
        .filter(|g| g.is_possible(12, 13, 14))
        .map(|g| g.id)
        .sum()
}

fn d2_p2(input: &str) -> usize {
    input
        .lines()
        .map(|l| Game::from(l).min_cubes_required().power())
        .sum()
}

fn main() {
    let d2_input = load_problem_input(2).unwrap();
    println!("Day 2 Part 1: {}", d2_p1(&d2_input));
    println!("Day 2 Part 2: {}", d2_p2(&d2_input));
}

#[cfg(test)]
mod test {
    use crate::*;
    use aoc2023::load_test_input;

    #[test]
    fn is_possible() {
        let cubes = CubeSet {
            red: 20,
            green: 8,
            blue: 6,
        };

        let game = Game {
            id: 1,
            sets: vec![cubes],
        };

        assert!(!game.is_possible(12, 13, 14));
        assert!(game.is_possible(20, 20, 20));
    }

    #[test]
    fn min_cubes_required() {
        let cubes = CubeSet {
            red: 20,
            green: 8,
            blue: 6,
        };

        let cubes2 = CubeSet {
            red: 12,
            green: 9,
            blue: 5,
        };

        let game = Game {
            id: 1,
            sets: vec![cubes, cubes2],
        };

        let min = game.min_cubes_required();

        assert_eq!(
            min,
            CubeSet {
                red: 12,
                green: 8,
                blue: 5
            }
        );
    }

    #[test]
    fn day2_part1() {
        let input = load_test_input(2).unwrap();

        assert_eq!(8, d2_p1(&input));
    }

    #[test]
    fn day2_part2() {
        let input = load_test_input(2).unwrap();

        assert_eq!(2286, d2_p2(&input));
    }
}
