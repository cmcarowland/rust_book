use std::io;
use rand::Rng;

fn main() {
    let mut grid = vec![vec![0; 10]; 4];
    add_random_to_grid(&mut grid);

    loop {
        print_grid(&grid);
        match get_user_input() {
            'w' => {
                println!("Move Up");
            }
            's' => {
                println!("Move Down");
            }
            'a' => {
                println!("Move Left");
                for row in &mut grid {
                    move_left(row);
                }
            }
            'd' => {
                println!("Move Right");
                for row in &mut grid {
                    move_right(row);
                }
            }
            _ => {
                println!("Invalid Input!!");
            }
        };

        add_random_to_grid(&mut grid);
    }
}

fn add_random_to_grid(grid: &mut Vec<Vec<i32>>) {
    loop {
        let row: usize = rand::thread_rng().gen_range(0, 4);
        let col: usize = rand::thread_rng().gen_range(0, 10);
        println!("Row : {} Col : {}", row, col);

        if grid[row][col] == 0 {
            grid[row][col] = 2;
            break;
        }
    }
}

fn print_grid(grid: &Vec<Vec<i32>>) {
    for row in grid {
        println!("{:?}", row);
    }
}

fn get_user_input() -> char {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    buf.chars().next().unwrap()
}

fn move_left(row: &mut Vec<i32>) {
    for i in 0..9 {
        if row[i] != 0 {
            add_row(row);
            continue;
        }

        let next_num = get_index(row, 1);
        println!("Found Next Number At Index {}", next_num);
        if next_num == -1 {
            add_row(row);
            break;
        }
        
        swap_items(i, next_num as usize, row);
        add_row(row);
    }
}

fn add_row(row: &mut Vec<i32>) -> i32 {
    let mut score = 0;
    for i in (1..9).rev() {
        println!("{}", row[i]);
        if row[i] != 0 && i > 0 {
            if row[i] == row[i - 1] {
                println!("Adding {} + {} at indexes {} and {}", row[i], row[i - 1],  i, i - 1);
                row[i - 1] *= 2;
                score += row[i - 1];
                row[i] = 0;
            }
        }
    }

    score
}

fn move_right(row: &mut Vec<i32>) {
    for i in (0..10).rev() {
        if row[i] != 0 {
            add_row(row);
            continue;
        }

        let next_num = get_index_right(row);
        println!("Found Next Number At Index {}", next_num);
        if next_num == -1 {
            add_row(row);
            break;
        }
        
        swap_items(i, next_num as usize, row);
        add_row_right(row);
    }
}

fn add_row_right(row: &mut Vec<i32>) -> i32 {
    let mut score = 0;
    for i in 0..9 {
        println!("{}", row[i]);
        if row[i] != 0 && i > 0 {
            if row[i] == row[i + 1] {
                row[i + 1] *= 2;
                score += row[i + 1];
                row[i] = 0;
            }
        }
    }

    score
}

fn swap_items(a: usize, b: usize, row: &mut Vec<i32>) {
    let temp = row[a];
    row[a] = row[b];
    row[b] = temp;
}

fn get_index(row: &mut Vec<i32>, direction: i32) -> i32 {
    for i in 0..9 as i32 {
        if row[i as usize] == 0 {
            let mut j = i + direction;

            loop {
                if j <= 9 && row[j as usize] > 0 {
                    return j;
                }

                j += direction;
                //println!("j = {} {}", j, row[j as usize]);
                if j == 10 {
                    return -1;
                }
            }
        }
    }

    -1
}

fn get_index_right(row: &mut Vec<i32>) -> i32 {
    for i in (0..10).rev() {
        println!("Move Right {}", i);
        if row[i as usize] == 0 {
            let mut j = i - 1;

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