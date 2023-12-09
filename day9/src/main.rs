const SAMPLE: &str = r#"
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"#;

fn main() {
    run(SAMPLE);
    run(include_str!("input.txt"));
}

fn run(input: &str) {
    let sequences = input
        .trim()
        .lines()
        .map(|l| {
            l.split_whitespace()
                .filter_map(|n| n.parse::<i64>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut total_part_1 = 0;
    let mut total_part_2 = 0;
    for sequence in sequences {
        let mut prediction = vec![];
        let mut new_sequences = sequence;
        while !new_sequences.iter().all(|&n| n == 0) {
            let current_sequences = new_sequences
                .windows(2)
                .map(|arr| arr[1] - arr[0])
                .collect();
            prediction.push(new_sequences);
            new_sequences = current_sequences;
        }

        let mut current_r = 0;
        let mut current_l = 0;
        for seq in prediction.iter().rev() {
            current_r += seq[seq.len() - 1];
            current_l = seq[0] - current_l;
        }
        total_part_1 += current_r;
        total_part_2 += current_l;
    }
    println!("part 1: {}", total_part_1);

    println!("part 2: {}", total_part_2);
}
