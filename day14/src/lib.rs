use itertools::Itertools;
use std::collections::HashSet;
use std::error::Error;
use std::fs;

pub struct Config {
    file_path: String,
    problem_number: i32,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("must provide input file_path and which problem to solve");
        }
        let file_path = args[1].clone();
        let problem_number = args[2].clone().parse::<i32>().unwrap_or_default();

        Ok(Config {
            file_path,
            problem_number,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let result = solve(&contents, config.problem_number)?;
    println!("result -> {result}");
    Ok(())
}

// immediate thoughts: building this seems ok,
// stop condition seems to be when the only option is out of bounds
// bounds should be determined by rock positions
// can slide over a rock path with iterator windows (probably some fasterthanlime genius way but idk)
fn solve(contents: &str, problem_number: i32) -> Result<usize, Box<dyn Error>> {
    let (mut rock_set, max_i, max_j, min_j) = rock_set(contents);
    if problem_number == 1 {
        return how_much_sand(&mut rock_set, max_i, max_j, min_j);
    } else {
        return piled_sand(&mut rock_set, max_i + 2);
    }
}

fn piled_sand(
    rock_set: &mut HashSet<(usize, usize)>,
    floor: usize,
) -> Result<usize, Box<dyn Error>> {
    let mut sand_dropped = 1;

    let mut full = false;
    while !full {
        // drop sand
        let mut spot: (usize, usize) = (0, 500);

        while 1 == 1 {
            let next_spot = next_spot(spot, &rock_set);
            if spot == next_spot || next_spot.0 == floor {
                if next_spot == (0, 500) {
                    full = true;
                    break;
                }
                // can no longer move
                rock_set.insert(spot);
                sand_dropped += 1;
                break;
            }
            spot = next_spot;
        }
    }

    Ok(sand_dropped)
}

fn how_much_sand(
    rock_set: &mut HashSet<(usize, usize)>,
    max_i: usize,
    max_j: usize,
    min_j: usize,
) -> Result<usize, Box<dyn Error>> {
    let mut sand_dropped = 0;

    let mut reached_bottom = false;
    let mut reached_side = false;
    while !reached_bottom && !reached_side {
        // drop sand
        let mut spot: (usize, usize) = (0, 500);

        while 1 == 1 {
            let next_spot = next_spot(spot, &rock_set);
            if spot == next_spot {
                // can no longer move
                rock_set.insert(spot);
                sand_dropped += 1;
                break;
            }
            spot = next_spot;

            if spot.0 == max_i {
                reached_bottom = true;
                break;
            } else if spot.1 == max_j || spot.1 == min_j {
                reached_side = true;
                break;
            }
        }
    }

    Ok(sand_dropped)
}

fn next_spot(spot: (usize, usize), rock_set: &HashSet<(usize, usize)>) -> (usize, usize) {
    // check below
    if !rock_set.contains(&(spot.0 + 1, spot.1)) {
        return (spot.0 + 1, spot.1);
    } else if !rock_set.contains(&(spot.0 + 1, spot.1 - 1)) {
        return (spot.0 + 1, spot.1 - 1);
    } else if !rock_set.contains(&(spot.0 + 1, spot.1 + 1)) {
        return (spot.0 + 1, spot.1 + 1);
    }
    return spot;
}

fn rock_set(contents: &str) -> (HashSet<(usize, usize)>, usize, usize, usize) {
    let rock_lines: Vec<Vec<(usize, usize)>> = contents
        .lines()
        .map(|l| {
            l.split(" -> ")
                .map(|c| {
                    c.split(",")
                        .map(|num| num.parse::<usize>().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect()
        })
        .collect();

    let (max_i, max_j, min_j) = grid_dims(&rock_lines);

    let mut rocks: HashSet<(usize, usize)> = HashSet::new();

    for rl in &rock_lines {
        for line in rl.windows(2) {
            if line[0].0 == line[1].0 {
                // vertical line
                let start = line[0].1.min(line[1].1);
                let fin = line[0].1.max(line[1].1);
                for i in start..fin + 1 {
                    rocks.insert((i, line[0].0));
                }
            } else if line[0].1 == line[1].1 {
                // horizontal line
                let start = line[0].0.min(line[1].0);
                let fin = line[0].0.max(line[1].0);
                for j in start..fin + 1 {
                    rocks.insert((line[0].1, j));
                }
            }
        }
    }

    return (rocks, max_i, max_j, min_j);
}

fn grid_dims(rock_lines: &Vec<Vec<(usize, usize)>>) -> (usize, usize, usize) {
    let mut max_i = 0;
    let mut max_j = 500;
    let mut min_j = 500;
    for rl in rock_lines {
        for chunk in rl {
            if chunk.1 > max_i {
                max_i = chunk.1;
            }
            if chunk.0 > max_j {
                max_j = chunk.0;
            }
            if chunk.0 < min_j {
                min_j = chunk.0;
            }
        }
    }
    (max_i, max_j, min_j)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn p1() {
        let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
        let p = 1;

        let result = solve(input, p).unwrap_or_else(|err| {
            println!("failed to unwrap test result, {err}");
            return 1;
        });

        assert_eq!(24, result);
    }

    #[test]
    fn p2() {
        let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
        let p = 2;

        let result = solve(input, p).unwrap_or_else(|err| {
            println!("failed to unwrap test result, {err}");
            return 1;
        });

        assert_eq!(93, result);
    }
}
