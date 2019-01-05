extern crate rand;

use rand::Rng;
use std::{thread, time};

static ALIVE:u8  = 1;
static DEAD:u8  = 0;

fn main() {

let mut board: Vec<Vec<u8>>;

let ten_millis = time::Duration::from_millis(200);


board = create_board(10,10);
board[4][1]=ALIVE;
board[4][2]=ALIVE;
board[4][3]=ALIVE;

for i in 1..10 {
print_board(&board);
board = update_board(board);
thread::sleep(ten_millis);

}
}

#[test]
fn board_creation() {
let mut board: Vec<Vec<u8>>;
board = create_board(40,40);

for (i, x) in board.iter_mut().enumerate() {
    for (j, y) in x.iter_mut().enumerate() {
        assert_eq!(y as &u8, &0u8);
        *y = 1u8;
        //board[x as &usize][y as &usize]= 1;
    }
}

for (i, x) in board.iter().enumerate() {
    for (j, mut y) in x.iter().enumerate() {
        assert_eq!(y as &u8, &1u8);
        //y = &1u8;
        //board[x as &usize][y as &usize]= 1;
    }
}

//assert!(true);
//assert_eq!(1,1);
}

#[test]
fn alive_cell_checking(){
let mut board: Vec<Vec<u8>>;
board = create_board(10,10);

let startx:usize = 1;
let starty:usize = 1;

board[startx][starty] = ALIVE;

// Cecking that 1,1 is set to live.
assert_eq!(board[startx][starty], ALIVE);

//No other cells set. Assuming the cell dies.
println!("**** First check ****");
assert!(check_cell(&board, startx,starty)==DEAD);

// Setting one cell in the vincinity to live. Assuming dead cell.
board[startx-1][starty-1] = ALIVE;
println!("**** Second check ****");
assert!(check_cell(&board, startx,starty)==DEAD);

// Setting two cells in the vincinity to live. Assuming living cell.
board[startx-1][starty] = ALIVE;
println!("**** Third check ****");
assert!(check_cell(&board, startx,starty)==ALIVE);

// Setting three cells in the vincinity to live. Assuming living cell.
board[startx-1][starty+1] = ALIVE;
println!("**** Forth check ****");
assert!(check_cell(&board, startx,starty)==ALIVE);

// Setting four cells in the vincinity to live. Assuming living dead.
board[startx][starty-1] = ALIVE;
println!("**** Fifth check ****");
assert!(check_cell(&board, startx,starty)==DEAD);

// Setting four cells in the vincinity to live. Assuming living dead.
board[startx][starty+1] = ALIVE;
println!("**** Sixth check ****");
assert!(check_cell(&board, startx,starty)==DEAD);

// Setting four cells in the vincinity to live. Assuming living dead.
board[startx+1][starty-1] = ALIVE;
println!("**** Seventh check ****");
assert!(check_cell(&board, startx,starty)==DEAD);

// Setting four cells in the vincinity to live. Assuming living dead.
board[startx+1][starty] = ALIVE;
println!("**** Eight check ****");
assert!(check_cell(&board, startx,starty)==DEAD);

// Setting four cells in the vincinity to live. Assuming living dead.
board[startx+1][starty+1] = ALIVE;
println!("**** Ninth check ****");
assert!(check_cell(&board, startx,starty)==DEAD);

}

#[test]
fn dead_cell_checking(){
let mut board: Vec<Vec<u8>>;
board = create_board(10,10);

let startx:usize = 1;
let starty:usize = 1;

board[startx][starty] = DEAD;

// Cecking that 1,1 is set to live.
assert_eq!(board[startx][starty], DEAD);

//No other cells set. Assuming the cell dies.
println!("**** First check ****");
assert!(check_cell(&board, startx,starty)==DEAD);

// Setting one cell in the vincinity to live. Assuming dead cell.
board[startx-1][starty-1] = ALIVE;
println!("**** Second check ****");
assert!(check_cell(&board, startx,starty)==DEAD);

// Setting two cells in the vincinity to live. Assuming DEAD cell.
board[startx-1][starty] = ALIVE;
println!("**** Third check ****");
assert!(check_cell(&board, startx,starty)==DEAD);

// Setting three cells in the vincinity to live. Assuming living cell.
board[startx-1][starty+1] = ALIVE;
println!("**** Forth check ****");
assert!(check_cell(&board, startx,starty)==ALIVE);

// Setting four cells in the vincinity to live. Assuming dead cell.
board[startx+1][starty+1] = ALIVE;
println!("**** Fifth check ****");
assert!(check_cell(&board, startx,starty)==DEAD);

// Setting four cells in the vincinity to live. Assuming living dead.
board[startx][starty+1] = ALIVE;
println!("**** Sixth check ****");
assert!(check_cell(&board, startx,starty)==DEAD);

// Setting four cells in the vincinity to live. Assuming living dead.
board[startx+1][starty-1] = ALIVE;
println!("**** Seventh check ****");
assert!(check_cell(&board, startx,starty)==DEAD);

// Setting four cells in the vincinity to live. Assuming living dead.
board[startx+1][starty] = ALIVE;
println!("**** Eight check ****");
assert!(check_cell(&board, startx,starty)==DEAD);

// Setting four cells in the vincinity to live. Assuming living dead.
board[startx+1][starty+1] = ALIVE;
println!("**** Ninth check ****");
assert!(check_cell(&board, startx,starty)==DEAD);

}


