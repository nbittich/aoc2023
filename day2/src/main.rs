fn main() -> Result<(), &'static str> {
    run().ok_or("error")
}

fn position_digit(s: &str) -> Option<usize> {
    s.chars()
        .enumerate()
        .take_while(|(_, c)| c.is_ascii_digit())
        .map(|(idx, _)| idx)
        .last()
}

fn run() -> Option<()> {
    let input = include_str!("input.txt");
    let max_red_cube = 12;
    let max_green_cube = 13;
    let max_blue_cube = 14;
    let mut total_part1 = 0;
    let mut total_part2 = 0;

    for line in input.lines() {
        let line = &line[5..];
        let pos_id = position_digit(line)?;
        let id = line[0..=pos_id].to_string().parse::<u32>().ok()?;
        let line = &line[pos_id + 2..];
        let bags = line.split(';');
        let mut min_blue_cube = 1;
        let mut min_red_cube = 1;
        let mut min_green_cube = 1;
        let mut valid = true;
        for bag in bags {
            let cubes = bag.split(',');
            for cube in cubes.map(|c| c.trim()) {
                let pos_amount = position_digit(cube)?;
                let amount = cube[0..=pos_amount].to_string().parse::<u32>().ok()?;
                let color = &cube[pos_amount + 1..];
                match color.trim() {
                    "blue" => {
                        if amount > max_blue_cube {
                            valid = false;
                        }
                        if amount > min_blue_cube {
                            min_blue_cube = amount;
                        }
                    }
                    "red" => {
                        if amount > max_red_cube {
                            valid = false;
                        }
                        if amount > min_red_cube {
                            min_red_cube = amount;
                        }
                    }
                    "green" => {
                        if amount > max_green_cube {
                            valid = false;
                        }
                        if amount > min_green_cube {
                            min_green_cube = amount;
                        }
                    }
                    _ => {
                        panic!("something went wrong '{color}'");
                    }
                }
            }
        }
        if valid {
            total_part1 += id;
        }
        total_part2 += min_blue_cube * min_red_cube * min_green_cube;
    }

    println!("part1: {total_part1}");
    println!("part2: {total_part2}");

    Some(())
}
