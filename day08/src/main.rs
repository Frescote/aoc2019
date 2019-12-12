use std::fs::File;
use std::io::prelude::*;

fn main() {
    print_image(merge_layers(split_layers((25, 6), to_array(get_input_string()))), (25, 6));
}

fn print_image(image: Vec<char>, size: (i32, i32)){
    for i in 0..size.1 {
        for j in 0..size.0 {
            //print!("{}\n", j + size.0 * i)
            print!("{}", if image[(j + size.0 * i) as usize] == '0' { ' ' } else { '0' });
        }
        print!("\n");
    }
}

fn get_result(layer: Vec<char>) -> i32 {
    let one_count = layer.iter().fold(0, |acc, c| if *c == '1' { acc + 1 } else { acc });
    let two_count = layer.iter().fold(0, |acc, c| if *c == '2' { acc + 1 } else { acc });
    one_count * two_count
}

fn find_layer_with_fewest_zero(layers: Vec<Vec<char>>) -> Vec<char> {
    layers
        .iter()
        .map(|layer| (layer, layer.iter().fold(0, |acc, c| if *c == '0' { acc + 1 } else { acc })))
        .min_by_key(|x| x.1)
        .expect("found min")
        .0
        .clone()
}
 
fn merge_layers(layers: Vec<Vec<char>>) -> Vec<char> {
    (0..layers[0].len())
        .into_iter()
        .map(|pos| merge_pixels(layers.iter().map(|layer| layer[pos]).collect()))
        .collect()
}

fn merge_pixels(pixels: Vec<char>) -> char {
    pixels
        .iter()
        .fold('2', |acc, pixel| if acc == '2' { pixel.clone() } else { acc })
}

fn split_layers(size: (i32, i32), input: Vec<char>) -> Vec<Vec<char>> {
    let layer_length = size.0 * size.1;
    input.chunks(layer_length as usize)
        .map(|chunk| chunk.to_vec())
        .collect()
}

fn to_array(input: String) -> Vec<char> {
    input.chars().collect()
}

fn get_input_string() -> String {
    let mut file = File::open("/Users/aalvarez/Projects/DiaCriativo/AdventOfCode2019/inputs/day08.txt")
        .expect("Couldn´t open file");

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}