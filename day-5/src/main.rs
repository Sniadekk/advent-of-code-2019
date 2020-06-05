use common::intcode_computer::Computer;
use std::fs::read_to_string;

fn main() -> std::io::Result<()> {
    // read gravity assist program from input file
    let mut program: Vec<usize> = read_to_string("input.txt")?
        .split(",")
        .map(|num| {
            num.parse::<usize>()
                .expect("Unexpected string that is not a number")
        })
        .collect();

    program[1] = 12;
    program[2] = 2;

    let mut computer = Computer::new(&program);
    println!("ANSWER TO THE FIRST PART: {}", computer.run_program());

    for noun in 0..100 {
        for verb in 0..100 {
            program[1] = noun;
            program[2] = verb;
            computer.load_program(&program);
            if computer.run_program() == 1969_0720 {
                println!("ANSWER TO THE SECOND PART: {}", 100 * noun + verb);
            }
        }
    }

    Ok(())
}
