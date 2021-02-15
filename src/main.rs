extern crate rand;
extern crate num;

use std::sync::{Mutex, Arc};
use num::Integer;
use rand::Rng;
use std::thread;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
static GLOBAL_THREAD_COUNT: AtomicUsize = AtomicUsize::new(0);

fn main() {

    let num_runs: i128 = std::env::args().nth(1).expect("please specify the a valid number (< 2^127 - 1) for the number of runs to complete").parse::<i128>().unwrap();
    // let num_runs: i128 = std::i32::MAX as i128;
    let max_number: i128 = std::i128::MAX;
    // let mut co_primes: i128 = 0;
    // let co_primes = Arc::new(Mutex::new(0));

    let threads: i32 = 100;


    let co_primes = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // second number is the number of threads
    for _ in 0..threads {
        GLOBAL_THREAD_COUNT.fetch_add(1, Ordering::SeqCst);
        let co_primes = Arc::clone(&co_primes);
        let handle = thread::spawn(move || {
            let mut these_co_primes = 0;
            for _x in 0..num_runs/((threads) as i128) {
                let mut rng = rand::thread_rng();

                let num_1: i128 = rng.gen_range(1..max_number);
                let num_2: i128 = rng.gen_range(1..max_number);

                if num_1.gcd(&num_2) == 1 {
                    these_co_primes += 1;
                }
            }
            let mut num = co_primes.lock().unwrap();
            *num += these_co_primes;

            GLOBAL_THREAD_COUNT.fetch_sub(1, Ordering::SeqCst);
        });
        handles.push(handle);
    }

    while GLOBAL_THREAD_COUNT.load(Ordering::SeqCst) != 0 {
        thread::sleep(Duration::from_millis(1));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let c = *co_primes.lock().unwrap();
    println!("number of co primes found: {}", c);


    let pi: f64 = (6.0/(c as f64 / num_runs as f64)).sqrt();
    println!("pi ≅ {:.}", pi);



}
