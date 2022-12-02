use std::fs;
use std::error::Error;
use std::collections::HashMap;

pub struct Config {
    file_path: String,
    problem_number: i32,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("must provide input file_path and which problem to solve")
        }
        let file_path = args[1].clone();
        let problem_number = args[2].clone().parse::<i32>().unwrap_or_default();

        Ok(Config { file_path, problem_number })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let result = solve(&contents, &config.problem_number)?;
    println!("{result}");
    Ok(())
}

fn solve(contents: &str, problem_number: &i32) -> Result<i32, Box<dyn Error>> {
    let mut score = 0;

    let line_scorer: fn(line: &str) -> Result<i32, Box<dyn Error>> = match problem_number {
        1 => initial_line_score,
        2 => correct_line_score,
        _ => Err("must select problem 1 or 2")?
    };

    for line in contents.lines() {
        score += line_scorer(line)?;
    }
    
    Ok(score)
}

fn correct_line_score(line: &str) -> Result<i32, Box<dyn Error>> {
    let moves = line.split(" ").collect::<Vec<&str>>();
    if moves.len() < 2 {
        Err("line does not contain 2 moves")?
    }

    let mut local_score = 0;

    match moves[0] {
        "A" => {
            match moves[1] {
                "X" => local_score += 3,
                "Y" => local_score += 4,
                "Z" => local_score += 8,
                _ => {}
            }
        }
        "B" => {
            match moves[1] {
                "X" => local_score += 1,
                "Y" => local_score += 5,
                "Z" => local_score += 9,
                _ => {}
            }
        }
        "C" => {
            match moves[1] {
                "X" => local_score += 2,
                "Y" => local_score += 6,
                "Z" => local_score += 7,
                _ => {}
            }
        }
        _ => {}
    }

    Ok(local_score)
}

fn initial_line_score(line: &str) -> Result<i32, Box<dyn Error>> {
    let shape_value = HashMap::from([ ("X", 1), ("A", 1), ("Y", 2), ("B", 2), ("Z", 3), ("C", 3) ]);


    let moves = line.split(" ").collect::<Vec<&str>>();
    if moves.len() < 2 {
        Err("line does not contain 2 moves")?
    }

    let mut local_score = 0;
    match shape_value.get(moves[1]) {
        Some(value) => local_score += value,
        None => Err("invalid move {moves[1]}")?
    }


    // AX rock, BY paper, CZ scissors
    match moves[0] {
        "A" => {
            match moves[1] {
                "X" => local_score += 3,
                "Y" => local_score += 6,
                _ => {}
            }
        }
        "B" => {
            match moves[1] {
                "Z" => local_score += 6,
                "Y" => local_score += 3,
                _ => {}
            }
        }
        "C" => {
            match moves[1] {
                "X" => local_score += 6,
                "Z" => local_score += 3,
                _ => {}

            }
        }
        _ => Err("invalid move {moves[0]}")?
    }

    Ok(local_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let input = "A Y \nB X \nC Z \n";

        let p = 1;
        let result = solve(input, &p).unwrap_or_else(|err| {
            println!("failed to unwrap test result, {err}");
            return -1
        });

        assert_eq!(15, result);
    }

    #[test]
    fn p2() {
        let input = "A Y \nB X \nC Z \n";

        let p = 2;
        let result = solve(input, &p).unwrap_or_else(|err| {
            println!("failed to unwrap test result, {err}");
            return -1
        });

        assert_eq!(12, result);
    }
}
