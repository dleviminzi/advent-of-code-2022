use std::error::Error;
use std::fs;

pub struct Config {
    file_path: String,
    top_n: i32,
}

impl Config {
    // constructor                            ?? what is 2nd arg here       
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            // TODO: should make top number optional
            return Err("must provide input file_path and number to sum")
        }

        // The args variable in main is the owner of the argument values and is 
        // only letting the parse_config function borrow them, which means we’d 
        // violate Rust’s borrowing rules if Config tried to take ownership of the 
        // values in args. This is why we call clone to completely copy the data 
        // into a new object that Config can own. This is not efficient and uses
        // more memory than we technically need, but it is logic to follow. This 
        // way, we don't need to worry about lifetimes of references. 
        // 
        // remember -> It’s better to have a working program that’s a bit 
        // inefficient than to try to hyperoptimize code on your first pass.
        let file_path = args[1].clone();
        let top_n = args[2].clone().parse::<i32>().unwrap_or_default();

        Ok(Config { file_path, top_n })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let result = solve_top(&contents)?;
    let result_n = solve_top_n(&contents, config.top_n)?;
    println!("{result}");
    println!("{result_n}");
    Ok(())
}

pub fn solve_top_n(contents: &str, n: i32) -> Result<i32, Box<dyn Error>> {
    let mut vec = Vec::new();
    let mut curr = 0;

    // collect
    for line in contents.lines() {
        if line == "" {
            vec.push(curr);
            curr = 0;
            continue;
        }

        let num: i32 = line.parse::<i32>()?;
        curr += num;
    }

    // sort
    vec.sort();

    let mut i :usize = n as usize;
    let mut result = 0; 
    while i > 0 {
        result += vec[vec.len()-i];
        i -= 1;
    }
    Ok(result)
}

pub fn solve_top(contents: &str) -> Result<i32, Box<dyn Error>> {
    let mut max = 0;
    let mut curr = 0; 
    for line in contents.lines() {
        if line == "" {
            if curr > max {
                max = curr;
            }
            curr = 0;
            continue;
        }

        let num: i32 = line.parse::<i32>()?;
        curr += num;
    }

    Ok(max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_input() {
        let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";


        let result = solve_top(input).unwrap_or_else(|err| {
            println!("failed to unwrap test result, {err}");
            return -1
        });

        assert_eq!(24000, result);
    }
}
