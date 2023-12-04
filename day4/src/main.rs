use std::{collections::BTreeMap, u64};

fn main() {
    run();
}

fn run() {
    let input = include_str!("input.txt").trim();

    let mut cards = vec![];

    for line in input.lines() {
        let (_, line) = line.split_once(':').unwrap();
        let (winning_numbers, numbers) = line
            .split_once('|')
            .map(|(l, r)| {
                (
                    l.split_whitespace().collect::<Vec<_>>(),
                    r.split_whitespace().collect::<Vec<_>>(),
                )
            })
            .unwrap();

        cards.push((winning_numbers, numbers));
    }
    let winning_cards = cards
        .iter()
        .enumerate()
        .filter_map(|(idx, (winning_numbers, numbers))| {
            let matches = numbers
                .iter()
                .filter(|n| winning_numbers.iter().any(|wn| &wn == n))
                .count();
            if matches > 0 {
                Some((idx, matches))
            } else {
                None
            }
        })
        .collect::<BTreeMap<_, _>>();

    let tot_part_1 = winning_cards
        .values()
        .map(|matches| 2u64.pow(*matches as u32 - 1))
        .sum::<u64>();

    println!("part 1: {tot_part_1}");

    let mut tot_part_2 = 0;
    let mut queue = (0..cards.len()).collect::<Vec<_>>();

    while let Some(current_card_idx) = queue.pop() {
        tot_part_2 += 1;
        if let Some(&matches) = winning_cards.get(&current_card_idx) {
            let next_card = current_card_idx + 1;
            queue.append(&mut (next_card..next_card + matches).collect());
        }
    }
    println!("part 2: {tot_part_2}");
}
