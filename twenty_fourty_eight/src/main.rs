use std::io;
use rand::Rng;

const NUM_ROWS: usize = 4;
const NUM_COLS: usize = 4;

fn main() {
    let mut grid = vec![vec![0; NUM_COLS]; NUM_ROWS];
    let mut score: usize = 0;

    add_random_to_grid(&mut grid);
    add_random_to_grid(&mut grid);
    //grid[0][0] = 2;
    //grid[1][0] = 2;
    loop {
        print_grid(&grid, &score);
        match get_user_input() {
            'w' => {
                println!("Move Up");
                
                for i in 0..4 {
                    let col: Vec<i32> = grid.iter().map(|x| x[i]).collect();
                    let new_col = move_lower(&col, &mut score);
                    for j in 0..NUM_ROWS {
                        grid[j][i] = new_col[j];
                    }
                }
            }
            's' => {
                println!("Move Down");
                for i in 0..4 {
                    move_down(&mut grid, i, &mut score);
                }
            }
            'a' => {
                println!("Move Left");
                for row in &mut grid {
                    *row = move_lower(row, &mut score).clone();
                }
            }
            'd' => {
                println!("Move Right");
                for row in &mut grid {
                    move_right(row, &mut score);
                }
            }
            _ => {
                println!("Invalid Input!!");
            }
        };

        if !add_random_to_grid(&mut grid){
            break;
        }
    }

    println!("\n\nGAME OVER!!\nTotal Score : {}", score);
}

fn add_random_to_grid(grid: &mut Vec<Vec<i32>>) -> bool {
    let mut has_space = false;
    for row in &grid[..] {
        if row.iter().filter(|&x| *x == 0).count() > 0 {
            has_space = true;
            break;
        } 
    }

    if !has_space {
        return false;
    }

    loop {
        let row: usize = rand::thread_rng().gen_range(0, NUM_ROWS);
        let col: usize = rand::thread_rng().gen_range(0, NUM_COLS);
        //println!("Row : {} Col : {}", row, col);

        if grid[row][col] == 0 {
            grid[row][col] = 2;
            break;
        }
    }

    true
}

fn print_grid(grid: &Vec<Vec<i32>>, score: &usize) {
    for row in grid {
        println!("{}", "---------------------------------");
        for col in row {
            if *col > 0 {
                print!("|{:^7}", col);
            } else {
                print!("|{:^7}", " ");
            }
        }
        println!("|");
    }

    println!("{}", "---------------------------------");
    
    println!("\nScore : {}", score);
}

fn get_user_input() -> char {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    buf.chars().next().unwrap()
}

fn move_lower(row: &Vec<i32>, score: &mut usize) -> Vec<i32> {
    let mut filtered_row: Vec<i32> = row.iter().filter(|&x| *x != 0).copied().collect();
    let mut result = Vec::<i32>::new();
    //println!("{:?}", filtered_row);
    while filtered_row.len() > 0 {
        let mut value = filtered_row.remove(0);
        let mut second_value = 0;
        if filtered_row.len() > 0 {
            second_value = filtered_row.remove(0);
        }

        if second_value > 0 && second_value == value {
            value += second_value;
            *score += value as usize;
            filtered_row.insert(0, value);
            filtered_row = [result.as_slice(), filtered_row.as_slice()].concat();
            result.clear();
        } else {
            if second_value > 0 {
                filtered_row.insert(0, second_value);
            }

            result.push(value);
        }
      }

    while result.len() < NUM_COLS {
        result.push(0);
    }

    //println!("{:?}", result);
    result
}

fn move_left_old(row: &mut Vec<i32>, score: &mut usize) {
    for i in 0..NUM_COLS {
        if row[i] != 0 {
            *score += add_row(row);
            continue;
        }

        let next_num = get_index(row, 1);
        //println!("Found Next Number At Index {}", next_num);
        if next_num == -1 {
            *score += add_row(row);
            break;
        }
        
        swap_items(i, next_num as usize, row);
        *score += add_row(row);
    }
}

fn add_row(row: &mut Vec<i32>) -> usize {
    let mut row_score = 0;
    for i in (1..NUM_COLS).rev() {
        //println!("{}", row[i]);
        if row[i] != 0 && i > 0 {
            if row[i] == row[i - 1] {
                //println!("Adding {} + {} at indexes {} and {}", row[i], row[i - 1],  i, i - 1);
                row[i - 1] *= 2;
                row_score += row[i - 1];
                row[i] = 0;
            }
        }
    }

    row_score as usize
}

fn move_up(grid: &mut Vec<Vec<i32>>, col: usize, score: &mut usize) {
    for i in 0..NUM_COLS {
        if grid[i][col] != 0 {
            *score +=  add_row_up(grid, col);
            continue;
        }

        let next_num = get_index_up(grid, col);
        //println!("Found Next Number At Index {}", next_num);
        if next_num == -1 {
            *score += add_row_up(grid, col);
            break;
        }
        
        swap_items_col(i, next_num as usize, grid, col);
        *score += add_row_up(grid, col);
    }
}

