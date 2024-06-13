pub mod brute_force;
pub mod divide_and_conquer;
pub mod grid_generator;
use std::{io, time::Instant};

use grid_generator::{grid_gen, words_gen};

enum AlgorithmType {
    BruteForce,
    DivideAndConquer,
}
fn main() {
    println!("Brute Force 4x4 performance");
    test_performance(|| brute_force_impl(4, 4), AlgorithmType::BruteForce);
    println!("Divide and Conquer 4x4 performance");
    test_performance(|| divide_and_conquer_impl(4, 4), AlgorithmType::DivideAndConquer);
    println!("\n");

    println!("Brute Force 10x10 performance");
    test_performance(|| brute_force_impl(10, 10), AlgorithmType::BruteForce);
    println!("Divide and Conquer 10x10 performance");
    test_performance(|| divide_and_conquer_impl(10, 10), AlgorithmType::DivideAndConquer);
    println!("\n");

    println!("Brute Force 20x20 performance");
    test_performance(|| brute_force_impl(20, 20), AlgorithmType::BruteForce);
    println!("Divide and Conquer 20x20 performance");
    test_performance(|| divide_and_conquer_impl(20, 20), AlgorithmType::DivideAndConquer);
    println!("\n");

    let mut s: String = String::new();
    let _input = io::stdin().read_line(&mut s);
}

fn brute_force_impl(grid_long: i32, total_words:i32) { 
    let grid = grid_gen(grid_long);

    let words = words_gen(total_words);

    let _moves = brute_force::solve_brute_force(&grid, words);
    
    // for mov in moves {
    //     println!("from {:?} -> to {:?}", mov.0, mov.1);
    // }
}

fn divide_and_conquer_impl(grid_long: i32, total_words:i32) { 
    let grid = grid_gen(grid_long);

    let words = words_gen(total_words);

    let _moves = divide_and_conquer::solve_word_search(&grid, &words);
    
    // for mov in moves {
    //     println!("from {:?} -> to {:?}", mov.0, mov.1);
    // }
}

fn test_performance<T>(f: T, algo_type: AlgorithmType) where T: FnOnce() -> () {
    let start = Instant::now();
    let algo_type = match algo_type {
        AlgorithmType::BruteForce => "Brute Force",
        AlgorithmType::DivideAndConquer => "Divide and Conquer",
    };
    f();
    let duration = start.elapsed();
    println!("Time elapsed in {} is: {:?}", algo_type, duration);
}
