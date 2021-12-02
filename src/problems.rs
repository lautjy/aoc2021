pub mod prob01 {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    pub fn load_prob01_data() -> Vec<i32> {
        let filename = "data_problem01.txt";
        // Open the file in read-only mode (ignoring errors).
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut vec: Vec<i32> = Vec::new();
        // Read the file line by line using the lines() iterator from std::io::BufRead.
        for (_index, line) in reader.lines().enumerate() {
            let line = line.unwrap(); // Ignore errors.
            let my_int = line.parse::<i32>().unwrap();
            vec.push(my_int);
        }
        return vec;
    }

    pub fn prob1a(vec: &Vec<i32>) {
        let _result = vec //[0..10]
            .windows(2)
            .filter(|x| x[0] < x[1])
            .collect::<Vec<_>>();
        // Inspect is .. for inspecting the values I guess?
        // .inspect(|w| {1 if w[0] < w[1] else 0})
        // .collect::<Vec<_>>();
        println!("{}", _result.len());
    }

    pub fn prob1b(vec: &Vec<i32>) {
        let _result2 = vec //[0..10]
            .windows(4)
            // Turbofish type definition required!
            .filter(|x| x[0..3].iter().sum::<i32>() < x[1..4].iter().sum())
            // .filter(|x| x[0] + x[1] + x[2] < x[1] + x[2] + x[3])
            .collect::<Vec<_>>();
        println!("{}", _result2.len());
    }
}

pub mod prob02 {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    pub struct Prob02Line {
        dir: String,
        amount: i32,
    }
    pub fn load_prob02_data() -> Vec<Prob02Line> {
        let filename = "data_problem02.txt";
        // Open the file in read-only mode (ignoring errors).
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut vec: Vec<Prob02Line> = Vec::new();
        // Read the file line by line using the lines() iterator from std::io::BufRead.
        for (_index, line) in reader.lines().enumerate() {
            let line = line.unwrap(); // Ignore errors.
            let line_split = line.trim().split(" ").collect::<Vec<&str>>();
            let direction = line_split[0];
            let amount = line_split[1].parse::<i32>().unwrap();
            vec.push(Prob02Line {
                dir: direction.to_string(),
                amount: amount,
            });
        }
        return vec;
    }
    pub fn prob02a(vec: &Vec<Prob02Line>) {
        let mut depth: i32 = 0;
        let mut dist: i32 = 0;
        for item in vec.iter() {
            match item.dir.as_str() {
                "forward" => dist = dist + item.amount,
                "up" => depth = depth - item.amount,
                "down" => depth = depth + item.amount,
                _ => println!("WAAAAAAAwhat {}", item.dir),
            }
        }
        println!(
            "Problem 2a: depth {} dist {} -> total= {}",
            depth,
            dist,
            depth * dist
        );
    }

    /// down X increases your aim by X units.
    /// up X decreases your aim by X units.
    /// forward X does two things:
    ///   It increases your horizontal position by X units.
    ///   It increases your depth by your aim multiplied by X.
    pub fn prob02b(vec: &Vec<Prob02Line>) {
        let mut depth: i32 = 0;
        let mut dist: i32 = 0;
        let mut aim: i32 = 0;
        for item in vec.iter() {
            match item.dir.as_str() {
                "forward" => {
                    dist = dist + item.amount;
                    depth = depth + aim * item.amount;
                }
                "up" => aim = aim - item.amount,
                "down" => aim = aim + item.amount,
                _ => println!("WAAAAAAAwhat {}", item.dir),
            }
        }
        println!(
            "Problem 2a: depth {} dist {} aim {} -> total= {}",
            depth,
            dist,
            aim,
            depth * dist
        );
    }
}
