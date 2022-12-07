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
    let result = solve(&contents, config.problem_number)?;
    println!("result -> {result}");
    Ok(())
}

fn solve(contents: &str, problem_number: i32) -> Result<i32, Box<dyn Error>> {

    if problem_number == 1 {
        return sum_dir_under_n(contents, 100000);
    } else {
        return size_deleted(contents, 70000000, 30000000);
    }

}

fn size_deleted(contents: &str, max_storage: i32, update_size: i32) -> Result<i32, Box<dyn Error>> {

    let sizes = path_sizes(contents, false)?;
    let used = max_storage - sizes.values().sum::<i32>();
    let to_delete = update_size - used;

    dbg!(used);
    dbg!(&to_delete);

    let mut candidate = &max_storage; 
    for (_k, v) in sizes.iter() {
        dbg!(v);
        if *v >= to_delete && v < candidate {
            candidate = v; 
        }
    }

    Ok(*candidate)

}

// should solve by keeping list of dirs where each dir contains a ptr to its children
// but idk how to build a map/graph in rust with borrow checker
fn sum_dir_under_n(contents: &str, arg: i32) -> Result<i32, Box<dyn Error>> {

    let sizes = path_sizes(contents, true)?;

    let mut result = 0;
    for (_k, v) in sizes.iter() {
        if v < &arg {
            result += v;
        }
    }

    Ok(result)
}

fn path_sizes(contents: &str, rolled_up: bool) -> Result<HashMap<String, i32> ,Box<dyn Error>> {
    let mut size = HashMap::new();
    let mut path: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.starts_with("$") && line.contains("cd") {
            let dir = line.rsplitn(2, ' ').nth(0).unwrap();

            if dir == ".." {
                path.pop();
                continue;
            } else if dir == "/" {
                path.clear();
                path.push(dir);
                continue;
            }

            path.push(dir);

        }
        else if line.starts_with("$") && line.contains("ls") {
            continue;
        } else if line.starts_with("dir") {
            continue;
        }
        else {
            let file_size = line.split(' ').nth(0).unwrap().parse::<i32>().unwrap();

            if rolled_up { 
                // I know I know 
                let mut p = path.clone();
                while !p.is_empty() {
                    let key = p.join(" ");
                    size.entry(key).and_modify(|counter| *counter += file_size).or_insert(file_size);
                    p.pop();
                }
            } else {
                let key = path.join(" ");
                size.entry(key).and_modify(|counter| *counter += file_size).or_insert(file_size);
            }
        }
    }

    Ok(size) 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let input = "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k";
        let p = 1;

        let result = solve(input, p).unwrap_or_else(|err| {
            println!("failed to unwrap test result, {err}");
            return -1
        });

        assert_eq!(95437, result);
    }

    #[test]
    fn p2() {
        let input = "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k";

        let p = 2;
        let result = solve(input, p).unwrap_or_else(|err| {
            println!("failed to unwrap test result, {err}");
            return -1
        });

        assert_eq!(24933642, result);
    }
}
