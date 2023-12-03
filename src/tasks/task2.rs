use std::ops::Add;

use crate::task_handler::get_task;
use regex::{self, Regex};

pub fn tasks() {
    println!("{}", task1());
    println!("{}", task2());
}

type Red = usize;
type Blue = usize;
type Green = usize;

#[derive(Default, Debug, Clone)]
struct Color(Red, Blue, Green);

const VALIDATE: Color = Color(12, 13, 14);

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl std::iter::Sum for Color {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |a, b| a + b)
    }
}

impl Color {
    fn from_capture(input: (&str, &str)) -> Self {
        let (val, color) = input;
        let count: usize = val.parse().unwrap();
        match color {
            "red" => Self(count, 0, 0),
            "green" => Self(0, count, 0),
            "blue" => Self(0, 0, count),
            _ => unreachable!(),
        }
    }

    fn validate(self) -> Option<Self> {
        (self.0 <= VALIDATE.0 && self.1 <= VALIDATE.1 && self.2 <= VALIDATE.2).then_some(self)
    }

    fn max(self, rhs: &Self) -> Self {
        Self(self.0.max(rhs.0), self.1.max(rhs.1), self.2.max(rhs.2))
    }

    const fn power(self) -> usize {
        self.0 * self.1 * self.2
    }
}

fn task1() -> usize {
    let input = get_task(2);
    solve_task1(&input)
}

fn solve_task1(input: &str) -> usize {
    let re: Regex = Regex::new(r"((?P<count>\d*) (?P<color>blue|red|green))+").unwrap();
    let mut sum = 0;
    for (line, game) in input.lines().zip(1..) {
        if line
            .split(';')
            .map(|sub| {
                re.captures_iter(sub)
                    // .map(|x| dbg!(x))
                    .map(|x| {
                        (
                            x.name("count").unwrap().as_str(),
                            x.name("color").unwrap().as_str(),
                        )
                    })
                    .map(Color::from_capture)
                    .sum::<Color>()
                    .validate()
            })
            .all(|x| x.is_some())
        {
            sum += game;
        }
    }
    sum
}

fn task2() -> usize {
    let input = get_task(2);
    solve_task2(&input)
}

fn solve_task2(input: &str) -> usize {
    let re: Regex = Regex::new(r"((?P<count>\d*) (?P<color>blue|red|green))+").unwrap();
    input
        .lines()
        .map(|game| {
            game.split(';')
                .map(|sub| {
                    re.captures_iter(sub)
                        // .map(|x| dbg!(x))
                        .map(|captures| {
                            (
                                captures.name("count").unwrap().as_str(),
                                captures.name("color").unwrap().as_str(),
                            )
                        })
                        .map(Color::from_capture)
                        .sum()
                })
                .fold(Color::default(), |a, b| a.max(&b))
                .power()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::solve_task2;

    #[test]
    fn test_task2() {
        let input = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(solve_task2(input), 2286);
    }
}
