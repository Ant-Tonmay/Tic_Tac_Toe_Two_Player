use std::io;
const ALIEN_X: char = 'X';
const ALIEN_O: char = '0';
const BOARD_SIZE: usize = 8;
type Board = Vec<Vec<char>>;

fn init() -> Board {
    let mut board: Board = vec![vec![' '; BOARD_SIZE]; BOARD_SIZE];
    for i in 0..BOARD_SIZE {
        board[0][i] = '■';
        board[7][i] = '■';
    }
    for i in 1..=6 {
        board[i][0] = '■';
        board[i][7] = '■';
    }
    board[1][3] = '0';
    board[1][4] = '1';
    board[1][5] = '2';
    board[3][1] = '0';
    board[4][1] = '1';
    board[5][1] = '2';
    for i in 0..3 {
        for j in 0..3 {
            board[i + 3][j + 3] = '-';
        }
    }
    return board;
}
fn print_board(board: &Board) {
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            print!("{} ", board[i][j]);
        }
        print!("\n");
    }
}



fn start_game(board: &mut Board) {
    println!("**TO STOP THE GAME - CTRL+C**");

    loop {
        println!("Currnet Board");
        print_board(&board);

        let mut current_alien = ALIEN_X;
       

        current_alien = if current_alien == ALIEN_X {
            ALIEN_O
        } else {
            ALIEN_X
        }
    }
}
fn main() {
    let mut board: Board = init();
    print_board(&board);
    start_game(&mut board);
}
