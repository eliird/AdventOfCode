use std::fs;

const INPUT_PATH: &str = "./inputs.txt";


fn warp(x: i32, delta: i32, modulo: i32) -> i32 {
    ((x + delta) % modulo + modulo) % modulo
}

fn zero_crossings(x: i32, delta: i32, modulo: i32) -> (i32, i32){
    let value = ((x + delta) % modulo + modulo) % modulo;
    let mut temp = x;
    if delta < 0 {
        temp = modulo - x;
    }
    let mut zero_crossing_count = (temp + delta.abs()) / modulo;
    if x == 0 && delta < 0{
        zero_crossing_count -= 1;
    }
    (value, zero_crossing_count.abs())
}


fn get_password(inputs: &Vec<String>) -> i32{
    let mut counter: i32 = 50;
    let mut zero_count: i32 = 0;
    for input in inputs{
        let direction: String = input.chars()
            .take_while(|c| !c.is_ascii_digit())
            .collect();

        let value: String = input.chars()
            .skip_while(|c| !c.is_ascii_digit())
            .collect();
        let value: i32 = value.parse().unwrap();

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
    zero_count
}

fn get_password_2(inputs: &Vec<String>) -> i32 {
    let mut counter = 50;
    let mut password = 0;
    for input in inputs{
        let zero_crossing_count ;
        let direction: String = input.chars()
            .take_while(|c| !c.is_ascii_digit())
            .collect();

        let value: String = input.chars()
            .skip_while(|c| !c.is_ascii_digit())
            .collect();
        let value: i32 = value.parse().unwrap();

        match direction.chars().next(){
            Some('L') => {
                // rotate left and count zero
                let delta = -1 * value;
                (counter, zero_crossing_count) = zero_crossings(counter, delta, 100);
            },
            Some('R') => {
                //rotate right and count 0
                let delta = value;
                (counter, zero_crossing_count) = zero_crossings(counter, delta, 100);
            },
            _ => {
                panic!("Sanity Check: Input should not contian anything other than L or R");
            }
        };

        password += zero_crossing_count;

    }


    password
}

fn main() {

    let inputs: Vec<String> = fs::read_to_string(INPUT_PATH).expect("unable to read the file")
        .lines()
        .map(|s| s.to_string())
        .collect();

    let zero_count = get_password(&inputs);
    println!("Password is {}", zero_count);

    let zero_count = get_password_2(&inputs);
    // println!("Password using method 2 is {}", zero_count);
    // println!("{:>5} | {:>5} | {:>5}", "x", "delta", "count");
    // for i in -20..20 {
    //     let x = 0;
    //     let modulo = 10;
    //     let (_temp, zero_count) = zero_crossings(x, i, modulo);
    //     println!("{:>5} | {:>5} | {:>5}", x, i, zero_count);
    // }

}
