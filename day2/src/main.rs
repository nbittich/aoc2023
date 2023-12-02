const MAX_RED_CUBE: u32 = 12;
const MAX_GREEN_CUBE: u32 = 13;
const MAX_BLUE_CUBE: u32 = 14;

fn main() -> Result<(), &'static str> {
    run().ok_or("error")
}

fn run() -> Option<()> {
    let input = include_str!("input.txt");

    let mut total_part1 = 0;
    let mut total_part2 = 0;

    for line in input.lines() {
        let line = &line[5..];
        let (id, line) = line.split_once(':')?;

        let bags = line.split(';');
        let mut min_blue_cube = 1;
        let mut min_red_cube = 1;
        let mut min_green_cube = 1;
        let mut valid = true;
        for bag in bags {
            for cube in bag.split(',').map(|c| c.trim()) {
                let (amount, color) = cube.split_once(' ')?;
                let amount = amount.parse::<u32>().ok()?;
                match color.trim() {
                    "blue" => {
                        if amount > MAX_BLUE_CUBE {
                            valid = false;
                        }
                        if amount > min_blue_cube {
                            min_blue_cube = amount;
                        }
                    }
                    "red" => {
                        if amount > MAX_RED_CUBE {
                            valid = false;
                        }
                        if amount > min_red_cube {
                            min_red_cube = amount;
                        }
                    }
                    "green" => {
                        if amount > MAX_GREEN_CUBE {
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
            total_part1 += id.parse::<u32>().ok()?;
        }

        total_part2 += min_blue_cube * min_red_cube * min_green_cube;
    }

    println!("part1: {total_part1}");
    println!("part2: {total_part2}");

    Some(())
}
