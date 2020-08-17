fn main() {
    let program: Program = read_to_string(program_path)
        .expect("You provided invalid program file!")
        .split(",")
        .map(|num| {
            num.parse::<i32>()
                .expect("Unexpected string that is not a number")
        })
        .collect();
}
