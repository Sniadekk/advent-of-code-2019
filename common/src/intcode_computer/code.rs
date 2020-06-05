pub enum CodeType {
    Addition,
    Multiplication,

}

impl CodeType {

    pub fn new(code: i16) -> Self {
        match code {
            0 =>
        }
    }
}

struct Code {

}

impl Code {
    pub fn new(codes: i8) -> Self {
        let codes: Vec<i8> = [0..5]
            .iter().map(|i| code.get(i).map_or(|c| c, 0))
            .collect();

        let code_type = CodeType::new(codes.get(4).unwrap());
    }
}
