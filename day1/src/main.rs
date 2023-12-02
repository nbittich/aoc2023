use std::{collections::VecDeque, usize};

fn main() {
    println!("*************** day 1 ***************");
    part1();
    part2();
    println!("*************** day 1 ***************");
}

fn part1() {
    let input = include_str!("input.txt");

    let total: u32 = input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<VecDeque<_>>()
        })
        .filter_map(|numbers| {
            let first = numbers.front();
            let last = numbers.back();

            if let (Some(first), Some(last)) = (first, last) {
                Some(first * 10 + last)
            } else {
                None
            }
        })
        .sum();
    println!("part 1: {total}");
}
fn part2() {
    let input = include_str!("input.txt");

    let mut buffer = String::new();
    let mut numbers = VecDeque::new();

    let mut total = 0;

    const SPELLED_NUMBERS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for line in input.lines() {
        for c in line.chars() {
            if let Some(number) = c.to_digit(10) {
                numbers.push_back(number as usize);
            } else {
                buffer.push(c);
                let maybe_number = SPELLED_NUMBERS.iter().position(|sp| *sp == buffer);
                if let Some(number) = maybe_number {
                    numbers.push_back(number + 1);
                } else {
                    while !buffer.is_empty()
                        && !SPELLED_NUMBERS.iter().any(|sp| sp.starts_with(&buffer))
                    {
                        buffer.remove(0);
                    }
                }
            }
        }

        let first = numbers.front();
        let last = numbers.back();

        if let (Some(first), Some(last)) = (first, last) {
            total += first * 10 + last;
        }
        buffer.clear();
        numbers.clear();
    }
    println!("part 2: {total}");
}
