pub mod brute_force;
pub mod divide_and_conquer;
use std::time::Instant;

enum AlgorithmType {
    BruteForce,
    DivideAndConquer,
}
fn main() {
    test_performance(brute_force_impl, AlgorithmType::BruteForce);
    test_performance(divide_and_conquer_impl, AlgorithmType::DivideAndConquer);
}

fn brute_force_impl() { 
    let grid = vec![
        vec!['c', 'a', 't', 'x', 'y', 'z', 'w', 'o', 'r', 'd'],
        vec!['d', 'o', 'g', 'b', 'r', 'u', 'i', 'n', 'e', 'p'],
        vec!['r', 'a', 't', 'g', 'e', 't', 'o', 'n', 'i', 'a'],
        vec!['m', 'o', 'u', 's', 'e', 'r', 'v', 'i', 'c', 'e'],
        vec!['p', 'r', 'o', 'g', 'r', 'a', 'm', 'm', 'i', 'n'],
        vec!['h', 'e', 'l', 'l', 'o', 'a', 'n', 'd', 'c', 'o'],
        vec!['j', 'a', 'v', 'a', 's', 'c', 'r', 'i', 'p', 't'],
        vec!['p', 'y', 't', 'h', 'o', 'n', 'a', 'l', 'i', 'c'],
        vec!['k', 'o', 't', 'l', 'i', 'n', 'b', 'r', 'a', 'v'],
        vec!['c', 'p', 'l', 'u', 's', 'p', 'l', 'u', 's', 'p'],
    ];

    let words = vec![
        "cat", "dog", "rat", "mouse", "service", "programming", 
        "hello", "javascript", "python", "kotlin", "cplusplus"
    ];

    let moves = brute_force::solve_brute_force(&grid, words);
    
    // for mov in moves {
    //     println!("from {:?} -> to {:?}", mov.0, mov.1);
    // }
}

fn divide_and_conquer_impl() { 
    let grid = vec![
        vec!['c', 'a', 't', 'x', 'y', 'z', 'w', 'o', 'r', 'd'],
        vec!['d', 'o', 'g', 'b', 'r', 'u', 'i', 'n', 'e', 'p'],
        vec!['r', 'a', 't', 'g', 'e', 't', 'o', 'n', 'i', 'a'],
        vec!['m', 'o', 'u', 's', 'e', 'r', 'v', 'i', 'c', 'e'],
        vec!['p', 'r', 'o', 'g', 'r', 'a', 'm', 'm', 'i', 'n'],
        vec!['h', 'e', 'l', 'l', 'o', 'a', 'n', 'd', 'c', 'o'],
        vec!['j', 'a', 'v', 'a', 's', 'c', 'r', 'i', 'p', 't'],
        vec!['p', 'y', 't', 'h', 'o', 'n', 'a', 'l', 'i', 'c'],
        vec!['k', 'o', 't', 'l', 'i', 'n', 'b', 'r', 'a', 'v'],
        vec!['c', 'p', 'l', 'u', 's', 'p', 'l', 'u', 's', 'p'],
    ];

    let words = vec![
        "cat", "dog", "rat", "mouse", "service", "programming", 
        "hello", "javascript", "python", "kotlin", "cplusplus"
    ];

    let moves = divide_and_conquer::solve_word_search(&grid, &words);
    
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
