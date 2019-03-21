extern crate rand;
extern crate num;

use num::Integer;
use rand::Rng;
use std::thread;
use std::sync::mpsc;
fn main() {

    let num_runs: i128 = std::i128::MAX;
    let max_number: i128 =std::i128::MAX;
    let mut co_primes: i128 = 0;

    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    thread::spawn( move || {
        for _x in 0..num_runs/2 {
            let num_1: i128 = rand::thread_rng().gen_range(1, max_number);
            let num_2: i128 = rand::thread_rng().gen_range(1, max_number);
            if num_1.gcd(&num_2) == 1 {
                co_primes = co_primes + 1;
                println!("{}", (_x as f64/ num_runs as f64) * 100.0)
            }
        }

        tx1.send(co_primes).unwrap();
    });

    thread::spawn( move || {
        for _x in num_runs/2..num_runs {
            let num_1: i128 = rand::thread_rng().gen_range(1, max_number);
            let num_2: i128 = rand::thread_rng().gen_range(1, max_number);
            if num_1.gcd(&num_2) == 1 {
                co_primes = co_primes + 1;
                println!("{}", (_x as f64/ num_runs as f64) * 100.0)
            }
        }

        tx2.send(co_primes).unwrap();
    });

    let co_primes = co_primes + rx1.recv().unwrap();
    let co_primes = co_primes + rx2.recv().unwrap();
    println!("Co primes: {}", co_primes);

    let pi: f64 = (6.0/(co_primes as f64 / num_runs as f64)).sqrt();
    println!("pi: {}", pi);

    
}


// From here: https://doc.rust-lang.org/std/ops/trait.Div.html
// fn gcd(x: i128, y: i128) -> i128 {
//     let mut x = x;
//     let mut y = y;
//     while y != 0 {
//         let t = y;
//         y = x % y;
//         x = t;
//     }
//     return x
// }
