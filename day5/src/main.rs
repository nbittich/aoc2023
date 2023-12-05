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

    fn process(out: &mut [u32], maps: &[(&str, Vec<Vec<u32>>)]) {
        for (_, mapper) in maps {
            out.par_iter_mut().for_each(|value| {
                for map in mapper {
                    let dest = map[0];
                    let src = map[1];
                    let len = map[2];
                    let delta = dest - src;
                    if *value >= src && *value < src + len {
                        *value += delta;
                        break;
                    }
                }
            });
        }
    }
    let seeds = maps
        .iter()
        .take(1)
        .flat_map(|(_, seeds)| seeds.iter())
        .flatten()
        .copied()
        .collect::<Vec<_>>();

    // part 1
    let mut out_part_1 = seeds.clone();
    process(&mut out_part_1, &maps[1..]);

    println!("part 1: {}", out_part_1.iter().min().unwrap());

    // part 2
    let mut out_part_2 = seeds
        .par_chunks(2)
        .flat_map(|chunk| {
            let start = chunk[0];
            let len = chunk[1];
            start..start + len
        })
        .collect::<Vec<_>>();
    process(&mut out_part_2, &maps[1..]);

    println!("part 2: {}", out_part_2.iter().min().unwrap());
    let end_t = SystemTime::now();

    println!(
        "took: {} secs",
        end_t.duration_since(start_t).unwrap().as_secs()
    );
}
