use std::fs::read_to_string;

struct Computer {
    program: Vec<usize>,
    curr: usize,
}

impl Computer {
    fn new(program: &Vec<usize>) -> Self {
        Computer {
            program: program.clone(),
            curr: 0,
        }
    }

    fn load_program(&mut self, program: &Vec<usize>) {
        self.program = program.clone();
        self.curr = 0;
    }

    fn get_chunk(&self) -> (usize, usize, usize) {
        return (
            self.program[self.program[self.curr + 1]],
            self.program[self.program[self.curr + 2]],
            self.program[self.curr + 3],
        );
    }

    fn run_program(&mut self) -> usize {
        while let Some(code) = self.scan_code() {
            match code {
                1 | 2 => {
                    let (a, b, replace_position) = self.get_chunk();
                    let new_value = if code == 1 { a + b } else { a * b };
                    self.program[replace_position] = new_value;
                    self.curr += 4;
                }

                99 => break,
                _ => {
                    panic!("Unknown op code");
                }
            }
        }
        self.program[0]
    }

    fn scan_code(&mut self) -> Option<usize> {
        let code = self.program.get(self.curr);
        code.cloned()
    }
}

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
