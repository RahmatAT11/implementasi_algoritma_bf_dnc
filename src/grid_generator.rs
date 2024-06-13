pub fn grid_gen(grid_long: i32) -> Vec<Vec<char>> {
    match grid_long {
        4 => {
            vec![
                vec!['c', 'a', 't', 'f'],
                vec!['b', 'g', 'e', 's'],
                vec!['t', 't', 'a', 'e'],
                vec!['s', 'b', 'o', 't'],
            ]
        },
        10 => {
            vec![
                vec!['c', 'a', 't', 'x', 'y', 'z', 'a', 'b', 'c', 'd'],
                vec!['d', 'o', 'g', 'e', 'f', 'g', 'h', 'i', 'j', 'k'],
                vec!['r', 'a', 't', 'l', 'm', 'n', 'o', 'p', 'q', 'r'],
                vec!['a', 'b', 'c', 'a', 'p', 'p', 'l', 'e', 'x', 'y'],
                vec!['b', 'o', 'o', 'k', 's', 't', 'a', 'c', 'k', 'z'],
                vec!['w', 'a', 't', 'e', 'r', 'm', 'e', 'l', 'o', 'n'],
                vec!['t', 'r', 'e', 'e', 'h', 'o', 'u', 's', 'e', 'a'],
                vec!['n', 'o', 't', 'e', 'b', 'o', 'o', 'k', 'b', 'y'],
                vec!['g', 'a', 'r', 'd', 'e', 'n', 'p', 'a', 'r', 'k'],
                vec!['j', 'u', 'i', 'c', 'e', 'b', 'o', 'x', 'c', 'd'],
            ]
        }
        20 => {
            vec![
                vec!['c', 'a', 't', 'x', 'y', 'z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n'],
                vec!['d', 'o', 'g', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u'],
                vec!['r', 'a', 't', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'a', 'b'],
                vec!['a', 'b', 'c', 'a', 'p', 'p', 'l', 'e', 'x', 'y', 'z', 'b', 'o', 'o', 'k', 's', 'c', 'd', 'e', 'f'],
                vec!['b', 'o', 'o', 'k', 's', 't', 'a', 'c', 'k', 'z', 'w', 'a', 't', 'e', 'r', 'm', 'e', 'l', 'o', 'n'],
                vec!['w', 'a', 't', 'e', 'r', 'm', 'e', 'l', 'o', 'n', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'],
                vec!['t', 'r', 'e', 'e', 'h', 'o', 'u', 's', 'e', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k'],
                vec!['n', 'o', 't', 'e', 'b', 'o', 'o', 'k', 'b', 'y', 't', 'e', 'c', 'o', 'd', 'e', 'a', 'b', 'c', 'd'],
                vec!['g', 'a', 'r', 'd', 'e', 'n', 'p', 'a', 'r', 'k', 'j', 'a', 'v', 'a', 'r', 'u', 's', 't', 'a', 'b'],
                vec!['j', 'u', 'i', 'c', 'e', 'b', 'o', 'x', 'm', 'o', 'v', 'i', 'e', 't', 'i', 'm', 'e', 's', 'x', 'y'],
                vec!['l', 'a', 'p', 't', 'o', 'p', 'a', 'b', 'r', 'u', 'b', 'y', 'o', 'n', 'r', 'a', 'i', 'l', 's', 'z'],
                vec!['s', 'p', 'r', 'i', 'n', 'g', 'b', 'o', 'o', 't', 'c', 'o', 'm', 'p', 'u', 't', 'e', 'r', 'a', 'b'],
                vec!['k', 'e', 'y', 'b', 'o', 'a', 'r', 'd', 'j', 'a', 'v', 'a', 's', 'c', 'r', 'i', 'p', 't', 'e', 'f'],
                vec!['m', 'i', 'c', 'r', 'o', 's', 'o', 'f', 't', 'w', 'o', 'r', 'd', 'e', 'x', 'c', 'e', 'l', 'a', 'b'],
                vec!['g', 'i', 't', 'h', 'u', 'b', 'g', 'i', 't', 'l', 'a', 'b', 'd', 'o', 'c', 'k', 'e', 'r', 'a', 'b'],
                vec!['r', 'e', 'a', 'c', 't', 'n', 'o', 'd', 'e', 'j', 's', 'a', 'n', 'g', 'u', 'l', 'a', 'r', 'a', 'b'],
                vec!['p', 'y', 't', 'h', 'o', 'n', 'r', 'u', 's', 't', 'c', 'o', 'd', 'i', 'n', 'g', 'i', 's', 'f', 'u'],
                vec!['v', 's', 'c', 'o', 'd', 'e', 'p', 'y', 'c', 'o', 'd', 'e', 'r', 'a', 'l', 'g', 'o', 'r', 'i', 't'],
                vec!['f', 'r', 'a', 'm', 'e', 'w', 'o', 'r', 'k', 'r', 'e', 'a', 'c', 't', 'n', 'a', 't', 'i', 'v', 'e'],
                vec!['e', 'x', 'p', 'r', 'e', 's', 's', 'j', 'a', 'v', 'a', 'l', 'i', 'n', 'u', 'x', 'o', 's', 'a', 't'],
            ]
        }
        _ => vec![vec![]]
    }
}

pub fn words_gen(total_words: i32) -> Vec<&'static str> {
    match total_words {
        4 => {
            vec![
                "cat", "bats", "tab", "tags"
            ]
        }
        10 => {
            vec![
                "cat", "dog", "rat", "apple", "stack",   // words that exist in the grid
                "banana", "orange", "grape", "pear", "mango" // words that do not exist in the grid
            ]
        },
        20 => {
            vec![
                "cat", "dog", "rat", "apple", "stack", "watermelon", "treehouse", "notebook", "garden", "movie", // words that exist in the grid
                "banana", "orange", "grape", "pear", "mango", "cherry", "peach", "plum", "berry", "kiwi" // words that do not exist in the grid
            ]
        }
        _ => vec![]
    }
}