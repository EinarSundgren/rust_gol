extern crate rand;

use rand::Rng;

static alive:u8  = 1;
static dead:u8  = 0;

fn main() {

let mut board: Vec<Vec<u8>>;
 
 board = initiate_board(create_board(40,40), 40.0);
 
 loop {
 match check_cell(&board, 20,20) {
     1=> {println!("Will be alive next round.");
     break;},
     _=> println!("Will be dead next round."),
 }
 }

print_board(&board);

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

print_board(&board);

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

board[1][1] = alive;
assert!(check_cell(&board, 1,1)==dead);


board[0][0] = 1u8;
assert!(check_cell(&board, 1,1)==dead);

board[0][1] = 1u8;
assert!(check_cell(&board, 1,1)==dead);

board[0][2] = 1u8;
assert!(check_cell(&board, 1,1)==alive);

}

#[test]
fn dead_cell_checking(){
let mut board: Vec<Vec<u8>>;
board = create_board(10,10);

board[1][1] = dead;

assert!(check_cell(&board, 1,1)==dead);

board[0][0] = alive;
assert!(check_cell(&board, 1,1)==dead);

board[0][1] = alive;
assert!(check_cell(&board, 1,1)==alive);

board[0][2] = alive;
//assert!(check_cell(&board, 1,1)==dead);

}



fn create_board(w:usize, h:usize) -> Vec<Vec<u8>> { 
    let mut board = vec![vec![0u8; w]; h];
    return board;
}

fn print_board(board:&Vec<Vec<u8>>) {
let mut rownum: u8 = 0;

print!("___");
for (i, item) in board.iter().enumerate() {
    print!("{:?}", i);
    if i <10  {print!("__");}
    if i >=10 {print!("_");}
}
println!("");

for row in board.iter() {
    print!("{:?} ", rownum);
      if rownum <10  {print!("__");}
    if rownum >=10 {print!("_");}
    for val in row.iter() {
    match val {
    1 => print!("00_"),
   // 2 => print!(" "),
    _ => print!("  _"),
    }
    }
    println!("");
    rownum +=1;
}
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

fn check_cell(board: &Vec<Vec<u8>>, x:usize,y:usize) -> u8{
    let mut live_neighbous: u8 = 0;
 
    if x >= board.len() || y >= board[x].len() {
        println!("Checking item outside of board.");
        return dead;    
    }

    let is_alive: bool = board[x][y] < 0;
    

    for check_x in -1i8..2 {
        for check_y in -1i8..2 {
            
            // println!("Checking: x:{:?}, y:{:?}", x as i8 + check_x, y as i8 + check_y);
            if x as i8 + check_x >= 0 
                && ((x as i8 + check_x) as usize) < board.len() 
                && y as i8 + check_y >= 0 
                && ((y as i8 + check_y) as usize) < board[x].len() 
            {
              //  println!("Valid item, checking content.");
              
                if board[(x as i8 + check_x) as usize][(y as i8 + check_y) as usize] == alive 
                {
                    live_neighbous +=1;
                }
            } else {
                println!("Item outside of bounds.");
            }
            //println!("");            
        }
    }

    match is_alive {
        true => match live_neighbous {
            2 | 3 =>    return alive,
            _ =>        return dead,
        },

        false => match live_neighbous {
            3 =>        return alive,
            _ =>        return dead,
        },
    }


}