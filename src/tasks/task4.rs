use std::collections::HashSet;

use crate::task_handler::get_task;

pub fn tasks() {
    let input = get_task(4);
    println!("{}", task1(&input));
    println!("{}", task2(&input));
}

fn task1(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.split_once(':').unwrap().1)
        .map(|entry| {
            let intersection_size = entry
                .split('|')
                .map(|x| {
                    x.split_whitespace()
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect::<HashSet<_>>()
                })
                .reduce(|a, b| a.intersection(&b).copied().collect::<HashSet<_>>())
                .expect("Somehow there are no numbers in the input.")
                .len();

            if intersection_size == 0 {
                0
            } else {
                2_usize.pow(intersection_size.saturating_sub(1).try_into().unwrap())
            }
        })
        .sum()
}

fn task2(input: &str) -> usize {
    let mut card_counts: Vec<usize> = vec![1; input.lines().count()];

    input
        .lines()
        .map(|line| {
            line.split_once(':')
                .unwrap()
                .1
                .split('|')
                .map(|x| {
                    x.split_whitespace()
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect::<HashSet<_>>()
                })
                .reduce(|a, b| a.intersection(&b).copied().collect::<HashSet<_>>())
                .expect("Somehow there are no numbers in the input.")
                .len()
        })
        .enumerate()
        .for_each(|(index, win_count)| {
            let number_new_cards = card_counts[index];
            card_counts
                .iter_mut()
                .skip(index + 1)
                .take(win_count)
                .for_each(|x| *x += number_new_cards);
        });

    card_counts.iter().sum()
}

#[cfg(test)]
mod test {
    use super::task2;

    use super::task1;
    static TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_task1_example() {
        assert_eq!(task1(TEST_INPUT), 13);
    }

    #[test]
    fn test_task2_example() {
        assert_eq!(task2(TEST_INPUT), 30);
    }

    #[test]
    fn test_task1() {
        let input = super::get_task(4);
        assert_eq!(task1(&input), 27059);
    }

    #[test]
    fn test_task2() {
        let input = super::get_task(4);
        assert_eq!(task2(&input), 5_744_979);
    }
}
