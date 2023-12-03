use crate::task_handler::get_task;
use itertools::Itertools;

pub fn tasks() {
    println!("{}", task1());
    println!("{}", task2());
}

fn helper_1(grid: &Vec<Vec<char>>, line_no: usize, start: usize, end: usize) -> bool {
    let start = start.saturating_sub(1);
    let end = (end + 1).min(grid[line_no].len() - 1);
    let line_lower = line_no.saturating_sub(1);
    let line_upper = (line_no + 1).min(grid.len() - 1);
    (line_lower..=line_upper).any(|line_no| {
        grid[line_no][start..=end]
            .iter()
            .any(|&x| !x.is_numeric() && x != '.')
    })
}

fn helper_2(grid: &Vec<Vec<char>>, line_no: usize, gear_index: usize) -> usize {
    let line_lower = line_no.saturating_sub(1);
    let line_upper = (line_no + 1).min(grid.len() - 1);

    // Search for the lines around the gear.
    let x = (line_lower..=line_upper)
        .flat_map(|line_no| {
            grid[line_no]
                .iter()
                .enumerate()
                // Gather all the numeric characters (the numbers) into groups
                .map(|(i, c)| c.is_numeric().then_some((i, c)))
                .group_by(std::option::Option::is_some)
                .into_iter()
                .filter(|(k, _)| *k)
                .map(|x| x.1.flatten())
                // Get the range of the start end end of the number
                .map(|f| {
                    let (i, c): (Vec<usize>, Vec<&char>) = f.unzip();
                    ((*i.first().unwrap(), *i.last().unwrap()), c)
                })
                // Find all numbers that are touching the gear
                .filter(|((start, end), _)| *start <= gear_index + 1 && gear_index - 1 <= *end)
                // Convert to a usize
                .map(|((_start, _end), chars)| {
                    chars
                        .iter()
                        .copied()
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<usize>>();

    if x.len() == 2 {
        x[0] * x[1]
    } else {
        0
    }
}

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    grid
}

fn task1() -> usize {
    let input = get_task(3);
    let grid = parse_grid(&input);

    let mut sum: usize = 0;
    for (line_no, line) in grid.iter().enumerate() {
        for ((start, end), chars) in line
            .iter()
            .enumerate()
            .map(|(i, c)| c.is_numeric().then_some((i, c)))
            .group_by(std::option::Option::is_some)
            .into_iter()
            .filter(|(k, _)| *k)
            .map(|x| x.1.flatten())
            .map(|f| {
                let (i, c): (Vec<usize>, Vec<&char>) = f.unzip();
                ((*i.first().unwrap(), *i.last().unwrap()), c)
            })
        {
            if helper_1(&grid, line_no, start, end) {
                sum += chars
                    .iter()
                    .copied()
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap();
            }
        }
    }
    sum
}

fn task2() -> usize {
    let input = get_task(3);
    let grid = parse_grid(&input);

    grid.iter()
        .enumerate()
        .map(|(line_no, line)| {
            line.iter()
                .enumerate()
                .filter(|(_index, &c)| c == '*')
                .map(|(index, _)| helper_2(&grid, line_no, index))
                .sum::<usize>()
        })
        .sum()
}
