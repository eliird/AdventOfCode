use std::{default, fs};

const INPUT_PATH: &str = "./inputs.txt";

enum Direction{
    Left,
    Right,
}

fn warp(x: i16, delta: i16, modulo: i16) -> i16 {
    ((x + delta) % modulo + modulo) % modulo
}

fn main() {

    let inputs: Vec<String> = fs::read_to_string(INPUT_PATH).expect("unable to read the file")
        .lines()
        .map(|s| s.to_string())
        .collect();

    let mut counter: i16 = 50;
    let mut zero_count: i16 = 0;

    for input in inputs{
        let direction: String = input.chars()
            .take_while(|c| !c.is_ascii_digit())
            .collect();

        let value: String = input.chars()
            .skip_while(|c| !c.is_ascii_digit())
            .collect();
        let value: i16 = value.parse().unwrap();

        match direction.chars().next(){
            Some('L') => {
                // rotate left and count zero
                let delta = -1 * value;
                counter = warp(counter, delta, 100);
            },
            Some('R') => {
                //rotate right and count 0
                let delta = value;
                counter = warp(counter, delta, 100);

            },
            _ => {
                panic!("Sanity Check: Input should not contian anything other than L or R");
            }
        };

        if counter == 0 {
            zero_count += 1;
        }

    }

    println!("Password is {}", zero_count);
}
