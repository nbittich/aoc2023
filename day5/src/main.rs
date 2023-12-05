use std::time::SystemTime;

use rayon::prelude::*;

fn main() {
    run();
}

fn run() {
    let start_t = SystemTime::now();
    let input = include_str!("input.txt").trim();
    let maps = input
        .split("\n\n")
        .filter_map(|m| {
            m.split_once(':').map(|(k, v)| {
                (
                    k.trim(),
                    v.trim()
                        .lines()
                        .map(|l| l.split(' '))
                        .map(|numbers| {
                            numbers
                                .filter_map(|n| n.parse::<u32>().ok())
                                .collect::<Vec<_>>()
                        })
                        .collect::<Vec<_>>(),
                )
            })
        })
        .collect::<Vec<_>>();

    fn process(value: &u32, maps: &[(&str, Vec<Vec<u32>>)]) -> u32 {
        let mut new_value = *value;
        for (_, mapper) in maps {
            for map in mapper {
                let dest = map[0];
                let src = map[1];
                let len = map[2];
                let delta = dest - src;
                if new_value >= src && new_value < src + len {
                    new_value += delta;
                    break;
                }
            }
        }
        new_value
    }
    let seeds = maps
        .iter()
        .take(1)
        .flat_map(|(_, seeds)| seeds.iter())
        .flatten()
        .copied()
        .collect::<Vec<_>>();

    // part 1
    let out_part_1 = seeds
        .par_iter()
        .map(|v| process(v, &maps[1..]))
        .min()
        .unwrap();
    println!("part 1: {}", out_part_1);

    // part 2

    let out_part_2 = seeds
        .par_chunks(2)
        .filter_map(|chunk| {
            let start = chunk[0];
            let len = chunk[1];
            (start..start + len)
                .into_par_iter()
                .map(|v| process(&v, &maps[1..]))
                .min()
        })
        .min()
        .unwrap();

    println!("part 2: {}", out_part_2);
    let end_t = SystemTime::now();

    println!(
        "took: {} secs",
        end_t.duration_since(start_t).unwrap().as_secs()
    );
}
