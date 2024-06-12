use std::collections::HashMap;

pub fn solve_word_search<'a>(grid: &'a Vec<Vec<char>>, words: &'a Vec<&'a str>) -> HashMap<&'a str, Option<(usize, usize)>> {
    let mut results = HashMap::new();
    for &word in words {
        let mut found = false;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if divide_and_conquer_search(grid, word, 0, i as isize, j as isize) {
                    results.insert(word, Some((i, j)));
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
        if !found {
            results.insert(word, None);
        }
    }
    results
}

fn divide_and_conquer_search(grid: &Vec<Vec<char>>, word: &str, index: usize, x: isize, y: isize) -> bool {
    if index == word.len() {
        return true;
    }
    if x < 0 || x >= grid.len() as isize || y < 0 || y >= grid[0].len() as isize {
        return false;
    }
    if grid[x as usize][y as usize] != word.chars().nth(index).unwrap() {
        return false;
    }
    
    let directions = vec![
        (0, 1), (1, 0), (0, -1), (-1, 0), 
        (1, 1), (-1, -1), (1, -1), (-1, 1)
    ];
    
    for &(dx, dy) in &directions {
        let new_x = x + dx;
        let new_y = y + dy;
        if divide_and_conquer_search(grid, word, index + 1, new_x, new_y) {
            return true;
        }
    }
    
    false
}