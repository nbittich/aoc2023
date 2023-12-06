const _SAMPLE: &str = r#"
Time:      7  15   30
Distance:  9  40  200
"#;

const INPUT: &str = r#"
Time:        40     81     77     72
Distance:   219   1012   1365   1089
"#;

fn main() {
    part1(INPUT);
    part2(INPUT);
}

fn part1(input: &str) {
    let input = input
        .trim()
        .lines()
        .filter_map(|line| {
            line.split_once(':').map(|(_, s)| {
                s.split_whitespace()
                    .filter_map(|s| s.parse::<u32>().ok())
                    .collect::<Vec<_>>()
            })
        })
        .collect::<Vec<_>>();
    let time = &input[0];
    let distance = &input[1];

    let mut total = 1;

    for (&time, &distance) in time.iter().zip(distance.iter()) {
        let mut count = 0;
        for i in 1..time {
            let remain = time - i;
            let dist = remain * i;
            if dist > distance {
                count += 1;
            }
        }
        total *= count;
    }
    println!("{total}");
}

fn part2(input: &str) {
    let input = input
        .trim()
        .lines()
        .filter_map(|line| {
            line.split_once(':')
                .and_then(|(_, s)| {
                    s.trim()
                        .chars()
                        .filter(|c| !c.is_whitespace())
                        .collect::<String>()
                        .parse::<u128>()
                        .ok()
                })
        })
        .collect::<Vec<_>>();
    let time = input[0];
    let distance = input[1];

    let mut total = 0;
    for i in 1..time {
        let remain = time - i;
        let dist = remain * i;
        if dist > distance {
            total += 1;
        }
    }
    println!("{total}");
}
