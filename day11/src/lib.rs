use std::error::Error;
use std::fs;

pub struct Config {
    file_path: String,
    problem_number: i128,
}
#[derive(Debug, Default, Clone, Copy)]
enum Op {
    #[default]
    Mult,
    Add,
}

#[derive(Debug, Default, Clone)]
struct Monkey {
    items: Vec<i128>,
    operation: (Op, i128, bool),
    test_denom: i128,
    pos_receiver: usize,
    neg_receiver: usize,
}

impl Monkey {
    pub fn new(monkey_desc: &str) -> Result<Self, Box<dyn Error>> {
        let mut new_monkey = Monkey::default();

        let desc_parts: Vec<&str> = monkey_desc.split("\n").map(|p| p.trim()).collect();
        for p in desc_parts {
            let instr: Vec<&str> = p.split(":").collect();
            match instr[0] {
                "Starting items" => new_monkey.parse_starting_items(instr)?,
                "Operation" => new_monkey.parse_operation(instr)?,
                "Test" => new_monkey.parse_test(instr)?,
                "If true" => new_monkey.parse_receiver(instr, true)?,
                "If false" => new_monkey.parse_receiver(instr, false)?,
                _ => continue,
            }
        }

        Ok(new_monkey)
    }

    fn parse_starting_items(&mut self, starting_items: Vec<&str>) -> Result<(), Box<dyn Error>> {
        if starting_items.len() < 2 {
            Err("starting items did not contain list of numbers")?
        }

        self.items = starting_items[1]
            .split(",")
            .map(|item| item.trim().parse::<i128>().unwrap())
            .collect();

        Ok(())
    }

    fn parse_operation(&mut self, operation: Vec<&str>) -> Result<(), Box<dyn Error>> {
        if operation.len() < 2 {
            Err("starting items did not contain list of numbers")?
        }

        let mut op = Op::default();
        let mut o = "*".to_string();
        if operation[1].contains("*") {
            op = Op::Mult;
        } else if operation[1].contains("+") {
            op = Op::Add;
            o = "+".to_string();
        }

        let mut me = bool::default();
        let amt = operation[1]
            .split(&o)
            .map(|i| i.trim())
            .nth(1)
            .unwrap()
            .parse::<i128>()
            .unwrap_or_else(|_| {
                me = true;
                i128::default()
            });

        self.operation = (op, amt, me);
        Ok(())
    }

    fn parse_test(&mut self, test: Vec<&str>) -> Result<(), Box<dyn Error>> {
        if test.len() < 2 {
            Err("starting items did not contain list of numbers")?
        }

        self.test_denom = test[1].split(" ").nth(3).unwrap().parse::<i128>().unwrap();

        Ok(())
    }

    fn parse_receiver(&mut self, receiver: Vec<&str>, pos: bool) -> Result<(), Box<dyn Error>> {
        if receiver.len() < 2 {
            Err("starting items did not contain list of numbers")?
        }

        if pos {
            self.pos_receiver = receiver[1]
                .split(" ")
                .nth(4)
                .unwrap()
                .parse::<usize>()
                .unwrap();
        } else {
            self.neg_receiver = receiver[1]
                .split(" ")
                .nth(4)
                .unwrap()
                .parse::<usize>()
                .unwrap();
        }

        Ok(())
    }
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("must provide input file_path and which problem to solve");
        }
        let file_path = args[1].clone();
        let problem_number = args[2].clone().parse::<i128>().unwrap_or_default();

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

fn solve(contents: &str, problem_number: i128) -> Result<i128, Box<dyn Error>> {
    if problem_number == 1 {
        let mut inspect_cnts = monkey_inspection_counts(contents);
        inspect_cnts.sort();
        let l = inspect_cnts.len();
        Ok(inspect_cnts[l - 1].clone() * inspect_cnts[l - 2].clone())
    } else {
        Ok(i128::default())
    }
}

fn monkey_inspection_counts(contents: &str) -> Vec<i128> {
    let mut monkeys: Vec<Monkey> = contents
        .trim()
        .split("Monkey")
        .map(|md| Monkey::new(md).unwrap())
        .collect();

    monkeys.remove(0);

    let mut inspect_cnts = vec![i128::default(); monkeys.len()];

    for round in 0..10000 {
        println!("round: {}", round);
        for monkey_num in 0..monkeys.len() {
            let items = monkeys[monkey_num].items.clone();
            for mut item in items {
                // monkey inspects item
                inspect_cnts[monkey_num] += 1;
                let mut multiplier = monkeys[monkey_num].operation.1;
                if monkeys[monkey_num].operation.2 {
                    multiplier = item;
                }

                match monkeys[monkey_num].operation.0 {
                    Op::Mult => item *= multiplier,
                    Op::Add => item += multiplier,
                }

                // monkey get bored
                // https://mathworld.wolfram.com/Congruence.html
                // 11. a=b (mod m_1) and a=b (mod m_2)=>a=b (mod [m_1,m_2]), where [m_1,m_2] is the least common multiple.
                item = item % 9699690;

                let p = monkeys[monkey_num].pos_receiver;
                let n = monkeys[monkey_num].neg_receiver;

                // monkey throws item
                if item % monkeys[monkey_num].test_denom == 0 {
                    monkeys[p].items.push(item);
                } else {
                    monkeys[n].items.push(item);
                }
            }
            monkeys[monkey_num].items = Vec::new();
        }
    }

    return inspect_cnts;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn p1() {
        let input = "Monkey 0:
        Starting items: 79, 98
        Operation: new = old * 19
        Test: divisible by 23
          If true: throw to monkey 2
          If false: throw to monkey 3
      
      Monkey 1:
        Starting items: 54, 65, 75, 74
        Operation: new = old + 6
        Test: divisible by 19
          If true: throw to monkey 2
          If false: throw to monkey 0
      
      Monkey 2:
        Starting items: 79, 60, 97
        Operation: new = old * old
        Test: divisible by 13
          If true: throw to monkey 1
          If false: throw to monkey 3
      
      Monkey 3:
        Starting items: 74
        Operation: new = old + 3
        Test: divisible by 17
          If true: throw to monkey 0
          If false: throw to monkey 1";
        let p = 1;

        let result = solve(input, p).unwrap_or_else(|err| {
            println!("failed to unwrap test result, {err}");
            return 1;
        });

        assert_eq!(10605, result);
    }

    // #[test]
    // fn p2() {
    //     let input = "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k";

    //     let p = 2;
    //     let result = solve(input, p).unwrap_or_else(|err| {
    //         println!("failed to unwrap test result, {err}");
    //         return -1;
    //     });

    //     assert_eq!(24933642, result);
    // }
}
