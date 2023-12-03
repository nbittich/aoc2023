fn main() {
    run();
}

fn run() {
    let input = include_str!("input.txt");
    let height = input.lines().count();
    let input: String = input.lines().map(|l| l.trim()).collect();

    let numbers = input
        .match_indices(|p: char| p.is_ascii_digit())
        .map(|(pos, num)| ((pos / height, pos % height), num))
        .collect::<Vec<_>>();
    let symbols = input
        .match_indices(|p: char| !p.is_ascii_digit() && p != '.')
        .map(|(pos, symbol)| ((pos / height, pos % height), symbol))
        .collect::<Vec<_>>();

    let mut real_numbers_and_range = vec![];
    for row in 0..height {
        let mut acc = String::new();
        let mut positions = vec![];
        let row = numbers
            .iter()
            .filter(|((r2, _), _)| *r2 == row)
            .collect::<Vec<_>>();

        for ((r, c), num) in row {
            if !positions.is_empty()
                && positions
                    .iter()
                    .last()
                    .filter(|(_, c2)| *c2 + 1 == *c)
                    .is_none()
            {
                let res = acc.parse::<u64>().unwrap();
                real_numbers_and_range.push((res, positions.clone()));

                acc.clear();
                positions.clear();
            }
            positions.push((*r, *c));
            acc += num;
        }
        if !positions.is_empty() {
            let res = acc.parse::<u64>().unwrap();
            real_numbers_and_range.push((res, positions.clone()));
        }
    }

    // part 1
    let numbers_adj = numbers
        .iter()
        .filter(|((row, column), _)| {
            symbols.iter().copied().any(|((r, c), _)| {
                let (row, column) = (*row, *column);
                (r == row || r + 1 == row || r - 1 == row)
                    && (c == column || c + 1 == column || c - 1 == column)
            })
        })
        .collect::<Vec<_>>();
    let total_part_1: u64 = real_numbers_and_range
        .iter()
        .filter_map(|(n, positions)| {
            if positions.iter().any(|(r1, c1)| {
                numbers_adj
                    .iter()
                    .any(|((r2, c2), _)| (*r1, *c1) == (*r2, *c2))
            }) {
                Some(n)
            } else {
                None
            }
        })
        .sum();
    println!("part 1: {total_part_1}");

    // part 2
    let mut total_part_2: u64 = 0;
    for ((r, c), _) in symbols.into_iter().filter(|(_, symbol)| *symbol == "*") {
        let numbers_adj = real_numbers_and_range
            .iter()
            .filter_map(|(num, positions)| {
                if positions.iter().copied().any(|(row, column)| {
                    (r == row || r + 1 == row || r - 1 == row)
                        && (c == column || c + 1 == column || c - 1 == column)
                }) {
                    Some(num)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        if numbers_adj.len() == 2 {
            total_part_2 += numbers_adj.into_iter().cloned().product::<u64>();
        }
    }
    println!("part 2: {total_part_2}");
}
