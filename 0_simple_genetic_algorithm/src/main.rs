use std::convert::Infallible;
use std::io::Read;
// import commonly used items from the prelude:
use rand::prelude::*;

fn valid_input(s: &str) -> bool {
    for c in s.chars() {
        if !"abcdefghijklmnopqrstuvwxyz ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789'\".".contains(c) {
            return false;
        }
    }
    true
}

fn mutate(agent: &str) -> String {
    let mut new_agent: String = String::from(agent);
    let lucky_gene: usize = thread_rng().gen_range(0..new_agent.len());
    let new_char = "abcdefghijklmnopqrstuvwxyz ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789'\"."
        .chars()
        .choose(&mut thread_rng())
        .unwrap();
    let new_char = &String::from(new_char);

    let mut result: String = String::new();
    // code to replace lucky gene with a the new_char
    for i in 0..new_agent.len() {
        if i == lucky_gene {
            result += new_char;
            continue;
        }
        result += &String::from(new_agent.chars().nth(i).unwrap());
    }

    result
}

fn combine() {}

fn fitness(guess: &str, key: &str) -> f32 {
    if (guess.len() != key.len()) {
        panic!(
            "The size of the guess and key is not equal! keysize:{0} guesssize:{1}",
            key.len(),
            guess.len()
        );
    }

    let mut score = 0;
    for i in 0..key.len() {
        // if key.chars().nth(i) == guess.chars().nth(i) {
        if key.as_bytes()[i] == guess.as_bytes()[i] {
            score += 1;
        }
    }

    (100f32 / key.len() as f32) * score as f32
}

fn ran_word(len: u8) -> String {
    let mut rng = thread_rng();
    let validChars = "abcdefghijklmnopqrstuvwxyz ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789'\".".chars();

    let mut result: String = String::new();

    for i in 0..len {
        result.push(validChars.clone().choose(&mut rng).unwrap());
    }

    result
}

fn main() {
    let mut population_size: u64 = 1000;
    let mut generation_count: u64 = 50;
    println!("Please determine the key : ");
    let mut key = String::new();
    loop {
        key.clear();
        let input = std::io::stdin().read_line(&mut key);
        if key.trim().len() <= 50 && key.trim().len() >= 3 {
            if !valid_input(key.trim()) {
                println!("Invalid characters are used, only alphabet and numbers are allowed!");
            } else {
                break;
            }
        } else {
            println!("The size must be between 3 and 50");
        }
    }

    let mut input: String = String::new();
    loop {
        input.clear();
        println!("Please enter the population size : ");
        let line = std::io::stdin().read_line(&mut input).unwrap();
        input = String::from(input.to_owned().trim());

        match input.parse::<u64>() {
            Ok(a) => {
                if a < 10 {
                    println!(
                        "Too small, population size cannot be smaller than 10 , {}",
                        a
                    );
                } else if a > 5000 {
                    println!("Too big, population size cannot be bigger than 5000");
                } else {
                    population_size = a;
                    break;
                }
            }
            Err(e) => {
                println!("Please input a number! {}", e)
            }
        }
    }

    let mut input: String = String::new();
    loop {
        input.clear();
        println!("Please enter the generation count : ");
        let line = std::io::stdin().read_line(&mut input).unwrap();
        input = String::from(input.to_owned().trim());

        match input.parse::<u64>() {
            Ok(a) => {
                if a < 5 {
                    println!(
                        "Too small, generation count cannot be smaller than 5 , {}",
                        a
                    );
                } else if a > 250 {
                    println!("Too big, generation count cannot be bigger than 250");
                } else {
                    generation_count = a;
                    break;
                }
            }
            Err(e) => {
                println!("Please input a number! {}", e)
            }
        }
    }

    key = String::from(key.to_owned().trim());

    let key_size = key.len();
    let default = (String::from("undefined"), 0.0f32);

    let mut generation: Vec<(String, f32)> = Vec::with_capacity(population_size as usize);
    let mut every_generation: Vec<Vec<(String, f32)>> = Vec::new();

    let mut guess: String;
    let mut percent: f32;

    let mut agents_count: u64 = 0;

    for i in 0..population_size {
        agents_count += 1;
        guess = ran_word(key.len() as u8);
        percent = fitness(&guess, &key);

        generation.push((guess, percent));
    }
    generation.sort_by(|item, other| other.1.partial_cmp(&item.1).unwrap());
    every_generation.push(generation);

    'every_generator: for c in 0..generation_count {
        let mut generation: Vec<(String, f32)> = Vec::with_capacity(population_size as usize);

        for i in 0..(population_size / 10) {
            for f in 0..10 {
                agents_count += 1;
                let x = every_generation
                    .get(c as usize)
                    .unwrap()
                    .get(f as usize)
                    .unwrap();
                if i == 0 {
                    println!(
                        "Generation:{c} ,agent_count:{agents_count}, best_agent:{0}",
                        &x.0
                    );
                }
                if &x.0 == &key {
                    println!("Found the key, breaking...");
                    break 'every_generator;
                }
                guess = String::from(mutate(&x.0));
                percent = fitness(&guess, &key);
                generation.push((guess, percent));
            }
        }
        generation.sort_by(|item, other| other.1.partial_cmp(&item.1).unwrap());
        every_generation.push(generation);
    }

    println!("The key was : \"{key}\"");
}

// We can use random() immediately. It can produce values of many common types:
// let x: u8 = random();

// If we want to be a bit more explicit (and a little more efficient) we can
// make a handle to the thread-local generator:

// let mut rng = thread_rng();

// let x = char::from(rng.gen::<u8>()); // random number in range [0, 1)

// println!("Die roll: {}", rng.gen_range(1..=6));

// arrows_iter.choose(&mut rng).unwrap()
// nums.shuffle(&mut rng);
