use std::fs;
use std::error::Error;
use std::collections::HashSet;


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
    let result = solve(&contents, config.problem_number)?;
    println!("result -> {result}");
    Ok(())
}

fn solve(contents: &str, problem_number: i32) -> Result<i32, Box<dyn Error>> {

    if problem_number == 1 {
        return duplicate_items(contents)
    } else {
        return group_badge(contents)
    }

}

fn group_badge(contents: &str) -> Result<i32, Box<dyn Error>> {
    let mut result = 0;
    let mut lines = contents.lines().peekable();

    while lines.peek().is_some() == true {
        let c1 = lines.next().unwrap();
        let c2 = lines.next().unwrap();
        let c3 = lines.next().unwrap();

        let mut compartment_one_chars = HashSet::new();
        for c in c1.chars() {
            compartment_one_chars.insert(c);
        }

        let mut compartment_two_chars = HashSet::new();
        for c in c2.chars() {
            compartment_two_chars.insert(c);
        }

        for c in c3.chars() {
            if compartment_one_chars.contains(&c) && compartment_two_chars.contains(&c) {
                result += calculate_priority(c);
                break;
            }
        }
    }

    Ok(result)
}

fn duplicate_items(contents: &str) -> Result<i32, Box<dyn Error>> {
    let mut result = 0;

    for line in contents.lines() {
        let mut compartment_one_chars = HashSet::new();
        let compartment_one = &line[0..line.len()/2];
        let compartment_two = &line[line.len()/2..line.len()];

        for c in compartment_one.chars() {
            compartment_one_chars.insert(c);
        }

        for c in compartment_two.chars() {
            if compartment_one_chars.contains(&c) {
                result += calculate_priority(c);
                break;
            }
        }
    }

    Ok(result)
}

fn calculate_priority(c: char) -> i32 {
    let c_num = c as i32;
    if c_num > 96 {
        return c_num - 96
    } else {
        return c_num - 65 + 27
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";

        let p = 1;
        let result = solve(input, p).unwrap_or_else(|err| {
            println!("failed to unwrap test result, {err}");
            return -1
        });

        assert_eq!(157, result);
    }

    #[test]
    fn p2() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";

        let p = 2;
        let result = solve(input, p).unwrap_or_else(|err| {
            println!("failed to unwrap test result, {err}");
            return -1
        });

        assert_eq!(70, result);
    }
}
