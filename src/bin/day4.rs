use std::collections::HashSet;

use aoc2023::load_problem_input;

struct Card {
    winning: HashSet<usize>,
    yours: HashSet<usize>,
}

impl From<&str> for Card {
    fn from(s: &str) -> Self {
        // Thankfully this is not a "pretty parsing" competition...I hope
        let numbers = s.split(':').nth(1).unwrap();

        let winning = numbers.split('|').nth(0).unwrap().trim();
        let winning: HashSet<usize> = winning
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        let yours = numbers.split('|').nth(1).unwrap().trim();
        let yours: HashSet<usize> = yours
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        Card { winning, yours }
    }
}

impl Card {
    fn num_winners(&self) -> usize {
        self.winning.intersection(&self.yours).count()
    }

    fn value(&self) -> usize {
        if self.num_winners() == 0 {
            return 0;
        }

        2_usize.pow((self.num_winners() - 1) as u32)
    }
}

fn p1(input: &str) -> usize {
    input.lines().map(Card::from).map(|c| c.value()).sum()
}

fn p2(input: &str) -> usize {
    let cards: Vec<Card> = input.lines().map(Card::from).collect();
    let mut copies = vec![1; cards.len()];

    for (id, card) in cards.iter().enumerate() {
        let num_won = card.num_winners();

        for x in id + 1..=id + num_won {
            copies[x] += copies[id];
        }
    }

    copies.iter().sum()
}

fn main() {
    let input = load_problem_input(4).unwrap();
    println!("Day X Part 1: {}", p1(&input));
    println!("Day X Part 2: {}", p2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    use aoc2023::load_test_input;

    #[test]
    fn d4p1() {
        let input = load_test_input(4).unwrap();

        assert_eq!(13, p1(&input));
    }

    #[test]
    fn d4p2() {
        let input = load_test_input(4).unwrap();

        assert_eq!(30, p2(&input));
    }
}
