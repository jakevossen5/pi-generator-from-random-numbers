extern crate rand;
extern crate num;

use std::sync::{Mutex, Arc};
use num::Integer;
use rand::Rng;
use std::thread;
fn main() {

    let num_runs: i128 = std::i128::MAX;
    let max_number: i128 =std::i128::MAX;
    // let mut co_primes: i128 = 0;
    let co_primes = Arc::new(Mutex::new(0));
    let threads: i32 = 4;

    // let (tx, rx) = mpsc::channel();

    for _core in 1..threads {
        let co_primes = Arc::clone(&co_primes);
        thread::spawn( move || {
            for _x in 0..10000 {
                println!("am I even doing anything??");
                let num_1: i128 = rand::thread_rng().gen_range(1, max_number);
                let num_2: i128 = rand::thread_rng().gen_range(1, max_number);
                if num_1.gcd(&num_2) == 1 {
                    let mut num = co_primes.lock().unwrap();
                    *num +=1;
                    println!("{}", (_x as f64/ num_runs as f64) * 100.0)
                        
                }
            }

            //tx.send(co_primes).unwrap();
        });
    }

    
    // for received in rx {
        // println!("Got: {}", received);
        // let co_primes = co_primes + received;
    // }
    // co_primes.lock().unwrap();
    let co_primes = Arc::clone(&co_primes);
    let mut num = co_primes.lock().unwrap();

    println!("Co primes: {}", num);
 
    // co_primes.lock().unwrap();   
    // let pi: f64 = (6.0/(co_primes as f64 / num_runs as f64)).sqrt();
    // println!("pi: {}", pi);

    
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
