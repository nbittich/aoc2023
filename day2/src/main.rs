fn main() -> Result<(), &'static str> {
    part1().ok_or("error")?;
    part2().ok_or("error")?;
    Ok(())
}

fn position_digit(s: &str) -> Option<usize> {
    s.chars()
        .enumerate()
        .take_while(|(_, c)| c.is_ascii_digit())
        .map(|(idx, _)| idx)
        .last()
}

fn part1() -> Option<()> {
    let input = include_str!("input.txt");
    let max_red_cube = 12;
    let max_green_cube = 13;
    let max_blue_cube = 14;
    let mut total = 0;

    for line in input.lines() {
        let line = &line[5..];
        let pos_id = position_digit(line)?;
        let id = line[0..=pos_id].to_string().parse::<u32>().ok()?;
        let line = &line[pos_id + 2..];
        let bags = line.split(';');

        let mut valid = true;
        'inner: for bag in bags {
            let cubes = bag.split(',');
            for cube in cubes.map(|c| c.trim()) {
                let pos_amount = position_digit(cube)?;
                let amount = cube[0..=pos_amount].to_string().parse::<u32>().ok()?;
                let color = &cube[pos_amount + 1..];
                match color.trim() {
                    "blue" => {
                        if amount > max_blue_cube {
                            valid = false;
                            break 'inner;
                        }
                    }
                    "red" => {
                        if amount > max_red_cube {
                            valid = false;
                            break 'inner;
                        }
                    }
                    "green" => {
                        if amount > max_green_cube {
                            valid = false;
                            break 'inner;
                        }
                    }
                    _ => {
                        panic!("something went wrong '{color}'");
                    }
                }
            }
        }
        if valid {
            total += id;
        }
    }
    println!("part1: {total}");

    Some(())
}

fn part2() -> Option<()> {
    let input = include_str!("input.txt");
    let mut total = 0;

    for line in input.lines() {
        let line = &line[5..];
        let pos_id = position_digit(line)?;
        let line = &line[pos_id + 2..];
        let bags = line.split(';');
        let mut min_blue_cube = 1;
        let mut min_red_cube = 1;
        let mut min_green_cube = 1;
        for bag in bags {
            let cubes = bag.split(',');
            for cube in cubes.map(|c| c.trim()) {
                let pos_amount = position_digit(cube)?;
                let amount = cube[0..=pos_amount].to_string().parse::<u32>().ok()?;
                let color = &cube[pos_amount + 1..];
                match color.trim() {
                    "blue" => {
                        if amount > min_blue_cube {
                            min_blue_cube = amount;
                        }
                    }
                    "red" => {
                        if amount > min_red_cube {
                            min_red_cube = amount;
                        }
                    }
                    "green" => {
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
        total += min_blue_cube * min_red_cube * min_green_cube;
    }
    println!("part2: {total}");

    Some(())
}
