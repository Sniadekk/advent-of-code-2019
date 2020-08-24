use std::fs::read_to_string;
use std::slice::Chunks;

const LAYER_WIDTH: usize = 25;
const LAYER_HEIGHT: usize = 6;

type Rows<'a> = &'a [&'a [u32]];
#[derive(Debug)]
struct Layer<'a> {
    rows: Rows<'a>,
}

impl<'a> Layer<'a> {
    pub fn new(rows: Rows<'a>) -> Self {
        Self { rows }
    }

    pub fn count(&self, number: u32) -> usize {
        self.rows
            .iter()
            .map(|row| row.iter().filter(|p| p == &&number).count())
            .sum()
    }
}

fn get_input() -> Vec<u32> {
    read_to_string("input.txt")
        .expect("You need to provide some input")
        .chars()
        .map(|c| c.to_digit(10).expect("Bad character in your input!"))
        .collect()
}

fn main() {
    let input = get_input();

    let rows: Vec<&[u32]> = input.chunks(LAYER_WIDTH).collect();
    let layers: Vec<Layer> = rows.chunks(LAYER_HEIGHT).map(Layer::new).collect();
    let layer = layers
        .iter()
        .fold(None, |first: Option<&Layer>, second| {
            if let Some(layer) = first {
                if layer.count(0) < second.count(0) {
                    return Some(layer);
                } else {
                    return Some(second);
                }
            } else {
                return Some(second);
            }
        })
        .expect("There should be a layer with fewest number of  zeros!");

    println!("First part: {}", layer.count(1) * layer.count(2));
}
