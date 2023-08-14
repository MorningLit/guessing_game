use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io;

fn main() {
    let mut v: Vec<i64> = vec![1, 0, 0, -1, 5, 12, 4, 0];
    let (median, mode) = find_median_mode_clone(&mut v);
    println!("{}, {}", median, mode);
    let (median, mode) = find_median_mode(&mut v);
    println!("{}, {}", median, mode);
}

fn find_median_mode(v: &mut Vec<i64>) -> (i64, i64) {
    v.sort();
    let n = v.len();
    let median = v[n / 2];
    let mut hashmap: HashMap<i64, usize> = HashMap::new();
    let mut mode = 0;
    let mut actual_mode = -1;
    for x in v {
        let mut modey = 0;
        if hashmap.get(x).is_none() {
            hashmap.insert(*x, 1);
            modey = 1;
        } else {
            let val = hashmap.get(x).unwrap() + 1;
            hashmap.insert(*x, val);
            modey = val;
        }

        if modey > mode {
            mode = modey;
            actual_mode = *x;
        }
    }

    (median, actual_mode.try_into().unwrap())
}

fn find_median_mode_clone(v: &Vec<i64>) -> (i64, i64) {
    let mut v = v.clone();
    v.sort();
    let n = v.len();
    let median = v[n / 2];
    let mut hashmap: HashMap<i64, usize> = HashMap::new();
    let mut mode = 0;
    let mut actual_mode = -1;
    for x in v {
        let mut modey = 0;
        if hashmap.get(&x).is_none() {
            hashmap.insert(x, 1);
            modey = 1;
        } else {
            let val = hashmap.get(&x).unwrap() + 1;
            hashmap.insert(x, val);
            modey = val;
        }

        if modey > mode {
            mode = modey;
            actual_mode = x;
        }
    }

    (median, actual_mode.try_into().unwrap())
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn to_farenheit(celsius: f64) -> f64 {
    return (celsius * 9.0 / 5.0) + 32.0;
}

fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);

    // if let _n = 0 {
    //    return 0
    // } else if let _n = 1 {
    //     return 1
    // } else {
    //     return fibonacci(n-1) + fibonacci(n-2)
    // }

    // match n {
    //     0 => 0,
    //     1 => 1,
    //     _ => fibonacci(n-1) + fibonacci(n-2),
    // }
}

fn guess() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Step right up and guess the number!");
    println!("The number is between 1 and 100.");
    loop {
        let mut guess = String::new();
        println!("Please input your guess.");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
