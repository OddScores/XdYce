use rand::prelude::*;
use core::panic;
use std::io;


fn main() {

    let (input, rollarr) = intro();

    loopy(input , rollarr);
}

fn intro() -> (String, [i32;2]) {
    println!("Input format 'XdY' where X is amount of dice and Y is dice type");
    let input: String = getinput();
    let rollarr: [i32; 2] = parse_i(input.clone());
    (input, rollarr)
}


fn diceroll(rollarr: [i32;2]) {
    let mut rng = rand::thread_rng();
    let mut roll: Vec<i32> = Vec::new(); 

    for _ in 0..rollarr[0] {
        roll.push(rng.gen_range(1..=rollarr[1]));
    }

    let sum: i32 = roll.iter().sum();

    println!("Dice Roll: {:?} | Sum = {}", roll, sum);
}


fn loopy(mut input: String, mut rollarr: [i32;2]) {

    diceroll(rollarr);

    loop {
        println!("Re-roll? Enter 'y', Your next roll or 'n' to Exit");

        let inp: String = getinput();

        if inp.trim() == "y" {
        diceroll(rollarr);
        }
        else if inp.trim() == "n" {
            break;
        }
        else {
            input.clear();
            input.push_str(&inp);
            rollarr = parse_i(input.clone());
            diceroll(rollarr);
        }
    }

}



fn getinput() -> String {
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read");
    return input.trim().to_string();
}


fn parse_i(input: String) -> [i32; 2] {
    let parts: Vec<&str> = input.split('d').collect();

    if parts.len() != 2 {
        panic!("Invalid input format");
    }

    let amount: i32 = parts[0].parse().expect("Failed to parse amount");

    let dicetype: i32 = parts[1].parse().expect("Failed to parse Dice Type");

    let worm = [amount, dicetype];

    return worm;
}