fn add_row_up(grid: &mut Vec<Vec<i32>>, col: usize) -> usize {
    let mut row_score = 0;
    for i in (1..NUM_COLS).rev() {
        //println!("{}", row[i]);
        if grid[i][col] != 0 && i > 0 {
            if grid[i][col] == grid[i - 1][col] {
                //println!("Adding {} + {} at indexes {} and {}", row[i], row[i - 1],  i, i - 1);
                grid[i - 1][col] *= 2;
                row_score += grid[i - 1][col];
                grid[i][col] = 0;
            }
        }
    }

    row_score as usize
}

fn move_right(row: &mut Vec<i32>, score: &mut usize) {
    for i in (0..NUM_COLS).rev() {
        if row[i] != 0 {
            *score += add_row_right(row);
            continue;
        }

        let next_num = get_index_right(row);
        //println!("Found Next Number At Index {}", next_num);
        if next_num == -1 {
            *score += add_row_right(row);
            break;
        }
        
        swap_items(i, next_num as usize, row);
        *score += add_row_right(row);
    }
}

fn add_row_right(row: &mut Vec<i32>) -> usize {
    let mut row_score = 0;
    for i in 0..NUM_COLS - 1 {
        //println!("{}", row[i]);
        if row[i] != 0 && i < 9 {
            if row[i] == row[i + 1] {
                row[i + 1] *= 2;
                row_score += row[i + 1];
                row[i] = 0;
            }
        }
    }

    row_score as usize
}

fn move_down(grid: &mut Vec<Vec<i32>>, col: usize, score: &mut usize) {
    for i in (0..NUM_COLS).rev() {
        if grid[i][col] != 0 {
            *score += add_row_down(grid, col);
            continue;
        }

        let next_num = get_index_down(grid, col);
        //println!("Found Next Number At Index {}", next_num);
        if next_num == -1 {
            *score += add_row_down(grid, col);
            break;
        }
        
        swap_items_col(i, next_num as usize, grid, col);
        *score += add_row_down(grid, col);
    }
}

fn add_row_down(grid: &mut Vec<Vec<i32>>, col: usize) -> usize {
    let mut row_score = 0;
    for i in 0..NUM_COLS - 1 {
        //println!("{}", row[i]);
        if grid[i][col] != 0 && i < 9 {
            if grid[i][col] == grid[i + 1][col] {
                grid[i + 1][col] *= 2;
                row_score += grid[i + 1][col];
                grid[i][col] = 0;
            }
        }
    }

    row_score as usize
}

fn swap_items(a: usize, b: usize, row: &mut Vec<i32>) {
    let temp = row[a];
    row[a] = row[b];
    row[b] = temp;
}

fn swap_items_col(a: usize, b: usize, grid: &mut Vec<Vec<i32>>, col: usize) {
    let temp = grid[a][col];
    grid[a][col] = grid[b][col];
    grid[b][col] = temp;
}

fn get_index(row: &mut Vec<i32>, direction: i32) -> i32 {
    for i in 0..((NUM_COLS - 1) as i32) {
        if row[i as usize] == 0 && i < ((NUM_COLS - 1) as i32){
            let mut j = i + direction;

            loop {
                if j <= (NUM_COLS as i32) && row[j as usize] > 0 {
                    return j;
                }

                j += direction;
                //println!("j = {} {}", j, row[j as usize]);
                if j == (NUM_COLS as i32) {
                    return -1;
                }
            }
        }
    }

    -1
}

fn get_index_up(grid: &mut Vec<Vec<i32>>, col: usize) -> i32 {
    for i in 0..((NUM_COLS - 1) as i32) {
        if grid[i as usize][col] == 0 && i < ((NUM_COLS - 1) as i32){
            let mut j = i + 1;

            loop {
                if j <= (NUM_COLS as i32) && grid[j as usize][col] > 0 {
                    return j;
                }

                j += 1;
                //println!("j = {} {}", j, row[j as usize]);
                if j == (NUM_COLS as i32) {
                    return -1;
                }
            }
        }
    }

    -1
}

fn get_index_right(row: &mut Vec<i32>) -> i32 {
    for i in (0..(NUM_COLS) as i32).rev() {
        //println!("Move Right {}", i);
        if row[i as usize] == 0 && i > 0{
            let mut j: i32 = i - 1;
            //println!("{}", j.testing());

            loop {
                if j >= 0 && row[j as usize] > 0 {
                    return j;
                }

                j -= 1;
                //println!("j = {} {}", j, row[j as usize]);
                if j == -1{
                    return -1;
                }
            }
        }
    }

    -1
}

fn get_index_down(grid: &mut Vec<Vec<i32>>, col: usize) -> i32 {
    for i in (0..(NUM_COLS) as i32).rev() {
        //println!("Move Right {}", i);
        if grid[i as usize][col] == 0 && i > 0{
            let mut j: i32 = i - 1;
            //println!("{}", j.testing());

            loop {
                if j >= 0 && grid[j as usize][col] > 0 {
                    return j;
                }

                j -= 1;
                //println!("j = {} {}", j, row[j as usize]);
                if j == -1{
                    return -1;
                }
            }
        }
    }

    -1
}