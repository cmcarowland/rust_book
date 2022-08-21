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
                    let col: Vec<i32> = grid.iter().map(|x| x[i]).collect();
                    let new_col = move_upper(&col, &mut score);
                    for j in 0..NUM_ROWS {
                        grid[j][i] = new_col[j];
                    }
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
                    *row = move_upper(row, &mut score).clone();
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
    if !is_space_available(grid) {
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

fn is_space_available(grid: &mut Vec<Vec<i32>>) -> bool {
    for row in &grid[..] {
        if row.iter().filter(|&x| *x == 0).count() > 0 {
            return true;
        } 
    }

    false
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

fn move_upper(row: &Vec<i32>, score: &mut usize) -> Vec<i32> {
    let mut filtered_row: Vec<i32> = row.iter().filter(|&x| *x != 0).copied().collect();
    let mut result = Vec::<i32>::new();
    //println!("{:?}", filtered_row);
    while filtered_row.len() > 0 {
        let mut value = filtered_row.pop().unwrap();
        let mut second_value = 0;
        if filtered_row.len() > 0 {
            second_value = filtered_row.pop().unwrap();
        }

        if second_value > 0 && second_value == value {
            value += second_value;
            *score += value as usize;
            filtered_row.push(value);
            filtered_row = [filtered_row.as_slice(), result.as_slice()].concat();
            result.clear();
        } else {
            if second_value > 0 {
                filtered_row.push(second_value);
            }

            result.insert(0, value);
        }
      }

    while result.len() < NUM_COLS {
        result.insert(0, 0);
    }

    //println!("{:?}", result);
    result
}