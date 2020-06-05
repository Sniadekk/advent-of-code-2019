use common::intcode_computer::Computer;
use std::fs::read_to_string;

fn main() -> std::io::Result<()> {
    // read gravity assist program from input file
    let mut program: Vec<i16> = read_to_string("input.txt")?
        .split(",")
        .map(|num| {
            num.parse::<i16>()
                .expect("Unexpected string that is not a number")
        })
        .collect();

    let mut computer = Computer::new(&program);
    println!("ANSWER TO THE FIRST PART: {}", computer.run_program());

    Ok(())
}
