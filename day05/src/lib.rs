use std::fs;
use std::error::Error;


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

fn solve(contents: &str, problem_number: i32) -> Result<String, Box<dyn Error>> {

    let parts: Vec<&str> = contents.split("\n\n").collect();

    if parts.len() < 2 {
        Err("did not provide starting stacks and instructions in proper format")?
    }

    let top: Vec<&str> = parts[0].split("\n").collect();
    // might be the single worst line of code I've ever seen
    let cnt = top[top.len()-1].chars().nth(top[top.len()-1].len()-2).unwrap().to_digit(10).unwrap();

    let stacks: Vec<Vec<char>> = create_starting_stacks(top[0..top.len()-1].to_vec(), cnt as usize);
    let instructions = parts[1];

    if problem_number == 1 {
        return top_boxes_9000(stacks, instructions);
    } else {
        return top_boxes_9001(stacks, instructions);
    }

}

fn top_boxes_9001(mut stacks: Vec<Vec<char>>, instructions: &str) -> Result<String, Box<dyn Error>> {
    for instr in instructions.lines() {
        let (to_move, source, dest) = parse_instruction(instr);

        let top = stacks[source].len();
        let bottom = top-to_move as usize;
        let mut to_append: Vec<char> = stacks[source].splice(bottom..top, []).collect();
        stacks[dest].append(&mut to_append);
    }

    Ok(top_boxes_string(stacks))
}


fn top_boxes_9000(mut stacks: Vec<Vec<char>>, instructions: &str) -> Result<String, Box<dyn Error>> {
    for instr in instructions.lines() {
        let (to_move, source, dest) = parse_instruction(instr);

        for _i in 0..to_move {
            let c = stacks[source].pop().unwrap();
            stacks[dest].push(c);
        }
    }

    Ok(top_boxes_string(stacks))
}

fn parse_instruction(instr: &str) -> (i32, usize, usize) {
    let parts: Vec<&str> = instr.split("from").collect();
    let to_move = parts[0].split(" ").nth(1).unwrap().parse::<i32>().unwrap();
    let source = parts[1].split(" ").nth(1).unwrap().parse::<usize>().unwrap();
    let dest = parts[1].split(" ").nth(3).unwrap().parse::<usize>().unwrap();

    return (to_move, source-1, dest-1)
}

fn top_boxes_string(stacks: Vec<Vec<char>>) -> String {
    let mut result = String::new();
    for i in 0..stacks.len() {
        result.push(stacks[i][stacks[i].len()-1])
    }

    return result
}


fn create_starting_stacks(top: Vec<&str>, cnt: usize) -> Vec<Vec<char>> {

    let mut stacks = vec![vec![]; cnt];

    for line in top.iter().rev() {
        let chars: Vec<char> = line.chars().collect();
        let mut x = 1;
        let mut stack_num = 0;
        while x < chars.len() && stack_num < stacks.len() {
            if chars[x] != ' ' {
                stacks[stack_num].push(chars[x]);
            }
            x += 4;
            stack_num += 1;
        }

    }

    return stacks;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let input = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2";
        let p = 1;

        let result = solve(input, p).unwrap_or_else(|err| {
            println!("failed to unwrap test result, {err}");
            return String::new()
        });

        assert_eq!("CMZ", result);
    }

    #[test]
    fn p2() {
        let input = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2";
        let p = 2;

        let result = solve(input, p).unwrap_or_else(|err| {
            println!("failed to unwrap test result, {err}");
            return String::new()
        });

        assert_eq!("MCD", result);
    }
}