fn create_board(w:usize, h:usize) -> Vec<Vec<u8>> { 
    let mut board = vec![vec![0u8; w]; h];
    return board;
}

fn print_board(board:&Vec<Vec<u8>>) {
//let mut rownum: u8 = 0;



for (rownum, x) in board.iter().enumerate().rev() {
    print!("{:?} ", rownum);
    if rownum <  10  {print!("__");}
    if rownum >= 10 {print!("_");}
    
    for (colnum, y) in x.iter().enumerate() {
        match board[rownum][colnum] {
        1 => print!("00_"),
        _ => print!("  _"),
    }
    }
    println!("");
    //rownum +=1;
}

print!("    ");
for (i, item) in board.iter().enumerate() {
    print!("{:?}", i);
    if i <10  {print!("__");}
    if i >=10 {print!("_");}
}
println!("");
}

fn initiate_board(mut board: Vec<Vec<u8>>, chance_of_life:f64) -> Vec<Vec<u8>> {
    let mut i = 0;
    let mut rng = rand::thread_rng();
    for row in board.iter_mut() {
        for val in row.iter_mut() {
                        let mut chance = rng.gen_range(0.0, 100.0);
                        if  chance <= chance_of_life
                        {
                            i +=1;
                            //println!("Item {:?}, Chance {:?}", i, chance);
                            *val = 1u8;
                        }
        }
    }
    return board;
}

fn update_board(board: Vec<Vec<u8>>) -> Vec<Vec<u8>> {

    let mut new_board: Vec<Vec<u8>> = create_board(board.len(), board[0].len());
    for (x, row) in board.iter().enumerate() {
        for (y, val) in row.iter().enumerate() {
                      new_board[x][y] = check_cell(&board,x,y);
        }
    }
    
    return new_board;
}

fn check_cell(board: &Vec<Vec<u8>>, x:usize,y:usize) -> u8{
    let mut live_neighbous: u8 = 0;
 
    if x >= board.len() || y >= board[x].len() {
      //  println!("Checking item outside of board.");
        return DEAD;    
    }

    let is_alive: bool = board[x][y] != DEAD;
    

    for check_x in -1i8..2i8 {
        for check_y in -1i8..2i8 {
            
        //    println!("Checking: x:{:?}, y:{:?} Relative from start: ({:?},{:?})",  (check_x + (x as i8)) as i8, y as i8 + check_y, check_x, check_y);
            
            if check_x != 0i8 || check_y != 0i8 {           

            if  (x as i8 + check_x >= 0) 
                && ((x as i8 + check_x) as usize) < board.len() 
                && (y as i8 + check_y >= 0) 
                && ((y as i8 + check_y) as usize) < board[x].len()
            {
          //      println!("Valid item, checking content.");
              
                if board[(x as i8 + check_x) as usize][(y as i8 + check_y) as usize] == ALIVE 
                {
                    live_neighbous +=1;
            //        println!("It was alive.");

                } else {
                   // println!("It was dead.");
                }
            } else {
                // println!("Item invalid.");
            }
            // println!("Checked x: {:?} y: {:?}. Counting {:?} live neighbours.", x as i8 + check_x, y as i8 + check_y, live_neighbous);
            // println!("");    
            } else {
               // println!("Centered item. Did not check.");
            }       
        }
    }

    match is_alive {
        true => match live_neighbous {
            2 | 3 =>    return ALIVE,
            _ =>         return DEAD,
        },

        false => match live_neighbous {
            3 =>        return ALIVE,
            _ =>        return DEAD,
        },
    }
}