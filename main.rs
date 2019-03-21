extern crate rand;
extern crate num;

use std::sync::{Mutex, Arc};
use num::Integer;
use rand::Rng;
use std::thread;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::time::Duration;
static GLOBAL_THREAD_COUNT: AtomicUsize = ATOMIC_USIZE_INIT;

fn main() {

    let num_runs: i128 = std::i128::MAX;
    let max_number: i128 = 100000;//std::i128::MAX;
    // let mut co_primes: i128 = 0;
    let co_primes = Arc::new(Mutex::new(0));
    let threads: i32 = 100;


    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // second number is the number of threads
    for i in 0..threads {
        GLOBAL_THREAD_COUNT.fetch_add(1, Ordering::SeqCst);
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _x in 0..num_runs/((threads) as i128) {
                let num_1: i128 = rand::thread_rng().gen_range(1, max_number);
                let num_2: i128 = rand::thread_rng().gen_range(1, max_number);
                if num_1.gcd(&num_2) == 1 {
                    // println!("{}-th thread reporting", i);
                    let mut num = counter.lock().unwrap();
                    *num +=1;
                    // println!("{}", (_x as f64 / ((num_runs as f64) / ((threads) as f64))) * 100.0)
                }
            }
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

    println!("Result: {}", *counter.lock().unwrap());
    let mut c = *counter.lock().unwrap();

    let mut pi: f64 = (6.0/(c as f64 / num_runs as f64)).sqrt();
    println!("pi: {}", pi);


    
}    

    // let (tx, rx) = mpsc::channel();

    // for _core in 1..threads {
    //     let co_primes = Arc::clone(&co_primes);
    //     thread::spawn( move || {
    //         for _x in 0..10000 {
    //             println!("am I even doing anything??");
    //             let num_1: i128 = rand::thread_rng().gen_range(1, max_number);
    //             let num_2: i128 = rand::thread_rng().gen_range(1, max_number);
    //             if num_1.gcd(&num_2) == 1 {
    //                 let mut num = co_primes.lock().unwrap();
    //                 *num +=1;
    //                 println!("{}", (_x as f64/ num_runs as f64) * 100.0)
                        
    //             }
    //         }

    //         //tx.send(co_primes).unwrap();
    //     });
    // }

    
    // // for received in rx {
    //     // println!("Got: {}", received);
    //     // let co_primes = co_primes + received;
    // // }
    // // co_primes.lock().unwrap();
    // let co_primes = Arc::clone(&co_primes);
    // let mut num = co_primes.lock().unwrap();

    // println!("Co primes: {}", num);
 
    // // co_primes.lock().unwrap();   
    // // let pi: f64 = (6.0/(co_primes as f64 / num_runs as f64)).sqrt();
    // // println!("pi: {}", pi);

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
