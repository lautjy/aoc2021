mod problems;

fn main() {
    println!("Hello, world!");

    // P 01
    let vec = problems::prob01::load_prob01_data();
    println!("{}", vec.len());
    println!("Problme 01 a:");
    problems::prob01::prob1a(&vec);
    println!("Problme 01 b:");
    problems::prob01::prob1b(&vec);

    // P 02
    let vec = problems::prob02::load_prob02_data();
    problems::prob02::prob02a(&vec);
    problems::prob02::prob02b(&vec);
}
