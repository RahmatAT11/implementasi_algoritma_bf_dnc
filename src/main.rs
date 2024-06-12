pub mod brute_force;

fn main() {
    brute_force_impl();
}

fn brute_force_impl() {
    let grid = vec![
        vec!['c', 'a', 't'],
        vec!['d', 'o', 'g'],
        vec!['r', 'a', 't'],
    ];
    let words = vec!["cat", "rat", "aoa"];
    
    let moves = brute_force::solve_brute_force(&grid, words);
    
    for mov in moves {
        println!("from {:?} -> to {:?}", mov.0, mov.1);
    }
}
