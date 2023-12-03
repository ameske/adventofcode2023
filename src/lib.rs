use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::Read;

pub fn run_day(day: u64, part1: fn(&str) -> i64, part2: fn(&str) -> i64) {
    let input = load_problem_input(day).unwrap();
    println!("Day {} Part 1: {}", day, part1(&input));
    println!("Day {} Part 2: {}", day, part2(&input));
}

pub fn load_problem_input(day: u64) -> io::Result<String> {
    let path = format!("inputs/day{}.txt", day);
    load_input(&path)
}

pub fn load_test_input(day: u64) -> io::Result<String> {
    let path = format!("inputs/day{}_test.txt", day);
    load_input(&path)
}

fn load_input(path: &str) -> io::Result<String> {
    let mut fd = fs::File::open(path)?;

    let mut s = String::new();

    fd.read_to_string(&mut s)?;

    Ok(s)
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn adjacent(&self) -> Vec<Point> {
        vec![
            Point::new(self.x, self.y - 1),
            Point::new(self.x, self.y + 1),
            Point::new(self.x - 1, self.y),
            Point::new(self.x + 1, self.y),
        ]
    }

    pub fn three_by_three_grid(&self) -> Vec<Point> {
        vec![
            Point::new(self.x - 1, self.y - 1),
            Point::new(self.x, self.y - 1),
            Point::new(self.x + 1, self.y - 1),
            Point::new(self.x - 1, self.y),
            Point::new(self.x, self.y),
            Point::new(self.x + 1, self.y),
            Point::new(self.x - 1, self.y + 1),
            Point::new(self.x, self.y + 1),
            Point::new(self.x + 1, self.y + 1),
        ]
    }

    pub fn adjacent_diaganoal(&self) -> Vec<Point> {
        vec![
            Point::new(self.x - 1, self.y - 1),
            Point::new(self.x, self.y - 1),
            Point::new(self.x + 1, self.y - 1),
            Point::new(self.x - 1, self.y),
            Point::new(self.x + 1, self.y),
            Point::new(self.x - 1, self.y + 1),
            Point::new(self.x, self.y + 1),
            Point::new(self.x + 1, self.y + 1),
        ]
    }
}

#[derive(Clone)]
pub struct Grid {
    pub points: HashMap<Point, i64>,
}

impl From<HashMap<Point, i64>> for Grid {
    fn from(points: HashMap<Point, i64>) -> Self {
        Self { points }
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}

impl Grid {
    pub fn new() -> Self {
        Self {
            points: HashMap::new(),
        }
    }

    pub fn from_text_grid(s: &str) -> Self {
        let mut points = HashMap::new();

        for (y, line) in s.lines().enumerate() {
            for (x, value) in line.chars().enumerate() {
                points.insert(
                    Point::new(x as i64, y as i64),
                    value.to_digit(10).unwrap() as i64,
                );
            }
        }

        Self { points }
    }

    pub fn print(&self) {
        let max_x = self.max_x();
        let max_y = self.max_y();

        for y in 0..=max_y {
            for x in 0..max_x {
                print!("{}", self.points[&Point::new(x, y)]);
            }
            println!()
        }
    }

    pub fn insert(&mut self, p: Point, value: i64) {
        self.points.insert(p, value);
    }

    pub fn max_x(&self) -> i64 {
        self.points.keys().max_by_key(|p| p.x).unwrap().x
    }

    pub fn max_y(&self) -> i64 {
        self.points.keys().max_by_key(|p| p.y).unwrap().y
    }

    pub fn adjacent_cardinal_points(&self, point: &Point) -> Vec<Point> {
        point
            .adjacent()
            .into_iter()
            .filter(|p| self.points.contains_key(p))
            .collect()
    }

    pub fn all_adjacent_points(&self, point: &Point) -> Vec<Point> {
        point
            .adjacent_diaganoal()
            .into_iter()
            .filter(|p| self.points.contains_key(p))
            .collect()
    }

    pub fn points(&self) -> impl Iterator<Item = &Point> {
        self.points.keys()
    }

    pub fn values(&self) -> impl Iterator<Item = &i64> {
        self.points.values()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Point, &i64)> {
        self.points.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&Point, &mut i64)> {
        self.points.iter_mut()
    }
}
