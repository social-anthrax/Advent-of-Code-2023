use crate::task_handler::get_task;

pub fn tasks() {
    println!("{}", task1());
    println!("{}", task2());
}

fn task1() -> usize {
    let task = get_task(1);
    let lines = task.lines().filter(|&x| !x.is_empty());
    let mut sum = 0;
    for line in lines {
        let mut i = line.chars().filter(|x| x.is_numeric());
        let first = i.next().unwrap();
        let second = i.last().unwrap_or(first);
        sum += &format!("{first}{second}").parse().unwrap();
    }
    sum
}

fn task2() -> usize {
    let task = get_task(1);
    txt_replace(&task)
}

// 55218
fn txt_replace(task: &str) -> usize {
    let lines = task.lines().filter(|&x| !x.is_empty());
    let mut sum = 0;

    for line in lines {
        let replaced_line = line
            .to_lowercase()
            .replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "t3e")
            .replace("four", "4")
            .replace("five", "5e")
            .replace("six", "s6")
            .replace("seven", "7n")
            .replace("eight", "e8t")
            .replace("nine", "n9e")
            .replace("zero", "0o");

        let mut i = replaced_line.chars().filter(|x| x.is_numeric());
        let first = i.next().unwrap();
        let second = i.last().unwrap_or(first);
        sum += &format!("{first}{second}").parse().unwrap();
    }

    sum
}

#[allow(dead_code)]
fn txt_replace_golf(task: &str) -> usize {
    task.lines()
        .filter(|&x| !x.is_empty())
        .map(|line| {
            line.to_lowercase()
                .replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "t3e")
                .replace("four", "4")
                .replace("five", "5e")
                .replace("six", "s6")
                .replace("seven", "7n")
                .replace("eight", "e8t")
                .replace("nine", "n9e")
                .replace("zero", "0o")
                .chars()
                .filter(|x| x.is_numeric())
                .collect::<Vec<_>>()
        })
        .map(|x| {
            format!("{}{}", x.first().unwrap(), x.last().unwrap())
                .parse::<usize>()
                .unwrap()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use crate::{
        task_handler::get_task,
        tasks::task1::{txt_replace, txt_replace_golf},
    };

    use super::task2;

    #[test]
    fn test_task2() {
        assert_eq!(task2(), 55218);
    }

    #[test]
    fn test_jank() {
        let input = get_task(1);
        assert_eq!(txt_replace_golf(&input), txt_replace(&input));
    }
}
