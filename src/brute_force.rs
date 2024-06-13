pub fn solve_brute_force(grid: &Vec<Vec<char>>, words: Vec<&str>) -> Vec<((usize, usize), (usize, usize))> {
    let mut moves = Vec::new();
    let directions = vec![
        (0, 1),  // right
        (1, 0),  // down
        (0, -1), // left
        (-1, 0), // up
        (1, 1),  // down-right
        (1, -1), // down-left
        (-1, 1), // up-right
        (-1, -1), // up-left
    ];
    
    let n = grid.len();
    let m = grid[0].len();
    
    for i in 0..n {
        for j in 0..m {
            for word in &words {
                if word.starts_with(grid[i][j]) {
                    for direction in &directions {
                        check_brute_force(grid, &mut words.clone(), i as isize, j as isize, *direction, &mut moves);
                    }
                }
            }
        }
    }
    
    moves
}

fn check_brute_force(
    grid: &Vec<Vec<char>>,
    words: &mut Vec<&str>,
    mut i: isize,
    mut j: isize,
    direction: (isize, isize),
    moves: &mut Vec<((usize, usize), (usize, usize))>,
) {
    let n = grid.len() as isize;
    let m = grid[0].len() as isize;
    let (start_i, start_j) = (i as usize, j as usize);
    let mut substring = String::new();
    
    while i >= 0 && i < n && j >= 0 && j < m {
        substring.push(grid[i as usize][j as usize]);
        if let Some(pos) = words.iter().position(|&word| word == substring) {
            moves.push(((start_i, start_j), (i as usize, j as usize)));
            words.remove(pos);
            break;
        }
        
        i += direction.0;
        j += direction.1;
    }
}