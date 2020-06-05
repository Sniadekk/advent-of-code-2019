mod code;
use code::Code;

pub struct Computer {
    program: Vec<i16>,
    curr: usize,
}

impl Computer {
    pub fn new(program: &Vec<i16>) -> Self {
        Computer {
            program: program.clone(),
            curr: 0,
        }
    }

    pub fn load_program(&mut self, program: &Vec<i16>) {
        self.program = program.clone();
        self.curr = 0;
    }

    fn get_chunk(&self) -> (i16, i16, i16) {
        return (
            self.program[self.program[self.curr + 1]],
            self.program[self.program[self.curr + 2]],
            self.program[self.curr + 3],
        );
    }

    pub fn run_program(&mut self) -> usize {
        while let Some(codes) = self.scan_code() {
            let code = Code::new(&codes);

            //            match code {
            //                1 | 2 => {
            //                    let (a, b, replace_position) = self.get_chunk();
            //                    let new_value = if code == 1 { a + b } else { a * b };
            //                    self.program[replace_position] = new_value;
            //                    self.curr += 4;
            //                }
            //
            //                99 => break,
            //                _ => {
            //                    panic!("Unknown op code");
            //                }
            //            }
        }
        self.program[0]
    }

    fn scan_code(&mut self) -> Option<usize> {
        let code = self.program.get(self.curr);
        code.cloned()
    }
}
