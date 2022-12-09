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

fn solve(contents: &str, problem_number: i32) -> Result<i32, Box<dyn Error>> {
    if problem_number == 1 {
        return visible_tree_count(contents);
    } else {
        return highest_tree_score(contents);
    }
}

fn highest_tree_score(contents: &str) -> Result<i32, Box<dyn Error>> {
    let grid = make_grid(contents);

    let (num_rows, num_cols) = (grid.len(), grid[0].len());
    let (mut i, mut j) = (0, 0);
    let mut result = 0;

    while i < num_rows {
        if i == 0 || i == num_rows - 1 {
            i += 1;
            continue;
        }
        while j < num_cols {
            if j == 0 || j == num_cols - 1 {
                j += 1;
                continue;
            }


            dbg!("========", (i, j));
            let hs = horiz_score(&grid, i, j, num_cols);
            let vs = vert_score(&grid, i, j, num_rows);

            if hs * vs > result {
                result = hs * vs;
            }

            j += 1;
        }
        j = 0;
        i += 1;
    }

    Ok(result)
}

fn vert_score(grid: &Vec<Vec<i32>>, i: usize, j: usize, num_rows: usize) -> i32 {
    let mut ind: i32 = i as i32 - 1;
    let val = grid[i][j];
    let mut result = 1;

    while ind >= 0 {
        let index = ind as usize;
        if grid[index][j] >= val || index == 0{
            dbg!(i - index);
            result *= i - index;
            break;
        }
        ind -= 1;
    }

    let mut ind = i + 1;
    while ind < num_rows {
        if grid[ind][j] >= val || ind == num_rows-1 {
            dbg!(ind - i);
            result *= ind - i;
            break;
        }
        ind += 1;
    }

    return result as i32;
}

fn horiz_score(grid: &Vec<Vec<i32>>, i: usize, j: usize, num_cols: usize) -> i32 {
    let mut ind: i32 = j as i32 - 1;
    let val = grid[i][j];
    let mut result = 1;
    while ind >= 0 {
        let index = ind as usize;
        if grid[i][index] >= val || index == 0 {
            dbg!(j - index);
            result *= j - index;
            break;
        }
        ind -= 1;
    }

    let mut ind = j + 1;
    while ind < num_cols {
        if grid[i][ind] >= val || ind == num_cols-1 {
            dbg!(ind - j);
            result *= ind - j;
            break;
        }
        ind += 1;
    }

    return result as i32;
}

fn visible_tree_count(contents: &str) -> Result<i32, Box<dyn Error>> {
    let grid = make_grid(contents);
    let mut visible_coords = HashSet::new();
    let (num_rows, num_cols) = (grid.len(), grid[0].len());
    let (mut i, mut j) = (0, 0);

    while i < num_rows {
        while j < num_cols {
            let curr_val = &grid[i][j];
            let ml = grid[i][0..j].into_iter().max().unwrap_or(&-1);
            let mr = grid[i][j + 1..num_cols].into_iter().max().unwrap_or(&-1);
            let ma = max_above(&grid, i, j);
            let mb = max_below(&grid, i, j, num_rows - 1);

            // dbg!(i, j, curr_val, max_left, max_right, max_above, max_below);

            if curr_val > ml || curr_val > mr || curr_val > &ma || curr_val > &mb {
                visible_coords.insert((i, j));
            }

            j += 1;
        }
        j = 0;
        i += 1;
    }

    Ok(visible_coords.len() as i32)
}

fn max_below(grid: &Vec<Vec<i32>>, i: usize, j: usize, num_rows: usize) -> i32 {
    let mut max = -1;
    let mut ind = num_rows;

    while ind > i {
        if grid[ind][j] > max {
            max = grid[ind][j];
        }

        ind -= 1
    }

    return max;
}

fn max_above(grid: &Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
    let mut max = -1;
    let mut ind = 0;

    while ind < i {
        if grid[ind][j] > max {
            max = grid[ind][j]
        }

        ind += 1
    }

    return max;
}

fn make_grid(contents: &str) -> Vec<Vec<i32>> {
    contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let input = "30373\n25512\n65332\n33549\n35390";
        let p = 1;

        let result = solve(input, p).unwrap_or_else(|err| {
            println!("failed to unwrap test result, {err}");
            return -1;
        });

        assert_eq!(21, result);
    }

    #[test]
    fn p2() {
        let input = "30373\n25512\n65332\n33549\n35390";
        let p = 2;

        let result = solve(input, p).unwrap_or_else(|err| {
            println!("failed to unwrap test result, {err}");
            return -1;
        });

        assert_eq!(8, result);
    }
}
