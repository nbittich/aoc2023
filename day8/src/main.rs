use std::collections::HashMap;

fn main() {
    run(include_str!("input.txt"));
}

fn run(input: &str) {
    let (instructions, elts) = input.trim().split_once("\n\n").unwrap();
    let elts = elts
        .lines()
        .filter_map(|line| line.split_once('='))
        .map(|(k, v)| (k.trim(), v.trim()))
        .filter_map(|(k, v)| {
            v.split_once(',')
                .map(|(l, r)| (l.trim(), r.trim()))
                .map(|(l, r)| (&l[1..], &r[0..r.len() - 1]))
                .map(|tuple| (k, tuple))
        })
        .collect::<HashMap<_, _>>();

    println!(
        "part 1: {}",
        find("AAA", |position| position == "ZZZ", instructions, &elts)
    );

    let starting_positions: Vec<_> = elts.keys().filter(|k| k.ends_with('A')).copied().collect();

    let lcm = starting_positions
        .iter()
        .map(|sp| find(sp, |pos| pos.ends_with('Z'), instructions, &elts))
        .fold(1, |lcm, n| lcm * n / gcd(lcm, n));

    println!("part 2: {lcm:?}");
}

fn gcd(mut a: u128, mut b: u128) -> u128 {
    while a * b != 0 {
        if a > b {
            a -= b;
        } else {
            b -= a;
        }
    }
    if a == 0 {
        b
    } else {
        a
    }
}

fn find<F>(start: &str, search: F, instructions: &str, elts: &HashMap<&str, (&str, &str)>) -> u128
where
    F: Fn(&str) -> bool,
{
    let mut count = 0;
    let mut position = start;

    for instr in instructions.chars().cycle() {
        let (left, right) = elts[position];
        position = if instr == 'L' { left } else { right };
        count += 1;
        if search(position) {
            break;
        }
    }
    count
}
