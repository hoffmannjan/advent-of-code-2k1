use std::fs;

struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32
}

impl Position {
    fn new() -> Position {
        Position { horizontal: 0, depth: 0, aim: 0 }
    }

    pub fn value(&self) -> (i32, i32) {
        (self.horizontal, self.depth)
    }

    fn increase_horizontal(&mut self, num: i32) {
        self.horizontal += num;
    }

    fn increase_depth(&mut self, num: i32) {
        self.depth += num;
    }

    fn increase_aim(&mut self, num: i32) {
        self.aim += num;
    }

    fn decrease_aim(&mut self, num: i32) {
        self.aim -= num;
    }

    fn parse_line(&mut self, line: &str) {
        let (name, value) = line.split_once(" ").unwrap(); 
        let value_i32 = value.parse::<i32>().unwrap();

        match name.as_ref() {
            "forward" => {
                self.increase_horizontal(value_i32);
                self.increase_depth(self.aim * value_i32);
            }
            "down" => {
                self.increase_aim(value_i32);
            }
            "up" => { 
                self.decrease_aim(value_i32);
            }
            _ => {}
        }
    }
}

fn main() {
    let csv_file_path = "./data.csv";
    let data = fs::read_to_string(csv_file_path).expect("Unable to read file");

    let mut position = Position::new();

    data.lines().for_each(|line| { position.parse_line(&line) });

    let (horizontal, depth) = position.value();

    println!("horizontal: {}, depth: {}, multiplied: {}", horizontal, depth, horizontal * depth);
}
